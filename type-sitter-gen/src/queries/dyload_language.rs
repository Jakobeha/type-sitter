use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::RwLock;
use cc::Build;
use lazy_static::lazy_static;
use libloading::Library;
use tree_sitter::Language;
use walkdir::WalkDir;
use crate::Error;
use crate::queries::{has_extension, language_name};

lazy_static! {
    // We don't want to load the same library multiple times, and we also need to store the Library
    //    so that it doesn't get unloaded.
    static ref LOADED_LANGUAGES: RwLock<HashMap<PathBuf, (Library, Language)>> = RwLock::new(HashMap::new());
}

pub fn dyload_language(path: impl AsRef<Path>) -> Result<Language, Error> {
    let path = path.as_ref();
    // Check if the language has already been loaded
    if let Some((_, language)) = LOADED_LANGUAGES.read().unwrap().get(path) {
        return Ok(language.clone());
    }

    // Load the language
    let (library, language) = dyload_new_language(path)?;
    // Cache the language
    let language_clone = language.clone();
    LOADED_LANGUAGES.write().unwrap().insert(path.to_path_buf(), (library, language));
    Ok(language_clone)
}

fn dyload_new_language(path: &Path) -> Result<(Library, Language), Error> {
    let dylib_path = dylib_path(path);
    // Symbol name = language name, and it has type fn() -> Language
    //     e.g. tree-sitter-rust => { extern "C" fn tree_sitter_rust() -> Language ... }
    let symbol_name = language_name(path)?;
    build_dylib_if_needed(path, &dylib_path)?;
    eprintln!("Dynamically loading {}...", symbol_name);
    // SAFETY: We are literally calling into arbitrary code, so...we can't rule out UB. However,
    //     assuming we have a regular tree-sitter binary and not something evil, we are loading and
    //     calling a function which really does return a [tree_sitter::Language] and is as safe as
    //     tree_sitter.
    unsafe {
        // We need the canonical path so it doesn't try to load the dylib from random system
        // directories if it can't find it locally (that would lead to some tricky errors...)
        let dylib = Library::new(&dylib_path.canonicalize()?).map_err(Error::LoadDylibFailed)?;
        let language_fn = dylib
            .get::<fn() -> Language>(symbol_name.as_bytes())
            .map_err(Error::LoadDylibSymbolFailed)?;
        let language = language_fn();
        Ok((dylib, language))
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

fn dylib_path(path: &Path) -> PathBuf {
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
    let sources = src_dir.read_dir()?
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
            .args(["-dynamiclib", "-undefined", "error", "-o"])
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
    } else {
        return Err(Error::LinkDylibUnsupported);
    };
    if !status.success() {
        return Err(Error::LinkDylibFailed { exit_status: status });
    }

    Ok(())
}

fn find_object_files_in(dir: &Path) -> impl Iterator<Item=PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| has_extension(entry.path(), "o"))
        .map(|entry| entry.into_path())
}
