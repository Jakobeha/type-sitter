use std::fmt::Debug;
use tree_sitter::TextProvider;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};
#[cfg(feature = "yak-sitter")]
use yak_sitter::{PointRange, Tree};
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::Point;
use crate::typed_query::match_captures::TypedQueryMatchCaptures;
use crate::TypedQuery;

/// Iterate a typed query's matches (see [tree_sitter::QueryMatches])
///
/// [tree_sitter::QueryMatches] is NOT a real iterator, it's a [StreamingIterator] (see
///     <https://github.com/tree-sitter/tree-sitter/issues/608>). Therefore this doesn't implement
///     [Iterator]
#[cfg(feature = "yak-sitter")]
pub struct TypedQueryMatches<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I> = &'cursor Tree, I: AsRef<[u8]> = &'cursor str> {
    typed_query: &'cursor Query,
    untyped_matches: tree_sitter::QueryMatches<'cursor, 'tree, Text, I>,
    current_match: Option<Query::Match<'cursor, 'tree>>,
    tree: &'tree Tree,
}

/// Iterate a typed query's matches (see [tree_sitter::QueryMatches])
///
/// [tree_sitter::QueryMatches] is NOT a real iterator, it's a [StreamingIterator] (see
///     <https://github.com/tree-sitter/tree-sitter/issues/608>). Therefore this doesn't implement
///     [Iterator]
#[cfg(not(feature = "yak-sitter"))]
pub struct TypedQueryMatches<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> {
    typed_query: &'cursor Query,
    untyped_matches: tree_sitter::QueryMatches<'cursor, 'tree, Text, I>,
    current_match: Option<Query::Match<'cursor, 'tree>>,
}

/// A match from a [TypedQuery] with [crate::TypedNode]s
pub trait TypedQueryMatch<'cursor, 'tree: 'cursor>: Debug {
    /// The type of query this match came from
    type Query: TypedQuery<Match<'cursor, 'tree> = Self>;

    /// The query this match came from
    fn query(&self) -> &'cursor Self::Query;

    /// The tree this match came from
    #[cfg(feature = "yak-sitter")]
    fn tree(&self) -> &'tree Tree;

    /// The underlying [tree_sitter::QueryMatch]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree>;

    /// Destruct into the underlying [tree_sitter::QueryMatch]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree>;

    /// See [tree_sitter::QueryMatch::captures]
    #[inline]
    fn captures(&self) -> TypedQueryMatchCaptures<'cursor, 'tree, Self::Query> {
        // SAFETY: Captures come from the same query
        unsafe { TypedQueryMatchCaptures::new(
            self.query(),
            self.raw().captures,
            #[cfg(feature = "yak-sitter")] self.tree()
        ) }
    }

    /// See [tree_sitter::QueryMatch::remove]
    #[inline]
    fn remove(self) where Self: Sized {
        self.into_raw().remove()
    }
}

impl<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> TypedQueryMatches<'cursor, 'tree, Query, Text, I> {
    /// SAFETY: The matches must have come from the same query
    #[inline]
    pub(super) unsafe fn new(
        typed_query: &'cursor Query,
        untyped_matches: tree_sitter::QueryMatches<'cursor, 'tree, Text, I>,
        #[cfg(feature = "yak-sitter")]
        tree: &'tree Tree,
    ) -> Self {
        Self {
            typed_query,
            untyped_matches,
            current_match: None,
            #[cfg(feature = "yak-sitter")] tree
        }
    }

    /// Limit matches to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_matches.set_byte_range(range)
    }

    /// Limit matches to a point range
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn set_point_range(&mut self, range: PointRange) {
        self.untyped_matches.set_point_range(range.to_ts_point_range())
    }

    /// Limit matches to a point range
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn set_point_range(&mut self, range: std::ops::Range<Point>) {
        self.untyped_matches.set_point_range(range)
    }
}

impl<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> StreamingIterator for TypedQueryMatches<'cursor, 'tree, Query, Text, I> {
    type Item = Query::Match<'cursor, 'tree>;

    #[inline]
    fn advance(&mut self) {
        // SAFETY: Matches come from the same query
        self.current_match = unsafe {
            self.untyped_matches.next().map(|m| self.typed_query.wrap_match(
                m,
                #[cfg(feature = "yak-sitter")] self.tree
            ))
        }
    }

    #[inline]
    fn get(&self) -> Option<&Self::Item> {
        self.current_match.as_ref()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.untyped_matches.size_hint()
    }
}

impl<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<I>, I: AsRef<[u8]>> StreamingIteratorMut for TypedQueryMatches<'cursor, 'tree, Query, Text, I> {
    #[inline]
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.current_match.as_mut()
    }
}