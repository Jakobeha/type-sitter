use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Ident, LitStr, parse_str};
use crate::Error;

macro_rules! modularize {
    ($module:ident) => {
        crate::mk_syntax::modularize!($module (use super::*))
    };
    ($module:ident (use $($imports:tt)+)) => {
        if $module.is_empty() {
            quote!()
        } else {
            quote! {
                pub mod $module {
                    #[allow(unused_imports)]
                    use $($imports)+;
                    #$module
                }
            }
        }
    }
}
pub(crate) use modularize;

/// Concatenate strings for documentation
macro_rules! concat_doc {
    ($($term:expr),*) => {{
        let mut result = String::new();
        $(
            result.push_str(&$term);
        )*
        quote::quote! { #result }
    }}
}
pub(crate) use concat_doc;

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

/// Create a literal array
pub fn lit_array<T: ToTokens>(contents: impl Iterator<Item=T>) -> TokenStream {
    quote! { [#(#contents),*] }
}
