/// From <https://github.com/serde-rs/json/issues/404#issuecomment-892957228>
mod deserialize_json_array_as_stream;
mod detail_doc;
pub(crate) mod types;
pub(crate) mod print;
mod generated_tokens;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::Error;
use crate::node_types::deserialize_json_array_as_stream::iter_json_array;
use crate::node_types::types::NodeType;
pub use generated_tokens::GeneratedNodeTokens;

/// Generate source code (tokens) for typed AST node wrappers.
///
/// # Parameters
/// - `path`: Path to the `node-types.json` file of the language.
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either [crate::tree_sitter] or
///   [crate::type_sitter_lib_wrapper], but you can provide a path to your own wrapper as well.
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
    let path = path.as_ref();
    let node_types = iter_json_array::<NodeType, _>(BufReader::new(File::open(path)?));
    node_types
        .map(|node_type| node_type.map(|x| x.print(tree_sitter)).map_err(Error::from))
        .collect::<Result<GeneratedNodeTokens, _>>()
}
