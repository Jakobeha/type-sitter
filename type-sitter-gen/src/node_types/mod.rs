/// From <https://github.com/serde-rs/json/issues/404#issuecomment-892957228>
mod deserialize_json_array_as_stream;
mod detail_doc;
pub(crate) mod types;
pub(crate) mod print;
mod generated_tokens;

use crate::names::PrevNodeRustNames;
use crate::node_types::deserialize_json_array_as_stream::iter_json_array;
use crate::node_types::types::NodeType;
use crate::Error;
pub use generated_tokens::GeneratedNodeTokens;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Generate source code (tokens) for typed AST node wrappers.
///
/// # Parameters
/// - `path`: Path to the `node-types.json` file of the language.
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either [crate::tree_sitter] or
///   [crate::yak_sitter], but you can provide a path to your own wrapper as well.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::{generate_nodes, tree_sitter};
/// use syn::parse_quote;
///
/// fn main() {
///     println!("{}", generate_nodes("../vendor/tree-sitter-rust/src/node-types.json", &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_nodes(path: impl AsRef<Path>, tree_sitter: &syn::Path) -> Result<GeneratedNodeTokens, Error> {
    parse_node_types(path)?
        .map(|r| r.map(|node_type| node_type.print(tree_sitter)))
        .collect::<Result<GeneratedNodeTokens, _>>()
}

/// Parse a `node-types.json` file into a stream of [`NodeType`]s.
pub fn parse_node_types(path: impl AsRef<Path>) -> Result<impl Iterator<Item=Result<NodeType, Error>>, Error> {
    let path = path.as_ref();
    let node_types = iter_json_array::<NodeType, _>(BufReader::new(File::open(path)?));
    let mut prev_names = PrevNodeRustNames::new();
    Ok(node_types.map(move |node_type| {
        let mut node_type = node_type?;
        node_type.name.disambiguate_rust_names(&mut prev_names);
        Ok(node_type)
    }))
}