use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::iter::successors;
use std::fs::DirEntry;
use crate::Error;
use crate::errors::InOutPairParseError;
use crate::path_utils::has_extension;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Type of input to process. By default, will infer based on the first input path
    #[arg(short = 't', long = "type", value_enum)]
    pub input_type: Option<InputType>,
    /// Output directory. Defaults to "src/type_sitter".
    ///
    /// **You must explicitly create the module file for this directory (e.g. `src/type_sitter.rs`
    /// or `src/type_sitter/mod.rs`) and export the submodules. `type-sitter-cli` won't do this
    /// itself, so otherwise, you won't be able to import the generated sources.** Typically you
    /// would do `pub mod <language>;`, but you may choose to export them differently.
    ///
    /// The output module name for each input will be the language name, unless overridden by
    /// specifying the input as a `input/path=output/path` pair.
    #[arg(short = 'o', long = "output-dir", default_value = "src/type_sitter")]
    pub output_dir: PathBuf,
    /// Output name. Don't include the `.rs` extension. Defaults to the language name.
    #[arg(short = 'n', long = "name", default_value = "src/type_sitter_generated")]
    pub output_name: Option<String>,
    /// Language directory. Not used/needed when generating node wrappers. Inferred by default
    #[arg(short = 'l', long = "language")]
    pub language_dir: Option<PathBuf>,
    /// Generate code which for the `yak-sitter` feature. Unset by default, like the feature itself.
    ///
    /// [`yak-sitter`](https://docs.rs/yak-sitter/latest/yak-sitter) is a wrapper for `tree-sitter`
    /// that provides some additional functionality at the cost of performance. It's an optional
    /// feature in `type-sitter`, and code that depends on `type-sitter` with the feature is
    /// incompatible with code that depends on `type-sitter` without it.
    #[arg(long = "use-yak-sitter")]
    pub use_yak_sitter: bool,
    /// Input path(s): must be node-types.json, query file, query folder, or tree-sitter language
    /// roots.
    ///
    /// Instead of just a path, you may specify a pair `input/path=output/path`. This functions the
    /// same as `input/path`, except it creates a module at `output/path` instead of the language
    /// name.
    pub items: Vec<InOutPair>,
}

/// Input path or input path / output path pair
#[derive(Debug, Clone)]
pub struct InOutPair {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InputType {
    /// Process node-types (json)
    NodeTypes,
    /// Process query file (scm) or folder of query files
    Query,
    /// Process tree-sitter language root (process `src/node-types.json` and `queries/`)
    LanguageRoot,
}

impl InputType {
    pub fn infer(path: &Path) -> crate::errors::Result<Self> {
        if has_extension(path, "json") {
            Ok(Self::NodeTypes)
        } else if has_extension(path, "scm") {
            Ok(Self::Query)
        } else if path.is_dir() {
            let entries = Self::read_parent_dir(path, Error::io("inferring input type"))?.collect::<Vec<_>>();
            if entries.iter().any(|e| has_extension(&e.path(), "scm")) {
                Ok(Self::Query)
            } else if entries.iter().any(|e| e.path().ends_with( "src")) {
                Ok(Self::LanguageRoot)
            } else {
                Err(Error::CouldntInferInputType)
            }
        } else {
            Err(Error::CouldntInferInputType)
        }
    }

    pub fn infer_language_dir(&self, input_path: &Path) -> Option<PathBuf> {
        match self {
            InputType::NodeTypes | InputType::Query => {
                successors(input_path.parent(), |p| p.parent())
                    .find(|parent| {
                        Self::read_parent_dir(parent, Error::io("inferring language directory")).ok()
                            .map_or(false, |mut i| i.any(|e| e.path().ends_with( "package.json")))
                    })
                    .map(|p| p.to_path_buf())
            }
            InputType::LanguageRoot => Some(input_path.to_path_buf())
        }
    }

    pub fn is_output_a_dir(&self) -> bool {
        match self {
            InputType::NodeTypes | InputType::Query => false,
            InputType::LanguageRoot => true,
        }
    }

    fn read_parent_dir(path: &Path, io_ctor: impl FnOnce(std::io::Error) -> Error) -> crate::errors::Result<impl Iterator<Item=DirEntry>> {
        Ok(path.read_dir().map_err(io_ctor)?.filter_map(|e| e.ok()))
    }
}

impl FromStr for InOutPair {
    type Err = InOutPairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split('=');
        let input = PathBuf::from(components.next().ok_or(InOutPairParseError::MissingInput)?);
        let output = components.next().map(PathBuf::from);
        if components.next().is_some() {
            return Err(InOutPairParseError::TooManySeparators)
        }
        Ok(Self { input, output })
    }
}
