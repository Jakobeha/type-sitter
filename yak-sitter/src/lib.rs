use std::cmp::Ordering;
use std::collections::Bound;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::iter::{FusedIterator, once, Once};
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};
use std::str::Utf8Error;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, BitOr, BitOrAssign, RangeBounds};
#[cfg(unix)]
use std::os::fd::AsRawFd;
#[cfg(windows)]
use std::os::fd::AsRawHandle;
use std::ptr::NonNull;
use utf8_error_offset_by::Utf8ErrorOffsetBy;

mod utf8_error_offset_by;
mod define_custom_wrapper;

/// Wrapper around [tree_sitter::Tree] which stores its text, filepath, and extra data that is
/// accessible from any node. It also uses and is used by [tree_sitter_wrapper] wrapper classes.
#[derive(Debug)]
pub struct Tree<Custom = ()> {
    tree: tree_sitter::Tree,
    byte_text: Vec<u8>,
    path: Option<PathBuf>,
    /// Custom data which you can access behind a shared reference from any [Node]
    pub custom: Custom
}

/// Wrapper around [tree_sitter::Node] which can access its text, tree's filepath, and tree's
/// `Custom` data behind a shared reference. It also uses and is used by [tree_sitter_wrapper]
/// wrapper classes.
pub struct Node<'tree, Custom = ()> {
    node: tree_sitter::Node<'tree>,
    tree: &'tree Tree<Custom>,
}

/// Raw pointer equivalent of [Node].
///
/// `Custom` is still in the type signature, but you can safely `transmute` between different
/// `Custom` types and this is exposed via [NodePtr::cast_custom].
pub struct NodePtr<Custom = ()> {
    node_data: NodeData,
    tree: NonNull<Tree<Custom>>
}

/// Taken straight from [tree_sitter::ffi::TSNode]. This must maintain the same layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
struct NodeData {
    context: [u32; 4usize],
    id: *const (),
    tree: *const (),
}

/// Wrapper around `usize` (aka node id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeId(usize);

/// Wrapper around [tree_sitter::TreeCursor], which can actually go outside of its "local" node,
/// albeit with degraded performance (we just do standard lookups)
pub struct TreeCursor<'tree, Custom = ()> {
    cursor: tree_sitter::TreeCursor<'tree>,
    tree: &'tree Tree<Custom>,
    child_depth: usize,
}

/// Wrapper around [tree_sitter::QueryCursor]
pub struct QueryCursor {
    query_cursor: tree_sitter::QueryCursor
}

/// Wrapper around [tree_sitter::QueryMatches]
///
/// [tree_sitter::QueryMatches] is NOT a real iterator, it's a [StreamingIterator] (see
///     <https://github.com/tree-sitter/tree-sitter/issues/608>). Therefore this doesn't implement
///     [Iterator]
pub struct QueryMatches<'query, 'tree: 'query, Custom = ()> {
    query_matches: tree_sitter::QueryMatches<'query, 'tree, &'query Tree<Custom>, &'query str>,
    current_match: Option<QueryMatch<'query, 'tree, Custom>>,
    tree: &'tree Tree<Custom>,
    query: &'query Query
}

/// Wrapper around [tree_sitter::QueryMatch]
pub struct QueryMatch<'query, 'tree, Custom = ()> {
    query_match: tree_sitter::QueryMatch<'query, 'tree>,
    tree: &'tree Tree<Custom>,
    query: &'query Query
}

/// Wrapper around [tree_sitter::QueryCapture]
pub struct QueryCaptures<'query, 'tree, Custom = ()> {
    query_captures: tree_sitter::QueryCaptures<'query, 'tree, &'query Tree<Custom>, &'query str>,
    tree: &'tree Tree<Custom>,
    query: &'query Query
}

/// Wrapper around [tree_sitter::QueryCapture]
pub struct QueryCapture<'query, 'tree, Custom = ()> {
    pub node: Node<'tree, Custom>,
    pub index: usize,
    pub name: &'query str,
}

/// Variant of [std::ops::Range] which can be copied and displays as `:line:column-:line:column`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PointRange {
    pub start: Point,
    pub end: Point,
}

/// Wrapper around [tree_sitter::Point], which displays as `:line:column`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Point(tree_sitter::Point);

/// A byte and point range in one data-structure. Wrapper around [tree_sitter::Range] which displays
/// as `:line:column-:line:column`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Range(tree_sitter::Range);

/// Wrapper around [tree_sitter::Parser]
#[repr(transparent)]
pub struct Parser(tree_sitter::Parser);

/// Re-exports [tree_sitter::Language]
pub type Language = tree_sitter::Language;
pub type LanguageRef<'a> = tree_sitter::LanguageRef<'a>;
/// Re-exports [tree_sitter::LanguageError]
pub type LanguageError = tree_sitter::LanguageError;
/// Re-exports [tree_sitter::QueryError]
pub type Query = tree_sitter::Query;
/// Re-exports [tree_sitter::QueryError]
pub type QueryProperty = tree_sitter::QueryProperty;
/// Re-exports [tree_sitter::QueryError]
pub type IncludedRangesError = tree_sitter::IncludedRangesError;
/// Re-exports [tree_sitter::InputEdit]
pub type InputEdit = tree_sitter::InputEdit;

/// Error from parsing a tree
#[derive(Debug)]
pub enum TreeParseError {
    IO(std::io::Error),
    ParsingFailed,
    NotUtf8(Utf8Error)
}

/// General-purpose way to store TSNode separately from the tree, e.g. if you need to serialize it.
/// Unfortunately this is just done by storing the text and range, there's not much else we can do
pub struct SubTree<Custom = ()> {
    /// Node's text
    pub text: String,
    /// Node's range within the tree
    pub range: Range,
    /// Tree's virtual path (if any)
    pub path: Option<PathBuf>,
    /// Node which can be dereferenced in case the tree is still alive,
    /// otherwise it is dangling
    pub root: Option<NodePtr<Custom>>,
}

/// Describes the previous traversal action in a pre-order traversal which iterates nodes with
/// children both up and down, so we can get the next node in the traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TraversalState {
    Start,
    Down,
    Right,
    Up,
    End
}

/// Iterator over a tree in pre-order traversal which iterates nodes with children both up and down
pub struct PreorderTraversal<'tree, Custom = ()> {
    cursor: TreeCursor<'tree, Custom>,
    last_state: TraversalState
}

