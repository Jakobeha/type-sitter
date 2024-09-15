use std::marker::PhantomData;
use std::os::fd::AsRawFd;
#[cfg(feature = "yak-sitter")]
use yak_sitter::{InputEdit, LanguageRef, Range, Tree, TreeCursor};
#[cfg(not(feature = "yak-sitter"))]
use tree_sitter::{InputEdit, LanguageRef, Range, Tree, TreeCursor};
use crate::{IncorrectKind, IncorrectTreeKind, TypedNode};

/// Tree whose root is a typed node. Currently we don't know which node is the root, so you must
/// manually construct this with the correct type argument (still checked though).
///
/// Unfortunately, [Higher-Rank Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html) are
/// necessary here but not supported by structs. As a workaround, we define the trait [TypedNodeGAT]
/// which has you specify the intended typed node as an associated type. It's a bit verbose
/// unfortunately, but safe and works for arbitrary typed nodes.
///
/// ## Example
///
/// ```
/// #[cfg(feature = "yak-sitter")]
/// use yak_sitter::{Node, Tree};
/// #[cfg(not(feature = "yak-sitter"))]
/// use tree_sitter::{Node, Tree};
/// use type_sitter_lib::{TypedNodeGAT, TypedTree};
/// # use type_sitter_lib::{TypedNode, IncorrectKind};
/// #
/// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// # pub struct Program<'tree>(Node<'tree>);
/// # impl<'tree> TryFrom<Node<'tree>> for Program<'tree> {
/// #     type Error = IncorrectKind<'tree>;
/// #     fn try_from(node: Node<'tree>) -> Result<Self, Self::Error> { todo!() }
/// # }
/// # impl<'tree> TypedNode<'tree> for Program<'tree> {
/// #   const KIND: &'static str = "program";
/// #   fn node(&self) -> &Node<'tree> { todo!() }
/// #   fn node_mut(&mut self) -> &mut Node<'tree> { todo!() }
/// #   fn into_node(self) -> Node<'tree> { todo!() }
/// # }
///
/// pub struct ProgramGAT;
///
/// impl TypedNodeGAT for ProgramGAT {
///   type This<'tree> = Program<'tree>;
/// }
///
/// fn wrap(tree: Tree) -> TypedTree<ProgramGAT> {
///   TypedTree::try_from(tree).unwrap()
/// }
/// #
/// # fn main() { println!("it works") }
/// ```
///
/// The type of `Root` must have the `'static` lifetime (e.g. `Program<'static>`). Notice the bound
/// `for<'tree> TypedNode<'tree>`; this requires that `Root` must implement `TypedNode<'tree>` for
/// every `'tree`. Fortunately, a typed node should be covariant in its lifetime, so `'static` can
/// be reduced to an arbitrary lifetime, and thus it satisfies the bound
#[derive(Debug)]
pub struct TypedTree<Root: TypedNodeGAT> {
    tree: Tree,
    node_type: PhantomData<Root>
}

impl<Root: TypedNodeGAT> TryFrom<Tree> for TypedTree<Root> {
    type Error = IncorrectTreeKind;

    fn try_from(tree: Tree) -> Result<Self, Self::Error> {
        match Root::This::try_from(tree.root_node()) {
            Ok(_) => Ok(Self {
                tree,
                node_type: PhantomData
            }),
            Err(IncorrectKind { node: _, kind }) => Err(IncorrectTreeKind {
                tree,
                kind
            })
        }
    }
}

