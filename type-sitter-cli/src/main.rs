use std::fs::{create_dir_all, remove_dir_all};
use std::process::exit;

use clap::Parser;

use thiserror::Error;
use args::Args;
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
    for item in &args.items {
        if let Err(err) = process::process(item, &args, &tree_sitter) {
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