/// Iterated node in a traversal (includes field name and last state)
pub struct TraversalItem<'tree, Custom = ()> {
    /// The node
    pub node: Node<'tree, Custom>,
    /// The node's field name
    pub field_name: Option<&'static str>,
    /// Last traversal state to reach this node
    pub last_state: TraversalState
}

impl Parser {
    /// Create a new parser for the given language. See [tree_sitter::Parser::set_language]
    #[inline]
    pub fn new(language: &Language) -> Result<Self, LanguageError> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language)?;
        Ok(Self(parser))
    }

    /// Set the language of the parser. See [tree_sitter::Parser::set_language]
    #[inline]
    pub fn set_language(&mut self, language: &Language) -> Result<(), LanguageError> {
        self.0.set_language(language)
    }

    /// Set the ranges of text the parser should include when parsing. See
    /// [tree_sitter::Parser::set_included_ranges]
    #[inline]
    pub fn set_included_ranges(&mut self, ranges: &[Range]) -> Result<(), IncludedRangesError> {
        self.0.set_included_ranges(Range::slice_as_ts(ranges))
    }

    /// Parse a file. See [tree_sitter::Parser::parse]
    ///
    /// The file must be valid utf-8 or this will return `Err`. The file's path is used for stable
    /// node comparison between trees.
    #[inline]
    pub fn parse_file<Custom>(
        &mut self, 
        path: &Path, 
        old_tree: Option<&Tree<Custom>>, 
        custom: Custom
    ) -> Result<Tree<Custom>, TreeParseError> {
        let byte_text = std::fs::read(path)?;
        self.parse_bytes(byte_text, Some(path), old_tree, custom)
    }

    /// Parse a string. See [tree_sitter::Parser::parse]
    ///
    /// The path is passed to [TsCustom::new]. If your [TsCustom] doesn't use paths, jusrt pass
    /// `None`.
    #[inline]
    pub fn parse_string<Custom>(
        &mut self, 
        text: String, 
        path: Option<impl AsRef<Path>>, 
        old_tree: Option<&Tree<Custom>>, 
        custom: Custom
    ) -> Result<Tree<Custom>, TreeParseError> {
        self.parse_bytes(text.into_bytes(), path, old_tree, custom)
    }

    /// Parse a byte string. See [tree_sitter::Parser::parse].
    ///
    /// Note that the wrappers expect and assume UTF-8, so this will fail if the text is not valid
    /// UTF-8.
    ///
    /// The path is passed to [TsCustom::new]. If your [TsCustom] doesn't use paths, jusrt pass
    /// `None`.
    #[inline]
    pub fn parse_bytes<Custom>(
        &mut self,
        byte_text: Vec<u8>, 
        path: Option<impl AsRef<Path>>,
        old_tree: Option<&Tree<Custom>>,
        custom: Custom
    ) -> Result<Tree<Custom>, TreeParseError> {
        let tree = self.0.parse(&byte_text, old_tree.map(|t| &t.tree)).ok_or(TreeParseError::ParsingFailed)?;
        Ok(Tree::new(tree, byte_text, path.map(|p| p.as_ref().to_path_buf()), custom)?)
    }
}

impl<Custom> Tree<Custom> {
    /// Wrap the tree and its associated text. Note that the wrappers expect and assume UTF-8, so
    /// this will fail if the text is not valid UTF-8.
    #[inline]
    fn new(
        tree: tree_sitter::Tree,
        byte_text: Vec<u8>, 
        path: Option<PathBuf>,
        custom: Custom
    ) -> Result<Self, Utf8Error> {
        Self::validate_utf8(&tree, &byte_text)?;
        Ok(Self { tree, byte_text, path, custom })
    }

    fn validate_utf8(tree: &tree_sitter::Tree, byte_text: &[u8]) -> Result<(), Utf8Error> {
        let _ = std::str::from_utf8(byte_text)?;
        let mut cursor = tree.walk();
        let mut went_up = false;
        while (!went_up && cursor.goto_first_child()) ||
            cursor.goto_next_sibling() ||
            { went_up = true; cursor.goto_parent() } {
            let node = cursor.node();
            let range = node.byte_range();
            let _ = std::str::from_utf8(&byte_text[range]).map_err(|e| {
                e.offset_by(node.start_byte())
            })?;
        }
        Ok(())
    }

    /// Get the underlying text. This includes text which isn't in the [Self::included_ranges].
    #[inline]
    pub fn text(&self) -> &str {
        // SAFETY: we ran validate_utf8 before constructing so the text is valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.byte_text) }
    }

    /// Get the path the tree is associated with, if any.
    ///
    /// The path may be virtual, it's used for stable node comparison between trees.
    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Get the root node.
    #[inline]
    pub fn root_node(&self) -> Node<'_, Custom> {
        // SAFETY: The node is from this tree
        unsafe { Node::new(self.tree.root_node(), self) }
    }

    /// Create a [TreeCursor] starting at the root node.
    #[inline]
    pub fn walk(&self) -> TreeCursor<'_, Custom> {
        TreeCursor::new(self.tree.walk(), self, true)
    }

    /// Get the included ranges used to parse the tree.
    #[inline]
    pub fn included_ranges(&self) -> Vec<Range> {
        Range::vec_from_ts(self.tree.included_ranges())
    }

    /// Get the changed ranges. See [tree_sitter::Tree::changed_ranges]
    #[inline]
    pub fn changed_ranges(&self, other: &Tree) -> impl ExactSizeIterator<Item=Range> {
        self.tree.changed_ranges(&other.tree).map(Range)
    }

    /// Get the language used to parse the tree.
    #[inline]
    pub fn language(&self) -> LanguageRef<'_> {
        self.tree.language()
    }

    /// Print a dot graph of the tree to the given file. See [tree_sitter::Tree::print_dot_graph]
    #[inline]
    pub fn print_dot_graph(
        &self,
        #[cfg(unix)] file: &impl AsRawFd,
        #[cfg(windows)] file: &impl AsRawHandle,
    ) {
        self.tree.print_dot_graph(file)
    }

    /// Edit the tree. See [tree_sitter::Tree::edit]
    #[inline]
    pub fn edit(&mut self, edit: &InputEdit) {
        self.tree.edit(edit)
    }
}

