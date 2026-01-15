use std::ffi::OsString;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

use rust_format::{Formatter, RustFmt};

use crate::errors;
use crate::errors::Error;

static RUST_FMT: LazyLock<RustFmt> = LazyLock::new(RustFmt::new);

pub fn write(path: &Path, contents: impl Display) -> errors::Result<()> {
    let mut file = File::create(path).map_err(Error::io("creating file for generated code"))?;
    write!(file, "{}", contents).map_err(Error::io("writing generated code"))?;
    RUST_FMT.format_file(path)?;
    Ok(())
}

pub fn is_dir_of_only_rust_files(dir: &Path) -> bool {
    dir.read_dir().is_ok_and(|mut d| {
        d.all(|f| {
            f.is_ok_and(|f| {
                f.metadata().is_ok_and(|m| {
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
