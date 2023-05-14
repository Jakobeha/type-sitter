use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::Path;
use std::iter::{FusedIterator, once, Once};
use std::str::Utf8Error;
use std::hash::{Hash, Hasher};
use std::os::fd::AsRawFd;
use std::ptr::NonNull;
use utf8_error_offset_by::Utf8ErrorOffsetBy;

mod utf8_error_offset_by;

/// Wrapper around [tree_sitter::Tree] which stores the text and marked nodes
#[derive(Debug)]
pub struct Tree {
    tree: tree_sitter::Tree,
    byte_text: Vec<u8>,
    marked_nodes: RefCell<HashMap<NodeId, Bitmask>>
}

/// Wrapper around [tree_sitter::Node]
#[derive(Clone, Copy)]
pub struct Node<'tree> {
    node: tree_sitter::Node<'tree>,
    tree: &'tree Tree,
}

/// Raw pointer equivalent of [Node]
#[derive(Clone, Copy)]
pub struct NodePtr {
    node_data: NodeData,
    tree: NonNull<Tree>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
/// Taken straight from [tree_sitter::ffi::TSNode]. This must maintain the same layout
struct NodeData {
    context: [u32; 4usize],
    id: *const (),
    tree: *const (),
}

/// Wrapper around `usize` (aka node id)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeId(usize);

/// Wrapper around [tree_sitter::TreeCursor], which can actually go outside of its "local" node,
/// albeit with degraded performance (we just do standard lookups)
#[derive(Clone)]
pub struct TreeCursor<'tree> {
    cursor: tree_sitter::TreeCursor<'tree>,
    tree: &'tree Tree,
    child_depth: usize,
}

/// Wrapper around [tree_sitter::QueryCursor]
pub struct QueryCursor {
    query_cursor: tree_sitter::QueryCursor
}

/// Wrapper around [tree_sitter::QueryMatches]
pub struct QueryMatches<'query, 'tree: 'query> {
    query_matches: tree_sitter::QueryMatches<'query, 'tree, &'query Tree>,
    tree: &'tree Tree,
    query: &'query Query
}

/// Wrapper around [tree_sitter::QueryMatch]
pub struct QueryMatch<'query, 'tree> {
    query_match: tree_sitter::QueryMatch<'query, 'tree>,
    tree: &'tree Tree,
    query: &'query Query
}

/// Wrapper around [tree_sitter::QueryCapture]
pub struct QueryCaptures<'query, 'tree> {
    query_captures: tree_sitter::QueryCaptures<'query, 'tree, &'query Tree>,
    tree: &'tree Tree,
    query: &'query Query
}

/// Wrapper around [tree_sitter::QueryCapture]
#[derive(Debug, Clone, Copy)]
pub struct QueryCapture<'query, 'tree> {
    pub node: Node<'tree>,
    pub name: &'query str,
}

/// Wrapper around [tree_sitter::Range], which displays as `:line:column-:line:column`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Range(tree_sitter::Range);

/// Wrapper around [tree_sitter::Point], which displays as `:line:column`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Point(tree_sitter::Point);

/// Wrapper around [tree_sitter::Parser]
#[repr(transparent)]
pub struct Parser(tree_sitter::Parser);

/// Re-exports [tree_sitter::Language]
pub type Language = tree_sitter::Language;
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

#[derive(Debug)]
pub enum TreeParseError {
    IO(std::io::Error),
    ParsingFailed,
    NotUtf8(Utf8Error)
}

/// General-purpose way to store TSNode separately from the tree, e.g. if you need to serialize it.
/// Unfortunately this is just done by storing the text and range, there's not much else we can do
#[derive(Debug, Clone)]
pub struct SubTree {
    pub text: String,
    pub range: Range,
    /// Node which can be dereferenced in case the tree is still alive,
    /// otherwise it is dangling
    pub root: Option<NodePtr>,
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
#[derive(Clone)]
pub struct PreorderTraversal<'tree> {
    cursor: TreeCursor<'tree>,
    last_state: TraversalState
}

/// Iterated node in a traversal (includes field name and last state)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraversalItem<'tree> {
    /// The node
    pub node: Node<'tree>,
    /// The node's field name
    pub field_name: Option<&'static str>,
    /// Last traversal state to reach this node
    pub last_state: TraversalState
}

