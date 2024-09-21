use crate::query::match_captures::QueryMatchCaptures;
use crate::{raw, Query};
use std::fmt::Debug;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::Point;
#[cfg(feature = "yak-sitter")]
use yak_sitter::PointRange;

/// Iterate a typed query's matches (see [tree-sitter's `QueryMatches`](raw::QueryMatches)).
///
/// `QueryMatches` (in both this crate and tree-sitter) is NOT a real iterator, it's a
///     [`StreamingIterator`] (see <https://github.com/tree-sitter/tree-sitter/issues/608>).
///     Therefore this doesn't implement [`Iterator`].
#[cfg(feature = "yak-sitter")]
pub struct QueryMatches<'query, 'tree: 'query, Query: crate::Query> {
    typed_query: &'query Query,
    untyped_matches: tree_sitter::QueryMatches<'query, 'tree, &'tree raw::Tree, &'tree str>,
    untyped_query: &'query raw::Query,
    tree: &'tree raw::Tree,
    current_match: Option<Query::Match<'query, 'tree>>,
}

/// Iterate a typed query's matches (see [tree-sitter's `QueryMatches`](raw::QueryMatches)).
///
/// `QueryMatches` (in both this crate and tree-sitter) is NOT a real iterator, it's a
///     [`StreamingIterator`] (see <https://github.com/tree-sitter/tree-sitter/issues/608>).
///     Therefore this doesn't implement [`Iterator`].
#[cfg(not(feature = "yak-sitter"))]
pub struct QueryMatches<'query, 'tree: 'query, Query: crate::Query + 'tree, Text: raw::TextProvider<I>, I: AsRef<[u8]>> {
    typed_query: &'query Query,
    untyped_matches: raw::QueryMatches<'query, 'tree, Text, I>,
    current_match: Option<Query::Match<'query, 'tree>>,
}

/// A match from a [Query] with [typed nodes](Node)
pub trait QueryMatch<'query, 'tree: 'query>: Debug {
    /// The type of query this match came from
    type Query: Query<Match<'query, 'tree> = Self>;

    /// The query this match came from
    fn query(&self) -> &'query Self::Query;

    /// The tree this match came from
    #[cfg(feature = "yak-sitter")]
    fn tree(&self) -> &'tree raw::Tree;

    /// The underlying tree-sitter [`QueryMatch`]
    fn raw(&self) -> &raw::QueryMatch<'query, 'tree>;

    /// Destruct into the underlying tree-sitter [`QueryMatch`]
    fn into_raw(self) -> raw::QueryMatch<'query, 'tree>;

    /// See [tree-sitter's `QueryMatch::captures`](raw::QueryMatch::captures)
    #[cfg(feature = "yak-sitter")]
    #[inline]
    fn captures(&self) -> QueryMatchCaptures<'query, 'tree, Self::Query> {
        // SAFETY: Captures come from the same query
        unsafe { QueryMatchCaptures::new(
            self.query(),
            self.raw().as_inner().captures,
            self.tree()
        ) }
    }

    /// See [tree-sitter's `QueryMatch::captures`](raw::QueryMatch::captures)
    #[cfg(not(feature = "yak-sitter"))]
    #[inline]
    fn captures(&self) -> QueryMatchCaptures<'query, 'tree, Self::Query> {
        // SAFETY: Captures come from the same query
        unsafe { QueryMatchCaptures::new(self.query(), self.raw().captures) }
    }

    /// Remove the match (honestly I don't know what this does because it's not documented)
    // I don't know why `tree: 'query` is required, since it's not in any bounds from anything in
    // the function body.
    #[inline]
    fn remove(self) where Self: Sized, 'tree: 'query {
        self.into_raw().remove()
    }
}

#[cfg(feature = "yak-sitter")]
impl<'query, 'tree: 'query, Query: crate::Query + 'tree> QueryMatches<'query, 'tree, Query> {
    /// Wrap untyped matches along with the query.
    ///
    /// # Safety
    /// The matches must have come from the same query.
    #[inline]
    pub(super) unsafe fn from_raw(
        typed_query: &'query Query,
        untyped_matches: raw::QueryMatches<'query, 'tree>
    ) -> Self {
        let (untyped_matches, untyped_query, tree) =
            untyped_matches.into_inner();
        Self {
            typed_query,
            untyped_matches,
            untyped_query,
            tree,
            current_match: None
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
}

#[cfg(not(feature = "yak-sitter"))]
impl<'query, 'tree: 'query, Query: crate::Query + 'tree, Text: raw::TextProvider<I>, I: AsRef<[u8]>> QueryMatches<'query, 'tree, Query, Text, I> {
    /// Wrap untyped matches along with the query.
    ///
    /// # Safety
    /// The matches must have come from the same query.
    #[inline]
    pub(super) unsafe fn from_raw(
        typed_query: &'query Query,
        untyped_matches: raw::QueryMatches<'query, 'tree, Text, I>
    ) -> Self {
        Self {
            typed_query,
            untyped_matches,
            current_match: None
        }
    }

    /// Limit matches to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_matches.set_byte_range(range)
    }

    /// Limit matches to a point range
    #[inline]
    pub fn set_point_range(&mut self, range: std::ops::Range<Point>) {
        self.untyped_matches.set_point_range(range)
    }
}

//noinspection DuplicatedCode
#[cfg(feature = "yak-sitter")]
impl<'query, 'tree: 'query, Query: crate::Query + 'tree> StreamingIterator for QueryMatches<'query, 'tree, Query> {
    type Item = Query::Match<'query, 'tree>;

    #[inline]
    fn advance(&mut self) {
        // SAFETY: Matches come from the same query and tree
        self.current_match = unsafe {
            self.untyped_matches.next().map(|m|
                self.typed_query.wrap_match(raw::QueryMatch::new(m, self.untyped_query, self.tree)))
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

//noinspection DuplicatedCode
#[cfg(not(feature = "yak-sitter"))]
impl<'query, 'tree: 'query, Query: crate::Query + 'tree, Text: raw::TextProvider<I>, I: AsRef<[u8]>> StreamingIterator for QueryMatches<'query, 'tree, Query, Text, I> {
    type Item = Query::Match<'query, 'tree>;

    #[inline]
    fn advance(&mut self) {
        // SAFETY: Matches come from the same query
        self.current_match = unsafe {
            self.untyped_matches.next().map(|m| self.typed_query.wrap_match(m))
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

//noinspection DuplicatedCode
#[cfg(feature = "yak-sitter")]
impl<'query, 'tree: 'query, Query: crate::Query + 'tree> StreamingIteratorMut for QueryMatches<'query, 'tree, Query> {
    #[inline]
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.current_match.as_mut()
    }
}

//noinspection DuplicatedCode
#[cfg(not(feature = "yak-sitter"))]
impl<'query, 'tree: 'query, Query: crate::Query + 'tree, Text: raw::TextProvider<I>, I: AsRef<[u8]>> StreamingIteratorMut for QueryMatches<'query, 'tree, Query, Text, I> {
    #[inline]
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.current_match.as_mut()
    }
}