use crate::{raw, Node, Point, UntypedNode};
#[cfg(feature = "yak-sitter")]
use yak_sitter::TraversalState;

/// A stateful object for walking a syntax tree efficiently. See [`raw::TreeCursor`]
#[derive(Clone)]
#[repr(transparent)]
pub struct TreeCursor<'tree>(pub raw::TreeCursor<'tree>);

impl<'tree> TreeCursor<'tree> {
    /// Transmute an immutable reference, since they have the same representation.
    pub fn wrap_ref<'a>(cursor: &'a raw::TreeCursor) -> &'a Self {
        // SAFETY: Same repr
        unsafe { &*(cursor as *const raw::TreeCursor as *const Self) }
    }

    /// Transmute a mutable reference, since they have the same representation.
    pub fn wrap_mut<'a>(cursor: &'a mut raw::TreeCursor) -> &'a mut Self {
        // SAFETY: Same repr
        unsafe { &mut *(cursor as *mut raw::TreeCursor as *mut Self) }
    }

    /// Gets the cursor's current node
    #[inline]
    pub fn node(&self) -> UntypedNode<'tree> {
        UntypedNode::new(self.0.node())
    }

    /// Gets the field name of the cursor's current node
    #[inline]
    pub fn field_name(&mut self) -> Option<&'static str> {
        self.0.field_name()
    }

    /// Re-initialize the cursor to point to the given node
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn goto(&mut self, node: impl Node<'tree>) {
        self.0.reset(node.into_raw())
    }

    /// Re-initialize the cursor to point to the given node
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn reset(&mut self, node: impl Node<'tree>) {
        self.0.reset(node.into_raw())
    }

    /// Move the cursor to the first child of the current node and return `true`, or return `false`
    /// if the current node has no children
    #[inline]
    pub fn goto_first_child(&mut self) -> bool {
        self.0.goto_first_child()
    }

    /// Move the cursor to the first child of the current node that extends beyond the given byte
    /// offset, and return its index.
    ///
    /// Returns `None` if the current node has no children past that offset.
    #[inline]
    pub fn goto_first_child_for_byte(&mut self, index: usize) -> Option<usize> {
        self.0.goto_first_child_for_byte(index)
    }

    /// Move the cursor to the first child of the current node that extends beyond the given row
    /// and column, and return its index.
    ///
    /// Returns `None` if the current node has no children past that row and column.
    #[inline]
    pub fn goto_first_child_for_point(&mut self, point: Point) -> Option<usize> {
        self.0.goto_first_child_for_point(point)
    }

    /// Move the cursor to the next sibling of the current node and return `true`, or return `false`
    /// if the current node has no next sibling.
    ///
    /// Unlike [`tree_sitter::TreeCursor.goto_next_sibling`], this will try (isn't guaranteed to
    /// return `false`) if the cursor is rooted (e.g. reset) to its current node.
    #[cfg(feature = "yak-sitter")]
    #[inline]
    pub fn goto_next_sibling(&mut self) -> bool {
        self.0.goto_next_sibling()
    }

    /// Move the cursor to the next sibling of the current node and return `true`, or return `false`
    /// if the current node has no next sibling.
    ///
    /// Also returns `false` if the cursor was initialized or re-initialized to this node, even if
    /// it has a sibling.
    #[cfg(not(feature = "yak-sitter"))]
    #[inline]
    pub fn goto_next_sibling(&mut self) -> bool {
        self.0.goto_next_sibling()
    }

    /// Move the cursor to the parent of the current node and return `true`, or return `false`
    /// if the current node is a tree root.
    ///
    /// Unlike [`tree_sitter::TreeCursor.goto_parent`], this will try (isn't guaranteed to return
    /// `false`) if the cursor is rooted (e.g. reset) to its current node.
    #[cfg(feature = "yak-sitter")]
    #[inline]
    pub fn goto_parent(&mut self) -> bool {
        self.0.goto_parent()
    }

    /// Move the cursor to the parent of the current node and return `true`, or return `false`
    /// if the current node is a tree root.
    ///
    /// Also returns `false` if the cursor was initialized or re-initialized to this node, even if
    /// it has a parent.
    #[cfg(not(feature = "yak-sitter"))]
    #[inline]
    pub fn goto_parent(&mut self) -> bool {
        self.0.goto_parent()
    }

    /// Move the cursor to the next node in pre-order traversal order, where nodes with children are
    /// traversed on both the up and down.
    ///
    /// This uses `last_state` to determine where to go next (i.e. up or right/down on a node with
    /// children), and returns the next state which you would pass to the next `goto_preorder` call
    /// and so on.
    #[cfg(feature = "yak-sitter")]
    #[inline]
    pub fn goto_preorder(&mut self, last_state: TraversalState) -> TraversalState {
        self.0.goto_preorder(last_state)
    }
}
