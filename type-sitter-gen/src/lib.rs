#![doc = include_str!("../README.md")]

use proc_macro2::TokenStream;
use syn::parse_quote;

pub use crate::error::*;
pub use crate::node_types::*;
pub(crate) use crate::print_ctx::*;
pub use crate::queries::*;
pub use anon_unions::*;

mod error;
mod node_types;
mod mk_syntax;
mod queries;
mod anon_unions;
mod print_ctx;
mod vec_set;

/// = `parse_quote!(::type_sitter)`. The default path to the `type_sitter` crate.
pub fn type_sitter() -> syn::Path {
    parse_quote!(::type_sitter)
}

/// = `parse_quote!(::type_sitter::raw)`. The default path to re-exported `tree-sitter` or
/// `yak-sitter` from `type-sitter`.
pub fn type_sitter_raw() -> syn::Path {
    parse_quote!(::type_sitter::raw)
}

/// = `parse_quote!(::type_sitter_lib)`. The default path to the `type_sitter_lib` crate.
pub fn type_sitter_lib() -> syn::Path {
    parse_quote!(::type_sitter_lib)
}

/// = `parse_quote!(::tree_sitter)`. The default path to the `tree_sitter` crate.
pub fn tree_sitter() -> syn::Path {
    parse_quote!(::tree_sitter)
}

/// = `parse_quote!(::yak_sitter)`. Path to a wrapper which provides convenience functions for
/// tree-sitter nodes at the cost of worse performance.
pub fn yak_sitter() -> syn::Path {
    parse_quote!(::yak_sitter)
}

/// = `parse_quote!(super::nodes)`. The default path to the typed nodes crate from the queries
/// crate. What you will often pass to [generate_queries] and co.
pub fn super_nodes() -> syn::Path {
    parse_quote!(super::nodes)
}

/// Utility to pretty-print tokens.
///
/// It's used by [`GeneratedNodeTokens`] and [`GeneratedQueryTokens`] to provide readable output
/// when using `type-sitter-gen` or `type-sitter-cli`. `type-sitter-proc` doesn't get
/// pretty-printed tokens because the proc-macro returns a [`TokenStream`] instead of a string.
fn pretty_print(tokens: &TokenStream) -> String {
    let str = tokens.to_string();
    syn::parse_file(&str).map(|f| prettyplease::unparse(&f)).unwrap_or_else(|err| {
        eprintln!("Failed to pretty print tokens: {}", err);
        str
    })
}
