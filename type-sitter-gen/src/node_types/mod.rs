/// From <https://github.com/serde-rs/json/issues/404#issuecomment-892957228>
mod deserialize_json_array_as_stream;
mod detail_doc;
mod types;
mod print;
mod generated_tokens;
mod rust_names;
mod names;

use crate::node_types::deserialize_json_array_as_stream::iter_json_array;
use crate::{type_sitter, type_sitter_raw, Error, PrintCtx};
pub use generated_tokens::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub use types::*;
pub use rust_names::*;
pub use names::*;

/// Generate source code (tokens) for typed AST node wrappers.
///
/// # Parameters
/// - `path`: Path to the `node-types.json` file of the language.
///
/// # Example
///
/// ```rust
/// use type_sitter_gen::generate_nodes;
///
/// fn main() {
///     println!("{}", generate_nodes(
///         "../vendor/tree-sitter-rust/src/node-types.json"
///     ).unwrap().into_string());
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
///
/// fn main() {
///     println!("{}", generate_nodes_with_custom_module_paths(
///         "../vendor/tree-sitter-rust/src/node-types.json",
///         &tree_sitter(),
///         &type_sitter_lib()
///     ).unwrap().into_string());
/// }
/// ```
pub fn generate_nodes_with_custom_module_paths(path: impl AsRef<Path>, tree_sitter: &syn::Path, type_sitter_lib: &syn::Path) -> Result<GeneratedNodeTokens, Error> {
    let all_types = parse_node_type_map(path)?;

    let ctx = PrintCtx {
        all_types: &all_types,
        tree_sitter,
        type_sitter_lib,
    };

    Ok(all_types.values()
        .map(|r| r.print(ctx))
        .collect::<GeneratedNodeTokens>())
}

/// Parse a `node-types.json` file into a map of [SEXP name](NodeName::sexp_name) to [`NodeType`].
pub(crate) fn parse_node_type_map(path: impl AsRef<Path>) -> Result<NodeTypeMap, Error> {
    let path = path.as_ref();
    let reader = BufReader::new(File::open(path)?);
    let elems = iter_json_array::<ContextFreeNodeType, _>(reader)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(NodeTypeMap::new(elems))
}

pub fn generate_code<T, E>(all_types: T) -> Result<GeneratedNodeTokens, E>
where T: TryInto<NodeTypeMap, Error = E>{
    generate_code_with_custom_module_paths(all_types, &type_sitter_raw(), &type_sitter())
}

pub fn generate_code_with_custom_module_paths<T, E>(
    all_types: T,
    tree_sitter: &syn::Path,
    type_sitter_lib: &syn::Path)
-> Result<GeneratedNodeTokens, E>
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

