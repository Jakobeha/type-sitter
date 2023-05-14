#[allow(non_upper_case_globals)]
static __Highlights__: once_cell::race::OnceBox<tree_sitter::Query> = once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Highlights() -> tree_sitter::Query {
    let mut query = tree_sitter::Query::new(
            tree_sitter_json::language(),
            "(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n",
        )
        .expect(
            "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_json' correct, and did you use the same tree-sitter / tree_sitter_json version?",
        );
    query
}
/**Typed version of the query:

```sexp
(pair
  key: (_) @string.special.key)

(string) @string

(number) @number

[
  (null)
  (true)
  (false)
] @constant.builtin

(escape_sequence) @escape

(comment) @comment

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
pub type HighlightsMatches<'cursor, 'tree> = type_sitter_lib::TypedQueryMatches<
    'cursor,
    'tree,
    HighlightsMatch<'cursor, 'tree>,
>;
pub type HighlightsCaptures<'cursor, 'tree> = type_sitter_lib::TypedQueryCaptures<
    'cursor,
    'tree,
    HighlightsMatch<'cursor, 'tree>,
>;
/**A match returned by the query [Highlights]:

```sexp
(pair
  key: (_) @string.special.key)

(string) @string

(number) @number

[
  (null)
  (true)
  (false)
] @constant.builtin

(escape_sequence) @escape

(comment) @comment

```*/
#[derive(Debug)]
pub struct HighlightsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
/**A capture returned by the query [Highlights]:

```sexp
(pair
  key: (_) @string.special.key)

(string) @string

(number) @number

[
  (null)
  (true)
  (false)
] @constant.builtin

(escape_sequence) @escape

(comment) @comment

```*/
#[derive(Debug)]
pub enum HighlightsCapture<'cursor, 'tree> {
    ///A `string.special.key`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///pair @string.special.key
    ///```
    StringSpecialKey {
        node: super::nodes::Pair<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `string`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @string*/
    ///```
    String {
        node: super::nodes::Pair<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `number`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @number*/
    ///```
    Number {
        node: super::nodes::Pair<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `constant.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @constant.builtin*/
    ///```
    ConstantBuiltin {
        node: super::nodes::Pair<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `escape`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @escape*/
    ///```
    Escape {
        node: super::nodes::Pair<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `comment`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @comment*/
    ///```
    Comment {
        node: super::nodes::Pair<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Highlights {
    type Match<'cursor, 'tree> = HighlightsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree> = HighlightsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n"
    }
    fn query(&self) -> &'static tree_sitter::Query {
        __Highlights__.get_or_init(__Mk__Highlights)
    }
    #[inline]
    unsafe fn wrap_match<'cursor, 'tree>(
        &self,
        match_: tree_sitter::QueryMatch<'cursor, 'tree>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_, tree }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture.index as usize {
            0usize => {
                Self::Capture::StringSpecialKey {
                    node: <super::nodes::Pair<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(unsafe {
                        type_sitter_lib::tree_sitter_wrapper::Node::new(
                            capture.node,
                            tree,
                        )
                    }),
                    match_,
                }
            }
            1usize => {
                Self::Capture::String {
                    node: <super::nodes::Pair<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(unsafe {
                        type_sitter_lib::tree_sitter_wrapper::Node::new(
                            capture.node,
                            tree,
                        )
                    }),
                    match_,
                }
            }
            2usize => {
                Self::Capture::Number {
                    node: <super::nodes::Pair<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(unsafe {
                        type_sitter_lib::tree_sitter_wrapper::Node::new(
                            capture.node,
                            tree,
                        )
                    }),
                    match_,
                }
            }
            3usize => {
                Self::Capture::ConstantBuiltin {
                    node: <super::nodes::Pair<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(unsafe {
                        type_sitter_lib::tree_sitter_wrapper::Node::new(
                            capture.node,
                            tree,
                        )
                    }),
                    match_,
                }
            }
            4usize => {
                Self::Capture::Escape {
                    node: <super::nodes::Pair<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(unsafe {
                        type_sitter_lib::tree_sitter_wrapper::Node::new(
                            capture.node,
                            tree,
                        )
                    }),
                    match_,
                }
            }
            5usize => {
                Self::Capture::Comment {
                    node: <super::nodes::Pair<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(unsafe {
                        type_sitter_lib::tree_sitter_wrapper::Node::new(
                            capture.node,
                            tree,
                        )
                    }),
                    match_,
                }
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsMatch<'cursor, 'tree> {
    ///Returns an iterator over the nodes captured by `string.special.key`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///pair @string.special.key
    ///```
    #[inline]
    pub fn string_special_key(&self) -> Option<super::nodes::Pair<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `string`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @string*/
    ///```
    #[inline]
    pub fn string(&self) -> Option<super::nodes::Pair<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `number`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @number*/
    ///```
    #[inline]
    pub fn number(&self) -> Option<super::nodes::Pair<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `constant.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @constant.builtin*/
    ///```
    #[inline]
    pub fn constant_builtin(&self) -> Option<super::nodes::Pair<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `escape`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @escape*/
    ///```
    #[inline]
    pub fn escape(&self) -> Option<super::nodes::Pair<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `comment`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @comment*/
    ///```
    #[inline]
    pub fn comment(&self) -> Option<super::nodes::Pair<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
for HighlightsMatch<'cursor, 'tree> {
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Highlights
    }
    #[inline]
    fn tree(&self) -> &'tree type_sitter_lib::tree_sitter_wrapper::Tree {
        self.tree
    }
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.0
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsCapture<'cursor, 'tree> {
    ///Try to interpret this capture as a `string.special.key`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///pair @string.special.key
    ///```
    #[inline]
    pub fn string_special_key(&self) -> Option<&super::nodes::Pair<'tree>> {
        match self {
            Self::StringSpecialKey { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `string`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @string*/
    ///```
    #[inline]
    pub fn string(&self) -> Option<&super::nodes::Pair<'tree>> {
        match self {
            Self::String { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `number`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @number*/
    ///```
    #[inline]
    pub fn number(&self) -> Option<&super::nodes::Pair<'tree>> {
        match self {
            Self::Number { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `constant.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @constant.builtin*/
    ///```
    #[inline]
    pub fn constant_builtin(&self) -> Option<&super::nodes::Pair<'tree>> {
        match self {
            Self::ConstantBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `escape`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @escape*/
    ///```
    #[inline]
    pub fn escape(&self) -> Option<&super::nodes::Pair<'tree>> {
        match self {
            Self::Escape { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `comment`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(pair
  key: (_) @string.special.key) @comment*/
    ///```
    #[inline]
    pub fn comment(&self) -> Option<&super::nodes::Pair<'tree>> {
        match self {
            Self::Comment { node, .. } => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for HighlightsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::StringSpecialKey { node, .. } => {
                Self::StringSpecialKey {
                    node: *node,
                    match_: None,
                }
            }
            Self::String { node, .. } => {
                Self::String {
                    node: *node,
                    match_: None,
                }
            }
            Self::Number { node, .. } => {
                Self::Number {
                    node: *node,
                    match_: None,
                }
            }
            Self::ConstantBuiltin { node, .. } => {
                Self::ConstantBuiltin {
                    node: *node,
                    match_: None,
                }
            }
            Self::Escape { node, .. } => {
                Self::Escape {
                    node: *node,
                    match_: None,
                }
            }
            Self::Comment { node, .. } => {
                Self::Comment {
                    node: *node,
                    match_: None,
                }
            }
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
for HighlightsCapture<'cursor, 'tree> {
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Highlights
    }
    #[inline]
    fn match_(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::StringSpecialKey { match_, .. } => match_.as_ref(),
            Self::String { match_, .. } => match_.as_ref(),
            Self::Number { match_, .. } => match_.as_ref(),
            Self::ConstantBuiltin { match_, .. } => match_.as_ref(),
            Self::Escape { match_, .. } => match_.as_ref(),
            Self::Comment { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::StringSpecialKey { match_, .. } => match_,
            Self::String { match_, .. } => match_,
            Self::Number { match_, .. } => match_,
            Self::ConstantBuiltin { match_, .. } => match_,
            Self::Escape { match_, .. } => match_,
            Self::Comment { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(
        &self,
    ) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::StringSpecialKey { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "string.special.key",
                }
            }
            Self::String { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "string",
                }
            }
            Self::Number { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "number",
                }
            }
            Self::ConstantBuiltin { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "constant.builtin",
                }
            }
            Self::Escape { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "escape",
                }
            }
            Self::Comment { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "comment",
                }
            }
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::StringSpecialKey { node, .. } => node.node(),
            Self::String { node, .. } => node.node(),
            Self::Number { node, .. } => node.node(),
            Self::ConstantBuiltin { node, .. } => node.node(),
            Self::Escape { node, .. } => node.node(),
            Self::Comment { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::StringSpecialKey { node, .. } => node.node_mut(),
            Self::String { node, .. } => node.node_mut(),
            Self::Number { node, .. } => node.node_mut(),
            Self::ConstantBuiltin { node, .. } => node.node_mut(),
            Self::Escape { node, .. } => node.node_mut(),
            Self::Comment { node, .. } => node.node_mut(),
        }
    }
}
