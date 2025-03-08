use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Need at least one input (see --help)")]
    NoInputs,
    #[error("IO error {action}: {source}")]
    IO {
        action: String,
        #[source]
        source: std::io::Error,
    },
    #[error("codegen error: {0}")]
    GeneratingTokens(#[from] type_sitter_gen::Error),
    #[error("codegen formatting (rustfmt) error: {0}")]
    Formatting(#[from] rust_format::Error),
    #[error("in {location}: {source}")]
    Nested {
        location: String,
        #[source]
        source: Box<Error>,
    },
    #[error("couldn't infer input type")]
    CouldntInferInputType,
    #[error("couldn't infer language dir for query(s)")]
    CouldntInferLanguage,
    #[error("couldn't infer output name for node-types")]
    CouldntInferOutputName,
    #[error("language codegen dir we'd overwrite doesn't only contain rust files, so we don't know if its safe to clear")]
    CodegenDirNotOnlyRustFiles,
    #[error("some files failed to process")]
    SomeFailures,
}

#[derive(Debug, Error)]
pub enum InOutPairParseError {
    #[error("missing input")]
    MissingInput,
    #[error("too many '=' separators")]
    TooManySeparators,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn io(action: impl ToString) -> impl FnOnce(std::io::Error) -> Self {
        move |source| Self::IO {
            action: action.to_string(),
            source,
        }
    }

    pub fn nested(self, location: impl AsRef<str>) -> Self {
        Self::Nested {
            location: location.as_ref().to_string(),
            source: Box::new(self),
        }
    }
}
