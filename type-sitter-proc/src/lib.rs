#![doc = include_str!("../README.md")]

use crate::generate_queries_args::GenerateQueriesArgs;
use generate_nodes_args::GenerateNodesArgs;
use syn::parse_macro_input;

mod generate_nodes_args;
mod generate_queries_args;

/// Generate typed AST node wrappers.
///
/// # Parameters
/// - `0`: Path to the `node-types.json` file of the language, relative to the crate root.
///
/// # Note
///
/// If you included the tree-sitter language as a dependency that is downloaded from the internet,
/// you need to vendor it or at least vendor its `node-types.json`, because this macro needs its
/// path and I don't know how to get it from the crate.
///
/// # Example
///
/// ```ignore
/// # Doc tests give hygiene errors, so instead we use crate `type-sitter` to test these
/// use type_sitter_proc::generate_nodes;
///
/// generate_nodes!("vendor/tree-sitter-json/src/node-types.json");
/// ```
#[proc_macro]
pub fn generate_nodes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(item as GenerateNodesArgs);
    type_sitter_gen::generate_nodes(&args.path)
        .map(|g| g.collapse())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Generate typed wrappers for tree-sitter queries.
///
/// # Parameters
/// - `0`: Path to the queries, relative to the crate root. Must point to a `.scm` or directory of
///   `.scm` files. If a directory, this function will generate submodules for each `.scm`.
/// - `1`: the path to the tree-sitter language root, relative to the crate root. Typically
///   `vendor/tree-sitter-<language>`.
/// - `2`: Path to the crate with the typed node wrappers. Typically `super::nodes` (and then put an
///   invocation of [`generate_nodes!`] at top-level in the sister crate `nodes.rs`).
///
/// # Example
///
/// ```ignore
/// # Doc tests give hygiene errors, so instead we use crate `type-sitter` to test these
/// use type_sitter_proc::generate_queries;
///
/// generate_queries!("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", super::typescript_nodes);
/// generate_queries!("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", super::rust_nodes);
/// ```
#[proc_macro]
pub fn generate_queries(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(item as GenerateQueriesArgs);
    type_sitter_gen::generate_queries(&args.path, &args.language_path, &args.nodes, use_yak_sitter())
        .map(|g| g.collapse(&args.nodes))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[cfg(feature = "yak-sitter")]
fn use_yak_sitter() -> bool {
    true
}

#[cfg(not(feature = "yak-sitter"))]
fn use_yak_sitter() -> bool {
    false
}