/// Reprint a [Tree] with or without nodes with and without certain markings
#[derive(Debug)]
pub struct DisplayTree<'a> {
    tree: &'a Tree,
    include_mark: Bitmask,
    exclude_mark: Bitmask,
}

/// Primitive bitmask
pub type Bitmask = u64;

impl Parser {
    /// Create a new parser for the given language. See [tree_sitter::Parser::set_language]
    #[inline]
    pub fn new(language: tree_sitter::Language) -> Result<Self, LanguageError> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(language)?;
        Ok(Self(parser))
    }

    /// Set the language of the parser. See [tree_sitter::Parser::set_language]
    #[inline]
    pub fn set_language(&mut self, language: Language) -> Result<(), LanguageError> {
        self.0.set_language(language)
    }

    /// Set the ranges of text the parser should include when parsing. See
    /// [tree_sitter::Parser::set_included_ranges]
    #[inline]
    pub fn set_included_ranges(&mut self, ranges: &[Range]) -> Result<(), IncludedRangesError> {
        // SAFETY: Same repr
        self.0.set_included_ranges(unsafe { std::mem::transmute(ranges) })
    }

    /// Parse a file. See [tree_sitter::Parser::parse]
    #[inline]
    pub fn parse_file(&mut self, path: &Path, old_tree: Option<&Tree>) -> Result<Tree, TreeParseError> {
        self.parse_bytes(std::fs::read(path)?, old_tree)
    }

    /// Parse a string. See [tree_sitter::Parser::parse]
    #[inline]
    pub fn parse_string(&mut self, text: String, old_tree: Option<&Tree>) -> Result<Tree, TreeParseError> {
        self.parse_bytes(text.into_bytes(), old_tree)
    }

    /// Parse a byte string. See [tree_sitter::Parser::parse]. Note that the wrappers expect and
    /// assume UTF-8, so this will fail if the text is not valid UTF-8.
    #[inline]
    pub fn parse_bytes(&mut self, byte_text: Vec<u8>, old_tree: Option<&Tree>) -> Result<Tree, TreeParseError> {
        let tree = self.0.parse(&byte_text, old_tree.map(|t| &t.tree)).ok_or(TreeParseError::ParsingFailed)?;
        Ok(Tree::new(tree, byte_text)?)
    }
}

impl Tree {
    /// Wrap the tree and its associated text. Note that the wrappers expect and assume UTF-8, so
    /// this will fail if the text is not valid UTF-8.
    #[inline]
    fn new(tree: tree_sitter::Tree, byte_text: Vec<u8>) -> Result<Self, Utf8Error> {
        Self::validate_utf8(&tree, &byte_text)?;
        Ok(Self { tree, byte_text, marked_nodes: RefCell::default() })
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

    /// Get the root node.
    #[inline]
    pub fn root_node(&self) -> Node<'_> {
        // SAFETY: The node is from this tree
        unsafe { Node::new(self.tree.root_node(), self) }
    }

    /// Create a [TreeCursor] starting at the root node.
    #[inline]
    pub fn walk(&self) -> TreeCursor<'_> {
        TreeCursor::new(self.tree.walk(), self, true)
    }

    /// Get the included ranges used to parse the tree.
    #[inline]
    pub fn included_ranges(&self) -> Vec<Range> {
        // SAFETY: Same repr
        unsafe { std::mem::transmute(self.tree.included_ranges()) }
    }

    /// Get the changed ranges. See [tree_sitter::Tree::changed_ranges]
    #[inline]
    pub fn changed_ranges(&self, other: &Tree) -> impl ExactSizeIterator<Item=Range> {
        // SAFETY: Same repr
        self.tree.changed_ranges(&other.tree).map(|r| unsafe { std::mem::transmute(r) })
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
    pub fn display_skipping(&self, include_mark: Bitmask, exclude_mark: Bitmask) -> DisplayTree<'_> {
        DisplayTree {
            tree: self,
            include_mark,
            exclude_mark,
        }
    }
}

impl<'tree> tree_sitter::TextProvider<'tree> for &'tree Tree {
    type I = Once<&'tree [u8]>;

    #[inline]
    fn text(&mut self, node: tree_sitter::Node<'_>) -> Self::I {
        once(&self.byte_text[node.byte_range()])
    }
}

