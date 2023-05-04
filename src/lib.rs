#![doc = include_str!("../README.md")]

use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

mod generate;
/// From https://github.com/serde-rs/json/issues/404#issuecomment-892957228
mod deserialize_json_array_as_stream;
mod error;
mod node_types;
mod print;

pub use generate::generate;;

/// Generate AST wrappers for the language specified by the input path, which must point to its
/// `node-types.json`.
///
/// Note: You will need to have the `node-types.json` separate from the actual rust dependency.
/// Simply including the dependency isn't enough, you will either need to either vendor it or store
/// its node-types.json separately.
///
/// # Example
///
/// ```rust
/// use type_sitter::generate;
///
/// generate!("vendor/tree-sitter-rust/src/node-types.json")
/// ```
#[proc_macro]
pub fn generate(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate::generate(PathBuf::from(parse_macro_input!(item as LitStr).value()))
        .unwrap_or_else(syn::Error::to_compile_error).into()

}
