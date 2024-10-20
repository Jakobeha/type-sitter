use crate::raw;
#[cfg(feature = "yak-sitter")]
use crate::PointRange;
use crate::{InputEdit, Point, Range};
pub use cursor::*;
pub use incorrect_kind::*;
pub use parser::*;
use std::fmt::Debug;
use std::hash::Hash;
#[cfg(not(feature = "yak-sitter"))]
use std::str::Utf8Error;
pub use tree::*;
pub use unwrap_and_flatten_multi::*;
pub use wrappers::*;

/// Extra, missing, untyped nodes
mod wrappers;
/// Errors when a node has the wrong kind so it can't be wrapped
mod incorrect_kind;
/// Unwrapping multiple `Try`-types at once
mod unwrap_and_flatten_multi;
mod tree;
mod cursor;
mod parser;

/// Typed node wrapper.
///
/// This implements `TryFrom<tree_sitter::Node<'tree>>`, which will succeed iff the node is of the correct type.
/// That is how you convert untyped nodes into types nodes. If you're absolutely sure the node is
/// correct, you may also use [`Node::from_raw_unchecked`], though it's honestly probably not
/// worth the possible performance gain.
pub trait Node<'tree>: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// The same type, but with a different lifetime.
    type WithLifetime<'a>: Node<'a>;

    /// Kind of nodes this wraps.
    ///
    /// For nodes that map directly to tree-sitter nodes, this is the tree-sitter node's name. For
    /// nodes like unions, [`UntypedNode`], or [`NodeResult`]s, which don't have a simple kind
    /// (especially static), this is `{...}`.
    const KIND: &'static str;

    /// Check that the tree-sitter node is the correct kind, and if it is, wrap.
    ///
    /// Returns `Err` if the node is not of the correct kind.
    fn try_from_raw(node: raw::Node<'tree>) -> NodeResult<'tree, Self>;

    /// Assume that tree-sitter node is the correct kind and wrap.
    ///
    /// # Safety
    /// The node must be of the correct kind.
    #[inline]
    unsafe fn from_raw_unchecked(node: raw::Node<'tree>) -> Self {
        Self::try_from_raw(node).expect("from_raw_unchecked failed")
    }

    /// The wrapped tree-sitter node.
    ///
    /// Note that most methods you should call on this struct directly.
    fn raw(&self) -> &raw::Node<'tree>;

    /// The wrapped tree-sitter node (mutable reference, rarely needed).
    ///
    /// Note that most methods you should call on this struct directly.
    fn raw_mut(&mut self) -> &mut raw::Node<'tree>;

    /// Destruct into the wrapped tree-sitter node.
    ///
    /// Note that most methods you should call on this struct directly.
    fn into_raw(self) -> raw::Node<'tree>;

    /// Upcast into an untyped node.
    ///
    /// The inverse is [`UntypedNode::downcast`].
    #[inline]
    fn upcast(self) -> UntypedNode<'tree> {
        UntypedNode::new(self.into_raw())
    }

    // region [Node] delegate
    /// See [tree-sitter's `Node::text`](raw::Node::text)
    #[cfg(feature = "yak-sitter")]
    fn text(&self) -> &'tree str {
        self.raw().text()
    }

    /// See [tree-sitter's `Node::utf8_text`](raw::Node::utf8_text)
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    fn utf8_text<'a>(&self, source: &'a [u8]) -> Result<&'a str, Utf8Error> {
        self.raw().utf8_text(source)
    }

    /// See [tree-sitter's `Node::utf16_text`](raw::Node::utf16_text)
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    fn utf16_text<'a>(&self, source: &'a [u16]) -> &'a [u16] {
        self.raw().utf16_text(source)
    }

    /// Returns any [extra](raw::Node::is_extra) nodes before this one, e.g., comments.
    ///
    /// Nodes are iterated first to last (by source location).
    #[inline]
    fn prefixes(&self) -> impl Iterator<Item=UntypedNode<'tree>> {
        Prefixes::new(*self.raw())
    }

    /// Returns any [extra](raw::Node::is_extra) nodes after this one, e.g., comments.
    ///
    /// Nodes are iterated first to last (by source location).
    #[inline]
    fn suffixes(&self) -> impl Iterator<Item=UntypedNode<'tree>> {
        Suffixes::new(*self.raw())
    }

    /// Get a cursor for this node
    #[inline]
    fn walk(&self) -> TreeCursor<'tree> {
        TreeCursor(self.raw().walk())
    }

    /// Get the node's immediate parent
    #[inline]
    fn parent(&self) -> Option<UntypedNode<'tree>> {
        self.raw().parent().map(UntypedNode::from)
    }

    /// Get the node's immediate named next sibling
    #[inline]
    fn next_named_sibling(&self) -> Option<UntypedNode<'tree>> {
        self.raw().next_named_sibling().map(UntypedNode::from)
    }

    /// Get the node's immediate named previous sibling
    #[inline]
    fn prev_named_sibling(&self) -> Option<UntypedNode<'tree>> {
        self.raw().prev_named_sibling().map(UntypedNode::from)
    }

    /// Get the number of named children
    #[inline]
    fn named_child_count(&self) -> usize {
        self.raw().named_child_count()
    }

    /// Print the node as an s-expression
    #[inline]
    fn to_sexp(&self) -> String {
        self.raw().to_sexp()
    }

    /// Get this node's tree-sitter name. See [tree-sitter's `Node::kind`](raw::Node::kind)
    #[inline]
    fn kind(&self) -> &'static str {
        self.raw().kind()
    }

    /// Check if this node is *named*. See [tree-sitter's `Node::is_named`](raw::Node::is_named)
    #[inline]
    fn is_named(&self) -> bool {
        self.raw().is_named()
    }

    /// Check if this node has been edited
    #[inline]
    fn has_changes(&self) -> bool {
        self.raw().has_changes()
    }

    /// Check if this node represents a syntax error or contains any syntax errors anywhere within
    /// it
    #[inline]
    fn has_error(&self) -> bool {
        self.raw().has_error()
    }

    /// Get the byte offset where this node starts
    #[inline]
    fn start_byte(&self) -> usize {
        self.raw().start_byte()
    }

    /// Get the byte offset where this node ends
    #[inline]
    fn end_byte(&self) -> usize {
        self.raw().end_byte()
    }

    /// Get the row and column where this node starts
    #[inline]
    fn start_position(&self) -> Point {
        self.raw().start_position()
    }

    /// Get the row and column where this node ends
    #[inline]
    fn end_position(&self) -> Point {
        self.raw().end_position()
    }

    /// Get the byte range and row and column range where this node is located
    #[inline]
    fn range(&self) -> Range {
        self.raw().range()
    }

    /// Get the byte range where this node is located
    #[inline]
    fn byte_range(&self) -> std::ops::Range<usize> {
        self.raw().byte_range()
    }

    /// Get the row and column range where this node is located
    #[inline]
    #[cfg(feature = "yak-sitter")]
    fn position_range(&self) -> PointRange {
        self.raw().position_range()
    }

    /// Edit this node to keep it in-sync with source code that has been edited. See
    /// [tree-sitter's `Node::edit`](raw::Node::edit)
    #[inline]
    fn edit(&mut self, edit: &InputEdit) {
        self.raw_mut().edit(edit)
    }
    // endregion
}

