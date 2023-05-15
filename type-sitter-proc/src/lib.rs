#![doc = include_str!("../README.md")]

use syn::parse_macro_input;
use generate_nodes_args::GenerateNodesArgs;
use type_sitter_gen::{tree_sitter, type_sitter_lib_wrapper};
use crate::generate_queries_args::GenerateQueriesArgs;

mod generate_nodes_args;
mod generate_queries_args;

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
    let args = parse_macro_input!(item as GenerateNodesArgs);
    type_sitter_gen::generate_nodes(
        &args.path,
        &args.tree_sitter
    ).map(|g| g.collapse()).unwrap_or_else(|err| err.to_compile_error()).into()
}

/// Generate typed wrappers for tree-sitter queries.
///
/// # Parameters
/// - `0`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `1`: the path to the tree-sitter language root. Typically `vendor/tree-sitter-<language>`.
/// - `2`: Path to the crate with the typed node wrappers. Typically `super::nodes`
/// - `3`: Whether to use `tree_sitter` or `type_sitter_lib::tree_sitter_wrapper`
/// - `4` (optional): Path to the `tree_sitter` crate. Defaults to `tree_sitter` (if `use_wrapper`
///   is true) or `type_sitter_lib::tree_sitter_wrapper` (if `use_wrapper` is false), but you can
///   provide a path to your own wrapper.
///
/// # Example
///
/// ```ignore
/// # Doc tests give hygiene errors, so instead we use type-sitter-proc-tests to test these
/// use type_sitter_proc::generate_queries;
///
/// generate_queries!("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", super::typescript_nodes, false);
/// generate_queries!("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", super::rust_nodes, true, type_sitter_lib::tree_sitter_wrapper);
/// ```
#[proc_macro]
pub fn generate_queries(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(item as GenerateQueriesArgs);
    type_sitter_gen::generate_queries(
        &args.path,
        &args.language_path,
        &args.nodes,
        args.use_wrapper.value,
        &args.tree_sitter.unwrap_or_else(|| match args.use_wrapper.value {
            false => tree_sitter(),
            true => type_sitter_lib_wrapper(),
        })
    ).map(|g| g.collapse(&args.nodes)).unwrap_or_else(|err| err.to_compile_error()).into()
}