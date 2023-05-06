use proc_macro2::Span;
use syn::{Ident, LitStr, parse_str};

/// Create an identifier
macro_rules! ident {
    ($name:expr, $fmt:literal $(, $arg:expr)*) => {
        $crate::mk_syntax::_ident($name, || format!($fmt $(, $arg)*))
    };
}
pub(crate) use ident;

pub fn _ident(name: &str, desc: impl FnOnce() -> String) -> Ident {
    match parse_str::<Ident>(name).or_else(|_| parse_str::<Ident>(&format!("r#{}", name))) {
        Ok(ident) => ident,
        Err(_) => panic!("not an identifier: {} `{}`", desc(), name)
    }
}

/// Create a literal string
pub fn lit_str(contents: &str) -> LitStr {
    LitStr::new(contents, Span::call_site())
}