struct Prefixes<'tree> {
    cursor: raw::TreeCursor<'tree>,
    end: raw::Node<'tree>,
}

struct Suffixes<'tree> {
    cursor: raw::TreeCursor<'tree>,
}

impl<'tree> Prefixes<'tree> {
    fn new(raw: raw::Node<'tree>) -> Self {
        let Some(parent) = raw.parent() else {
            return Self { cursor: raw.walk(), end: raw };
        };

        let mut cursor = parent.walk();
        cursor.goto_first_child();

        'outer: loop {
            if cursor.node() == raw {
                break Self { cursor: raw.walk(), end: raw };
            }

            if cursor.node().is_extra() {
                let mut cursor2 = cursor.clone();
                while cursor2.node().is_extra() {
                    if !cursor2.goto_next_sibling() {
                        break 'outer Self { cursor: raw.walk(), end: raw };
                    }

                    if cursor2.node() == raw {
                        break 'outer Self { cursor, end: raw };
                    }
                }
            }

            if !cursor.goto_next_sibling() {
                break Self { cursor: raw.walk(), end: raw };
            }
        }
    }
}

impl<'tree> Suffixes<'tree> {
    fn new(raw: raw::Node<'tree>) -> Self {
        let Some(parent) = raw.parent() else {
            return Self { cursor: raw.walk() }
        };

        let mut cursor = parent.walk();
        cursor.goto_first_child();

        while cursor.node() != raw {
            let next = cursor.goto_next_sibling();
            assert!(next, "node not found in parent");
        }

        Self { cursor }
    }
}

impl<'tree> Iterator for Prefixes<'tree> {
    type Item = UntypedNode<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.node() == self.end {
            return None;
        }

        let result = UntypedNode::new(self.cursor.node());
        debug_assert!(
            self.cursor.node().is_extra(),
            "node before our iteration target isn't an extra, but we thought it would be"
        );
        let next = self.cursor.goto_next_sibling();
        assert!(next, "node (that we've been iterating the prefixes of) not found in parent");
        Some(result)
    }
}

impl<'tree> Iterator for Suffixes<'tree> {
    type Item = UntypedNode<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.cursor.goto_next_sibling() || !self.cursor.node().is_extra() {
            return None
        }

        Some(UntypedNode::new(self.cursor.node()))
    }
}