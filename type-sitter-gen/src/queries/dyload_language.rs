use std::collections::HashMap;
use std::fs::read_to_string;
use std::mem::transmute;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::RwLock;
use cp_r::CopyOptions;
use lazy_static::lazy_static;
use libloading::Library;
use tempfile::tempdir_in;
use tree_sitter::Language;
use crate::{Error, errors};
use crate::errors::Error;

lazy_static! {
    // We don't want to load the same library multiple times, and we also need to store the Library
    //    so that it doesn't get unloaded.
    static ref LOADED_LANGUAGES: RwLock<HashMap<PathBuf, (Library, Language)>> = HashMap::new();
}

pub fn dyload_language(path: impl AsRef<Path>) -> errors::Result<Language> {
    let path = path.as_ref();
    // Check if the language has already been loaded
    if let Some((_, language)) = LOADED_LANGUAGES.read().unwrap().get(path) {
        return Ok(*language);
    }

    // Load the language
    let (library, language) = dyload_new_language(path)?;
    // Cache the language
    LOADED_LANGUAGES.write().unwrap().insert(path.to_path_buf(), (library, language));
    Ok(language)
}

fn dyload_new_language(path: &Path) -> errors::Result<(Library, Language)> {
    cargo_build_if_needed(path)?;
    let dylib_path = dylib_path(path).ok_or(Error::MissingDylib)?;
    let ts_language_symbol_name = dylib_path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("lib"))
        .ok_or(Error::UnknownTSLanguageSymbolName)?
        .as_bytes();
    // SAFETY: We are running arbitrary code, so...we can't rule out UB or anything really.
    //     However, assuming we have a regular tree-sitter binary, we are not doing anything which
    //     should cause UB since we're only accessing it's TSLanguage pointer and casting to a
    //     tree_sitter::Language.
    unsafe {
        let dylib = Library::new(dylib_path)?;
        let ts_language_symbol = dylib.get::<()>(ts_language_symbol_name)?;
        // SAFETY: assuming this is a regular tree-sitter binary, this should be a valid pointer
        // to a TSLanguage, which is the exact repr of tree_sitter::Language.
        let language = transmute::<_, Language>(ts_language_symbol.into_raw().into_raw());
        Ok((dylib, language))
    }
}

fn cargo_build_if_needed(path: &Path) -> errors::Result<()> {
    if dylib_path(path).is_some() {
        Ok(())
    } else {
        cargo_build(path)
    }
}

fn dylib_path(path: &Path) -> Option<PathBuf> {
    path.join("target/release").read_dir().ok()?.find_map(|entry| {
        entry.ok().map(|e| e.path()).filter(|p| {
            match p.extension().and_then(|e| e.to_str()) {
                Some("so") | Some("dylib") => true,
                _ => false
            }
        })
    })
}

fn cargo_build(path: &Path) -> errors::Result<()> {
    // Copy the directory so we can modify Cargo.toml
    let newdir = tempdir_in(path.parent().unwrap_or(path))?;
    let tmp_path = newdir.path();
    CopyOptions::new().copy_tree(path, tmp_path)?;

    let cargo_toml = tmp_path.join("Cargo.toml");
    std::fs::write(&cargo_toml, read_to_string(&cargo_toml)?.replacen(
        "\n[lib]\n", "\n[lib]\ncrate-type = [\"cdylib\"]\n",
        1
    ))?;

    let output = Command::new("cargo")
        .args(["build", "--release", "--quiet"])
        .current_dir(tmp_path)
        .stdout(Stdio::null())
        .spawn()?
        .wait_with_output()?;
    if !output.status.success() {
        let exit_code = status.code().unwrap_or(255);
        let log = String::from_utf8(output.stderr).unwrap_or_else(|e| e.to_string());
        return Err(Error::BuildFailed { exit_code, log });
    }

    // Copy back built dylib (copies the entire target in case we modified anything else as well)
    CopyOptions::new().copy_tree(tmp_path.join("target"), path.join("target"))?;
    Ok(())
}