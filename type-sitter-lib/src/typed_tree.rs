use std::marker::PhantomData;
use std::os::fd::AsRawFd;
#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::{Bitmask, DisplayTree, InputEdit, Language, Range, Tree, TreeCursor};
#[cfg(not(feature = "tree-sitter-wrapper"))]
use tree_sitter::{InputEdit, Language, Range, Tree, TreeCursor};
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
/// # #![cfg(feature = "tree-sitter-wrapper")]
/// use type_sitter_lib::tree_sitter_wrapper::{Node, Tree};
/// use type_sitter_lib::{TypedNodeGAT, TypedTree};
///
/// pub struct Program<'tree>(Node<'tree>);
/// pub struct ProgramGAT;
///
/// impl TypedNodeGAT for ProgramGAT {
///   type This<'tree> = Program<'tree>;
/// }
///
/// fn wrap(tree: Tree) -> TypedTree<ProgramGAT> {
///   TypedTree::try_from(tree).unwrap()
/// }
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
    #[cfg(feature = "tree-sitter-wrapper")]
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
    pub fn language(&self) -> Language {
        self.tree.language()
    }

    /// Print a dot graph of the tree to the given file. See [tree_sitter::Tree::print_dot_graph]
    #[inline]
    pub fn print_dot_graph(&self, file: &impl AsRawFd) {
        self.tree.print_dot_graph(file)
    }

    /// Edit the tree. See [tree_sitter::Tree::edit]
    #[inline]
    pub fn edit(&mut self, edit: &InputEdit) {
        self.tree.edit(edit)
    }

    /// Re-print this tree, skipping nodes without `include_mark` and with `exclude_mark`.
    /// This will reuse the parsed text when possible, but if a lot of nodes are skipped you may
    /// still need to format to produce something readable.
    #[inline]
    #[cfg(feature = "tree-sitter-wrapper")]
    pub fn display_skipping(&self, include_mark: Bitmask, exclude_mark: Bitmask) -> DisplayTree<'_> {
        self.tree.display_skipping(include_mark, exclude_mark)
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
/// # #![cfg(feature = "tree-sitter-wrapper")]
/// use type_sitter_lib::tree_sitter_wrapper::{Node, Tree};
/// use type_sitter_lib::{TypedNodeGAT, TypedTree};
///
/// pub struct Program<'tree>(Node<'tree>);
/// pub struct ProgramGAT;
///
/// impl TypedNodeGAT for ProgramGAT {
///   type This<'tree> = Program<'tree>;
/// }
///
/// fn wrap(tree: Tree) -> TypedTree<ProgramGAT> {
///   TypedTree::try_from(tree).unwrap()
/// }
/// ```
pub trait TypedNodeGAT {
    /// The lifetime-irrelevant node type
    type This<'tree>: TypedNode<'tree>;
}

#[cfg(test)]
#[cfg(not(feature = "tree-sitter-wrapper"))]
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
        parser.set_language(tree_sitter_json::language()).expect("failed to set JSON language (?)");
        let json_str = r#"{ "key": [{ "key2": "value" }] }"#;
        let untyped_tree = parser.parse(json_str, None).expect("failed to parse test JSON");
        let tree = TypedTree::<UntypedNodeGAT>::try_from(untyped_tree).expect("failed to create typed tree of an untyped node");
        assert_eq!(tree.root().utf8_text(json_str.as_bytes()).expect("failed to get parsed source"), json_str);
    }
}