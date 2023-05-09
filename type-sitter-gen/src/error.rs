use proc_macro2::{Span, TokenStream};
use std::fmt::{Display, Formatter};
use tree_sitter::QueryError;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseNodeTypes(serde_json::Error),
    BuildFailed { exit_code: i32, log: String },
    MissingDylib,
    LoadDylibFailed(libloading::Error),
    UnknownTSLanguageSymbolName,
    ParseQuery(QueryError),
}

impl Error {
    pub fn into_syn(self) -> syn::Error {
        syn::Error::new(Span::call_site(), self)
    }

    pub fn to_compile_error(self) -> TokenStream {
        self.into_syn().to_compile_error()
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseNodeTypes(e)
    }
}

impl From<libloading::Error> for Error {
    fn from(e: libloading::Error) -> Self {
        Error::LoadDylibFailed(e)
    }
}

impl From<QueryError> for Error {
    fn from(e: QueryError) -> Self {
        Error::ParseQuery(e)
    }
}

impl Into<syn::Error> for Error {
    fn into(self) -> syn::Error {
        self.into_syn()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO error: {}", e),
            Error::ParseNodeTypes(e) => write!(f, "Error parsing node-types: {}", e),
            Error::BuildFailed { exit_code, log } => write!(f, "failed with exit code {}; ***\n{}", exit_code, log),
            Error::MissingDylib => write!(f, "couldn't find tree-sitter language dylib (internal error or fs weirdness)"),
            Error::LoadDylibFailed(e) => write!(f, "couldn't load tree-sitter language dylib: {}", e),
            Error::UnknownTSLanguageSymbolName => write!(f, "couldn't resolve tree-sitter language symbol name"),
            Error::ParseQuery(e) => write!(f, "Error parsing query: {}", e),
        }
    }
}

impl std::error::Error for Error {}