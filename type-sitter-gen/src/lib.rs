#![doc = include_str!("../README.md")]

use syn::parse_quote;

pub use crate::error::Error;
pub use crate::node_types::generate_nodes;
pub use crate::queries::{generate_queries, generate_queries_from_dir, generate_query_from_file};

mod error;
mod names;
mod node_types;
mod mk_syntax;
mod queries;
mod anon_unions;

/// = `parse_quote!(type_sitter)`. The default path to the `type_sitter` crate.
pub fn type_sitter() -> syn::Path {
    parse_quote!(type_sitter)
}

/// = `parse_quote!(type_sitter_lib)`. The default path to the `type_sitter_lib` crate.
pub fn type_sitter_lib() -> syn::Path {
    parse_quote!(type_sitter_lib)
}

/// = `parse_quote!(tree_sitter)`. The default path to the `tree_sitter` crate.
pub fn tree_sitter() -> syn::Path {
    parse_quote!(tree_sitter)
}

/// = `parse_quote!(yak_sitter)`. Path to a wrapper which provides convenience functions for
/// tree-sitter nodes at the cost of worse performance.
pub fn yak_sitter() -> syn::Path {
    parse_quote!(yak_sitter)
}

/// = `parse_quote!(super::nodes)`. The default path to the typed nodes crate from the queries
/// crate. What you will often pass to [generate_queries] and co.
pub fn super_nodes() -> syn::Path {
    parse_quote!(super::nodes)
}
