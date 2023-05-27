#[cfg(feature = "yak-sitter")]
use yak_sitter::{QueryCursor, PointRange};
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::{Point, QueryCursor, TextProvider};
use crate::{TypedNode, TypedQuery, TypedQueryCaptures, TypedQueryMatches};

/// Wraps [QueryCursor] where `matches` and `captures` are always typed
pub struct TypedQueryCursor(QueryCursor);

/// Allows [QueryCursor] to run typed queries.
pub trait QueryCursorExt {
    /// Run a typed query on the cursor, iterating over the matches in order they are found.
    ///
    /// Each match contains the index of the pattern that matched, and a list of captures. Because
    /// multiple patterns can match the same set of nodes, one match may contain captures that
    /// appear before some of the captures from a previous match.
    #[cfg(feature = "yak-sitter")]
    fn typed_matches<'cursor, 'tree: 'cursor, Query: TypedQuery>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
    ) -> TypedQueryMatches<'cursor, 'tree, Query>;

    /// Run a typed query on the cursor, iterating over the matches in order they are found.
    ///
    /// Each match contains the index of the pattern that matched, and a list of captures. Because
    /// multiple patterns can match the same set of nodes, one match may contain captures that
    /// appear before some of the captures from a previous match.
    #[cfg(not(feature = "yak-sitter"))]
    fn typed_matches<'cursor, 'tree, Query: TypedQuery, Text: TextProvider<'cursor> + 'cursor>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
        text: Text
    ) -> TypedQueryMatches<'cursor, 'tree, Query, Text>;

    /// Run a typed query on the cursor, iterating over the captures in order they appear.
    ///
    /// This is useful if you don’t care about which pattern matched, and just want a single,
    /// ordered sequence of captures.
    #[cfg(feature = "yak-sitter")]
    fn typed_captures<'cursor, 'tree: 'cursor, Query: TypedQuery>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
    ) -> TypedQueryCaptures<'cursor, 'tree, Query>;

    /// Run a typed query on the cursor, iterating over the captures in order they appear.
    ///
    /// This is useful if you don’t care about which pattern matched, and just want a single,
    /// ordered sequence of captures.
    #[cfg(not(feature = "yak-sitter"))]
    fn typed_captures<'cursor, 'tree, Query: TypedQuery, Text: TextProvider<'cursor> + 'cursor>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
        text: Text
    ) -> TypedQueryCaptures<'cursor, 'tree, Query, Text>;
}

impl QueryCursorExt for QueryCursor {
    #[cfg(feature = "yak-sitter")]
    #[inline]
    fn typed_matches<'cursor, 'tree: 'cursor, Query: TypedQuery>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
    ) -> TypedQueryMatches<'cursor, 'tree, Query> {
        let (untyped_matches, _, tree) =
            self.matches(query.query(), node.into_node()).into_inner();
        unsafe {
            TypedQueryMatches::new(
                query,
                untyped_matches,
                tree
            )
        }
    }

    #[cfg(not(feature = "yak-sitter"))]
    #[inline]
    fn typed_matches<'cursor, 'tree, Query: TypedQuery, Text: TextProvider<'cursor> + 'cursor>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
        text: Text
    ) -> TypedQueryMatches<'cursor, 'tree, Query, Text> {
        unsafe {
            TypedQueryMatches::new(
                query,
                self.matches(query.query(), node.into_node(), text)
            )
        }
    }

    #[cfg(feature = "yak-sitter")]
    #[inline]
    fn typed_captures<'cursor, 'tree: 'cursor, Query: TypedQuery>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
    ) -> TypedQueryCaptures<'cursor, 'tree, Query> {
        let (untyped_captures, _, tree) =
            self.captures(query.query(), node.into_node()).into_inner();
        unsafe {
            TypedQueryCaptures::new(
                query,
                untyped_captures,
                tree
            )
        }
    }

    #[cfg(not(feature = "yak-sitter"))]
    #[inline]
    fn typed_captures<'cursor, 'tree, Query: TypedQuery, Text: TextProvider<'cursor> + 'cursor>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
        text: Text
    ) -> TypedQueryCaptures<'cursor, 'tree, Query, Text> {
        unsafe {
            TypedQueryCaptures::new(
                query,
                self.captures(query.query(), node.into_node(), text)
            )
        }
    }
}

impl TypedQueryCursor {
    /// Create a new cursor for executing a given query.
    ///
    /// The cursor stores the state that is needed to iteratively search for matches.
    #[inline]
    pub fn new() -> Self {
        Self(QueryCursor::new())
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
        self.0.match_limit() as u16
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
        self.0.set_match_limit(limit as u32)
    }

    /// Check if, on its last execution, this cursor exceeded its maximum number of in-progress matches.
    #[inline]
    pub fn did_exceed_match_limit(&self) -> bool {
        self.0.did_exceed_match_limit()
    }

    /// Set the range in which the query will be executed, in terms of byte offsets.
    ///
    /// Returns `self` for convenience (builder pattern)
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) -> &mut Self {
        self.0.set_byte_range(range);
        self
    }

    /// Set the range in which the query will be executed, in terms of rows and columns.
    ///
    /// Returns `self` for convenience (builder pattern)
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn set_point_range(&mut self, range: PointRange) -> &mut Self {
        self.0.set_point_range(range);
        self
    }

    /// Set the range in which the query will be executed, in terms of rows and columns.
    ///
    /// Returns `self` for convenience (builder pattern)
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
    pub fn matches<'cursor, 'tree: 'cursor, Query: TypedQuery>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
    ) -> TypedQueryMatches<'cursor, 'tree, Query> {
        self.0.typed_matches(query, node)
    }

    /// Run a typed query on the cursor, iterating over the matches in order they are found.
    ///
    /// Each match contains the index of the pattern that matched, and a list of captures. Because
    /// multiple patterns can match the same set of nodes, one match may contain captures that
    /// appear before some of the captures from a previous match.
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn matches<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<'cursor> + 'cursor>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
        text: Text
    ) -> TypedQueryMatches<'cursor, 'tree, Query, Text> {
        self.0.typed_matches(query, node, text)
    }

    /// Run a typed query on the cursor, iterating over the captures in order they appear.
    ///
    /// This is useful if you don’t care about which pattern matched, and just want a single,
    /// ordered sequence of captures.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn captures<'cursor, 'tree: 'cursor, Query: TypedQuery>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
    ) -> TypedQueryCaptures<'cursor, 'tree, Query> {
        self.0.typed_captures(query, node)
    }

    /// Run a typed query on the cursor, iterating over the captures in order they appear.
    ///
    /// This is useful if you don’t care about which pattern matched, and just want a single,
    /// ordered sequence of captures.
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn captures<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<'cursor> + 'cursor>(
        &'cursor mut self,
        query: &'cursor Query,
        node: impl TypedNode<'tree>,
        text: Text
    ) -> TypedQueryCaptures<'cursor, 'tree, Query, Text> {
        self.0.typed_captures(query, node, text)
    }
}