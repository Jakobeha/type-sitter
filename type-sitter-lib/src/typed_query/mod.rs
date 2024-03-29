#[cfg(feature = "yak-sitter")]
use yak_sitter::Tree;
pub use cursor_ext::*;
pub use match_captures::*;
pub use matches::*;
pub use captures::*;

/// [QueryCursorExt] to run typed queries
mod cursor_ext;
/// [TypedQueryMatchCaptures]
mod match_captures;
/// [TypedQueryMatches]
mod matches;
/// [TypedQueryCaptures]
mod captures;

/// A query which can generate type-safe matches and captures,
/// which contain [crate::TypedNode]s
pub trait TypedQuery {
    /// A match of this typed query (runtime pattern index)
    type Match<'cursor, 'tree: 'cursor>: TypedQueryMatch<'cursor, 'tree>;
    /// An capture of this typed query (runtime capture index)
    type Capture<'cursor, 'tree: 'cursor>: TypedQueryCapture<'cursor, 'tree>;

    /// The string used to generate this query
    fn query_str(&self) -> &'static str;

    /// The underlying [tree_sitter::Query]
    fn query(&self) -> &'static tree_sitter::Query;

    /// Wrap a match which you know came from this query
    ///
    /// SAFETY: The match must have come from this query
    unsafe fn wrap_match<'cursor, 'tree>(
        &self,
        r#match: tree_sitter::QueryMatch<'cursor, 'tree>,
        #[cfg(feature = "yak-sitter")]
        tree: &'tree Tree,
    ) -> Self::Match<'cursor, 'tree>;

    /// Wrap a capture which you know came from this query.
    /// If iterating [TypedQueryCaptures], `r#match` will contain the current match,
    /// but if iterating [TypedQueryMatchCaptures], `r#match` will be `None`.
    ///
    /// SAFETY: The capture must have come from this query.
    /// If `r#match` is `Some`, it must contain the capture.
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        r#match: Option<Self::Match<'cursor, 'tree>>,
        #[cfg(feature = "yak-sitter")]
        tree: &'tree Tree,
    ) -> Self::Capture<'cursor, 'tree>;
}
