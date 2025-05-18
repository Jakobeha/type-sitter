use crate::queries::{has_extension, language_name};
use crate::Error;
use cc::Build;
use libloading::Library;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs::create_dir_all;
use std::panic::UnwindSafe;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::RwLock;
use tree_sitter::Language;
use tree_sitter_language::LanguageFn;
use walkdir::WalkDir;

// We don't want to load the same library multiple times, and we also need to store the Library
//    so that it doesn't get unloaded.
const LOADED_LANGUAGES: LazyCell<RwLock<HashMap<PathBuf, LanguageFn>>> =
    LazyCell::new(|| RwLock::new(HashMap::new()));

#[cfg(unix)]
const REGISTERED_HANDLER: AtomicBool = AtomicBool::new(false);
const TESTING_LOADED_LANGUAGE: AtomicBool = AtomicBool::new(false);

pub(crate) fn dyload_language(path: impl AsRef<Path>) -> Result<Language, Error> {
    // On Unixes, we can sometimes intercept loading a corrupted language and exit gracefully.
    // On Windows, currently we can only exit gracefully on `panic!`.
    #[cfg(unix)]
    register_crash_handler_if_necessary();

    let path = path.as_ref();
    // Check if the language has already been loaded
    if let Some(language_fn) = LOADED_LANGUAGES.read().unwrap().get(path) {
        return Ok(copy_language_fn(language_fn).into());
    }

    // Load the language
    let language_fn = dyload_new_language(path)?;
    // Cache the language
    LOADED_LANGUAGES
        .write()
        .unwrap()
        .insert(path.to_path_buf(), copy_language_fn(&language_fn));
    Ok(language_fn.into())
}

fn dyload_new_language(path: &Path) -> Result<LanguageFn, Error> {
    let dylib_path = dylib_path(path);
    // Symbol name = language name, and it has type `fn() -> *const ()`
    //     e.g. `tree-sitter-rust => { extern "C" fn tree_sitter_rust() -> *const () ... }`
    // In order to convert this into a language, you need `LanguageFn::from_raw(language_fn).into()`
    let symbol_name =
        CString::new(language_name(path)?).map_err(|_| Error::IllegalTSLanguageSymbolName)?;
    build_dylib_if_needed(path, &dylib_path)?;
    eprintln!("Dynamically loading {}...", symbol_name.to_str().unwrap());
    // SAFETY: We are literally calling into arbitrary code, so...we can't rule out UB. However,
    //     assuming we have a regular tree-sitter binary and not something evil, we are loading and
    //     calling a function which really does return a [`tree_sitter::Language`] and is as safe as
    //     tree-sitter.
    unsafe {
        // We need the canonical path so it doesn't try to load the dylib from random system
        // directories if it can't find it locally (that would lead to some tricky errors...)
        let dylib = Library::new(&dylib_path.canonicalize()?).map_err(Error::LoadDylibFailed)?;
        let language_fn_symbol = dylib
            .get::<unsafe extern "C" fn() -> *const ()>(symbol_name.as_bytes())
            .map_err(Error::LoadDylibSymbolFailed)?;
        let language_fn = LanguageFn::from_raw(*language_fn_symbol);

        // Ensure the library doesn't get unloaded.
        std::mem::forget(dylib);

        // Test the language. If it's a bad pointer, this will segfault or return something weird.
        testing_loaded_language(|| {
            let language = Language::from(copy_language_fn(&language_fn));
            let version = language.abi_version();
            if version < tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION
                || version > tree_sitter::LANGUAGE_VERSION
            {
                return Err(Error::IncompatibleLanguageVersion { version });
            }
            Ok(())
        })?;

        Ok(language_fn)
    }
}

fn build_dylib_if_needed(path: &Path, dylib_path: &Path) -> Result<(), Error> {
    if !dylib_path.exists() {
        build_dylib(path, dylib_path)?;
    }
    if !dylib_path.exists() {
        return Err(Error::MissingDylib);
    }
    Ok(())
}

/// Returns the path of the dynamic library that type-sitter looks for when generating queries.
///
/// # Example
///
/// ```no_run
/// # use std::path::Path;
/// # use type_sitter_gen::dylib_path;
///
/// let path = dylib_path(Path::new("path/to/your/language"));
///
/// #[cfg(target_os = "macos")]
/// assert_eq!(path, Path::new("path/to/your/language/target/c-release-so/libtree-sitter.dylib"));
///
/// #[cfg(target_os = "windows")]
/// assert_eq!(path, Path::new("path/to/your/language/target/c-release-so/libtree-sitter.dll"));
///
/// #[cfg(target_os = "linux")]
/// assert_eq!(path, Path::new("path/to/your/language/target/c-release-so/libtree-sitter.so"));
/// ```

pub fn dylib_path(path: &Path) -> PathBuf {
    let mut path = path.join("target/c-release-so/libtree-sitter");
    if cfg!(target_os = "macos") {
        path.set_extension("dylib");
    } else if cfg!(target_os = "windows") {
        path.set_extension("dll");
    } else {
        path.set_extension("so");
    }
    path
}