impl<'tree, Custom> tree_sitter::TextProvider<&'tree str> for &'tree Tree<Custom> {
    type I = Once<&'tree str>;

    #[inline]
    fn text(&mut self, node: tree_sitter::Node<'_>) -> Self::I {
        // SAFETY: we ran validate_utf8 before constructing so every node's text is valid UTF-8
        once(unsafe { std::str::from_utf8_unchecked(&self.byte_text[node.byte_range()]) })
    }
}

impl<'tree, Custom> Node<'tree, Custom> {
    /// Wrap a [tree_sitter::Node]. Requires its associated [Tree] for convenience methods.
    ///
    /// SAFETY: The node must be from the given tree.
    #[inline]
    pub unsafe fn new(node: tree_sitter::Node<'tree>, tree: &'tree Tree<Custom>) -> Self {
        Self { node, tree }
    }

    /// Gets the node id which is a pointer. This means that nodes from separate trees are
    /// guaranteed to have different ids iff both trees are alive, but a node which is no longer
    /// alive may have the same id as a different node which is. The tree-sitter docs specifically
    /// mention that a if a tree is created from an older tree, nodes may be reused and from the old
    /// tree and these will have the same id.
    ///
    /// See [tree_sitter::Node::id]
    #[inline]
    pub fn id(&self) -> NodeId {
        NodeId::of_ts(self.node)
    }

    /// Get the node's kind. See [tree_sitter::Node::kind]
    #[inline]
    pub fn kind(&self) -> &'static str {
        self.node.kind()
    }

    /// Check if the node is named. See [tree_sitter::Node::is_named]
    #[inline]
    pub fn is_named(&self) -> bool {
        self.node.is_named()
    }

    /// Check if the node is missing. See [tree_sitter::Node::is_missing]
    #[inline]
    pub fn is_missing(&self) -> bool {
        self.node.is_missing()
    }

    /// Check if the node is extra. See [tree_sitter::Node::is_extra]
    #[inline]
    pub fn is_extra(&self) -> bool {
        self.node.is_extra()
    }

    /// Check if the node is an error. See [tree_sitter::Node::is_error]
    #[inline]
    pub fn is_error(&self) -> bool {
        self.node.is_error()
    }

    /// Check if the node has an error. See [tree_sitter::Node::has_error]
    #[inline]
    pub fn has_error(&self) -> bool {
        self.node.has_error()
    }

    /// Check if the node has changes. See [tree_sitter::Node::has_changes]
    #[inline]
    pub fn has_changes(&self) -> bool {
        self.node.has_changes()
    }

    /// Edit the node. See [tree_sitter::Node::edit]
    #[inline]
    pub fn edit(&mut self, edit: &InputEdit) {
        self.node.edit(edit)
    }

    /// Get the byte offsets where this node starts.
    #[inline]
    pub fn start_byte(&self) -> usize {
        self.node.start_byte()
    }

    /// Get the byte offsets where this node ends.
    #[inline]
    pub fn end_byte(&self) -> usize {
        self.node.end_byte()
    }

    /// Get the row and column where this node starts.
    #[inline]
    pub fn start_position(&self) -> Point {
        Point(self.node.start_position())
    }

    /// Get the row and column where this node ends.
    #[inline]
    pub fn end_position(&self) -> Point {
        Point(self.node.end_position())
    }

    /// Get the byte range where this node is located.
    #[inline]
    pub fn byte_range(&self) -> std::ops::Range<usize> {
        self.node.byte_range()
    }

    /// Get the row and column range where this node is located
    #[inline]
    pub fn position_range(&self) -> PointRange {
        PointRange {
            start: self.start_position(),
            end: self.end_position()
        }
    }

    /// Get the byte range and row and column range where this node is located.
    #[inline]
    pub fn range(&self) -> Range {
        Range(self.node.range())
    }

    /// Get the node's text as a byte string.
    ///
    /// **Warning:** the return value is unspecified if the tree is edited.
    #[inline]
    fn byte_text(&self) -> &'tree [u8] {
        &self.tree.byte_text[self.byte_range()]
    }

    /// Get the node's text.
    ///
    /// **Warning:** the return value is unspecified if the tree is edited.
    #[inline]
    pub fn text(&self) -> &'tree str {
        // SAFETY: we ran validate_utf8 before constructing so every node's text is valid UTF-8
        unsafe { std::str::from_utf8_unchecked(self.byte_text()) }
    }

    /// Path of the node's tree.
    ///
    /// This is the virtual path set on the tree's creation, used primarily for consistent ordering.
    #[inline]
    pub fn path(&self) -> Option<&'tree Path> {
        self.tree.path()
    }

    /// Custom data on the node's tree.
    ///
    /// Nodes can only get shared references to this since they have a shared reference on the tree
    /// itself. You must use a mutable reference to the tree or owned tree to get mutable or owned
    /// custom data.
    #[inline]
    pub fn custom(&self) -> &'tree Custom {
        &self.tree.custom
    }

    /// Get the node's named and unnamed children. See [tree_sitter::Node::children]
    #[inline]
    pub fn all_children<'a>(&self, cursor: &'a mut TreeCursor<'tree, Custom>) -> impl ExactSizeIterator<Item = Node<'tree, Custom>> + 'a {
        let tree = self.tree;
        // SAFETY: Same tree
        self.node.children(&mut cursor.cursor).map(move |node| unsafe { Node::new(node, tree) })
    }

    /// Get the node's named children. See [tree_sitter::Node::named_children]
    #[inline]
    pub fn named_children<'a>(&self, cursor: &'a mut TreeCursor<'tree, Custom>) -> impl ExactSizeIterator<Item = Node<'tree, Custom>> + 'a {
        let tree = self.tree;
        // SAFETY: Same tree
        self.node.named_children(&mut cursor.cursor).map(move |node| unsafe { Node::new(node, tree) })
    }

    /// Get the number of named and unnamed children.
    #[inline]
    pub fn any_child_count(&self) -> usize {
        self.node.child_count()
    }

    /// Get the number of named children.
    #[inline]
    pub fn named_child_count(&self) -> usize {
        self.node.named_child_count()
    }

    /// Get the node's immediate parent.
    #[inline]
    pub fn parent(&self) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.parent().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate next sibling, named or unnamed.
    #[inline]
    pub fn next_any_sibling(&self) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.next_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate named next sibling.
    #[inline]
    pub fn next_named_sibling(&self) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.next_named_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate previous sibling, named or unnamed.
    #[inline]
    pub fn prev_any_sibling(&self) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.prev_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate named previous sibling.
    #[inline]
    pub fn prev_named_sibling(&self) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.prev_named_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's child at the given index, named or unnamed. See [tree_sitter::Node::child]
    #[inline]
    pub fn any_child(&self, i: usize) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.child(i).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's named child at the given index. See [tree_sitter::Node::named_child]
    #[inline]
    pub fn named_child(&self, i: usize) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.named_child(i).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's last child, named or unnamed.
    #[inline]
    pub fn last_any_child(&self) -> Option<Node<'tree, Custom>> {
        // .child is already bounds-checked so we use wrapping_sub for iff the count is 0
        // SAFETY: Same tree
        self.node.child(self.any_child_count().wrapping_sub(1)).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's last named child.
    #[inline]
    pub fn last_named_child(&self) -> Option<Node<'tree, Custom>> {
        // .named_child is already bounds-checked so we use wrapping_sub for iff the count is 0
        // SAFETY: Same tree
        self.node.named_child(self.named_child_count().wrapping_sub(1)).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's first child of the given kind, named or unnamed. The cursor is used to
    /// iterate the node's immediate children.
    #[inline]
    pub fn child_of_kind(
        &self,
        kind: &str,
        cursor: &mut TreeCursor<'tree, Custom>
    ) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.named_children(&mut cursor.cursor)
            .find(|node| node.kind() == kind)
            .map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's children of the given kind, named or unnamed. The cursor is used to iterate
    /// the node's immediate children.
    #[inline]
    pub fn children_of_kind<'a>(
        &self,
        kind: &'a str,
        cursor: &'a mut TreeCursor<'tree, Custom>
    ) -> impl Iterator<Item = Node<'tree, Custom>> + 'a {
        // SAFETY: Same tree
        self.node.named_children(&mut cursor.cursor)
            .filter(move |node| node.kind() == kind)
            .map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the first child with the given field name.
    #[inline]
    pub fn child_by_field_name(&self, field_name: &str) -> Option<Node<'tree, Custom>> {
        // SAFETY: Same tree
        self.node.child_by_field_name(field_name).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's children with the given field name.
    #[inline]
    pub fn children_by_field_name<'a>(
        &self,
        field_name: &str,
        c: &'a mut TreeCursor<'tree, Custom>
    ) -> impl Iterator<Item = Node<'tree, Custom>> + 'a {
        // SAFETY: Same tree
        self.node.children_by_field_name(field_name, &mut c.cursor).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the field name of the child at the given index.
    #[inline]
    pub fn field_name_for_child(&self, i: usize) -> Option<&'static str> {
        self.node.field_name_for_child(i as u32)
    }

    /// Get the node's field name. This is done by looking at the parent's children and finding the
    /// one that matches this node, then returning its field name, hence the need for a cursor.
    pub fn field_name(&self, cursor: &mut TreeCursor<'tree, Custom>) -> Option<&'static str> {
        self.parent().and_then(|parent| {
            let i = parent.all_children(cursor)
                .position(|x| x == *self)
                .expect("node not one of its parent's children");
            parent.field_name_for_child(i)
        })
    }

    /// Get a [TreeCursor] that points to this node.
    #[inline]
    pub fn walk(&self) -> TreeCursor<'tree, Custom> {
        TreeCursor::new(self.node.walk(), self.tree, false)
    }

    /// Print the node as an s-expression
    #[inline]
    pub fn to_sexp(&self) -> String {
        self.node.to_sexp()
    }

    /// Get a raw pointer to this node (remove the 'tree lifetime).
    #[inline]
    pub fn to_ptr(&self) -> NodePtr<Custom> {
        NodePtr {
            node_data: NodeData::from(self.node),
            tree: NonNull::from(self.tree),
        }
    }

    /// Get a [SubTree] that represents this node without the lifetime.
    /// The text and range will persist even after this node's tree is deallocated,
    /// and if we know for certain it isn't, the original node can be `unsafe`ly recovered
    #[inline]
    pub fn to_subtree(&self) -> SubTree<Custom> {
        SubTree {
            text: self.text().to_string(),
            range: self.range(),
            path: self.path().map(|p| p.to_path_buf()),
            root: Some(self.to_ptr()),
        }
    }
}

