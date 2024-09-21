use std::fs::create_dir_all;
use std::process::exit;

use clap::Parser;

use crate::process::reprocess;
use args::Args;
use errors::Error;

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

    // Create output directory (e.g. `src/type_sitter`) if necessary
    create_dir_all(&args.output_dir).map_err(Error::io("creating output root directory"))?;

    // Process, overwriting old data if it exists
    let mut had_some_failures = false;
    for item in &args.items {
        if let Err(err) = reprocess(item, &args, args.use_yak_sitter) {
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