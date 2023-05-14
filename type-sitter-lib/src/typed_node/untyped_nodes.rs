#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::Node;
#[cfg(not(feature = "tree-sitter-wrapper"))]
use tree_sitter::Node;
use crate::{IncorrectKind, TypedNode};

/// Untyped "typed" named or anonymous node (implements [TypedNode] but don't actually have a type)
pub struct UntypedNode<'tree>(Node<'tree>);

/// Untyped "typed" named node (implements [TypedNode] but don't actually have a type, except that it's named)
pub struct UntypedNamedNode<'tree>(Node<'tree>);

impl<'tree> UntypedNode<'tree> {
    #[inline]
    pub fn new(node: Node<'tree>) -> Self {
        Self(node)
    }
}

impl<'tree> TryFrom<Node<'tree>> for UntypedNode<'tree> {
    type Error = IncorrectKind<'tree>;

    #[inline]
    fn try_from(node: Node<'tree>) -> Result<Self, Self::Error> {
        Ok(Self(node))
    }
}

impl<'tree> TypedNode<'tree> for UntypedNode<'tree> {
    const KIND: &'static str = "{untyped}";

    #[inline]
    fn node(&self) -> &Node<'tree> {
        &self.0
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node<'tree> {
        &mut self.0
    }

    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        Self(node)
    }
}

impl<'tree> UntypedNamedNode<'tree> {
    #[inline]
    pub fn new(node: Node<'tree>) -> Self {
        Self(node)
    }
}

impl<'tree> TryFrom<Node<'tree>> for UntypedNamedNode<'tree> {
    type Error = IncorrectKind<'tree>;

    #[inline]
    fn try_from(node: Node<'tree>) -> Result<Self, Self::Error> {
        Ok(Self(node))
    }
}

impl<'tree> TypedNode<'tree> for UntypedNamedNode<'tree> {
    const KIND: &'static str = "{untyped named}";

    #[inline]
    fn node(&self) -> &Node<'tree> {
        &self.0
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node<'tree> {
        &mut self.0
    }

    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        Self(node)
    }
}