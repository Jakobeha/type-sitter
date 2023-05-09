use proc_macro2::TokenStream;
use tree_sitter::Query;
use crate::Error;

/// Generate the AST for the typed wrapper of the given query.
///
/// # Parameters
/// - `query`: The query to generate the typed wrapper AST for
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
pub fn print_query(query: Query, nodes: &syn::Path, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    todo!()
}