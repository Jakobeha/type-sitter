use std::iter::FusedIterator;
#[cfg(feature = "yak-sitter")]
use crate::raw;

/// Captures from a [crate::QueryMatch]
pub struct QueryMatchCaptures<'query, 'tree, Query: crate::Query + 'tree> {
    query: &'query Query,
    captures: &'query [tree_sitter::QueryCapture<'tree>],
    #[cfg(feature = "yak-sitter")]
    tree: &'tree raw::Tree
}

/// Iterate captures from a [crate::QueryMatch]
pub struct QueryMatchCapturesIntoIter<'query, 'tree, Query: crate::Query + 'tree> {
    captures: QueryMatchCaptures<'query, 'tree, Query>,
    index: usize,
    limit: usize
}

impl<'query, 'tree, Query: crate::Query + 'tree> QueryMatchCaptures<'query, 'tree, Query> {
    /// Wraps an array of tree-sitter captures.
    ///
    /// This is actual tree-sitter, even when the `yak-sitter` feature is enabled.
    ///
    /// # Safety
    /// Captures must come from the same query, and if the `yak-sitter` feature is enabled, the same
    /// tree.
    pub(super) unsafe fn new(
        query: &'query Query,
        captures: &'query [tree_sitter::QueryCapture<'tree>],
        #[cfg(feature = "yak-sitter")]
        tree: &'tree raw::Tree
    ) -> Self {
        Self { query, captures, #[cfg(feature = "yak-sitter")] tree }
    }

    /// Iterate the captures
    pub fn iter(&self) -> impl Iterator<Item = Query::Capture<'query, 'tree>> + '_ {
        self.captures.iter().map(|capture| self.wrap_capture(*capture))
    }

    /// Get the capture at the index
    pub fn get(&self, index: usize) -> Option<Query::Capture<'query, 'tree>> {
        self.captures.get(index).map(|capture| self.wrap_capture(*capture))
    }

    /// Get the capture at the index. **Panics** if the index is out of bounds
    pub fn index(&self, index: usize) -> Query::Capture<'query, 'tree> {
        self.wrap_capture(self.captures[index])
    }

    fn wrap_capture(&self, capture: tree_sitter::QueryCapture<'tree>) -> Query::Capture<'query, 'tree> {
        // SAFETY: Captures come from the same query, and nodes from the same tree
        unsafe { self.query.wrap_capture(
            #[cfg(feature = "yak-sitter")]
            raw::QueryCapture {
                node: raw::Node::new(capture.node, self.tree),
                index: capture.index as usize,
                name: self.query.raw().capture_names()[capture.index as usize],
            },
            #[cfg(not(feature = "yak-sitter"))]
            capture,
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

impl<'query, 'tree, Query: crate::Query + 'tree> IntoIterator for QueryMatchCaptures<'query, 'tree, Query> {
    type Item = Query::Capture<'query, 'tree>;
    type IntoIter = QueryMatchCapturesIntoIter<'query, 'tree, Query>;

    fn into_iter(self) -> Self::IntoIter {
        let limit = self.len();
        QueryMatchCapturesIntoIter {
            captures: self,
            index: 0,
            limit
        }
    }
}

impl<'query, 'tree, Query: crate::Query + 'tree> Iterator for QueryMatchCapturesIntoIter<'query, 'tree, Query> {
    type Item = Query::Capture<'query, 'tree>;

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

impl<'query, 'tree, Query: crate::Query + 'tree> DoubleEndedIterator for QueryMatchCapturesIntoIter<'query, 'tree, Query> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.limit {
            return None
        }
        self.limit -= 1;
        let capture = self.captures.index(self.limit);
        Some(capture)
    }
}

impl<'query, 'tree, Query: crate::Query + 'tree> ExactSizeIterator for QueryMatchCapturesIntoIter<'query, 'tree, Query> {
    fn len(&self) -> usize {
        self.limit - self.index
    }
}

impl<'query, 'tree, Query: crate::Query + 'tree> FusedIterator for QueryMatchCapturesIntoIter<'query, 'tree, Query> {}
