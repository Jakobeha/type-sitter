use crate::{raw, InputEdit, LanguageRef, Node, NodeResult, Range, TreeCursor};
#[cfg(feature = "yak-sitter")]
use std::iter::Once;
use std::marker::PhantomData;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
#[cfg(feature = "yak-sitter")]
use std::path::Path;
#[cfg(feature = "yak-sitter")]
use tree_sitter::TextProvider;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Tree<Root: Node<'static>>(pub raw::Tree, PhantomData<Root>);

impl<Root: Node<'static>> Tree<Root> {
    /// Convert a tree-sitter tree into a typed tree by specifying the correct root node type.
    ///
    /// This will succeed even if `Root` is not the language's root node, because
    /// [`root_node`](Self::root_node) checks that the root node is the correct type.
    pub fn wrap(tree: raw::Tree) -> Self {
        Self(tree, PhantomData)
    }

    /// Version of [`wrap`](Self::wrap) for an immutable reference.
    pub fn wrap_ref(tree: &raw::Tree) -> &Self {
        // SAFETY: Same repr
        unsafe { &*(tree as *const raw::Tree as *const Self) }
    }

    /// Version of [`wrap`](Self::wrap) for a mutable reference.
    pub fn wrap_mut(tree: &mut raw::Tree) -> &mut Self {
        // SAFETY: Same repr
        unsafe { &mut *(tree as *mut raw::Tree as *mut Self) }
    }

    /// Get the underlying text. This includes text which isn't in the [`Self::included_ranges`]
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn text(&self) -> &str {
        self.0.text()
    }

    /// Get the path the tree is associated with, if any.
    ///
    /// The path may be virtual, it's used for stable node comparison between trees.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn path(&self) -> Option<&Path> {
        self.0.path()
    }

    /// Get the root node of this tree.
    ///
    /// Returns `Err` if the root is the wrong type, which may happen even if the language is
    /// correct if the entire tree is an error node.
    pub fn root_node(&self) -> NodeResult<Root::WithLifetime<'_>> {
        Root::WithLifetime::try_from_raw(self.0.root_node())
    }

    /// Create a cursor starting at the root node
    #[inline]
    pub fn walk(&self) -> TreeCursor {
        TreeCursor(self.0.walk())
    }

    /// Get the included ranges used to parse the tree
    #[inline]
    pub fn included_ranges(&self) -> Vec<Range> {
        self.0.included_ranges()
    }

    /// Get the changed ranges. See [`tree_sitter::Tree::changed_ranges`]
    #[inline]
    pub fn changed_ranges(&self, other: &Tree<Root>) -> impl ExactSizeIterator<Item=Range> {
        self.0.changed_ranges(&other.0)
    }

    /// Get the language used to parse the tree.
    #[inline]
    pub fn language(&self) -> LanguageRef<'_> {
        self.0.language()
    }

    //noinspection RsIncorrectFunctionArgumentCount - IntelliJ inspection bug.
    /// Print a dot graph of the tree to the given file. See [`tree_sitter::Tree::print_dot_graph`]
    #[inline]
    pub fn print_dot_graph(
        &self,
        #[cfg(unix)] file: &impl AsRawFd,
        #[cfg(windows)] file: &impl AsRawHandle,
    ) {
        self.0.print_dot_graph(file)
    }

    /// Edit the tree. See [`tree_sitter::Tree::edit`]
    #[inline]
    pub fn edit(&mut self, edit: &InputEdit) {
        self.0.edit(edit)
    }
}

#[cfg(feature = "yak-sitter")]
impl<'tree, Root: Node<'static>> TextProvider<&'tree str> for &'tree Tree<Root> {
    type I = Once<&'tree str>;

    fn text(&mut self, node: tree_sitter::Node) -> Self::I {
        TextProvider::text(&mut &self.0, node)
    }
}