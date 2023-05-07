use tree_sitter::Node;

/// Result of attempting to wrap a node
pub type NodeResult<'tree, T> = Result<T, IncorrectKind<'tree>>;

/// Error when attempting to wrap a node of the wrong kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IncorrectKind<'tree> {
    /// Node attempted to be wrapped
    pub node: Node<'tree>,
    /// Expected node kind
    pub kind: &'static str,
}

/// Underlying cause of why the node is the wrong kind
pub enum IncorrectKindCause {
    /// Node is an error node
    Error,
    /// Node is a missing node
    Missing,
    /// Node is an extra node
    Extra,
    /// Node is valid but simply of a different kind (bad node-types.json? Different language? Broken user invariant)
    OtherKind(&'static str),
}

impl<'tree> IncorrectKind<'tree> {
    /// Actual node kind
    #[inline]
    pub fn actual_kind(&self) -> &'static str {
        self.node.kind()
    }

    /// Is this an error node?
    #[inline]
    pub fn is_error(&self) -> bool {
        self.node.is_error()
    }

    /// Is this a missing node?
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.node.is_missing()
    }

    /// Is this an extra node?
    #[inline]
    pub fn is_extra(&self) -> bool {
        self.node.is_extra()
    }

    /// Underlying cause of why the node is the wrong kind
    #[inline]
    pub fn cause(&self) -> IncorrectKindCause {
        if self.is_error() {
            IncorrectKindCause::Error
        } else if self.is_missing() {
            IncorrectKindCause::Missing
        } else if self.is_extra() {
            IncorrectKindCause::Extra
        } else {
            IncorrectKindCause::OtherKind(self.actual_kind())
        }
    }
}