impl<Root: TypedNodeGAT> TypedTree<Root> {
    /// Get the root node of this tree
    #[inline]
    pub fn root(&self) -> Root::This<'_> {
        Root::This::try_from(self.tree.root_node()).unwrap()
    }

    /// Get the underlying tree-sitter tree
    #[inline]
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Get a mutable reference to the underlying tree-sitter tree
    #[inline]
    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }

    /// Destruct into the underlying tree-sitter tree
    #[inline]
    pub fn into_tree(self) -> Tree {
        self.tree
    }

    // region Tree delegate
    /// Get the underlying text. This includes text which isn't in the [Self::included_ranges].
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn text(&self) -> &str {
        self.tree.text()
    }

    /// Create a [TreeCursor] starting at the root node.
    #[inline]
    pub fn walk(&self) -> TreeCursor<'_> {
        self.tree.walk()
    }

    /// Get the included ranges used to parse the tree.
    #[inline]
    pub fn included_ranges(&self) -> Vec<Range> {
        self.tree.included_ranges()
    }

    /// Get the changed ranges. See [tree_sitter::Tree::changed_ranges]
    #[inline]
    pub fn changed_ranges(&self, other: &Tree) -> impl ExactSizeIterator<Item=Range> {
        self.tree.changed_ranges(other)
    }

    /// Get the language used to parse the tree.
    #[inline]
    pub fn language(&self) -> LanguageRef<'_> {
        self.tree.language()
    }

    /// Print a dot graph of the tree to the given file. See [Tree::print_dot_graph]
    #[inline]
    pub fn print_dot_graph(&self, file: &impl AsRawFd) {
        self.tree.print_dot_graph(file)
    }

    /// Edit the tree. See [tree_sitter::Tree::edit]
    #[inline]
    pub fn edit(&mut self, edit: &InputEdit) {
        self.tree.edit(edit)
    }
    // endregion
}

/// Workaround to specify [TypedNode] types which are lifetime-irrelevant
///
/// Unfortunately, [Higher-Rank Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html) are
/// needed in [TypedTree] but not supported by structs. As a workaround, we define this trait,
/// which has you specify the intended typed node as an associated type. It's a bit verbose
/// unfortunately, but safe and works for arbitrary typed nodes.
///
/// ## Example
///
/// ```
/// #[cfg(feature = "yak-sitter")]
/// use yak_sitter::{Node, Tree};
/// #[cfg(not(feature = "yak-sitter"))]
/// use tree_sitter::{Node, Tree};
/// use type_sitter_lib::{TypedNodeGAT, TypedTree};
/// # use type_sitter_lib::{TypedNode, IncorrectKind};
/// #
/// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// # pub struct Program<'tree>(Node<'tree>);
/// # impl<'tree> TryFrom<Node<'tree>> for Program<'tree> {
/// #     type Error = IncorrectKind<'tree>;
/// #     fn try_from(node: Node<'tree>) -> Result<Self, Self::Error> { todo!() }
/// # }
/// # impl<'tree> TypedNode<'tree> for Program<'tree> {
/// #   const KIND: &'static str = "program";
/// #   fn node(&self) -> &Node<'tree> { todo!() }
/// #   fn node_mut(&mut self) -> &mut Node<'tree> { todo!() }
/// #   fn into_node(self) -> Node<'tree> { todo!() }
/// # }
///
/// pub struct ProgramGAT;
///
/// impl TypedNodeGAT for ProgramGAT {
///   type This<'tree> = Program<'tree>;
/// }
///
/// fn wrap(tree: Tree) -> TypedTree<ProgramGAT> {
///   TypedTree::try_from(tree).unwrap()
/// }
/// #
/// # fn main() { println!("it works") }
/// ```
pub trait TypedNodeGAT {
    /// The lifetime-irrelevant node type
    type This<'tree>: TypedNode<'tree>;
}

#[cfg(test)]
#[cfg(not(feature = "yak-sitter"))]
mod tests {
    use tree_sitter::Parser;
    use crate::{TypedNodeGAT, TypedNode, TypedTree, UntypedNode};

    pub struct UntypedNodeGAT;
    impl TypedNodeGAT for UntypedNodeGAT {
        type This<'tree> = UntypedNode<'tree>;
    }

    #[test]
    fn test_static_lifetime() {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_json::LANGUAGE.into()).expect("failed to set JSON language (?)");
        let json_str = r#"{ "key": [{ "key2": "value" }] }"#;
        let untyped_tree = parser.parse(json_str, None).expect("failed to parse test JSON");
        let tree = TypedTree::<UntypedNodeGAT>::try_from(untyped_tree).expect("failed to create typed tree of an untyped node");
        assert_eq!(tree.root().utf8_text(json_str.as_bytes()).expect("failed to get parsed source"), json_str);
    }
}