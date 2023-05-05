use proc_macro2::{Span, TokenStream};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(serde_json::Error)
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
        Error::Parse(e)
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
            Error::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}
