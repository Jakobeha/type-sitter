use crate::raw;
use crate::{IncorrectKind, Node, NodeResult};
use std::fmt::Debug;

macro_rules! define_simple_wrapper {
    ($comment:literal, $Type:ident, $method:ident, $name:literal) => {
        #[doc = $comment]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $Type<'tree>(raw::Node<'tree>);

        impl<'tree> Node<'tree> for $Type<'tree> {
            type WithLifetime<'a> = $Type<'a>;

            const KIND: &'static str = concat!("{", $name, "}");

            fn try_from_raw(node: raw::Node<'tree>) -> NodeResult<'tree, Self> {
                if node.$method() {
                    Ok($Type(node))
                } else {
                    Err(IncorrectKind::new::<Self>(node))
                }
            }

            #[inline]
            unsafe fn from_raw_unchecked(node: raw::Node<'tree>) -> Self {
                debug_assert!(node.$method());
                $Type(node)
            }

            #[inline]
            fn raw(&self) -> &raw::Node<'tree> {
                &self.0
            }

            #[inline]
            fn raw_mut(&mut self) -> &mut raw::Node<'tree> {
                &mut self.0
            }

            #[inline]
            fn into_raw(self) -> raw::Node<'tree> {
                self.0
            }
        }
    };
}

define_simple_wrapper! {
    "A node that can annotate any other node, e.g. a comment.",
    Extra,
    is_extra,
    "extra"
}

define_simple_wrapper! {
    "A stub node that indicates a localized parse error.",
    Error,
    is_error,
    "error"
}

define_simple_wrapper! {
    "A stub node that indicates another node was expected.",
    Missing,
    is_missing,
    "missing"
}

define_simple_wrapper! {
    "A node that is untyped other than being named.",
    UntypedNamedNode,
    is_named,
    "untyped named"
}

/// A node that is untyped, but implements [`Node`] anyways.
///
/// It's purpose is normalization: e.g. a method might take a [`Node`] parameter, so you can
/// pass any kind of typed node, and you may want to just pass a regular node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UntypedNode<'tree>(raw::Node<'tree>);

impl<'tree> UntypedNode<'tree> {
    /// Wrap the tree-sitter node so that it can be used wherever a [`Node`] can, but provides no
    /// type guarantees.
    #[inline]
    pub fn new(node: raw::Node<'tree>) -> Self {
        Self(node)
    }

    /// Convert a reference to a tree-sitter node into one of an untyped node.
    ///
    /// This is a transmute, but safe since they have the same representation.
    #[inline]
    pub fn r#ref<'a>(node: &'a raw::Node<'tree>) -> &'a Self {
        unsafe { &*(node as *const raw::Node<'tree> as *const Self) }
    }

    /// Convert a mutable reference to a tree-sitter node into one of an untyped node.
    ///
    /// This is a transmute, but safe since they have the same representation.
    #[inline]
    pub fn r#mut<'a>(node: &'a mut raw::Node<'tree>) -> &'a mut Self {
        unsafe { &mut *(node as *mut raw::Node<'tree> as *mut Self) }
    }

    /// Is this an error node?
    #[inline]
    pub fn is_error(&self) -> bool {
        self.raw().is_error()
    }

    /// Is this a missing node?
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.raw().is_missing()
    }

    /// Is this an extra node?
    #[inline]
    pub fn is_extra(&self) -> bool {
        self.raw().is_extra()
    }

    /// Try to downcast to the given type.
    ///
    /// See [`Node::try_from_raw`].
    #[inline]
    pub fn downcast<Type: Node<'tree>>(&self) -> NodeResult<'_, Type> {
        Type::try_from_raw(self.0)
    }
}

impl<'tree> From<raw::Node<'tree>> for UntypedNode<'tree> {
    fn from(value: raw::Node<'tree>) -> Self {
        Self(value)
    }
}

impl<'tree> Node<'tree> for UntypedNode<'tree> {
    type WithLifetime<'a> = UntypedNode<'a>;

    const KIND: &'static str = "{untyped}";

    fn try_from_raw(node: raw::Node<'tree>) -> NodeResult<'tree, Self> {
        Ok(UntypedNode(node))
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: raw::Node<'tree>) -> Self {
        UntypedNode(node)
    }

    #[inline]
    fn raw(&self) -> &raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> raw::Node<'tree> {
        self.0
    }
}

impl<'tree> UntypedNamedNode<'tree> {
    /// Try to downcast to the given type.
    ///
    /// See [`Node::try_from_raw`].
    #[inline]
    pub fn downcast<Type: Node<'tree>>(&self) -> NodeResult<'_, Type> {
        Type::try_from_raw(self.0)
    }
}