impl<'tree> Node<'tree> {
    /// Wrap a [tree_sitter::Node]. Requires its associated [Tree] for convenience methods.
    ///
    /// SAFETY: The node must be from the given tree.
    #[inline]
    pub unsafe fn new(node: tree_sitter::Node<'tree>, tree: &'tree Tree) -> Self {
        Self { node, tree }
    }

    /// Get a tree-unique node id. See [tree_sitter::Node::id]
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

    /// Get the row and column range where this node is located.
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
        // SAFETY: we ran validate_utf8 before constructing so all nodes are valid UTF-8
        unsafe { std::str::from_utf8_unchecked(self.byte_text()) }
    }

    /// Get the node's named and unnamed children. See [tree_sitter::Node::children]
    #[inline]
    pub fn all_children<'a>(&self, cursor: &'a mut TreeCursor<'tree>) -> impl ExactSizeIterator<Item = Node<'tree>> + 'a {
        let tree = self.tree;
        // SAFETY: Same tree
        self.node.children(&mut cursor.cursor).map(move |node| unsafe { Node::new(node, tree) })
    }

    /// Get the node's named children. See [tree_sitter::Node::named_children]
    #[inline]
    pub fn named_children<'a>(&self, cursor: &'a mut TreeCursor<'tree>) -> impl ExactSizeIterator<Item = Node<'tree>> + 'a {
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
    pub fn parent(&self) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.parent().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate next sibling, named or unnamed.
    #[inline]
    pub fn next_any_sibling(&self) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.next_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate named next sibling.
    #[inline]
    pub fn next_named_sibling(&self) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.next_named_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate previous sibling, named or unnamed.
    #[inline]
    pub fn prev_any_sibling(&self) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.prev_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's immediate named previous sibling.
    #[inline]
    pub fn prev_named_sibling(&self) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.prev_named_sibling().map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's child at the given index, named or unnamed. See [tree_sitter::Node::child]
    #[inline]
    pub fn any_child(&self, i: usize) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.child(i).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's named child at the given index. See [tree_sitter::Node::named_child]
    #[inline]
    pub fn named_child(&self, i: usize) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.named_child(i).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's last child, named or unnamed.
    #[inline]
    pub fn last_any_child(&self) -> Option<Node<'tree>> {
        // .child is already bounds-checked so we use wrapping_sub for iff the count is 0
        // SAFETY: Same tree
        self.node.child(self.any_child_count().wrapping_sub(1)).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's last named child.
    #[inline]
    pub fn last_named_child(&self) -> Option<Node<'tree>> {
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
        cursor: &mut TreeCursor<'tree>
    ) -> Option<Node<'tree>> {
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
        cursor: &'a mut TreeCursor<'tree>
    ) -> impl Iterator<Item = Node<'tree>> + 'a {
        // SAFETY: Same tree
        self.node.named_children(&mut cursor.cursor)
            .filter(move |node| node.kind() == kind)
            .map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the first child with the given field name.
    #[inline]
    pub fn child_by_field_name(&self, field_name: &str) -> Option<Node<'tree>> {
        // SAFETY: Same tree
        self.node.child_by_field_name(field_name).map(|node| unsafe { Node::new(node, self.tree) })
    }

    /// Get the node's children with the given field name.
    #[inline]
    pub fn children_by_field_name<'a>(
        &self,
        field_name: &str,
        c: &'a mut TreeCursor<'tree>
    ) -> impl Iterator<Item = Node<'tree>> + 'a {
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
    pub fn field_name(&self, cursor: &mut TreeCursor<'tree>) -> Option<&'static str> {
        self.parent().and_then(|parent| {
            let i = parent.all_children(cursor)
                .position(|x| x == *self)
                .expect("node not one of its parent's children");
            parent.field_name_for_child(i)
        })
    }

    /// Get a [TreeCursor] that points to this node.
    #[inline]
    pub fn walk(&self) -> TreeCursor<'tree> {
        TreeCursor::new(self.node.walk(), self.tree, false)
    }

    /// Print the node as an s-expression
    #[inline]
    pub fn to_sexp(&self) -> String {
        self.node.to_sexp()
    }

    /// Get a raw pointer to this node (remove the 'tree lifetime).
    #[inline]
    pub fn to_ptr(&self) -> NodePtr {
        NodePtr {
            node_data: NodeData::from(self.node),
            tree: NonNull::from(self.tree),
        }
    }

    /// Get a [SubTree] that represents this node without the lifetime.
    /// The text and range will persist even after this node's tree is deallocated,
    /// and if we know for certain it isn't, the original node can be `unsafe`ly recovered
    #[inline]
    pub fn to_subtree(&self) -> SubTree {
        SubTree {
            text: self.text().to_string(),
            range: self.range(),
            root: Some(self.to_ptr()),
        }
    }

    /// Bit-or the mark to this node. Returns the old mark
    pub fn mark(&self, mark: Bitmask) -> Bitmask {
        let mut old_mark = 0;
        self.tree.marked_nodes.borrow_mut().entry(self.id())
            .and_modify(|m| {
                old_mark = *m;
                *m |= mark
            })
            .or_insert(mark);
        old_mark
    }

    /// Bit-and-not the mark to this node. Returns the old mark
    pub fn unmark(&self, mark: Bitmask) -> Bitmask {
        let mut old_mark = 0;
        self.tree.marked_nodes.borrow_mut().entry(self.id())
            .and_modify(|m| {
                old_mark = *m;
                *m &= !mark
            })
            .or_insert(0);
        old_mark
    }

    /// Bit-xor the mark to this node. Returns the old mark
    pub fn toggle_mark(&self, mark: Bitmask) -> Bitmask {
        let mut old_mark = 0;
        self.tree.marked_nodes.borrow_mut().entry(self.id())
            .and_modify(|m| {
                old_mark = *m;
                *m ^= mark
            })
            .or_insert(mark);
        old_mark
    }

    /// Bit-and the mark to this node. Returns the old mark
    pub fn filter_mark(&self, mark: Bitmask) -> Bitmask {
        let mut old_mark = 0;
        self.tree.marked_nodes.borrow_mut().entry(self.id())
            .and_modify(|m| {
                old_mark = *m;
                *m &= mark
            })
            .or_insert(0);
        old_mark
    }

    /// Get the node's mark
    #[inline]
    pub fn get_mark(&self) -> Bitmask {
        self.tree.marked_nodes.borrow().get(&self.id()).cloned().unwrap_or(0)
    }

    /// Bit-or the mark to all of the children except the given one. Returns if (any, all) child
    /// nodes were already marked with the entire mark.
    pub fn mark_children_except(&self, mark: Bitmask, except: Node<'tree>, cursor: &mut TreeCursor<'tree>) -> (bool, bool) {
        let mut marked_nodes = self.tree.marked_nodes.borrow_mut();
        let mut already_marked = (false, true);
        for child in self.all_children(cursor) {
            if child.id() != except.id() {
                let mut already_marked1 = false;
                marked_nodes.entry(child.id())
                    .and_modify(|m| {
                        already_marked1 = *m & mark == mark;
                        *m |= mark
                    })
                    .or_insert(mark);
                already_marked.0 |= already_marked1;
                already_marked.1 &= already_marked1;
            }
        }
        already_marked
    }

    /// Bit-and the mark to all of the children except the given one. Returns if (any, all) child
    /// nodes were already not marked with the entire mark.
    pub fn unmark_children_except(&self, mark: Bitmask, except: Node<'tree>, cursor: &mut TreeCursor<'tree>) -> (bool, bool) {
        let mut marked_nodes = self.tree.marked_nodes.borrow_mut();
        let mut already_unmarked = (false, true);
        for child in self.all_children(cursor) {
            if child.id() != except.id() {
                let mut already_unmarked1 = false;
                marked_nodes.entry(child.id())
                    .and_modify(|m| {
                        already_unmarked1 |= *m & mark == 0;
                        *m &= !mark
                    })
                    .or_insert(mark);
                already_unmarked.0 |= already_unmarked1;
                already_unmarked.1 &= already_unmarked1;
            }
        }
        already_unmarked
    }
}