impl<'tree, Custom> PartialEq for Node<'tree, Custom> {
    /// Equal if both nodes have the same id, which is a pointer. This means that nodes from
    /// separate trees are guaranteed to not be equal.
    #[inline]
    fn eq(&self, other: &Node<'tree, Custom>) -> bool {
        self.id() == other.id()
    }
}

impl<'tree, Custom> Eq for Node<'tree, Custom> {}

impl<'tree, Custom> PartialOrd for Node<'tree, Custom> {
    /// Equal if both nodes are the same (have the same id).
    ///
    /// Otherwise, compares based
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'tree, Custom> Ord for Node<'tree, Custom> {
    /// Equal if both nodes are the same (have the same id).
    ///
    /// Otherwise, compares based
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id() == other.id() {
            Ordering::Equal
        } else if std::ptr::eq(self.tree as *const _, other.tree as *const _) {
            self.start_byte().cmp(&other.start_byte())
                .then(self.end_byte().cmp(&other.end_byte()))
        } else {
            self.path().cmp(&other.path())
                .then(self.id().0.cmp(&other.id().0))
        }
    }
}

impl<'tree, Custom> Hash for Node<'tree, Custom> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl<Custom> NodePtr<Custom> {
    /// SAFETY: You must ensure that the tree the node came from is alive
    #[inline]
    pub unsafe fn to_node<'a>(self) -> Node<'a, Custom> {
        Node {
            node: self.node_data.to_node(),
            tree: self.tree.as_ref(),
        }
    }

    /// Cast `Custom` to a different type. This is safe because it's behind a pointer within a
    /// fixed-size struct, so the repr is the exact same and `Custom` can't be accessed without
    /// calling [Self::to_node] first
    #[inline]
    pub fn cast_custom<NewCustom>(self) -> NodePtr<NewCustom> {
        // SAFETY: See method documentation, same repr and impossible to access `Custom`
        unsafe { std::mem::transmute(self) }
    }
}

