use std::fmt::Debug;
use std::iter::FusedIterator;

/// A query which can generate type-safe matches and captures,
/// which contain [TypedNode]s
pub trait TypedQuery {
    /// A match of this typed query (runtime pattern index)
    type Match<'cursor, 'tree>: TypedQueryMatch<'cursor, 'tree>;
    /// An capture of this typed query (runtime capture index)
    type Capture<'tree>: TypedQueryCapture<'tree>;

    /// The string used to generate this query
    fn query_str(&self) -> &'static str;

    /// The underlying [tree_sitter::Query]
    fn query(&self) -> &'static tree_sitter::Query;

    /// Wrap a match which you know came from this query
    ///
    /// SAFETY: The match must have come from this query
    unsafe fn wrap_match<'cursor, 'tree>(
        &self,
        match_: tree_sitter::QueryMatch<'cursor, 'tree>
    ) -> Self::Match<'cursor, 'tree>;

    /// Wrap a capture which you know came from this query
    ///
    /// SAFETY: The capture must have come from this query
    unsafe fn wrap_capture<'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>
    ) -> Self::Capture<'tree>;
}

/// A match from a [TypedQuery] with [TypedNode]s
pub trait TypedQueryMatch<'cursor, 'tree>: Debug {
    /// The type of query this match came from
    type Query: TypedQuery;

    /// The query this match came from
    fn query(&self) -> &'static Self::Query;

    /// The underlying [tree_sitter::QueryMatch]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree>;

    /// Destruct into the underlying [tree_sitter::QueryMatch]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree>;

    /// See [tree_sitter::QueryMatch::captures]
    fn captures(&self) -> TypedQueryMatchCaptures<'cursor, 'tree, Self::Query> {
        // SAFETY: Captures come from the same query
        TypedQueryMatchCaptures {
            query: self.query(),
            captures: self.raw().captures
        }
    }

    /// See [tree_sitter::QueryMatch::remove]
    fn remove(self) {
        self.into_raw().remove()
    }
}

/// A capture from a [TypedQuery] with [TypedNode]s
pub trait TypedQueryCapture<'tree>: Debug + Clone + Copy {
    /// The type of query this capture came from
    type Query: TypedQuery;

    /// The query this capture came from
    fn query(&self) -> &'static Self::Query;

    /// Get the equivalent [tree_sitter::QueryCapture]
    fn to_raw(&self) -> tree_sitter::QueryCapture<'tree>;
}

/// Captures from a [TypedQueryMatch]
pub struct TypedQueryMatchCaptures<'cursor, 'tree, Query: TypedQuery> {
    query: &'static Query,
    captures: &'cursor [tree_sitter::QueryCapture<'tree>]
}

/// Iterate captures from a [TypedQueryMatch]
pub struct TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query: TypedQuery> {
    captures: TypedQueryMatchCaptures<'cursor, 'tree, Query>,
    index: usize,
    limit: usize
}

impl<'cursor, 'tree, Query: TypedQuery> TypedQueryMatchCaptures<'cursor, 'tree, Query> {
    /// Iterate the captures
    pub fn iter(&self) -> impl Iterator<Item = Query::Capture<'tree>> {
        self.captures.iter().map(|capture| {
            // SAFETY: Captures come from the same query
            unsafe { self.query.wrap_capture(*capture) }
        })
    }

    /// Get the capture at the index
    pub fn get(&self, index: usize) -> Option<Query::Capture<'tree>> {
        self.captures.get(index).map(|capture| {
            // SAFETY: Captures come from the same query
            unsafe { self.query.wrap_capture(*capture) }
        })
    }

    /// Get the capture at the index. **Panics** if the index is out of bounds
    pub fn index(&self, index: usize) -> Query::Capture<'tree> {
        // SAFETY: Captures come from the same query
        unsafe { self.query.wrap_capture(self.captures[index]) }
    }

    /// Are there any captures?
    pub fn is_empty(&self) -> bool {
        self.captures.is_empty()
    }

    /// Get the number of captures
    pub fn len(&self) -> usize {
        self.captures.len()
    }
}

impl<'cursor, 'tree, Query: TypedQuery> IntoIterator for TypedQueryMatchCaptures<'cursor, 'tree, Query> {
    type Item = TypedQueryMatchCapture<'cursor, 'tree>;
    type IntoIter = TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query>;

    fn into_iter(self) -> Self::IntoIter {
        TypedQueryMatchCapturesIntoIter {
            captures: self,
            index: 0,
            limit: self.len()
        }
    }
}

impl<'cursor, 'tree, Query: TypedQuery> Iterator for TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query> {
    type Item = Query::Capture<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.limit {
            return None
        }
        let capture = self.captures.index(self.index);
        self.index += 1;
        Some(capture)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.limit - self.index;
        (len, Some(len))
    }
}

impl<'cursor, 'tree, Query: TypedQuery> DoubleEndedIterator for TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.limit {
            return None
        }
        self.limit -= 1;
        let capture = self.captures.index(self.limit);
        Some(capture)
    }
}

impl<'cursor, 'tree, Query: TypedQuery> ExactSizeIterator for TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query> {
    fn len(&self) -> usize {
        self.limit - self.index
    }
}

impl<'cursor, 'tree, Query: TypedQuery> FusedIterator for TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query> {}