impl<'tree> PartialEq<Node<'tree>> for Node<'tree> {
    #[inline]
    fn eq(&self, other: &Node<'tree>) -> bool {
        self.id() == other.id()
    }
}

impl<'tree> Eq for Node<'tree> {}

impl<'tree> Hash for Node<'tree> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl NodePtr {
    /// SAFETY: You must ensure that the tree the node came from is alive
    #[inline]
    pub unsafe fn to_node(&self) -> Node {
        Node {
            node: self.node_data.to_node(),
            tree: self.tree.as_ref(),
        }
    }
}

impl PartialEq<NodePtr> for NodePtr {
    #[inline]
    fn eq(&self, other: &NodePtr) -> bool {
        self.node_data == other.node_data
    }
}

impl Eq for NodePtr {}

impl Hash for NodePtr {
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

impl<'tree> TreeCursor<'tree> {
    /// Wrap a [tree-sitter::TreeCursor]. You must also provide the tree and whether the cursor is
    /// at the root node.
    #[inline]
    fn new(cursor: tree_sitter::TreeCursor<'tree>, tree: &'tree Tree, is_rooted: bool) -> Self {
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
    pub fn node(&self) -> Node<'tree> {
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
    pub fn goto(&mut self, node: Node<'tree>) {
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

    /// Move the cursor to the first child of the current node that extends beyond the given point
    /// offset, and return its index. Returns `false` if the current node has no children past that
    /// offset.
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
    pub fn matches<'query, 'tree: 'query>(&'query mut self, query: &'query Query, node: Node<'tree>) -> QueryMatches<'query, 'tree> {
        QueryMatches {
            query_matches: self.query_cursor.matches(&query, node.node, node.tree),
            tree: node.tree,
            query
        }
    }

    /// Iterate over all captures in the order they appear. See [tree_sitter::QueryCursor::captures]
    #[inline]
    pub fn captures<'query, 'tree: 'query>(&'query mut self, query: &'query Query, node: Node<'tree>) -> QueryCaptures<'query, 'tree> {
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
    #[inline]
    pub fn set_byte_range(&mut self, range: std::ops::Range<usize>) {
        self.query_cursor.set_byte_range(range);
    }

    /// Set the range in which to search for matches, in terms of points.
    #[inline]
    pub fn set_point_range(&mut self, range: std::ops::Range<Point>) {
        // SAFETY: Same repr
        self.query_cursor.set_point_range(unsafe { std::mem::transmute(range) });
    }
}

impl<'query, 'tree: 'query> Iterator for QueryMatches<'query, 'tree> {
    type Item = QueryMatch<'query, 'tree>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.query_matches.next().map(|query_match| QueryMatch {
            query_match,
            tree: self.tree,
            query: self.query
        })
    }
}

