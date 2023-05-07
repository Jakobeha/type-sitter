#![doc = include_str!("../README.md")]

use std::str::Utf8Error;
use tree_sitter::Node;
pub use either_n::*;
pub use extra_or::*;
pub use incorrect_kind::*;
pub use unwrap_and_flatten_multi::*;

/// Tagged unions of arbitrary size
mod either_n;
/// Many typed node accessors can return an extra node instead of what is positionally-expected,
/// this create has the type to wrap those.
mod extra_or;
/// Errors when a node has the wrong kind so it can't be wrapped
mod incorrect_kind;
/// Unwrapping multiple `Try`-types at once
mod unwrap_and_flatten_multi;

/// Typed node wrapper
pub trait TypedNode<'tree>: TryFrom<Node<'tree>, Error=IncorrectKind<'tree>> {
    /// Kind of nodes this wraps. Note that it can wrap sub-kinds, so an instance's node's kind may
    /// not be this exact value
    const KIND: &'static str;
    /// The wrapped node
    fn node(&self) -> &Node<'tree>;
    /// Assume that the node is the correct type and wrap. UB if the node is incorrect type
    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        Self::try_from(node).expect("from_node_unchecked failed")
    }

    // region [Node] delegate
    /// See [Node::utf8_text]
    #[inline]
    fn utf8_text<'a>(&self, source: &'a [u8]) -> Result<&'a str, Utf8Error> {
        self.node().utf8_text(source)
    }

    /// See [Node::utf16_text]
    #[inline]
    fn utf16_text<'a>(&self, source: &'a [u16]) -> &'a [u16] {
        self.node().utf16_text(source)
    }

    /// See [Node::walk]
    #[inline]
    fn walk(&self) -> tree_sitter::TreeCursor<'tree> {
        self.node().walk()
    }

    /// See [Node::parent]
    #[inline]
    fn parent(&self) -> Option<Node<'tree>> {
        self.node().parent()
    }

    /// See [Node::next_sibling]
    #[inline]
    fn next_sibling(&self) -> Option<Node<'tree>> {
        self.node().next_sibling()
    }

    /// See [Node::prev_sibling]
    #[inline]
    fn prev_sibling(&self) -> Option<Node<'tree>> {
        self.node().prev_sibling()
    }

    /// See [Node::named_child_count]
    #[inline]
    fn named_child_count(&self) -> usize {
        self.node().named_child_count()
    }

    /// See [Node::to_sexp]
    #[inline]
    fn to_sexp(&self) -> String {
        self.node().to_sexp()
    }

    /// See [Node::kind]
    #[inline]
    fn kind(&self) -> &'tree str {
        self.node().kind()
    }

    /// See [Node::is_named]
    #[inline]
    fn is_named(&self) -> bool {
        self.node().is_named()
    }

    /// See [Node::has_changes]
    #[inline]
    fn has_changes(&self) -> bool {
        self.node().has_changes()
    }

    /// See [Node::has_error]
    #[inline]
    fn has_error(&self) -> bool {
        self.node().has_error()
    }

    /// See [Node::start_byte]
    #[inline]
    fn start_byte(&self) -> usize {
        self.node().start_byte()
    }

    /// See [Node::end_byte]
    #[inline]
    fn end_byte(&self) -> usize {
        self.node().end_byte()
    }

    /// See [Node::start_position]
    #[inline]
    fn start_position(&self) -> tree_sitter::Point {
        self.node().start_position()
    }

    /// See [Node::end_position]
    #[inline]
    fn end_position(&self) -> tree_sitter::Point {
        self.node().end_position()
    }

    /// See [Node::range]
    #[inline]
    fn range(&self) -> tree_sitter::Range {
        self.node().range()
    }

    /// See [Node::byte_range]
    #[inline]
    fn byte_range(&self) -> std::ops::Range<usize> {
        self.node().byte_range()
    }
    // endregion
}