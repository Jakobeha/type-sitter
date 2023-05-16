#![doc = include_str!("../README.md")]

use std::convert::Infallible;
pub use typed_node::*;
pub use typed_query::*;

/// "Private" structures for [type_sitter_gen](https://docs.rs/type-sitter-gen/latest/type_sitter_gen)
pub mod gen_internal;
/// Typed node trait
mod typed_node;
/// Typed query and related traits
mod typed_query;
/// Wrapper on tree-sitter's data-structures which provides convenience methods and functionality
/// in exchange for slightly worse performance and mandatory unicode support.
#[cfg(feature = "tree-sitter-wrapper")]
pub mod tree_sitter_wrapper;

/// Never type (for the weird case when there is an accessor that can't return anything)
pub type Never = Infallible;