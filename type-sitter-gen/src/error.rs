use proc_macro2::{Span, TokenStream};
use std::fmt::{Display, Formatter};
use std::process::ExitStatus;
use tree_sitter::QueryError;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseNodeTypes(serde_json::Error),
    BuildDylibFailed(cc::Error),
    MissingDylib,
    LoadDylibFailed(libloading::Error),
    LoadDylibSymbolFailed(libloading::Error),
    CorruptDylib,
    IncompatibleLanguageVersion {
        version: usize,
    },
    LinkDylibUnsupported,
    LinkDylibCmdFailed(std::io::Error),
    LinkDylibFailed {
        exit_status: ExitStatus,
    },
    IllegalTSLanguageSymbolName,
    ParseQuery(QueryError),
    IllegalIdentifier {
        type_desc: String,
        name: String,
        source: syn::Error,
    },
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

impl From<cc::Error> for Error {
    fn from(e: cc::Error) -> Self {
        Error::BuildDylibFailed(e)
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
            Error::BuildDylibFailed(e) => write!(f, "couldn't build tree-sitter language dylib: {}", e),
            Error::MissingDylib => write!(f, "couldn't find tree-sitter language dylib (internal error or fs weirdness)"),
            Error::LoadDylibFailed(e) => write!(f, "couldn't load tree-sitter language dylib: {}", e),
            Error::LoadDylibSymbolFailed(e) => write!(f, "couldn't load tree-sitter language dylib symbol: {}", e),
            Error::CorruptDylib => write!(f, "corrupt tree-sitter language dylib (or we read it wrong, because accessing trying to dereference the language caused a segfault)"),
            Error::IncompatibleLanguageVersion { version } => write!(
                f,
                "incompatible tree-sitter language version: {} (must be within {}..={})",
                version,
                tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION,
                tree_sitter::LANGUAGE_VERSION
            ),
            Error::LinkDylibUnsupported => write!(f, "dynamic linking isn't supported on this platform, use macOS, Unix, Windows, or file a PR"),
            Error::LinkDylibCmdFailed(e) => write!(f, "couldn't link tree-sitter language dylib: {}", e),
            Error::LinkDylibFailed { exit_status} => write!(f, "couldn't link tree-sitter language dylib: exit code {}", exit_status),
            Error::IllegalTSLanguageSymbolName => write!(f, "inferred language symbol name is not a valid UTF-8 string"),
            Error::ParseQuery(e) => write!(f, "Error parsing query: {}", e),
            Error::IllegalIdentifier { type_desc, name, source } => write!(f, "illegal identifier ({}): `{}`\n{}", type_desc, name, source)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(e) => Some(e),
            Error::ParseNodeTypes(e) => Some(e),
            Error::BuildDylibFailed(e) => Some(e),
            Error::MissingDylib => None,
            Error::LoadDylibFailed(e) => Some(e),
            Error::LoadDylibSymbolFailed(e) => Some(e),
            Error::CorruptDylib => None,
            Error::IncompatibleLanguageVersion { .. } => None,
            Error::LinkDylibUnsupported => None,
            Error::LinkDylibCmdFailed(e) => Some(e),
            Error::LinkDylibFailed { .. } => None,
            Error::IllegalTSLanguageSymbolName => None,
            Error::ParseQuery(e) => Some(e),
            Error::IllegalIdentifier { source, .. } => Some(source),
        }
    }
}