fn build_dylib(path: &Path, dylib_path: &Path) -> Result<(), Error> {
    let dylib_dir = dylib_path.parent().unwrap();
    create_dir_all(dylib_dir)?;

    // Derived from tree-sitter-rust's build.rs
    eprintln!("Building {}...", dylib_path.display());
    let src_dir = path.join("src");
    let sources = src_dir
        .read_dir()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| has_extension(p, "c"));
    Build::new()
        // Use the same target, optimization level, etc. as this crate was compiled with
        .host(env!("HOST"))
        .target(env!("TARGET"))
        .opt_level_str(env!("OPT_LEVEL"))
        .debug(env!("DEBUG") == "true")
        // Add tree-sitter flags
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        // Include sources
        .include(&src_dir)
        .files(sources)
        // Set to compile a shared object (doesn't work on macOS and Unix)
        .shared_flag(true)
        .cargo_metadata(false)
        // Compile dylib in dylib dir
        .out_dir(&dylib_dir)
        .try_compile("tree-sitter")?;

    // Even though shared-flag is true it doesn't actually do anything, so we need to manually
    // compile the dylib on macOS and Unix
    eprintln!("Dynamic linking {}...", dylib_path.display());
    let status = if cfg!(target_os = "macos") {
        Command::new("/usr/bin/clang")
            .args(["-dynamiclib", "-o"])
            .arg(&dylib_path)
            .args(find_object_files_in(dylib_dir))
            .status()
            .map_err(Error::LinkDylibCmdFailed)?
    } else if cfg!(target_family = "unix") {
        Command::new("/usr/bin/ld")
            .args(["-shared", "-o"])
            .arg(&dylib_path)
            .args(find_object_files_in(dylib_dir))
            .status()
            .map_err(Error::LinkDylibCmdFailed)?
    } else if cfg!(target_family = "windows") {
        Command::new("link")
            .arg("/DLL")
            .arg(format!("/OUT:{}", dylib_path.display()))
            .args(find_object_files_in(dylib_dir))
            .status()
            .map_err(Error::LinkDylibCmdFailed)?
    } else {
        return Err(Error::LinkDylibUnsupported);
    };
    if !status.success() {
        return Err(Error::LinkDylibFailed {
            exit_status: status,
        });
    }

    Ok(())
}

fn find_object_files_in(dir: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| has_extension(entry.path(), "o"))
        .map(|entry| entry.into_path())
        .map(|path| dunce::canonicalize(&path).unwrap_or(path))
}

fn copy_language_fn(language_fn: &LanguageFn) -> LanguageFn {
    unsafe {
        // SAFETY: `LanguageFn` is `Copy`. The tree-sitter devs forgot to add the impl in the
        // version at the time of this comment (v0.23.0), but it's `repr(transparent)` over
        // something copyable (a function pointer).
        std::mem::transmute_copy(language_fn)
    }
}

#[cfg(unix)]
fn register_crash_handler_if_necessary() {
    if !REGISTERED_HANDLER.swap(true, std::sync::atomic::Ordering::Relaxed) {
        unsafe {
            // SAFETY: Registering the signal handler, it's the correct unchecked type and the
            // `signal` function should be safe to call.
            // The function can be `unsafe` because if it is called, we already got a segfault so
            // "safety" doesn't matter anymore.
            libc::signal(libc::SIGSEGV, loaded_language_crash_handler as libc::size_t);
        }
    }
}

fn testing_loaded_language(
    f: impl FnOnce() -> Result<(), Error> + UnwindSafe,
) -> Result<(), Error> {
    TESTING_LOADED_LANGUAGE.store(true, std::sync::atomic::Ordering::Relaxed);
    let result = std::panic::catch_unwind(f).unwrap_or(Err(Error::CorruptDylib));
    TESTING_LOADED_LANGUAGE.store(false, std::sync::atomic::Ordering::Relaxed);
    result
}

#[cfg(unix)]
unsafe extern "C" fn loaded_language_crash_handler(signal: libc::c_int) {
    unsafe {
        if signal != libc::SIGSEGV
            || !TESTING_LOADED_LANGUAGE.load(std::sync::atomic::Ordering::Relaxed)
        {
            // Forward the signal to the default handler.
            libc::signal(signal, libc::SIG_DFL);
            libc::raise(signal);
        }

        // We convert the signal into a panic, which can be caught, so we can exit gracefully.
        // From https://gist.github.com/ksqsf/b90877ae12c293c933800e3ead11a2e3.
        #[allow(dangling_pointers_from_temporaries)]
        let sigs = std::mem::MaybeUninit::<libc::sigset_t>::uninit().as_mut_ptr();
        libc::sigemptyset(sigs);
        libc::sigaddset(sigs, signal);
        libc::sigprocmask(libc::SIG_UNBLOCK, sigs.cast_const(), std::ptr::null_mut());
    }
    panic!("SIGSEGV!");
}
