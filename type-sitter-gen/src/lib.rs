#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use proc_macro2::TokenStream;
use mk_syntax::ident;
use quote::{quote, ToTokens};
use syn::parse_quote;

pub use crate::error::Error;
pub use crate::node_types::generate_nodes;
pub use crate::queries::{generate_queries, generate_queries_from_dir, generate_query_from_file, nodes};


mod error;
mod names;
mod node_types;
mod mk_syntax;
mod queries;

/// = `parse_quote!(tree_sitter)`. The default path to the `tree_sitter` crate.
pub fn tree_sitter() -> syn::Path {
    parse_quote!(tree_sitter)
}

/// = `parse_quote!(type_sitter_lib::tree_sitter_wrapper)`. Path to a wrapper which provides
/// convenience functions for tree-sitter nodes at the cost of worse performance.
pub fn type_sitter_lib_wrapper() -> syn::Path {
    parse_quote!(type_sitter_lib::tree_sitter_wrapper)
}
