use std::fmt::Debug;
use tree_sitter::TextProvider;
#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::{Node, Point, QueryCapture, Range, Tree};
#[cfg(not(feature = "tree-sitter-wrapper"))]
use tree_sitter::{Node, Point, QueryCapture, Range, Tree};
use crate::TypedQuery;

/// Iterate a query's captures (see [tree_sitter::QueryCaptures])
#[cfg(feature = "tree-sitter-wrapper")]
pub struct TypedQueryCaptures<'cursor, 'tree, Capture: TypedQueryCapture, Text: TextProvider<'tree> = &'tree Tree> {
    typed_query: &'static Capture::Query,
    untyped_captures: tree_sitter::QueryCaptures<'cursor, 'tree, Text>,
    tree: &'tree Tree,
}

/// Iterate a query's captures (see [tree_sitter::QueryCaptures])
#[cfg(not(feature = "tree-sitter-wrapper"))]
pub struct TypedQueryCaptures<'cursor, 'tree, Capture: TypedQueryCapture, Text: TextProvider<'tree>> {
    typed_query: &'static Capture::Query,
    untyped_captures: tree_sitter::QueryCaptures<'cursor, 'tree, Text>,
}

/// A capture from a [TypedQuery] with [TypedNode]s
pub trait TypedQueryCapture<'cursor, 'tree>: Debug + Clone {
    /// The type of query this capture came from
    type Query: TypedQuery;

    /// The query this capture came from
    fn query(&self) -> &'static Self::Query;

    /// This capture's match, if iterating via [TypedQueryCaptures].
    /// If iterating via [TypedQueryMatchCaptures] this will be `None`.
    fn match_(&self) -> Option<&<Self::Query as TypedQuery>::Match<'cursor, 'tree>>;

    /// Destruct into this capture's match, if iterating via [TypedQueryCaptures].
    /// If iterating via [TypedQueryMatchCaptures] this will be `None`.
    fn into_match(self) -> Option<<Self::Query as TypedQuery>::Match<'cursor, 'tree>>
        where <Self::Query as TypedQuery>::Match<'cursor, 'tree>: Sized;

    /// Get the equivalent [tree_sitter::QueryCapture]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn to_raw(&self) -> QueryCapture<'static, 'tree>;

    /// Get the equivalent [tree_sitter::QueryCapture]
    #[cfg(not(feature = "tree-sitter-wrapper"))]
    fn to_raw(&self) -> QueryCapture<'tree>;

    /// Get the captured untyped node
    fn node(&self) -> &Node<'tree>;

    /// Get a mutable reference to the captured untyped node
    fn node_mut(&mut self) -> &mut Node<'tree>;

    /// Get the capture name
    #[inline]
    #[cfg(feature = "tree-sitter-wrapper")]
    fn name(&self) -> &'static str {
        self.to_raw().name
    }

    /// Get the capture index
    #[inline]
    #[cfg(not(feature = "tree-sitter-wrapper"))]
    fn index(&self) -> usize {
        self.to_raw().index
    }
}

impl<'cursor, 'tree, Capture: TypedQueryCapture, Text: TextProvider<'tree>> TypedQueryCaptures<'cursor, 'tree, Captures, Text> {
    /// SAFETY: The captures must have come from the same query
    #[inline]
    pub(super) unsafe fn new(
        typed_query: &'static Capture::Query,
        untyped_captures: tree_sitter::QueryCaptures<'cursor, 'tree, &'tree Tree>,
        #[cfg(feature = "tree-sitter-wrapper")]
        tree: &'tree Tree,
    ) -> Self {
        Self { typed_query, untyped_captures, #[cfg(feature = "tree-sitter-wrapper")] tree }
    }

    /// Limit captures to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_captures.set_byte_range(range)
    }

    /// Limit captures to a point range
    #[inline]
    #[cfg(feature = "tree-sitter-wrapper")]
    pub fn set_point_range(&mut self, range: Range) {
        self.untyped_captures.set_point_range(std::ops::Range {
            start: range.start_point().into(),
            end: range.end_point().into()
        })
    }

    /// Limit captures to a point range
    #[inline]
    #[cfg(not(feature = "tree-sitter-wrapper"))]
    pub fn set_point_range(&mut self, range: Range) {
        self.untyped_captures.set_point_range(std::ops::Range {
            start: range.start_point(),
            end: range.end_point()
        })
    }
}

impl<'cursor, 'tree, Capture: TypedQueryCapture, Text: TextProvider<'tree>> Iterator for TypedQueryCaptures<'cursor, 'tree, Capture, Text> {
    type Item = Capture;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: Match and captures come from the same query
        unsafe { self.untyped_captures.next().map(|(m, index)| {
            self.typed_query.wrap_capture(
                m.captures[index],
                Some(self.typed_query.wrap_match(
                    m,
                    #[cfg(feature = "tree-sitter-wrapper")] self.tree
                )),
                #[cfg(feature = "tree-sitter-wrapper")] self.tree
            )
        }) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.untyped_captures.size_hint()
    }
}