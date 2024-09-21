/// From <https://github.com/serde-rs/json/issues/404#issuecomment-892957228>
mod deserialize_json_array_as_stream;
mod detail_doc;
pub(crate) mod types;
pub(crate) mod print;
mod generated_tokens;

use crate::names::PrevNodeRustNames;
use crate::node_types::deserialize_json_array_as_stream::iter_json_array;
use crate::node_types::types::NodeType;
use crate::{type_sitter, type_sitter_raw, Error};
pub use generated_tokens::GeneratedNodeTokens;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Generate source code (tokens) for typed AST node wrappers.
///
/// # Parameters
/// - `path`: Path to the `node-types.json` file of the language.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::generate_nodes;
/// use syn::parse_quote;
///
/// fn main() {
///     println!("{}", generate_nodes("../vendor/tree-sitter-rust/src/node-types.json").unwrap());
/// }
/// ```
pub fn generate_nodes(path: impl AsRef<Path>) -> Result<GeneratedNodeTokens, Error> {
    generate_nodes_with_custom_module_paths(path, &type_sitter_raw(), &type_sitter())
}

/// Generate source code (tokens) for typed AST node wrappers, and the generated code will refer to
/// the provided modules instead of `type_sitter::raw` and `type_sitter` respectively.
///
/// # Parameters
/// - `path`: Path to the `node-types.json` file of the language.
/// - `tree_sitter`: Path to the crate with the tree-sitter API. In [`generate_nodes`] this is
///   [`type_sitter_raw`] but you can provide something else, like the re-exported [`tree_sitter`]
///   or [`yak_sitter`] directly.
/// - `type_sitter_lib`: Path to the crate with the type-sitter API. In [`generate_nodes`] this is
///   [`type_sitter`] but you can provide something else, like the re-exported [`type_sitter_lib`]
///   directly.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::{generate_nodes_with_custom_module_paths, tree_sitter, type_sitter_lib};
/// use syn::parse_quote;
///
/// fn main() {
///     println!("{}", generate_nodes_with_custom_module_paths("../vendor/tree-sitter-rust/src/node-types.json", &tree_sitter(), &type_sitter_lib()).unwrap());
/// }
/// ```
pub fn generate_nodes_with_custom_module_paths(path: impl AsRef<Path>, tree_sitter: &syn::Path, type_sitter_lib: &syn::Path) -> Result<GeneratedNodeTokens, Error> {
    parse_node_types(path)?
        .map(|r| r.map(|node_type| node_type.print(tree_sitter, type_sitter_lib)))
        .collect::<Result<GeneratedNodeTokens, _>>()
}

/// Parse a `node-types.json` file into a stream of [`NodeType`]s.
pub(crate) fn parse_node_types(path: impl AsRef<Path>) -> Result<impl Iterator<Item=Result<NodeType, Error>>, Error> {
    let path = path.as_ref();
    let node_types = iter_json_array::<NodeType, _>(BufReader::new(File::open(path)?));
    let mut prev_names = PrevNodeRustNames::new();
    Ok(node_types.map(move |node_type| {
        let mut node_type = node_type?;
        node_type.name.disambiguate_rust_names(&mut prev_names);
        Ok(node_type)
    }))
}