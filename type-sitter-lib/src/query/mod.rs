pub use captures::*;
pub use cursor::*;
pub use match_captures::*;
pub use matches::*;
use crate::raw;

/// [`QueryCursorExt`] to run typed queries
mod cursor;
/// [`QueryMatchCaptures`]
mod match_captures;
/// [`QueryMatches`]
mod matches;
/// [`QueryCaptures`]
mod captures;

/// A query which can generate type-safe matches and captures, which contain [typed nodes](Node)
pub trait Query {
    /// A match of this typed query (runtime pattern index)
    type Match<'query, 'tree: 'query>: QueryMatch<'query, 'tree>;
    /// An capture of this typed query (runtime capture index)
    type Capture<'query, 'tree: 'query>: QueryCapture<'query, 'tree>;

    /// The string used to generate this query
    fn as_str(&self) -> &'static str;

    /// The underlying tree-sitter `Query`
    fn raw(&self) -> &'static raw::Query;

    /// Wrap a tree-sitter `QueryMatch` which you know came from this query.
    ///
    /// # Safety
    /// The match must have come from this query.
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: raw::QueryMatch<'query, 'tree>
    ) -> Self::Match<'query, 'tree>;

    //noinspection RsDuplicatedTraitMethodBinding -- IntelliJ inspection bug.
    /// Wrap a tree-sitter `QueryCapture` which you know came from this query.
    ///
    /// If iterating [`QueryCaptures`], `r#match` will contain the current match, but if iterating
    /// [`QueryMatchCaptures`], `r#match` will be `None`.
    ///
    /// # Safety
    /// The capture must have come from this query. Additionally, if `r#match` is `Some`, it must
    /// contain the capture.
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        #[cfg(feature = "yak-sitter")]
        capture: raw::QueryCapture<'query, 'tree>,
        #[cfg(not(feature = "yak-sitter"))]
        capture: raw::QueryCapture<'tree>,
        r#match: Option<Self::Match<'query, 'tree>>
    ) -> Self::Capture<'query, 'tree>;
}
