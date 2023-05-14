use std::iter::FusedIterator;
#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::Tree;
use crate::typed_query::matches::TypedQueryMatch;
use crate::TypedQuery;

/// Captures from a [TypedQueryMatch]
pub struct TypedQueryMatchCaptures<'cursor, 'tree, Query: TypedQuery> {
    query: &'static Query,
    captures: &'cursor [tree_sitter::QueryCapture<'tree>],
    #[cfg(feature = "tree-sitter-wrapper")]
    tree: &'tree Tree
}

/// Iterate captures from a [TypedQueryMatch]
pub struct TypedQueryMatchCapturesIntoIter<'cursor, 'tree, Query: TypedQuery> {
    captures: TypedQueryMatchCaptures<'cursor, 'tree, Query>,
    index: usize,
    limit: usize
}

impl<'cursor, 'tree, Query: TypedQuery> TypedQueryMatchCaptures<'cursor, 'tree, Query> {
    /// SAFETY: Captures must come from the same query
    pub(super) unsafe fn new(
        query: &'static Query,
        captures: &'cursor [tree_sitter::QueryCapture<'tree>],
        #[cfg(feature = "tree-sitter-wrapper")]
        tree: &'tree Tree
    ) -> Self {
        Self { query, captures, #[cfg(feature = "tree-sitter-wrapper")] tree }
    }

    /// Iterate the captures
    pub fn iter(&self) -> impl Iterator<Item = Query::Capture<'cursor, 'tree>> {
        self.captures.iter().map(|capture| {
            // SAFETY: Captures come from the same query
            unsafe { self.query.wrap_capture(
                *capture,
                None,
                #[cfg(feature = "tree-sitter-wrapper")] self.tree
            ) }
        })
    }

    /// Get the capture at the index
    pub fn get(&self, index: usize) -> Option<Query::Capture<'cursor, 'tree>> {
        self.captures.get(index).map(|capture| {
            // SAFETY: Captures come from the same query
            unsafe { self.query.wrap_capture(
                *capture,
                None,
                #[cfg(feature = "tree-sitter-wrapper")] self.tree
            ) }
        })
    }

    /// Get the capture at the index. **Panics** if the index is out of bounds
    pub fn index(&self, index: usize) -> Query::Capture<'cursor, 'tree> {
        // SAFETY: Captures come from the same query
        unsafe { self.query.wrap_capture(
            self.captures[index],
            None,
            #[cfg(feature = "tree-sitter-wrapper")] self.tree
        ) }
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
    type Item = Query::Capture<'cursor, 'tree>;

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
