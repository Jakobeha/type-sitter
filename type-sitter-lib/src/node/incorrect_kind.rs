use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::{raw, Node, UntypedNode};

/// Result of attempting to wrap a node
pub type NodeResult<'tree, T> = Result<T, IncorrectKind<'tree>>;

/// Error when attempting to wrap a node of the wrong kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IncorrectKind<'tree> {
    /// Node attempted to be wrapped
    pub node: UntypedNode<'tree>,
    /// Expected node kind
    pub kind: &'static str,
}

/// Underlying cause of why the node is the wrong kind
pub enum IncorrectKindCause {
    /// Node is an error node
    Error,
    /// Node is a missing node
    Missing,
    /// Node is valid but simply of a different kind (bad node-types.json? Different language? Broken user invariant)
    OtherKind(&'static str),
}

#[derive(Debug)]
struct DisplayWithSourceRange<'a, 'tree>(&'a IncorrectKind<'tree>);

impl<'tree> IncorrectKind<'tree> {
    /// Create an error for another tree-sitter node when a typed node was expected.
    ///
    /// `Node` is the type of node that was expected.
    #[inline]
    pub fn new<Node: crate::Node<'tree>>(node: raw::Node<'tree>) -> Self {
        Self { node: UntypedNode::new(node), kind: Node::KIND }
    }

    /// The actual kind of node that we encountered, not `kind` which is the one we expected.
    #[inline]
    pub fn actual_kind(&self) -> &'static str {
        self.node.kind()
    }

    /// Is the actual node an error?
    #[inline]
    pub fn is_error(&self) -> bool {
        self.node.is_error()
    }

    /// Is the actual node missing?
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.node.is_missing()
    }

    /// Whether the node is an error, missing, extra, or is just some other node of unexpected kind.
    ///
    /// Typically [`IncorrectKind`] comes from an error or missing node. The latter cases (extra or
    /// or other) are more likely to be a bug in the code or the tree-sitter grammar, not the parsed
    /// AST.
    #[inline]
    pub fn cause(&self) -> IncorrectKindCause {
        if self.is_error() {
            IncorrectKindCause::Error
        } else if self.is_missing() {
            IncorrectKindCause::Missing
        } else {
            IncorrectKindCause::OtherKind(self.actual_kind())
        }
    }

    /// Print an error message for this with the source range.
    ///
    /// The default `impl Display` prints an error message without the source range.
    #[inline]
    pub fn with_source_range(&self) -> impl Display + '_ {
        DisplayWithSourceRange(self)
    }
}

impl<'tree, T: Node<'tree>> Node<'tree> for NodeResult<'tree, T> {
    type WithLifetime<'a> = NodeResult<'a, T::WithLifetime<'a>>;
    
    const KIND: &'static str = "{result}";

    fn try_from_raw(node: raw::Node<'tree>) -> NodeResult<'tree, Self> {
        Ok(T::try_from_raw(node))
    }

    unsafe fn from_raw_unchecked(node: raw::Node<'tree>) -> Self {
        T::try_from_raw(node)
    }

    fn raw(&self) -> &raw::Node<'tree> {
        match self {
            Ok(t) => t.raw(),
            Err(e) => e.node.raw(),
        }
    }

    fn raw_mut(&mut self) -> &mut raw::Node<'tree> {
        match self {
            Ok(t) => t.raw_mut(),
            Err(e) => e.node.raw_mut(),
        }
    }

    fn into_raw(self) -> raw::Node<'tree> {
        match self {
            Ok(t) => t.into_raw(),
            Err(e) => e.node.into_raw(),
        }
    }
}

#[cfg(feature = "yak-sitter")]
impl Display for DisplayWithSourceRange<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}: {}", self.0.node.range(), self.0)
    }
}

#[cfg(not(feature = "yak-sitter"))]
impl Display for DisplayWithSourceRange<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "at ")?;

        // Inline yak-sitter's `impl Display for Range`.
        let range = self.0.node.range();
        if range.start_byte == range.end_byte {
            write!(f, "{}:{}", range.start_point.row + 1, range.start_point.column + 1)
        } else if range.start_point.row == range.end_point.row {
            write!(f, "{}:{}-{}", range.start_point.row + 1, range.start_point.column + 1, range.end_point.column + 1)
        } else {
            write!(f, "{}:{}-{}:{}", range.start_point.row + 1, range.start_point.column + 1, range.end_point.row + 1, range.end_point.column + 1)
        }?;

        write!(f, ": {}", self.0)
    }
}

impl Display for IncorrectKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.cause() {
            IncorrectKindCause::Error => write!(f, "expected {}", self.kind),
            IncorrectKindCause::Missing => write!(f, "missing {}", self.kind),
            IncorrectKindCause::OtherKind(other) => write!(f, "expected {}, got {}", self.kind, other),
        }
    }
}

impl Error for IncorrectKind<'_> {}