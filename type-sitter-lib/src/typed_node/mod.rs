use std::fmt::Debug;
use std::hash::Hash;
#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::{Bitmask, InputEdit, Node, Point, PointRange, Range, TreeCursor};
#[cfg(not(feature = "tree-sitter-wrapper"))]
use std::str::Utf8Error;
#[cfg(not(feature = "tree-sitter-wrapper"))]
use tree_sitter::{InputEdit, Node, Point, Range, TreeCursor};
pub use extra_or::*;
pub use incorrect_kind::*;
pub use unwrap_and_flatten_multi::*;
pub use untyped_nodes::*;

/// Many typed node accessors can return an extra node instead of what is positionally-expected,
/// this create has the type to wrap those.
mod extra_or;
/// Errors when a node has the wrong kind so it can't be wrapped
mod incorrect_kind;
/// Unwrapping multiple `Try`-types at once
mod unwrap_and_flatten_multi;
/// Untyped "typed" nodes (implement [TypedNode] but don't actually have a type)
mod untyped_nodes;

/// Typed node wrapper
pub trait TypedNode<'tree>: TryFrom<Node<'tree>, Error=IncorrectKind<'tree>> + Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// Kind of nodes this wraps. Note that it can wrap sub-kinds, so an instance's node's kind may
    /// not be this exact value
    const KIND: &'static str;
    /// The wrapped node
    fn node(&self) -> &Node<'tree>;
    /// The wrapped node (mutable reference, rarely needed)
    fn node_mut(&mut self) -> &mut Node<'tree>;
    /// Destruct into the wrapped node
    fn into_node(self) -> Node<'tree>;
    /// Assume that the node is the correct type and wrap. UB if the node is an incorrect type
    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        Self::try_from(node).expect("from_node_unchecked failed")
    }

    // region [Node] delegate
    /// See [Node::text]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn text(&self) -> &'tree str {
        self.node().text()
    }

    /// See [Node::mark]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn mark(&self, mark: Bitmask) -> Bitmask {
        self.node().mark(mark)
    }

    /// See [Node::unmark]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn unmark(&self, mark: Bitmask) -> Bitmask {
        self.node().unmark(mark)
    }

    /// See [Node::get_mark]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn get_mark(&self) -> Bitmask {
        self.node().get_mark()
    }

    /// See [Node::utf8_text]
    #[inline]
    #[cfg(not(feature = "tree-sitter-wrapper"))]
    fn utf8_text<'a>(&self, source: &'a [u8]) -> Result<&'a str, Utf8Error> {
        self.node().utf8_text(source)
    }

    /// See [Node::utf16_text]
    #[inline]
    #[cfg(not(feature = "tree-sitter-wrapper"))]
    fn utf16_text<'a>(&self, source: &'a [u16]) -> &'a [u16] {
        self.node().utf16_text(source)
    }

    /// See [Node::walk]
    #[inline]
    fn walk(&self) -> TreeCursor<'tree> {
        self.node().walk()
    }

    /// See [Node::parent]
    #[inline]
    fn parent(&self) -> Option<Node<'tree>> {
        self.node().parent()
    }

    /// See [Node::next_named_sibling]
    #[inline]
    fn next_named_sibling(&self) -> Option<Node<'tree>> {
        self.node().next_named_sibling()
    }

    /// See [Node::prev_named_sibling]
    #[inline]
    fn prev_named_sibling(&self) -> Option<Node<'tree>> {
        self.node().prev_named_sibling()
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
    fn start_position(&self) -> Point {
        self.node().start_position()
    }

    /// See [Node::end_position]
    #[inline]
    fn end_position(&self) -> Point {
        self.node().end_position()
    }

    /// See [Node::range]
    #[inline]
    fn range(&self) -> Range {
        self.node().range()
    }

    /// See [Node::byte_range]
    #[inline]
    fn byte_range(&self) -> std::ops::Range<usize> {
        self.node().byte_range()
    }

    /// See [Node::point_range]
    #[inline]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn point_range(&self) -> PointRange {
        self.node().point_range()
    }

    /// See [Node::edit]
    #[inline]
    fn edit(&mut self, edit: &InputEdit) {
        self.node_mut().edit(edit)
    }
    // endregion
}