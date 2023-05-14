#![doc = include_str!("../README.md")]

use std::convert::Infallible;
pub use extra_or::*;
pub use incorrect_kind::*;
pub use unwrap_and_flatten_multi::*;
pub use typed_node::*;
pub use typed_query::*;

/// Many typed node accessors can return an extra node instead of what is positionally-expected,
/// this create has the type to wrap those.
mod extra_or;
/// Errors when a node has the wrong kind so it can't be wrapped
mod incorrect_kind;
/// Typed node trait
mod typed_node;
/// Typed query and related traits
mod typed_query;
/// Unwrapping multiple `Try`-types at once
mod unwrap_and_flatten_multi;
/// Wrapper on tree-sitter's data-structures which provides convenience methods and functionality
/// in exchange for slightly worse performance and mandatory unicode support.
#[cfg(feature = "tree-sitter-wrapper")]
pub mod tree_sitter_wrapper;

/// Never type (for the weird case when there is an accessor that can't return anything)
pub type Never = Infallible;