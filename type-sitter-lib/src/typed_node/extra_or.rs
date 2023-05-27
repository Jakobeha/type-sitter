use std::fmt::Debug;
#[cfg(feature = "yak-sitter")]
use yak_sitter::Node;
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::Node;
use crate::{IncorrectKind, TypedNode};

/// May be a comment or other "extra" instead of the positionally-expected node kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraOr<'tree, T> {
    /// A comment or other "extra"
    Extra(Node<'tree>),
    /// A regular node
    Regular(T)
}

impl<'tree, T> ExtraOr<'tree, T> {
    /// Unwrap the value, *panicking* if it's an extra
    pub fn unwrap(self) -> T {
        match self {
            ExtraOr::Extra(node) => panic!("ExtraOr::unwrap called on Extra({:?})", node),
            ExtraOr::Regular(value) => value
        }
    }

    /// Unwrap the value, *panicking* if it's regular
    pub fn unwrap_extra(self) -> Node<'tree> where T: Debug {
        match self {
            ExtraOr::Extra(node) => node,
            ExtraOr::Regular(value) => panic!("ExtraOr::unwrap_extra called on Regular({:?})", value)
        }
    }

    /// Unwrap the value, *panicking* if it's an extra with the given message
    pub fn expect(self, message: impl AsRef<str>) -> T {
        match self {
            ExtraOr::Extra(node) => panic!("ExtraOr::expect called on Extra({:?}): {}", node, message.as_ref()),
            ExtraOr::Regular(value) => value
        }
    }

    /// Unwrap the value, *panicking* if it's regular with the given message
    pub fn expect_extra(self, message: impl AsRef<str>) -> Node<'tree> where T: Debug {
        match self {
            ExtraOr::Extra(node) => node,
            ExtraOr::Regular(value) => panic!("ExtraOr::expect_extra called on Regular({:?}): {}", value, message.as_ref())
        }
    }

    /// Unwrap the value, returning `None` if it's an extra
    pub fn regular(self) -> Option<T> {
        match self {
            ExtraOr::Extra(_) => None,
            ExtraOr::Regular(value) => Some(value)
        }
    }

    /// Unwrap the value, returning `None` if it's regular
    pub fn extra(self) -> Option<Node<'tree>> {
        match self {
            ExtraOr::Extra(node) => Some(node),
            ExtraOr::Regular(_) => None
        }
    }

    /// Transform the regular node
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ExtraOr<'tree, U>  {
        match self {
            ExtraOr::Extra(node) => ExtraOr::Extra(node),
            ExtraOr::Regular(value) => ExtraOr::Regular(f(value))
        }
    }

    /// Transform the regular node, returning if `Extra`
    pub fn and_then<U>(self, f: impl FnOnce(T) -> ExtraOr<'tree, U>) -> ExtraOr<'tree, U>  {
        match self {
            ExtraOr::Extra(node) => ExtraOr::Extra(node),
            ExtraOr::Regular(value) => f(value)
        }
    }

    /// Returns `true` if this is a regular node which satisfies the predicate
    pub fn is_regular_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
        match self {
            ExtraOr::Extra(_) => false,
            ExtraOr::Regular(value) => f(value)
        }
    }

    /// Returns `true` if this is an extra node which satisfies the predicate
    pub fn is_extra_and(&self, f: impl FnOnce(&Node<'tree>) -> bool) -> bool {
        match self {
            ExtraOr::Extra(node) => f(node),
            ExtraOr::Regular(_) => false
        }
    }
}

impl<'tree, T: TypedNode<'tree>> TryFrom<Node<'tree>> for ExtraOr<'tree, T> {
    type Error = IncorrectKind<'tree>;

    fn try_from(value: Node<'tree>) -> Result<Self, Self::Error> {
        match value.is_extra() {
            false => T::try_from(value).map(ExtraOr::Regular),
            true => Ok(ExtraOr::Extra(value))
        }
    }
}

impl<'tree, T: TypedNode<'tree>> TypedNode<'tree> for ExtraOr<'tree, T> {
    const KIND: &'static str = "{extra or}";

    #[inline]
    fn node(&self) -> &Node<'tree> {
        match self {
            ExtraOr::Extra(node) => node,
            ExtraOr::Regular(value) => value.node()
        }
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node<'tree> {
        match self {
            ExtraOr::Extra(node) => node,
            ExtraOr::Regular(value) => value.node_mut()
        }
    }

    #[inline]
    fn into_node(self) -> Node<'tree> {
        match self {
            ExtraOr::Extra(node) => node,
            ExtraOr::Regular(value) => value.into_node()
        }
    }

    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        match node.is_extra() {
            false => ExtraOr::Regular(T::from_node_unchecked(node)),
            true => ExtraOr::Extra(node)
        }
    }
}