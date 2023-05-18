use std::marker::PhantomData;
use std::os::fd::AsRawFd;
#[cfg(feature = "tree-sitter-wrapper")]
use crate::tree_sitter_wrapper::{Bitmask, DisplayTree, InputEdit, Language, Range, Tree, TreeCursor};
#[cfg(not(feature = "tree-sitter-wrapper"))]
use tree_sitter::{InputEdit, Language, Range, Tree, TreeCursor};
use crate::{IncorrectKind, IncorrectTreeKind, TypedNode};

/// Tree whose root is a typed node. Currently we don't know which node is the root, so you must
/// manually construct this with the correct type argument (still checked though).
pub struct TypedTree<Root: TypedNode> {
    tree: Tree,
    node_type: PhantomData<Root>
}

impl<Root: TypedNode> TryFrom<Tree> for TypedTree<Root> {
    type Error = IncorrectTreeKind;

    fn try_from(tree: Tree) -> Result<Self, Self::Error> {
        match Root::try_from(tree.root_node()) {
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

impl<Root: TypedNode> TypedTree<Root> {
    /// Get the root node of this tree
    #[inline]
    pub fn root(&self) -> Root {
        Root::try_from(self.tree.root_node()).unwrap()
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
        self.included_ranges()
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