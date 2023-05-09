#![doc = include_str!("../README.md")]

use syn::parse_macro_input;
use lit_str_and_path::LitStrAndPath;
use crate::lit_str_and_lit_str_and_path_and_path::LitStrAndLitStrAndPathAndPath;

mod lit_str_and_path;
mod lit_str_and_lit_str_and_path_and_path;

/// Generate typed AST node wrappers.
///
/// # Parameters
/// - `0`: Path to the `node-types.json` file of the language.
/// - `1`: Path to the `tree_sitter` crate. Typically either `tree_sitter` or
///   `type_sitter_lib::tree_sitter_wrapper`, but you can provide a path to your own wrapper as
///   well.
///
/// # Note
///
/// You will need to have the `node-types.json` separate from the actual rust dependency. Simply
/// including the dependency isn't enough, you will either need to either vendor it or store its
/// node-types.json separately.
///
/// # Example
///
/// ```ignore
/// # Doc tests give hygiene errors, so instead we use type-sitter-proc-tests to test these
/// use type_sitter_proc::generate_nodes;
///
/// generate_nodes!("../vendor/tree-sitter-json/src/node-types.json", tree_sitter);
/// ```
#[proc_macro]
pub fn generate_nodes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(item as LitStrAndPath);
    type_sitter_gen::generate_nodes(
        &args.lit_str_path_buf,
        &args.path
    ).unwrap_or_else(|err| err.to_compile_error()).into()
}

/// Generate typed wrappers for tree-sitter queries.
///
/// # Parameters
/// - `0`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `1`: the path to the tree-sitter language root. Typically `vendor/tree-sitter-<language>`.
/// - `2`: Path to the crate with the typed node wrappers. Typically `super::nodes`
/// - `3`: Path to the `tree_sitter` crate. Typically either `tree_sitter` or
///   `type_sitter_lib::tree_sitter_wrapper`, but you can provide a path to your own wrapper as
///   well.
///
/// # Example
///
/// ```ignore
/// # Doc tests give hygiene errors, so instead we use type-sitter-proc-tests to test these
/// use type_sitter_proc::generate_queries;
///
/// generate_queries!("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", super::typescript_nodes, tree_sitter);
/// generate_queries!("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", super::rust_nodes, type_sitter_lib::tree_sitter_wrapper);
/// ```
#[proc_macro]
pub fn generate_queries(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(item as LitStrAndLitStrAndPathAndPath);
    type_sitter_gen::generate_queries(
        &args.lit_str_path_buf,
        &args.lit_str_path_buf2,
        &args.path,
        &args.path2
    ).unwrap_or_else(|err| err.to_compile_error()).into()
}