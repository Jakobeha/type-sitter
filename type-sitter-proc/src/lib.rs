#![doc = include_str!("../README.md")]

use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

/// Generate typed AST node wrappers for the language specified by the input path, which must point
/// to its `node-types.json`.
///
/// Note: You will need to have the `node-types.json` separate from the actual rust dependency.
/// Simply including the dependency isn't enough, you will either need to either vendor it or store
/// its node-types.json separately.
///
/// # Example
///
/// ```rust
/// use type_sitter_proc::generate_nodes;
///
/// generate_nodes!("vendor/tree-sitter-rust/src/node-types.json");
/// ```
#[proc_macro]
pub fn generate_nodes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    type_sitter_gen::generate_nodes(&PathBuf::from(parse_macro_input!(item as LitStr).value()))
        .unwrap_or_else(|err| err.to_compile_error()).into()
}

/// Generate typed wrappers for queries specified by the input path, which must point to a `.scm`
/// or directory of `.scm` files. If a directory, it will generate submodules for each `.scm`.
///
/// # Example
///
/// ```rust
/// use type_sitter_proc::generate_queries;
///
/// generate_queries!("vendor/tree-sitter-typescript/queries/tags.scm");
/// generate_queries!("vendor/tree-sitter-rust/queries");
/// ```
#[proc_macro]
pub fn generate_queries(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    type_sitter_gen::generate_queries(&PathBuf::from(parse_macro_input!(item as LitStr).value()))
        .unwrap_or_else(|err| err.to_compile_error()).into()
}