impl<'query, 'tree: 'query> Iterator for QueryCaptures<'query, 'tree> {
    type Item = QueryCapture<'query, 'tree>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.query_captures.next().map(|(query_match, index)|
            QueryCapture::new(query_match.captures[index], self.tree, self.query))
    }
}

impl<'query, 'tree> QueryMatch<'query, 'tree> {
    /// Iterate all captures in the order they appear.
    #[inline]
    pub fn iter_captures(&self) -> impl Iterator<Item = QueryCapture<'query, 'tree>> {
        self.query_match.captures.iter().map(|&query_capture|
            QueryCapture::new(query_capture, self.tree, self.query))
    }

    /// Get the capture at the given index (order it appears).
    #[inline]
    pub fn capture(&self, index: usize) -> Option<QueryCapture<'query, 'tree>> {
        self.query_match.captures.get(index).map(|&query_capture|
            QueryCapture::new(query_capture, self.tree, self.query))
    }

    /// Get the first occurrence of the capture with the given name.
    #[inline]
    pub fn capture_named(&self, name: &str) -> Option<QueryCapture<'query, 'tree>> {
        self.iter_captures().find(|capture| capture.name == name)
    }

    /// Get every occurrence of the captures with the given name.
    #[inline]
    pub fn captures_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = QueryCapture<'query, 'tree>> + 'a {
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
    pub fn nodes_for_capture_index(&self, capture_index: u32) -> impl Iterator<Item = Node<'tree>> + '_ {
        // SAFETY: Same tree
        self.query_match
            .nodes_for_capture_index(capture_index)
             .map(move |node| unsafe { Node::new(node, self.tree) })
    }
}

impl<'query, 'tree> QueryCapture<'query, 'tree> {
    /// Wrap a [tree_sitter::QueryCapture]. This also needs the tree and query for helper methods.
    #[inline]
    fn new(query_capture: tree_sitter::QueryCapture<'tree>, tree: &'tree Tree, query: &'query Query) -> Self {
        // SAFETY: fn is internal so the provided tree is always the same as the node's tree
        unsafe {
            Self {
                node: Node::new(query_capture.node, tree),
                name: &query.capture_names()[query_capture.index as usize]
            }
        }
    }
}

