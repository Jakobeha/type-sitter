use std::fs::create_dir_all;
use std::process::exit;

use clap::Parser;

use args::Args;
use errors::Error;
use type_sitter_gen::{tree_sitter, type_sitter_lib, yak_sitter};
use crate::process::reprocess;

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

    // Get common arg data
    let tree_sitter = match (args.facade.as_ref(), args.use_yak_sitter) {
        (None, false) => tree_sitter(),
        (None, true) => yak_sitter(),
        (Some(facade), _) => {
            syn::parse_str(facade).map_err(Error::CouldntParseWrapperNamespace)?
        },
    };
    let type_sitter_lib = type_sitter_lib();

    // Create output directory (e.g. `src/type_sitter`) if necessary
    create_dir_all(&args.output_dir).map_err(Error::io("creating output root directory"))?;

    // Process, overwriting old data if it exists
    let mut had_some_failures = false;
    for item in &args.items {
        if let Err(err) = reprocess(item, &args, args.use_yak_sitter, &tree_sitter, &type_sitter_lib) {
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