mod print;
mod dyload_language;

use std::fs::read_to_string;
use std::path::Path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use tree_sitter::{Language, Query};
use crate::Error;
use crate::mk_syntax::ident;
use crate::queries::dyload_language::dyload_language;
use crate::queries::print::print_query;

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `language`: path to the tree-sitter language module, where the [Language] will be
///   dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_queries, tree_sitter};
///
/// fn main() {
///     use type_sitter_gen::nodes;
///     println!("{}", generate_queries("vendor/tree-sitter-typescript/queries/tags.scm", tree_sitter_typescript::language(), &nodes(), &tree_sitter()).unwrap());
///     println!("{}", generate_queries("vendor/tree-sitter-rust/queries", tree_sitter_rust::language(), &nodes(), &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries(path: impl AsRef<Path>, language: impl AsRef<Path>, nodes: &syn::Path, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let path = path.as_ref();
    if path.is_dir() {
        generate_queries_from_dir(path, language, nodes, tree_sitter)
    } else {
        generate_query_from_file(path, language, nodes, tree_sitter)
    }
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to directory of `.scm` files. This function will
///   generate submodules for each `.scm`.
/// - `language`: path to the tree-sitter language module, where the [Language] will be
///   dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_queries_from_dir, tree_sitter};
///
/// fn main() {
///     use type_sitter_gen::nodes;
///     println!("{}", generate_queries_from_dir("vendor/tree-sitter-rust/queries", tree_sitter_rust::language(), &nodes(), &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries_from_dir(path: impl AsRef<Path>, language: impl AsRef<Path>, nodes: &syn::Path, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let path = path.as_ref();
    let language = language.as_ref();
    let mut queries = TokenStream::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry.metadata()?.is_dir() || has_extension(&entry_path, "scm") {
            let entry_name = entry_path.file_stem().unwrap().to_string_lossy();
            let entry_ident = ident!(&entry_name, "query filename");
            let entry_code = generate_queries(entry_path, language, nodes, tree_sitter)?;
            queries.extend(quote! {
                pub mod #entry_ident {
                    #entry_code
                }
            });
        }
    }
    return Ok(queries);
}

/// Generate source code (tokens) of a wrapper for a single query.
///
/// # Parameters
/// - `path`: Path to the query. Must point to a `.scm` file.
/// - `language`: path to the tree-sitter language module, where the [Language] will be
///   dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_query_from_file, tree_sitter};
///
/// fn main() {
///     use type_sitter_gen::nodes;
///     println!("{}", generate_query_from_file("vendor/tree-sitter-typescript/queries/tags.scm", tree_sitter_typescript::language(), &nodes(), &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_query_from_file(path: impl AsRef<Path>, language: impl AsRef<Path>, nodes: &syn::Path, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let path = path.as_ref();
    let language = dyload_language(language)?;
    let query_str = read_to_string(path)?;
    let query = Query::new(language, &query_str)?;
    print_query(query, nodes, tree_sitter)
}

/// Check if the path has the given extension
fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some(extension)
}

/// = `parse_quote!(super::nodes)`. The default path to the typed nodes crate from the queries crate.
pub fn nodes() -> syn::Path {
    parse_quote!(super::nodes)
}