impl<Custom> PartialEq for NodePtr<Custom> {
    #[inline]
    fn eq(&self, other: &NodePtr<Custom>) -> bool {
        self.node_data == other.node_data
    }
}

impl<Custom> Eq for NodePtr<Custom> {}

impl<Custom> Hash for NodePtr<Custom> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_data.hash(state)
    }
}

impl NodeData {
    /// SAFETY: You must ensure that the tree the node came from is alive
    #[inline]
    pub unsafe fn to_node<'tree>(self) -> tree_sitter::Node<'tree> {
        // SAFETY: tree_sitter::Node is POD (no Drop, Copy),
        // and sizes are compile_time checked to be the same
        std::mem::transmute(self)
    }
}

impl<'tree> From<tree_sitter::Node<'tree>> for NodeData {
    #[inline]
    fn from(node: tree_sitter::Node) -> Self {
        // SAFETY: We are storing this as opaquely, tree_sitter::Node is POD (no Drop, Copy),
        // and sizes are compile_time checked to be the same
        unsafe { std::mem::transmute::<tree_sitter::Node<'_>, NodeData>(node) }
    }
}

impl<'tree, Custom> TreeCursor<'tree, Custom> {
    /// Wrap a [tree-sitter::TreeCursor]. You must also provide the tree and whether the cursor is
    /// at the root node.
    #[inline]
    fn new(cursor: tree_sitter::TreeCursor<'tree>, tree: &'tree Tree<Custom>, is_rooted: bool) -> Self {
        Self {
            cursor,
            tree,
            child_depth: match is_rooted {
                false => 0,
                true => 1
            }
        }
    }

    /// Gets the cursor's current [Node].
    #[inline]
    pub fn node(&self) -> Node<'tree, Custom> {
        // SAFETY: Same tree
        unsafe { Node::new(self.cursor.node(), self.tree) }
    }

    /// Gets the field name of the cursor's current node.
    pub fn field_name(&mut self) -> Option<&'static str> {
        self.cursor.field_name().or_else(|| if self.child_depth > 0 {
            None
        } else {
            match self.node().parent() {
                None => None,
                Some(parent) => {
                    let original_node = self.node();
                    self.goto(parent);
                    self.goto_first_child();
                    while self.node() != original_node {
                        self.goto_next_sibling();
                    }
                    self.cursor.field_name()
                }
            }
        })
    }

    /// Re-initialize the cursor to point to the given node.
    #[inline]
    pub fn goto(&mut self, node: Node<'tree, Custom>) {
        if self.cursor.node() != node.node {
            self.cursor.reset(node.node);
            self.child_depth = 0;
        }
    }

    /// Move the cursor to the first child of the current node and return `true`, or return `false`
    /// if the current node has no children.
    #[inline]
    pub fn goto_first_child(&mut self) -> bool {
        if self.cursor.goto_first_child() {
            self.child_depth += 1;
            true
        } else {
            false
        }
    }


    /// Move the cursor to the first child of the current node that extends beyond the given byte
    /// offset, and return its index. Returns `false` if the current node has no children past that
    /// offset.
    #[inline]
    pub fn goto_first_child_for_byte(&mut self, index: usize) -> Option<usize> {
        self.cursor.goto_first_child_for_byte(index).map(|index| {
            self.child_depth += 1;
            index
        })
    }

    /// Move the cursor to the first child of the current node that extends beyond the given row
    /// and column, and return its index. Returns `false` if the current node has no children past
    /// that row and column.
    #[inline]
    pub fn goto_first_child_for_point(&mut self, point: Point) -> Option<usize> {
        self.cursor.goto_first_child_for_point(point.0).map(|index| {
            self.child_depth += 1;
            index
        })
    }

    /// Move the cursor to the next sibling of the current node and return `true`, or return `false`
    /// if the current node has no next sibling.
    ///
    /// Unlike [tree_sitter::TreeCursor.goto_next_sibling], this will actually work if the cursor is
    /// rooted (e.g. reset) to its current node.
    pub fn goto_next_sibling(&mut self) -> bool {
        self.cursor.goto_next_sibling() || if self.child_depth > 0 {
            false
        } else {
            match self.node().parent() {
                None => false,
                Some(parent) => {
                    let original_node = self.node();
                    self.goto(parent);
                    self.goto_first_child();
                    while self.node() != original_node {
                        self.goto_next_sibling();
                    }
                    self.goto_next_sibling()
                }
            }
        }
    }

    /// Move the cursor to the parent of the current node and return `true`, or return `false`
    /// if the current node is a tree root.
    ///
    /// Unlike [tree_sitter::TreeCursor.goto_parent], this will actually work if the cursor is
    /// rooted (e.g. reset) to its current node.
    pub fn goto_parent(&mut self) -> bool {
        if self.cursor.goto_parent() {
            debug_assert!(self.child_depth != 0);
            self.child_depth -= 1;
            true
        } else {
            if self.child_depth > 0 {
                false
            } else {
                match self.node().parent() {
                    None => false,
                    Some(parent) => {
                        self.goto(parent);
                        true
                    }
                }
            }
        }
    }

    /// Move the cursor to the next node in pre-order traversal order, where nodes with children are
    /// traversed on both the up and down. This uses `last_state` to determine where to go next
    /// (i.e. up or right/down on a node with children), and returns the next state which you would
    /// pass to the next `goto_preorder` call and so on.
    pub fn goto_preorder(&mut self, last_state: TraversalState) -> TraversalState {
        if !last_state.is_up() && self.goto_first_child() {
            TraversalState::Down
        } else if self.goto_next_sibling() {
            TraversalState::Right
        } else if self.goto_parent() {
            TraversalState::Up
        } else {
            TraversalState::End
        }
    }
}

impl QueryCursor {
    /// Creates a new cursor. See [tree_sitter::QueryCursor::new]
    #[inline]
    pub fn new() -> Self {
        Self { query_cursor: tree_sitter::QueryCursor::new() }
    }