impl Range {
    /// Get the start point
    #[inline]
    pub fn start_point(&self) -> Point {
        Point(self.0.start_point)
    }

    /// Get the end point
    #[inline]
    pub fn end_point(&self) -> Point {
        Point(self.0.end_point)
    }
}

impl Display for Range {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", Point(self.0.start_point), Point(self.0.end_point))
    }
}

impl Into<tree_sitter::Range> for Range {
    #[inline]
    fn into(self) -> tree_sitter::Range {
        self.0
    }
}

impl Display for Point {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0.row + 1, self.0.column + 1)
    }
}

impl Into<tree_sitter::Point> for Point {
    #[inline]
    fn into(self) -> tree_sitter::Point {
        self.0
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl PartialEq<SubTree> for SubTree {
    #[inline]
    fn eq(&self, other: &SubTree) -> bool {
        self.text == other.text &&
            self.range == other.range
    }
}

impl Eq for SubTree {}

impl Display for SubTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<'tree> PreorderTraversal<'tree> {
    /// Create a new preorder traversal which will use the given [TreeCursor].
    #[inline]
    pub fn with_cursor(cursor: TreeCursor<'tree>) -> Self {
        Self {
            cursor,
            last_state: TraversalState::Start,
        }
    }

    /// Create a new preorder traversal which will traverse the given [Tree].
    #[inline]
    pub fn of_tree(tree: &'tree Tree) -> Self {
        Self::with_cursor(tree.walk())
    }

    /// Create a new preorder traversal which will traverse the given [Node].
    #[inline]
    pub fn of_node(node: Node<'tree>) -> Self {
        Self::with_cursor(node.walk())
    }

    /// Get the current item without advancing the traversal.
    #[inline]
    pub fn peek(&mut self) -> TraversalItem<'tree> {
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

impl<'tree> Iterator for PreorderTraversal<'tree> {
    type Item = TraversalItem<'tree>;

    #[inline]
    fn next(&mut self) -> Option<TraversalItem<'tree>> {
        if self.last_state.is_end() {
            return None
        }
        let item = self.peek();
        self.last_state = self.cursor.goto_preorder(self.last_state);
        Some(item)
    }
}

impl<'tree> FusedIterator for PreorderTraversal<'tree> {}

impl<'a> Display for DisplayTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes_with_marked_child = HashSet::new();
        let mut did_write = false;
        let mut c = self.tree.walk();
        let mut c2 = self.tree.walk();
        let mut state = TraversalState::Start;
        let mut state2 = c2.goto_preorder(TraversalState::Start);
        loop {
            state = c.goto_preorder(state);
            debug_assert_eq!(state, state2);
            if state.is_end() {
                break
            }

            let mut has_marked_child = false;
            state2 = c2.goto_preorder(state2);
            if matches!(state2, TraversalState::Down) {
                let mut depth = 1;
                while depth > 0 {
                    if nodes_with_marked_child.contains(&c2.node().id()) {
                        // Child is marked and we already recorded that parents are too
                        has_marked_child = true;
                        while depth > 0 {
                            c2.goto_parent();
                            depth -= 1;
                        }
                        continue // `break`s since depth == 0
                    }
                    let mark = c2.node().get_mark();
                    if mark & self.include_mark == self.include_mark && mark & self.exclude_mark == 0 {
                        // Marked and not recorded, so we must record this and children
                        has_marked_child = true;
                        nodes_with_marked_child.insert(c2.node().id());
                        while depth > 0 {
                            c2.goto_parent();
                            nodes_with_marked_child.insert(c2.node().id());
                            depth -= 1;
                        }
                        continue // `break`s since depth == 0
                    }
                    // Haven't found a marked child yet
                    state2 = c2.goto_preorder(state2);
                    match state2 {
                        TraversalState::Down => depth += 1,
                        TraversalState::Up => depth -= 1,
                        _ => {}
                    }
                }
            }
            if !has_marked_child {
                if did_write {
                    write!(f, " ")?;
                } else {
                    did_write = true;
                }
                write!(f, "{}", c.node().text())?;

                // Skip children
                if matches!(state2, TraversalState::Down) {
                    state = TraversalState::Up;
                    state2 = TraversalState::Up;
                }
            }
        }
        Ok(())
    }
}

impl<'tree> Debug for Node<'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

impl Debug for NodePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node_data)
    }
}

impl<'query, 'tree> Debug for QueryMatch<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.query_match)
    }
}