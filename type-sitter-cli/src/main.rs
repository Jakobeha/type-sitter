use std::fs::{create_dir_all, remove_dir_all};
use std::process::exit;

use clap::Parser;

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

    // Clear output directory.
    // For safety, we only clear rust files.
    // We also don't clear mod.rs, because it allows the user to document and change how outputs are
    // exported as well as re-export content from other crates, and more importantly, it's necessary
    // actually use the outputs unless explicit outputs are given (we don't generate the mod.rs
    // ourselves, we probably should in the future)
    if args.output_dir.exists() {
        if !path_utils::is_dir_of_only_rust_files(&args.output_dir) {
            return Err(Error::OutputDirNotOnlyRustFiles);
        }
        for item in args.output_dir.read_dir().map_err(Error::io("removing old output directory (readdir)"))? {
            let item = item.map_err(Error::io("removing old output directory (readdir item)"))?;
            if item.file_name() != "mod.rs" {
                remove_dir_all(item.path()).map_err(Error::io("removing old output directory (remove item)"))?;
            }
        }
    }
    create_dir_all(&args.output_dir).map_err(Error::io("creating output directory"))?;

    // Get common arg data
    let tree_sitter = match (args.facade.as_ref(), args.use_yak_sitter) {
        (None, false) => tree_sitter(),
        (None, true) => type_sitter_lib_wrapper(),
        (Some(facade), _) => {
            syn::parse_str(facade).map_err(Error::CouldntParseWrapperNamespace)?
        },
    };

    // Process
    let mut had_some_failures = false;
    for item in &args.items {
        if let Err(err) = process::process(item, &args, args.use_yak_sitter, &tree_sitter) {
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