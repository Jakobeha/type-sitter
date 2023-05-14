use proc_macro2::Span;
use syn::{Ident, LitStr, parse_str};
use crate::Error;

/// Create an identifier
macro_rules! ident {
    ($name:expr, $fmt:literal $(, $arg:expr)*) => {
        $crate::mk_syntax::_ident(&$name, || format!($fmt $(, $arg)*))
    };
}
pub(crate) use ident;

pub fn _ident(name: &str, type_desc: impl FnOnce() -> String) -> Result<Ident, Error> {
    match parse_str::<Ident>(name).or_else(|_| parse_str::<Ident>(&format!("r#{}", name))) {
        Ok(ident) => Ok(ident),
        Err(_) => Err(Error::IllegalIdentifier {
            type_desc: type_desc(),
            name: name.to_string()
        })
    }
}

/// Create a literal string
pub fn lit_str(contents: &str) -> LitStr {
    LitStr::new(contents, Span::call_site())
}