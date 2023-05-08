use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{create_dir, create_dir_all, File, remove_dir_all};
use std::io::Write;
use std::iter::once;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use rust_format::{Formatter, RustFmt};

use thiserror::Error;
use type_sitter_gen::{tree_sitter, type_sitter_lib_wrapper};

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Type of input to process. By default, will infer based on the first input path
    #[arg(short = 't', value_enum)]
    input_type: Option<InputType>,
    /// Output directory. Defaults to "src/type_sitter_generated"
    #[arg(short = 'o', default_value = "src/type_sitter_generated")]
    output_dir: PathBuf,
    /// Generate code which uses tree-sitter-wrapper instead of native tree-sitter
    #[arg(long = "use-wrapper")]
    use_wrapper: bool,
    /// Path to node-types.json, query file, query folder, or tree-sitter language root
    path: PathBuf,
    /// Path to more node-types.json, query file, query folder, or tree-sitter language root
    paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum InputType {
    /// Process node-types.json
    NodeTypes,
    /// Process query file
    QueryFile,
    /// Process query folder
    QueryFolder,
    /// Process tree-sitter language root
    LanguageRoot,
}

#[derive(Debug, Error)]
enum Error {
    #[error("IO error; {0}")]
    IO(#[from] std::io::Error),
    #[error("codegen error; {0}")]
    GeneratingTokens(#[from] type_sitter_gen::Error),
    #[error("codegen formatting (rustfmt) error; {0}")]
    Formatting(#[from] rust_format::Error),
    #[error("in {location}; {source}")]
    Nested { location: String, #[source] source: Box<Error> },
    #[error("couldn't infer input type for path; {}", path.display())]
    CouldntInferInputType { path: PathBuf },
    #[error("output dir doesn't only contain rust files, so we don't know if its safe to clear")]
    OutputDirNotOnlyRustFiles,
    #[error("some files failed to process")]
    SomeFailures,
}

type Result<T> = std::result::Result<T, Error>;

impl InputType {
    fn infer(path: &Path) -> Result<Self> {
        if path_ends_with(path, "node-types.json") {
            Ok(Self::NodeTypes)
        } else if has_extension(path, "scm") {
            Ok(Self::QueryFile)
        } else if path.is_dir() {
            let entries = path.read_dir()?.filter_map(|e| e.ok()).collect::<Vec<_>>();
            if entries.iter().any(|entry| has_extension(&entry.path(), "scm")) {
                Ok(Self::QueryFolder)
            } else if entries.iter().any(|entry| path_ends_with(&entry.path(), "src")) {
                Ok(Self::LanguageRoot)
            } else {
                Err(Error::CouldntInferInputType { path: path.to_path_buf() })
            }
        } else {
            Err(Error::CouldntInferInputType { path: path.to_path_buf() })
        }
    }

    fn output_name(&self, input_path: &Path) -> PathBuf {
        match self {
            InputType::NodeTypes => PathBuf::from("mod.rs"),
            InputType::QueryFile => input_path.file_stem().map_or_else(|| Path::new("/"), Path::new).with_extension("rs"),
            InputType::QueryFolder => PathBuf::new(),
            InputType::LanguageRoot => PathBuf::from(language_name(input_path)),
        }
    }
}

impl Error {
    fn nested(self, location: impl AsRef<str>) -> Self {
        Self::Nested { location: location.as_ref().to_string(), source: Box::new(self) }
    }
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    // Get input type and paths
    let input_type = args.input_type.map_or_else(|| InputType::infer(&args.path), Ok)?;
    let paths = once(args.path).chain(args.paths);

    // Clear output directory (for safety, we only clear 1 layer since we only create 1 layer)
    if args.output_dir.exists() {
        if !is_dir_of_only_rust_files(&args.output_dir) {
            return Err(Error::OutputDirNotOnlyRustFiles);
        }
        remove_dir_all(&args.output_dir)?;
    }
    create_dir_all(&args.output_dir)?;

    // Process
    let mut had_some_failures = false;
    for path in paths {
        if let Err(err) = process(&path, input_type, &args.output_dir, args.use_wrapper) {
            eprintln!("Error processing {}: {}", path.display(), err);
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

fn is_dir_of_only_rust_files(dir: &Path) -> bool {
    dir.read_dir().map_or(false, |mut d| {
        d.all(|f| {
            f.map_or(false, |f| {
                f.metadata().map_or(false, |m| {
                    let path = f.path();
                    (m.is_file() && (has_extension(&path, "rs") || path_ends_with(&path, ".DS_Store"))) ||
                        (m.is_dir() && is_dir_of_only_rust_files(&path))
                })
            })
        })
    })
}

fn process(input_path: &Path, input_type: InputType, output_dir: &Path, use_wrapper: bool) -> Result<()> {
    let output_name = input_type.output_name(input_path);
    let output_path = output_dir.join(output_name);
    let tree_sitter = match use_wrapper {
        false => tree_sitter(),
        true => type_sitter_lib_wrapper()
    };
    match input_type {
        InputType::NodeTypes => {
            write(&output_path, type_sitter_gen::generate_nodes(input_path, &tree_sitter)?)?
        }
        InputType::QueryFile => {
            write(&output_path, type_sitter_gen::generate_queries_from_file(input_path, &tree_sitter)?)?;
        }
        InputType::QueryFolder => {
            for entry in std::fs::read_dir(input_path)? {
                let entry = entry?;
                let input_path = entry.path();
                if has_extension(&input_path, "scm") {
                    process(&input_path, InputType::QueryFile, &output_path, use_wrapper)
                        .map_err(|e| e.nested(entry.file_name().to_string_lossy()))?;
                }
            }
        }
        InputType::LanguageRoot => {
            create_dir(&output_path)?;
            process(&input_path.join("src/node-types.json"), InputType::NodeTypes, &output_path, use_wrapper)
                .map_err(|e| e.nested("node types"))?;
            process(&input_path.join("queries"), InputType::QueryFolder, &output_path, use_wrapper)?;
        }
    }
    Ok(())
}

lazy_static! {
    static ref RUST_FMT: RustFmt = RustFmt::new();
}

fn write(path: &Path, contents: impl Display) -> Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", contents)?;
    RUST_FMT.format_file(path)?;
    Ok(())
}

fn language_name(path: &Path) -> String {
    let temp = OsString::new();
    path.file_stem().unwrap_or(&temp).to_string_lossy()
        .trim_start_matches("tree-sitter-")
        .replace("-", "_")
}

fn path_ends_with(path: &Path, last_component: &str) -> bool {
    path.components().last().map_or(false, |c| c.as_os_str().to_str() == Some(last_component))
}

fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some(extension)
}