    /// Iterate over all matches in the order they were found. See
    /// [tree_sitter::QueryCursor::matches]
    #[inline]
    pub fn matches<'query, 'tree: 'query, Custom>(
        &'query mut self,
        query: &'query Query,
        node: Node<'tree, Custom>
    ) -> QueryMatches<'query, 'tree, Custom> {
        QueryMatches {
            query_matches: self.query_cursor.matches(&query, node.node, node.tree),
            current_match: None,
            tree: node.tree,
            query
        }
    }

    /// Iterate over all captures in the order they appear. See [tree_sitter::QueryCursor::captures]
    #[inline]
    pub fn captures<'query, 'tree: 'query, Custom>(
        &'query mut self,
        query: &'query Query,
        node: Node<'tree, Custom>
    ) -> QueryCaptures<'query, 'tree, Custom> {
        QueryCaptures {
            query_captures: self.query_cursor.captures(&query, node.node, node.tree),
            tree: node.tree,
            query
        }
    }

    /// Set the maximum number of in-progress matches for this cursor.
    #[inline]
    pub fn set_match_limit(&mut self, limit: u16) {
        self.query_cursor.set_match_limit(limit as u32)
    }

    /// Get the maximum number of in-progress matches for this cursor.
    #[inline]
    pub fn match_limit(&self) -> u16 {
        self.query_cursor.match_limit() as u16
    }

    /// Check if, on its last execution, this cursor exceeded its maximum number of in-progress
    /// matches.
    #[inline]
    pub fn did_exceed_match_limit(&self) -> bool {
        self.query_cursor.did_exceed_match_limit()
    }

    /// Set the range in which to search for matches, in terms of byte offsets.
    ///
    /// Returns `self` for convenience (builder pattern)
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) -> &mut Self {
        self.query_cursor.set_byte_range(range);
        self
    }

    /// Set the range in which to search for matches, in terms of rows and columns.
    ///
    /// Returns `self` for convenience (builder pattern)
    #[inline]
    pub fn set_point_range(&mut self, range: PointRange) -> &mut Self {
        self.query_cursor.set_point_range(range.to_ts_point_range());
        self
    }
}

impl<'query, 'tree: 'query, Custom> QueryMatches<'query, 'tree, Custom> {
    /// Get the underlying [tree_sitter::QueryMatches]
    #[inline]
    pub fn as_inner(&self) -> &tree_sitter::QueryMatches<'query, 'tree, &'query Tree<Custom>, &'query str> {
        &self.query_matches
    }

    /// Get the underlying [tree_sitter::QueryMatches]
    #[inline]
    pub fn as_inner_mut(&mut self) -> &mut tree_sitter::QueryMatches<'query, 'tree, &'query Tree<Custom>, &'query str> {
        &mut self.query_matches
    }

    /// Destruct into the underlying [tree_sitter::QueryMatches], query, and tree
    #[inline]
    pub fn into_inner(self) -> (tree_sitter::QueryMatches<'query, 'tree, &'query Tree<Custom>, &'query str>, &'query Query, &'tree Tree<Custom>) {
        (self.query_matches, self.query, self.tree)
    }
}

impl<'query, 'tree: 'query, Custom> StreamingIterator for QueryMatches<'query, 'tree, Custom> {
    type Item = QueryMatch<'query, 'tree, Custom>;

    #[inline]
    fn advance(&mut self) {
        self.current_match = self.query_matches.next().map(|query_match| QueryMatch {
            query_match,
            tree: self.tree,
            query: self.query
        });
    }

    #[inline]
    fn get(&self) -> Option<&Self::Item> {
        self.current_match.as_ref()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.query_matches.size_hint()
    }
}

impl<'query, 'tree: 'query, Custom> StreamingIteratorMut for QueryMatches<'query, 'tree, Custom> {
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.current_match.as_mut()
    }
}

impl<'query, 'tree: 'query, Custom> QueryCaptures<'query, 'tree, Custom> {
    /// Get the underlying [tree_sitter::QueryCaptures]
    #[inline]
    pub fn as_inner(&self) -> &tree_sitter::QueryCaptures<'query, 'tree, &'query Tree<Custom>, &'query str> {
        &self.query_captures
    }

    /// Get the underlying [tree_sitter::QueryCaptures]
    #[inline]
    pub fn as_inner_mut(&mut self) -> &mut tree_sitter::QueryCaptures<'query, 'tree, &'query Tree<Custom>, &'query str> {
        &mut self.query_captures
    }

    /// Destruct into the underlying [tree_sitter::QueryCaptures], query, and tree
    #[inline]
    pub fn into_inner(self) -> (tree_sitter::QueryCaptures<'query, 'tree, &'query Tree<Custom>, &'query str>, &'query Query, &'tree Tree<Custom>) {
        (self.query_captures, self.query, self.tree)
    }
}

impl<'query, 'tree: 'query, Custom> Iterator for QueryCaptures<'query, 'tree, Custom> {
    type Item = QueryCapture<'query, 'tree, Custom>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.query_captures.next().map(|(query_match, index)|
            QueryCapture::new(query_match.captures[index], self.tree, self.query))
    }
}

impl<'query, 'tree, Custom> QueryMatch<'query, 'tree, Custom> {
    /// Iterate all captures in the order they appear.
    #[inline]
    pub fn iter_captures(&self) -> impl Iterator<Item = QueryCapture<'query, 'tree, Custom>> {
        self.query_match.captures.iter().map(|&query_capture|
            QueryCapture::new(query_capture, self.tree, self.query))
    }

    /// Get the capture at the given index (order it appears).
    #[inline]
    pub fn capture(&self, index: usize) -> Option<QueryCapture<'query, 'tree, Custom>> {
        self.query_match.captures.get(index).map(|&query_capture|
            QueryCapture::new(query_capture, self.tree, self.query))
    }

    /// Get the first occurrence of the capture with the given name.
    #[inline]
    pub fn capture_named(&self, name: &str) -> Option<QueryCapture<'query, 'tree, Custom>> {
        self.iter_captures().find(|capture| capture.name == name)
    }

