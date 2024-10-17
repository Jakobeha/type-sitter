/// From <https://github.com/serde-rs/json/issues/404#issuecomment-892957228>
mod deserialize_json_array_as_stream;
mod detail_doc;
mod types;
mod print;
mod generated_tokens;
mod rust_names;
mod names;

use crate::{type_sitter, type_sitter_raw, PrintCtx};
pub use generated_tokens::*;
pub use names::*;
pub use rust_names::*;
pub use types::*;

/// Generate source code (tokens) for typed AST node wrappers.
///
/// # Parameters
/// - `types`: Anything that can be converted to a `NodeTypeMap`
///
/// # Examples
///
/// Using a file path:
///
/// ```rust
/// use type_sitter_gen::generate_nodes;
/// use std::path::Path;
///
/// fn main() {
///     let path = Path::new("../vendor/tree-sitter-rust/src/node-types.json");
///     let code = generate_nodes(path).unwrap().into_string();
///     assert!(code.contains("pub struct TraitItem"));
/// }
/// ```
///
/// Using the contents of `node-types.json` directly:
///
/// ```rust
/// use type_sitter_gen::generate_nodes;
///
/// fn main() {
///     let contents: &str = tree_sitter_rust::NODE_TYPES;
///     let code = generate_nodes(contents).unwrap().into_string();
///     assert!(code.contains("pub struct TraitItem"));
/// }
/// ```
///
/// Using a `NodeTypeMap`:
///
/// ```rust
/// use type_sitter_gen::{generate_nodes, NodeTypeMap};
///
/// fn main() {
///     let mut node_type_map = NodeTypeMap::try_from(tree_sitter_rust::NODE_TYPES).unwrap();
///     // customize node_type_map
///     let code = generate_nodes(node_type_map).unwrap().into_string();
///     assert!(code.contains("pub struct TraitItem"));
/// }
/// ```
pub fn generate_nodes<T, E>(types: T) -> Result<GeneratedNodeTokens, E>
where T: TryInto<NodeTypeMap, Error = E> {
    generate_nodes_with_custom_module_paths(types, &type_sitter_raw(), &type_sitter())
}

/// Generate source code (tokens) for typed AST node wrappers, and the generated code will refer to
/// the provided modules instead of `type_sitter::raw` and `type_sitter` respectively.
///
/// # Parameters
/// - `types`: Anything that can be converted to a `NodeTypeMap`
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
/// use std::path::Path;
///
/// fn main() {
///     let code = generate_nodes_with_custom_module_paths(
///         Path::new("../vendor/tree-sitter-rust/src/node-types.json"),
///         &tree_sitter(),
///         &type_sitter_lib()
///     ).unwrap().into_string();
///     assert!(code.contains("pub struct TraitItem"));
/// }
/// ```
pub fn generate_nodes_with_custom_module_paths<T, E>(all_types: T, tree_sitter: &syn::Path, type_sitter_lib: &syn::Path) -> Result<GeneratedNodeTokens, E>
where T: TryInto<NodeTypeMap, Error = E> {
    let all_types = all_types.try_into()?;

    let ctx = PrintCtx {
        all_types: &all_types,
        tree_sitter,
        type_sitter_lib,
    };

    Ok(all_types.values()
        .map(|r| r.print(ctx))
        .collect::<GeneratedNodeTokens>())
}

