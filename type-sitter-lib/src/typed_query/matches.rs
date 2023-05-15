use std::fmt::Debug;
use tree_sitter::TextProvider;
#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::{Range, Tree};
#[cfg(not(feature = "tree-sitter-wrapper"))]
use tree_sitter::Range;
use crate::typed_query::match_captures::TypedQueryMatchCaptures;
use crate::TypedQuery;

/// Iterate a typed query's matches (see [tree_sitter::QueryMatches])
#[cfg(feature = "tree-sitter-wrapper")]
pub struct TypedQueryMatches<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<'cursor> = &'cursor Tree> {
    typed_query: &'cursor Query,
    untyped_matches: tree_sitter::QueryMatches<'cursor, 'tree, Text>,
    tree: &'tree Tree,
}

/// Iterate a typed query's matches (see [tree_sitter::QueryMatches])
#[cfg(not(feature = "tree-sitter-wrapper"))]
pub struct TypeQueryMatches<'cursor, 'tree, Query: TypedQuery, Text: TextProvider<'cursor>> {
    typed_query: &'cursor Query,
    untyped_matches: tree_sitter::QueryMatches<'cursor, 'tree, Text>,
}

/// A match from a [TypedQuery] with [TypedNode]s
pub trait TypedQueryMatch<'cursor, 'tree: 'cursor>: Debug {
    /// The type of query this match came from
    type Query: TypedQuery<Match<'cursor, 'tree> = Self>;

    /// The query this match came from
    fn query(&self) -> &'cursor Self::Query;

    /// The tree this match came from
    #[cfg(feature = "tree-sitter-wrapper")]
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
            #[cfg(feature = "tree-sitter-wrapper")] self.tree()
        ) }
    }

    /// See [tree_sitter::QueryMatch::remove]
    #[inline]
    fn remove(self) where Self: Sized {
        self.into_raw().remove()
    }
}

impl<'cursor, 'tree: 'cursor, Query: TypedQuery, Text: TextProvider<'cursor>> TypedQueryMatches<'cursor, 'tree, Query, Text> {
    /// SAFETY: The matches must have come from the same query
    #[inline]
    pub(super) unsafe fn new(
        typed_query: &'cursor Query,
        untyped_matches: tree_sitter::QueryMatches<'cursor, 'tree, Text>,
        #[cfg(feature = "tree-sitter-wrapper")]
        tree: &'tree Tree,
    ) -> Self {
        Self { typed_query, untyped_matches, #[cfg(feature = "tree-sitter-wrapper")] tree }
    }

    /// Limit matches to a byte range
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.untyped_matches.set_byte_range(range)
    }

    /// Limit matches to a point range
    #[inline]
    #[cfg(feature = "tree-sitter-wrapper")]
    pub fn set_point_range(&mut self, range: Range) {
        self.untyped_matches.set_point_range(std::ops::Range {
            start: range.start_point().into(),
            end: range.end_point().into()
        })
    }

    /// Limit matches to a point range
    #[inline]
    #[cfg(not(feature = "tree-sitter-wrapper"))]
    pub fn set_point_range(&mut self, range: Range) {
        self.untyped_matches.set_point_range(std::ops::Range {
            start: range.start_point(),
            end: range.end_point()
        })
    }
}

impl<'cursor, 'tree, Query: TypedQuery, Text: TextProvider<'cursor>> Iterator for TypedQueryMatches<'cursor, 'tree, Query, Text> {
    type Item = Query::Match<'cursor, 'tree>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: Matches come from the same query
        unsafe { self.untyped_matches.next().map(|m| self.typed_query.wrap_match(
            m,
            #[cfg(feature = "tree-sitter-wrapper")] self.tree
        )) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.untyped_matches.size_hint()
    }
}
