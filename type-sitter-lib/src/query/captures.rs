use crate::{raw, Query, UntypedNode};
use std::fmt::Debug;
use streaming_iterator::StreamingIterator;
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::Point;
#[cfg(feature = "yak-sitter")]
use yak_sitter::PointRange;

/// Iterate a query's captures (see [tree-sitter's `QueryCaptures`](RawQueryCapture))
#[cfg(feature = "yak-sitter")]
pub struct QueryCaptures<'query, 'tree: 'query, Query: crate::Query> {
    typed_query: &'query Query,
    untyped_captures: raw::QueryCaptures<'query, 'tree>,
}

/// Iterate a query's captures (see [RawQueryCaptures])
#[cfg(not(feature = "yak-sitter"))]
pub struct QueryCaptures<'query, 'tree: 'query, Query: crate::Query, Text: raw::TextProvider<I>, I: AsRef<[u8]>> {
    typed_query: &'query Query,
    untyped_captures: raw::QueryCaptures<'query, 'tree, Text, I>,
}

/// A capture from a [`Query`] with [typed nodes](crate::Node)
pub trait QueryCapture<'query, 'tree: 'query>: Debug + Clone {
    /// The type of query this capture came from
    type Query: Query<Capture<'query, 'tree> = Self>;

    /// The query this capture came from
    fn query(&self) -> &'query Self::Query;

    /// Get the raw tree-sitter query capture
    #[cfg(feature = "yak-sitter")]
    fn raw(&self) -> raw::QueryCapture<'query, 'tree>;

    /// Get the raw tree-sitter query captures.
    #[cfg(not(feature = "yak-sitter"))]
    fn raw(&self) -> raw::QueryCapture<'tree>;

    /// Get the captured untyped node
    fn node(&self) -> &UntypedNode<'tree>;

    /// Get a mutable reference to the captured untyped node
    fn node_mut(&mut self) -> &mut UntypedNode<'tree>;

    /// Get the capture name
    fn name(&self) -> &'query str;

    /// Get the capture index
    // The cast is necessary in `tree-sitter` but not `yak-sitter`.
    #[allow(clippy::unnecessary_cast)]
    #[inline]
    fn index(&self) -> usize {
        self.raw().index as usize
    }
}

#[cfg(feature = "yak-sitter")]
impl<'query, 'tree: 'query, Query: crate::Query> QueryCaptures<'query, 'tree, Query> {
    /// Construct from [tree-sitter's `QueryCaptures`](raw::QueryCaptures)
    ///
    /// # Safety
    /// The captures must have come from the same query.
    #[inline]
    pub(super) unsafe fn new(
        typed_query: &'query Query,
        untyped_captures: raw::QueryCaptures<'query, 'tree>,
    ) -> Self {
        Self { typed_query, untyped_captures }
    }

    /// Limit captures to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_captures.set_byte_range(range)
    }

    /// Limit captures to a point range
    #[inline]
    pub fn set_point_range(&mut self, range: PointRange) {
        self.untyped_captures.set_point_range(range)
    }
}

#[cfg(not(feature = "yak-sitter"))]
impl<'query, 'tree: 'query, Query: crate::Query, Text: raw::TextProvider<I>, I: AsRef<[u8]>> QueryCaptures<'query, 'tree, Query, Text, I> {
    /// Construct from [tree-sitter's `QueryCaptures`](raw::QueryCaptures)
    ///
    /// # Safety
    /// The captures must have come from the same query.
    #[inline]
    pub(super) unsafe fn new(
        typed_query: &'query Query,
        untyped_captures: raw::QueryCaptures<'query, 'tree, Text, I>,
    ) -> Self {
        Self { typed_query, untyped_captures }
    }

    /// Limit captures to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_captures.set_byte_range(range)
    }

    /// Limit captures to a point range
    #[inline]
    pub fn set_point_range(&mut self, range: std::ops::Range<Point>) {
        self.untyped_captures.set_point_range(range)
    }
}

#[cfg(feature = "yak-sitter")]
impl<'query, 'tree: 'query, Query: crate::Query> Iterator for QueryCaptures<'query, 'tree, Query> {
    type Item = Query::Capture<'query, 'tree>;

    //noinspection RsIncorrectFunctionArgumentCount -- IntelliJ inspection bug.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let query = self.untyped_captures.query();
        let tree = self.untyped_captures.tree();

        // SAFETY: Captures come from the same query and tree, and node from the same tree
        unsafe { self.untyped_captures.as_inner_mut().next().map(|(m, index)| {
            let inner_capture = m.captures[*index];
            self.typed_query.wrap_capture(
                raw::QueryCapture {
                    node: raw::Node::new(inner_capture.node, tree),
                    index: inner_capture.index as usize,
                    name: query.capture_names()[inner_capture.index as usize],
                },
            )
        }) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.untyped_captures.size_hint()
    }
}

#[cfg(not(feature = "yak-sitter"))]
impl<'query, 'tree: 'query, Query: crate::Query, Text: raw::TextProvider<I>, I: AsRef<[u8]>> Iterator for QueryCaptures<'query, 'tree, Query, Text, I> {
    type Item = Query::Capture<'query, 'tree>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: Captures come from the same query
        unsafe { self.untyped_captures.next().map(|(m, index)| {
            self.typed_query.wrap_capture(m.captures[*index])
        }) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.untyped_captures.size_hint()
    }
}