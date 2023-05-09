use std::path::Path;
use std::fmt::Display;
use std::fs::File;
use std::ffi::OsString;
use lazy_static::lazy_static;
use rust_format::Formatter;
use crate::{errors, RUST_FMT};

lazy_static! {
    static ref RUST_FMT: RustFmt = RustFmt::new();
}

pub fn write(path: &Path, contents: impl Display) -> errors::Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", contents)?;
    RUST_FMT.format_file(path)?;
    Ok(())
}

pub fn is_dir_of_only_rust_files(dir: &Path) -> bool {
    dir.read_dir().map_or(false, |mut d| {
        d.all(|f| {
            f.map_or(false, |f| {
                f.metadata().map_or(false, |m| {
                    let path = f.path();
                    (m.is_file() && (has_extension(&path, "rs") || path.ends_with(".DS_Store"))) ||
                        (m.is_dir() && is_dir_of_only_rust_files(&path))
                })
            })
        })
    })
}

pub fn language_name(path: &Path) -> String {
    let temp = OsString::new();
    path.file_stem().unwrap_or(&temp).to_string_lossy()
        .trim_start_matches("tree-sitter-")
        .replace("-", "_")
}

pub fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some(extension)
}