    /// Get every occurrence of the captures with the given name.
    #[inline]
    pub fn captures_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = QueryCapture<'query, 'tree, Custom>> + 'a {
        self.iter_captures().filter(move |capture| capture.name == name)
    }

    /// Get the number of captures in this match.
    #[inline]
    pub fn capture_count(&self) -> usize {
        self.query_match.captures.len()
    }

    /// Get the pattern index of this match.
    #[inline]
    pub fn pattern_index(&self) -> usize {
        self.query_match.pattern_index
    }

    /// Get the id of this match  (honestly I don't know what this does because it's not documented)
    #[inline]
    pub fn id(&self) -> u32 {
        self.query_match.id()
    }

    /// Remove the match (honestly I don't know what this does because it's not documented)
    #[inline]
    pub fn remove(self) {
        self.query_match.remove()
    }

    /// Get the nodes that were captures by the capture at the given index.
    #[inline]
    pub fn nodes_for_capture_index(&self, capture_index: u32) -> impl Iterator<Item = Node<'tree, Custom>> + '_ {
        // SAFETY: Same tree
        self.query_match
            .nodes_for_capture_index(capture_index)
             .map(move |node| unsafe { Node::new(node, self.tree) })
    }
}

impl<'query, 'tree, Custom> QueryCapture<'query, 'tree, Custom> {
    /// Wrap a [tree_sitter::QueryCapture]. This also needs the tree and query for helper methods.
    #[inline]
    fn new(query_capture: tree_sitter::QueryCapture<'tree>, tree: &'tree Tree<Custom>, query: &'query Query) -> Self {
        // SAFETY: fn is internal so the provided tree is always the same as the node's tree
        unsafe {
            Self {
                node: Node::new(query_capture.node, tree),
                index: query_capture.index as usize,
                name: &query.capture_names()[query_capture.index as usize]
            }
        }
    }
}

impl PointRange {
    /// Convert this into an [std::ops::Range]
    #[inline]
    pub fn to_ops_range(self) -> std::ops::Range<Point> {
        self.start..self.end
    }

    /// Convert this into an [std::ops::Range] of [tree_sitter::Point]s
    #[inline]
    pub fn to_ts_point_range(self) -> std::ops::Range<tree_sitter::Point> {
        self.start.0..self.end.0
    }
}

impl RangeBounds<Point> for PointRange {
    fn start_bound(&self) -> Bound<&Point> {
        Bound::Excluded(&self.start)
    }

    fn end_bound(&self) -> Bound<&Point> {
        Bound::Excluded(&self.end)
    }
}

impl Display for PointRange {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl From<std::ops::Range<Point>> for PointRange {
    #[inline]
    fn from(value: std::ops::Range<Point>) -> Self {
        Self {
            start: value.start,
            end: value.end
        }
    }
}

impl Point {
    /// Create a new point
    #[inline]
    pub fn new(row: usize, column: usize) -> Self {
        Self(tree_sitter::Point { row, column })
    }

    /// Get the row of this point
    #[inline]
    pub fn row(&self) -> usize {
        self.0.row
    }

    /// Get the column of this point
    #[inline]
    pub fn column(&self) -> usize {
        self.0.column
    }
}

impl Display for Point {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0.row + 1, self.0.column + 1)
    }
}

impl Into<tree_sitter::Point> for Point {
    #[inline]
    fn into(self) -> tree_sitter::Point {
        self.0
    }
}

impl Range {
    #[inline]
    fn vec_from_ts(range: Vec<tree_sitter::Range>) -> Vec<Self> {
        // SAFETY: Same repr
        unsafe { std::mem::transmute(range) }
    }

    #[inline]
    fn slice_as_ts(ranges: &[Self]) -> &[tree_sitter::Range] {
        // SAFETY: Same repr
        unsafe { std::mem::transmute(ranges) }
    }

    /// Creates a new range
    #[inline]
    pub fn new(start_byte: usize, end_byte: usize, start_point: Point, end_point: Point) -> Self {
        Self(tree_sitter::Range {
            start_byte,
            end_byte,
            start_point: start_point.0,
            end_point: end_point.0
        })
    }

    /// Start byte
    #[inline]
    pub fn start_byte(&self) -> usize {
        self.0.start_byte
    }

    /// End byte
    #[inline]
    pub fn end_byte(&self) -> usize {
        self.0.end_byte
    }

    /// Start point
    #[inline]
    pub fn start_point(&self) -> Point {
        Point(self.0.start_point)
    }

    /// End point
    #[inline]
    pub fn end_point(&self) -> Point {
        Point(self.0.end_point)
    }
}

impl BitOr for Range {
    type Output = Range;

    /// Smallest range which contains both ranges
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
         Range::new(
            self.start_byte().min(rhs.start_byte()),
            self.end_byte().max(rhs.end_byte()),
            self.start_point().min(rhs.start_point()),
            self.end_point().max(rhs.end_point()),
         )
    }
}

impl BitOrAssign for Range {
    /// Smallest range which contains both ranges
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Range {
    type Output = Option<Range>;

    /// Largest range which both ranges contain if not disjoint, otherwise [None]
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        let start_byte = self.start_byte().max(rhs.start_byte());
        let end_byte = self.end_byte().min(rhs.end_byte());
        let start_point = self.start_point().max(rhs.start_point());
        let end_point = self.end_point().min(rhs.end_point());

        if start_byte <= end_byte && start_point <= end_point {
            Some(Range::new(start_byte, end_byte, start_point, end_point))
        } else {
            None
        }
    }
}

impl Display for Range {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start_point(), self.end_point())
    }
}

impl NodeId {
    /// "Magic" node id for an invalid node.
    ///
    /// In theory, there is a *tiny* chance this id could appear in a tree, but in practice it will
    /// never happen.
    pub const INVALID: Self = Self(usize::MAX);

    /// Get the id of the given [tree-sitter::Node]. See [tree_sitter::Node::id].
    #[inline]
    fn of_ts(node: tree_sitter::Node<'_>) -> Self {
        NodeId(node.id())
    }
}

impl From<u64> for NodeId {
    #[inline]
    fn from(value: u64) -> Self {
        NodeId(value as usize)
    }
}

impl Into<u64> for NodeId {
    #[inline]
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl Clone for TreeParseError {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            TreeParseError::IO(e) => TreeParseError::IO(std::io::Error::from(e.kind())),
            TreeParseError::ParsingFailed => TreeParseError::ParsingFailed,
            TreeParseError::NotUtf8(e) => TreeParseError::NotUtf8(*e)
        }
    }
}

impl From<std::io::Error> for TreeParseError {
    fn from(e: std::io::Error) -> Self {
        TreeParseError::IO(e)
    }
}

