use crate::{raw, Node, QueryCaptures, QueryMatches};
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::Point;
#[cfg(feature = "yak-sitter")]
use yak_sitter::PointRange;

/// Wraps [tree-sitter's `QueryCursor`](raw::QueryCursor) where `matches` and `captures` are always typed.
#[repr(transparent)]
pub struct QueryCursor(pub raw::QueryCursor);

impl QueryCursor {
    /// Create a new cursor for executing a given query.
    ///
    /// The cursor stores the state that is needed to iteratively search for matches.
    #[inline]
    pub fn new() -> Self {
        Self(raw::QueryCursor::new())
    }

    /// Return the maximum number of in-progress matches for this cursor.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn match_limit(&self) -> u16 {
        self.0.match_limit()
    }

    /// Return the maximum number of in-progress matches for this cursor.
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn match_limit(&self) -> u16 {
        u16::try_from(self.0.match_limit()).unwrap()
    }

    /// Set the maximum number of in-progress matches for this cursor.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn set_match_limit(&mut self, limit: u16) {
        self.0.set_match_limit(limit)
    }

    /// Set the maximum number of in-progress matches for this cursor.
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn set_match_limit(&mut self, limit: u16) {
        self.0.set_match_limit(u32::from(limit))
    }

    /// Check if, on its last execution, this cursor exceeded its maximum number of in-progress matches.
    #[inline]
    pub fn did_exceed_match_limit(&self) -> bool {
        self.0.did_exceed_match_limit()
    }

    /// Set the range in which the query will be executed, in terms of byte offsets.
    ///
    /// Like [tree-sitter's `QueryCursor::set_byte_range`](raw::QueryCursor::set_byte_range),
    /// returns `self` for convenience (builder pattern).
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) -> &mut Self {
        self.0.set_byte_range(range);
        self
    }

    /// Set the range in which the query will be executed, in terms of rows and columns.
    ///
    /// Like [tree-sitter's `QueryCursor::set_point_range`](raw::QueryCursor::set_point_range),
    /// returns `self` for convenience (builder pattern).
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn set_point_range(&mut self, range: PointRange) -> &mut Self {
        self.0.set_point_range(range);
        self
    }

    /// Set the range in which the query will be executed, in terms of rows and columns.
    ///
    /// Like [tree-sitter's `QueryCursor::set_point_range`](raw::QueryCursor::set_point_range),
    /// returns `self` for convenience (builder pattern).
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn set_point_range(&mut self, range: std::ops::Range<Point>) -> &mut Self {
        self.0.set_point_range(range);
        self
    }

    /// Run a typed query on the cursor, iterating over the matches in order they are found.
    ///
    /// Each match contains the index of the pattern that matched, and a list of captures. Because
    /// multiple patterns can match the same set of nodes, one match may contain captures that
    /// appear before some of the captures from a previous match.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn matches<'query, 'cursor: 'query, 'tree, Query: crate::Query + 'tree>(
        &'cursor mut self,
        query: &'query Query,
        node: impl Node<'tree>,
    ) -> QueryMatches<'query, 'tree, Query> {
        unsafe {
            QueryMatches::from_raw(query, self.0.matches(query.raw(), node.into_raw()))
        }
    }

    /// Run a typed query on the cursor, iterating over the matches in order they are found.
    ///
    /// Each match contains the index of the pattern that matched, and a list of captures. Because
    /// multiple patterns can match the same set of nodes, one match may contain captures that
    /// appear before some of the captures from a previous match.
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn matches<'query, 'cursor: 'query, 'tree, Query: crate::Query, Text: tree_sitter::TextProvider<I> + 'query, I: AsRef<[u8]>>(
        &'cursor mut self,
        query: &'query Query,
        node: impl Node<'tree>,
        text: Text
    ) -> QueryMatches<'query, 'tree, Query, Text, I> {
        unsafe {
            QueryMatches::from_raw(query, self.0.matches(query.raw(), node.into_raw(), text))
        }
    }

    /// Run a typed query on the cursor, iterating over the captures in order they appear.
    ///
    /// This is useful if you don’t care about which pattern matched, and just want a single,
    /// ordered sequence of captures.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn captures<'query, 'cursor: 'query, 'tree, Query: crate::Query>(
        &'cursor mut self,
        query: &'query Query,
        node: impl Node<'tree>,
    ) -> QueryCaptures<'query, 'tree, Query> {
        unsafe {
            QueryCaptures::new(query, self.0.captures(query.raw(), node.into_raw()))
        }
    }

    /// Run a typed query on the cursor, iterating over the captures in order they appear.
    ///
    /// This is useful if you don’t care about which pattern matched, and just want a single,
    /// ordered sequence of captures.
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn captures<'query, 'cursor: 'query, 'tree, Query: crate::Query, Text: tree_sitter::TextProvider<I> + 'query, I: AsRef<[u8]>>(
        &'cursor mut self,
        query: &'query Query,
        node: impl Node<'tree>,
        text: Text
    ) -> QueryCaptures<'query, 'tree, Query, Text, I> {
        unsafe {
            QueryCaptures::new(query, self.0.captures(query.raw(), node.into_raw(), text))
        }
    }
}