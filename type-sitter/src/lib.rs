#![doc = include_str!("../../README.md")]

pub use type_sitter_proc::*;
pub use type_sitter_lib::*;
#[cfg(feature = "yak-sitter")]
pub use yak_sitter::*;
#[cfg(not(feature = "yak-sitter"))]
pub use tree_sitter::*;
pub use streaming_iterator::StreamingIterator;