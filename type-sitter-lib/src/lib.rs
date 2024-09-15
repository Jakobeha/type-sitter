#![doc = include_str!("../README.md")]

use std::convert::Infallible;
pub use typed_node::*;
pub use typed_query::*;
pub use typed_tree::*;

/// Typed node trait
mod typed_node;
/// Typed query and related traits
mod typed_query;
/// Tree whose root is a typed node. Currently we don't know which node is the root, so you must
/// manually construct this with the correct type argument (still checked though).
mod typed_tree;

/// Never type (for the weird case when there is an accessor that can't return anything)
pub type Never = Infallible;