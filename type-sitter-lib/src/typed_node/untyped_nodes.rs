#[cfg(feature = "yak-sitter")]
use yak_sitter::Node;
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::Node;
use crate::{IncorrectKind, NodeResult, TypedNode};

/// Untyped "typed" named or anonymous node (implements [TypedNode] but don't actually have a type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntypedNode<'tree>(Node<'tree>);

/// Untyped "typed" named node (implements [TypedNode] but don't actually have a type, except that it's named)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntypedNamedNode<'tree>(Node<'tree>);

impl<'tree> UntypedNode<'tree> {
    /// Wrap the node so that it can be used wherever a [TypedNode] can, but provides no type guarantees
    #[inline]
    pub fn new(node: Node<'tree>) -> Self {
        Self(node)
    }

    /// Try to upcast to the given type. See [TypedNode]
    #[inline]
    pub fn to<Type: TypedNode<'tree>>(&self) -> NodeResult<Type> {
        Type::try_from(self.0)
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
    fn into_node(self) -> Node<'tree> {
        self.0
    }

    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        Self(node)
    }
}

impl<'tree> UntypedNamedNode<'tree> {
    /// Wrap the node so that it can be used wherever a [TypedNode] can, but provides no type guarantees
    #[inline]
    pub fn new(node: Node<'tree>) -> Self {
        Self(node)
    }

    /// Try to upcast to the given type. See [TypedNode]
    #[inline]
    pub fn to<Type: TypedNode<'tree>>(&self) -> NodeResult<Type> {
        Type::try_from(self.0)
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
    fn into_node(self) -> Node<'tree> {
        self.0
    }

    #[inline]
    unsafe fn from_node_unchecked(node: Node<'tree>) -> Self {
        Self(node)
    }
}