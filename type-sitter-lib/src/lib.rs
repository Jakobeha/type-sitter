#![doc = include_str!("../README.md")]

pub use node::*;
pub use query::*;
pub use raw::{IncludedRangesError, InputEdit, Language, LanguageError, LanguageRef, Point, QueryProperty, Range};
#[cfg(feature = "yak-sitter")]
pub use raw::{NodeId, NodePtr, PointRange, TreeParseError};
use std::convert::Infallible;
pub use streaming_iterator::StreamingIterator;
#[cfg(not(feature = "yak-sitter"))]
pub use tree_sitter as raw;
#[cfg(feature = "yak-sitter")]
pub use yak_sitter as raw;

/// Typed node trait
mod node;
/// Typed query and related traits
mod query;

/// Never type (for the weird case when there is an accessor that can't return anything)
pub type Never = Infallible;