use crate::errors;
use crate::errors::Error;
use rust_format::{Formatter, RustFmt};
use std::cell::LazyCell;
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const RUST_FMT: LazyCell<RustFmt> = LazyCell::new(|| RustFmt::new());

pub fn write(path: &Path, contents: impl Display) -> errors::Result<()> {
    let mut file = File::create(path).map_err(Error::io("creating file for generated code"))?;
    write!(file, "{}", contents).map_err(Error::io("writing generated code"))?;
    RUST_FMT.format_file(path)?;
    Ok(())
}

pub fn is_dir_of_only_rust_files(dir: &Path) -> bool {
    dir.read_dir().map_or(false, |mut d| {
        d.all(|f| {
            f.map_or(false, |f| {
                f.metadata().map_or(false, |m| {
                    let path = f.path();
                    (m.is_file() && (has_extension(&path, "rs") || path.ends_with(".DS_Store")))
                        || (m.is_dir() && is_dir_of_only_rust_files(&path))
                })
            })
        })
    })
}

pub fn language_name(path: &Path) -> String {
    let temp = OsString::new();
    path.file_stem()
        .unwrap_or(&temp)
        .to_string_lossy()
        .trim_start_matches("tree-sitter-")
        .replace("-", "_")
}

pub fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some(extension)
}