impl From<Utf8Error> for TreeParseError {
    fn from(e: Utf8Error) -> Self {
        TreeParseError::NotUtf8(e)
    }
}

impl Display for TreeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeParseError::IO(e) => write!(f, "IO error: {}", e),
            TreeParseError::ParsingFailed => write!(f, "Parsing failed"),
            TreeParseError::NotUtf8(e) => write!(f, "Not UTF-8: {}", e)
        }
    }
}

impl Error for TreeParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TreeParseError::IO(e) => Some(e),
            TreeParseError::ParsingFailed => None,
            TreeParseError::NotUtf8(e) => Some(e)
        }
    }
}

impl<Custom> PartialEq for SubTree<Custom> {
    #[inline]
    fn eq(&self, other: &SubTree<Custom>) -> bool {
        self.text == other.text &&
            self.range == other.range
    }
}

impl<Custom> Eq for SubTree<Custom> {}

impl<Custom> Display for SubTree<Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl TraversalState {
    /// Is this the up state?
    #[inline]
    pub fn is_up(&self) -> bool {
        match self {
            TraversalState::Up => true,
            _ => false
        }
    }

    /// Is this the final state (done traversing?)
    #[inline]
    pub fn is_end(&self) -> bool {
        match self {
            TraversalState::End => true,
            _ => false
        }
    }
}

impl<'tree, Custom> PreorderTraversal<'tree, Custom> {
    /// Create a new preorder traversal which will use the given [TreeCursor].
    #[inline]
    pub fn with_cursor(cursor: TreeCursor<'tree, Custom>) -> Self {
        Self {
            cursor,
            last_state: TraversalState::Start,
        }
    }

    /// Create a new preorder traversal which will traverse the given [Tree].
    #[inline]
    pub fn of_tree(tree: &'tree Tree<Custom>) -> Self {
        Self::with_cursor(tree.walk())
    }

    /// Create a new preorder traversal which will traverse the given [Node].
    #[inline]
    pub fn of_node(node: Node<'tree, Custom>) -> Self {
        Self::with_cursor(node.walk())
    }

    /// Get the current item without advancing the traversal.
    #[inline]
    pub fn peek(&mut self) -> TraversalItem<'tree, Custom> {
        TraversalItem {
            node: self.cursor.node(),
            field_name: self.cursor.field_name(),
            last_state: self.last_state
        }
    }

    /// Advance the traversal to the next item and return `true`, or return `false` if it's done.
    #[inline]
    pub fn goto_next(&mut self) -> bool {
        if self.last_state.is_end() {
            false
        } else {
            self.last_state = self.cursor.goto_preorder(self.last_state);
            true
        }
    }
}

impl<'tree, Custom> Iterator for PreorderTraversal<'tree, Custom> {
    type Item = TraversalItem<'tree, Custom>;

    #[inline]
    fn next(&mut self) -> Option<TraversalItem<'tree, Custom>> {
        if self.last_state.is_end() {
            return None
        }
        let item = self.peek();
        self.last_state = self.cursor.goto_preorder(self.last_state);
        Some(item)
    }
}

impl<'tree, Custom> FusedIterator for PreorderTraversal<'tree, Custom> {}

// region special "boilerplate" impls
impl<'tree, Custom> Debug for Node<'tree, Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

impl<Custom> Debug for NodePtr<Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node_data)
    }
}

impl<'query, 'tree, Custom> Debug for QueryMatch<'query, 'tree, Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.query_match)
    }
}
// endregion

// region boilerplate impls, because Custom doesn't need to be bounded
impl<'tree, Custom> PartialEq for TraversalItem<'tree, Custom> {
    fn eq(&self, other: &Self) -> bool {
        if self.node == other.node {
            debug_assert_eq!(self.field_name, other.field_name, "field_name must be the same if node is the same");
        }
        self.node == other.node && self.last_state == other.last_state
    }
}

impl<'tree, Custom> Eq for TraversalItem<'tree, Custom> {}

impl<'tree, Custom> Clone for Node<'tree, Custom> {
    fn clone(&self) -> Self {
        Self {
            node: self.node,
            tree: self.tree
        }
    }
}

impl<'tree, Custom> Copy for Node<'tree, Custom> {}

impl<Custom> Clone for NodePtr<Custom> {
    fn clone(&self) -> Self {
        Self {
            node_data: self.node_data,
            tree: self.tree
        }
    }
}

impl<Custom> Copy for NodePtr<Custom> {}

impl<'tree, Custom> Clone for TreeCursor<'tree, Custom> {
    fn clone(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            tree: self.tree,
            child_depth: self.child_depth
        }
    }
}

impl<'query, 'tree, Custom> Debug for QueryCapture<'query, 'tree, Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryCapture")
            .field("node", &self.node)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}

impl<'query, 'tree, Custom> Clone for QueryCapture<'query, 'tree, Custom> {
    fn clone(&self) -> Self {
        Self {
            node: self.node,
            index: self.index,
            name: self.name
        }
    }
}

impl<'query, 'tree, Custom> Copy for QueryCapture<'query, 'tree, Custom> {}

impl<Custom> Debug for SubTree<Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubTree")
            .field("text", &self.text)
            .field("range", &self.range)
            .field("path", &self.path)
            .finish()
    }
}

impl<Custom> Clone for SubTree<Custom> {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            range: self.range,
            path: self.path.clone(),
            root: self.root
        }
    }
}

impl<'tree, Custom> Clone for PreorderTraversal<'tree, Custom> {
    fn clone(&self) -> Self {
        Self {
            cursor: self.cursor.clone(),
            last_state: self.last_state
        }
    }
}

impl<'tree, Custom> Debug for TraversalItem<'tree, Custom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TraversalItem")
            .field("node", &self.node)
            .field("field_name", &self.field_name)
            .field("last_state", &self.last_state)
            .finish()
    }
}

impl<'tree, Custom> Clone for TraversalItem<'tree, Custom> {
    fn clone(&self) -> Self {
        Self {
            node: self.node,
            field_name: self.field_name,
            last_state: self.last_state
        }
    }
}

impl<'tree, Custom> Copy for TraversalItem<'tree, Custom> {}

impl<'tree, Custom> Hash for TraversalItem<'tree, Custom> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
        self.last_state.hash(state);
    }
}
// endregion