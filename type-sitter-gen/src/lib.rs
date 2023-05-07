#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;
use mk_syntax::ident;
use quote::{quote, ToTokens};
use syn::parse_quote;

use crate::deserialize_json_array_as_stream::iter_json_array;
pub use crate::error::Error;
use crate::generated_tokens::GeneratedNodeTokens;
use crate::types::NodeType;

/// From https://github.com/serde-rs/json/issues/404#issuecomment-892957228
mod deserialize_json_array_as_stream;
mod error;
mod types;
mod print;
mod names;
mod mk_syntax;
mod generated_tokens;

/// Generate source code (tokens) for typed AST node wrappers.
///
/// # Parameters
/// - `path`: Path to the `node-types.json` file of the language.
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::{generate_nodes, tree_sitter};
/// use syn::parse_quote;
///
/// fn main() {
///     println!("{}", generate_nodes("vendor/tree-sitter-rust/src/node-types.json", &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_nodes(path: impl AsRef<Path>, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let path = normalize(path);
    let node_types = iter_json_array::<NodeType, _>(BufReader::new(File::open(path)?));
    node_types
        .map(|node_type| node_type.map(|x| x.print(tree_sitter)).map_err(Error::from))
        .collect::<Result<GeneratedNodeTokens, _>>()
        .map(GeneratedNodeTokens::collapse)
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::{generate_queries, tree_sitter};
///
/// fn main() {
///     println!("{}", generate_queries("vendor/tree-sitter-typescript/queries/tags.scm", &tree_sitter()).unwrap());
///     println!("{}", generate_queries("vendor/tree-sitter-rust/queries", &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries(path: impl AsRef<Path>, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let path = normalize(path);
    if path.is_dir() {
        generate_queries_from_dir(path, tree_sitter)
    } else {
        generate_queries_from_file(path, tree_sitter)
    }
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to directory of `.scm` files. This function will
///   generate submodules for each `.scm`.
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::{generate_queries_from_dir, tree_sitter};
///
/// fn main() {
///     println!("{}", generate_queries_from_dir("vendor/tree-sitter-rust/queries", &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries_from_dir(path: impl AsRef<Path>, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let path = normalize(path);
    let mut queries = TokenStream::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry.metadata()?.is_dir() || entry_path.ends_with(".scm") {
            let entry_name = entry_path.file_stem().unwrap().to_string_lossy();
            let entry_ident = ident!(&entry_name, "query filename");
            let entry_code = generate_queries(entry_path, tree_sitter)?;
            queries.extend(quote! {
                pub mod #entry_ident {
                    #entry_code
                }
            });
        }
    }
    return Ok(queries);
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` file.
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::{generate_queries_from_file, tree_sitter};
///
/// fn main() {
///     println!("{}", generate_queries_from_file("vendor/tree-sitter-typescript/queries/tags.scm", &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries_from_file(path: impl AsRef<Path>, tree_sitter: &syn::Path) -> Result<TokenStream, Error> {
    let _path = normalize(path);
    todo!("generate queries from file (tree_sitter = {})", tree_sitter.to_token_stream())
}

/// = `parse_quote!(tree_sitter)`. The default path to the `tree_sitter` crate.
pub fn tree_sitter() -> syn::Path {
    parse_quote!(tree_sitter)
}

/// = `parse_quote!(type_sitter_lib::tree_sitter_wrapper)`. Path to a wrapper which provides
/// convenience functions for tree-sitter nodes at the cost of worse performance.
pub fn type_sitter_lib_wrapper() -> syn::Path {
    parse_quote!(type_sitter_lib::tree_sitter_wrapper)
}

/// Convert into PathBuf and prepend manifest
fn normalize(path: impl AsRef<Path>) -> PathBuf {
    let mut path = path.as_ref().to_owned();
    if let Ok(cargo_manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        path = PathBuf::from(cargo_manifest).join(path);
    }
    path
}