#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;
use mk_syntax::ident;
use quote::quote;

use crate::deserialize_json_array_as_stream::iter_json_array;
pub use crate::error::Error;
use crate::node_types::NodeType;

/// From https://github.com/serde-rs/json/issues/404#issuecomment-892957228
mod deserialize_json_array_as_stream;
mod error;
mod node_types;
mod print;
mod make_valid;
mod mk_syntax;

/// Generate source code (tokens) for typed AST node wrappers for the language specified by the
/// input path, which must point to its `node-types.json`.
///
/// # Example
///
/// ```rust
/// use tree_sitter_gen::generate_nodes;
///
/// fn main() {
///     println!("{}", generate_nodes("vendor/tree-sitter-rust/src/node-types.json").unwrap());
/// }
/// ```
pub fn generate_nodes(path: impl AsRef<Path>) -> Result<TokenStream, Error> {
    let path = normalize(path);
    let node_types = iter_json_array::<NodeType, _>(BufReader::new(File::open(path)?));
    node_types.map(|node_type| node_type.map(|x| x.print()).map_err(Error::from)).collect()
}

/// Generate source code (tokens) of wrappers for queries specified by the input path, which must
/// point to a `.scm` or directory of `.scm` files. If a directory, it will generate submodules for
/// each `.scm`.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::generate_queries;
///
/// fn main() {
///     println!("{}", generate_queries("vendor/tree-sitter-typescript/queries/tags.scm").unwrap());
///     println!("{}", generate_queries("vendor/tree-sitter-rust/queries").unwrap());
/// }
/// ```
pub fn generate_queries(path: impl AsRef<Path>) -> Result<TokenStream, Error> {
    let path = normalize(path);
    if path.is_dir() {
        generate_queries_from_dir(path)
    } else {
        generate_queries_from_file(path)
    }
}

/// Generate source code (tokens) of wrappers for queries specified by the input path, which must
/// point to a directory of `.scm` files. It will generate submodules for each `.scm`.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::generate_queries_from_dir;
///
/// fn main() {
///     println!("{}", generate_queries_from_dir("vendor/tree-sitter-rust/queries").unwrap());
/// }
/// ```
pub fn generate_queries_from_dir(path: impl AsRef<Path>) -> Result<TokenStream, Error> {
    let path = normalize(path);
    let mut queries = TokenStream::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry.metadata()?.is_dir() || entry_path.ends_with(".scm") {
            let entry_name = entry_path.file_stem().unwrap().to_string_lossy();
            let entry_ident = ident!(&entry_name, "query filename");
            let entry_code = generate_queries(entry_path)?;
            queries.extend(quote! {
                pub mod #entry_ident {
                    #entry_code
                }
            });
        }
    }
    return Ok(queries);
}

/// Generate source code (tokens) of wrappers for queries specified by the input path, which must
/// point to an `.scm` file.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::generate_queries_from_file;
///
/// fn main() {
///     println!("{}", generate_queries_from_file("vendor/tree-sitter-typescript/queries/tags.scm").unwrap());
/// }
/// ```
pub fn generate_queries_from_file(path: impl AsRef<Path>) -> Result<TokenStream, Error> {
    let _path = normalize(path);
    todo!("generate queries from file")
}

/// Convert into PathBuf and prepend manifest
fn normalize(path: impl AsRef<Path>) -> PathBuf {
    let mut path = path.as_ref().to_owned();
    if let Ok(cargo_manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        path = PathBuf::from(cargo_manifest).join(path);
    }
    path
}