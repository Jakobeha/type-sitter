use std::fmt::Debug;
use tree_sitter::TextProvider;
#[cfg(feature = "yak-sitter")]
use yak_sitter::{Node, QueryCapture, PointRange, Tree};
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::{Point, Node, QueryCapture};
use crate::TypedQuery;

/// Iterate a query's captures (see [tree_sitter::QueryCaptures])
#[cfg(feature = "yak-sitter")]
pub struct TypedQueryCaptures<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I> = &'cursor Tree, I: AsRef<[u8]> = &'cursor str> {
    typed_query: &'cursor Query,
    untyped_captures: tree_sitter::QueryCaptures<'cursor, 'tree, Text, I>,
    tree: &'tree Tree,
}

/// Iterate a query's captures (see [tree_sitter::QueryCaptures])
#[cfg(not(feature = "yak-sitter"))]
pub struct TypedQueryCaptures<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> {
    typed_query: &'cursor Query,
    untyped_captures: tree_sitter::QueryCaptures<'cursor, 'tree, Text, I>,
}

/// A capture from a [TypedQuery] with [crate::TypedNode]s
pub trait TypedQueryCapture<'cursor, 'tree: 'cursor>: Debug + Clone {
    /// The type of query this capture came from
    type Query: TypedQuery<Capture<'cursor, 'tree> = Self>;

    /// The query this capture came from
    fn query(&self) -> &'cursor Self::Query;

    /// This capture's match, if iterating via [TypedQueryCaptures].
    /// If iterating via [crate::TypedQueryMatchCaptures] this will be `None`.
    fn r#match(&self) -> Option<&<Self::Query as TypedQuery>::Match<'cursor, 'tree>>;

    /// Destruct into this capture's match, if iterating via [TypedQueryCaptures].
    /// If iterating via [crate::TypedQueryMatchCaptures] this will be `None`.
    fn into_match(self) -> Option<<Self::Query as TypedQuery>::Match<'cursor, 'tree>>
        where <Self::Query as TypedQuery>::Match<'cursor, 'tree>: Sized;

    /// Get the equivalent [tree_sitter::QueryCapture]
    #[cfg(feature = "yak-sitter")]
    fn to_raw(&self) -> QueryCapture<'static, 'tree>;

    /// Get the equivalent [tree_sitter::QueryCapture]
    #[cfg(not(feature = "yak-sitter"))]
    fn to_raw(&self) -> QueryCapture<'tree>;

    /// Get the captured untyped node
    fn node(&self) -> &Node<'tree>;

    /// Get a mutable reference to the captured untyped node
    fn node_mut(&mut self) -> &mut Node<'tree>;

    /// Get the capture name
    fn name(&self) -> &'static str;

    /// Get the capture index
    #[inline]
    fn index(&self) -> usize {
        self.to_raw().index as usize
    }
}

impl<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> TypedQueryCaptures<'cursor, 'tree, Query, Text, I> {
    /// SAFETY: The captures must have come from the same query
    #[inline]
    pub(super) unsafe fn new(
        typed_query: &'cursor Query,
        untyped_captures: tree_sitter::QueryCaptures<'cursor, 'tree, Text, I>,
        #[cfg(feature = "yak-sitter")]
        tree: &'tree Tree,
    ) -> Self {
        Self { typed_query, untyped_captures, #[cfg(feature = "yak-sitter")] tree }
    }

    /// Limit captures to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_captures.set_byte_range(range)
    }

    /// Limit captures to a point range
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn set_point_range(&mut self, range: PointRange) {
        self.untyped_captures.set_point_range(range.to_ts_point_range())
    }

    /// Limit captures to a point range
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn set_point_range(&mut self, range: std::ops::Range<Point>) {
        self.untyped_captures.set_point_range(range)
    }
}

impl<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> Iterator for TypedQueryCaptures<'cursor, 'tree, Query, Text, I> {
    type Item = Query::Capture<'cursor, 'tree>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: Match and captures come from the same query
        unsafe { self.untyped_captures.next().map(|(m, index)| {
            self.typed_query.wrap_capture(
                m.captures[index],
                Some(self.typed_query.wrap_match(
                    m,
                    #[cfg(feature = "yak-sitter")] self.tree
                )),
                #[cfg(feature = "yak-sitter")] self.tree
            )
        }) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.untyped_captures.size_hint()
    }
}