use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{create_dir, create_dir_all, DirEntry, File, remove_dir_all};
use std::io::Write;
use std::iter::{once, successors};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use rust_format::{Formatter, RustFmt};

use thiserror::Error;
use args::{Args, InOutPair, InputType};
use errors::Error;
use type_sitter_gen::{tree_sitter, type_sitter_lib_wrapper};

mod args;
mod errors;
mod process;
mod path_utils;

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

fn run(args: Args) -> errors::Result<()> {
    if args.items.is_empty() {
        return Err(Error::NoInputs);
    }

    // Clear output directory (for safety, we only clear 1 layer since we only create 1 layer)
    if args.output_dir.exists() {
        if !path_utils::is_dir_of_only_rust_files(&args.output_dir) {
            return Err(Error::OutputDirNotOnlyRustFiles);
        }
        remove_dir_all(&args.output_dir)?;
    }
    create_dir_all(&args.output_dir)?;

    // Get common arg data
    let tree_sitter = match args.use_wrapper {
        false => tree_sitter(),
        true => type_sitter_lib_wrapper()
    };

    // Process
    let mut had_some_failures = false;
    for item in args.items {
        if let Err(err) = process::process(&item, &args, &tree_sitter) {
            eprintln!("Error processing {}: {}", item.input.display(), err);
            had_some_failures = true;
        }
    }

    // Return
    if had_some_failures {
        Err(Error::SomeFailures)
    } else {
        Ok(())
    }
}