#[allow(non_upper_case_globals)]
static __Highlights__: type_sitter_lib::gen_internal::TypedQueryOnceBox<
    tree_sitter::Query,
> = type_sitter_lib::gen_internal::TypedQueryOnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Highlights() -> Box<tree_sitter::Query> {
    #[allow(unused_mut)]
    let mut query = tree_sitter::Query::new(
            tree_sitter_json::language(),
            "(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n",
        )
        .expect(
            "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_json' correct, and did you use the same tree-sitter / tree_sitter_json version?",
        );
    Box::new(query)
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
/**Matches returned by a query cursor running the query [Highlights]:

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
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'cursor, 'tree> = type_sitter_lib::TypedQueryMatches<
    'cursor,
    'tree,
    Highlights,
>;
/**Captures returned by a query cursor running the query [Highlights]:

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
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'cursor, 'tree> = type_sitter_lib::TypedQueryCaptures<
    'cursor,
    'tree,
    Highlights,
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
pub enum HighlightsCapture<'cursor, 'tree> {
    ///A `string.special.key` ([type_sitter_lib::UntypedNamedNode])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(_) @string.special.key
    ///```
    StringSpecialKey {
        node: type_sitter_lib::UntypedNamedNode<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `string` ([super::nodes::String])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string) @string
    ///```
    String {
        node: super::nodes::String<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `number` ([super::nodes::Number])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(number) @number
    ///```
    Number {
        node: super::nodes::Number<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `constant.builtin` ([anon_unions::ConstantBuiltin])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (null)
  (true)
  (false)
] @constant.builtin*/
    ///```
    ConstantBuiltin {
        node: anon_unions::ConstantBuiltin<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `escape` ([super::nodes::EscapeSequence])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    Escape {
        node: super::nodes::EscapeSequence<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `comment` ([super::nodes::Comment])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    Comment {
        node: super::nodes::Comment<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Highlights {
    type Match<'cursor, 'tree: 'cursor> = HighlightsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree: 'cursor> = HighlightsCapture<'cursor, 'tree>;
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
                    node: <type_sitter_lib::UntypedNamedNode<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(unsafe {
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
                    node: <super::nodes::String<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(unsafe {
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
                    node: <super::nodes::Number<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(unsafe {
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
                    node: <anon_unions::ConstantBuiltin<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(unsafe {
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
                    node: <super::nodes::EscapeSequence<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(unsafe {
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
                    node: <super::nodes::Comment<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(unsafe {
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
    ///Returns an iterator over the nodes captured by `string.special.key` ([type_sitter_lib::UntypedNamedNode])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(_) @string.special.key
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string_special_key(
        &self,
    ) -> Option<type_sitter_lib::UntypedNamedNode<'tree>> {
        {
            [0u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <type_sitter_lib::UntypedNamedNode<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `string` ([super::nodes::String])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string) @string
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string(&self) -> Option<super::nodes::String<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::String<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `number` ([super::nodes::Number])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(number) @number
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn number(&self) -> Option<super::nodes::Number<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Number<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `constant.builtin` ([anon_unions::ConstantBuiltin])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (null)
  (true)
  (false)
] @constant.builtin*/
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constant_builtin(&self) -> Option<anon_unions::ConstantBuiltin<'tree>> {
        {
            [3u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::ConstantBuiltin<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `escape` ([super::nodes::EscapeSequence])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn escape(&self) -> Option<super::nodes::EscapeSequence<'tree>> {
        {
            [4u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::EscapeSequence<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `comment` ([super::nodes::Comment])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn comment(&self) -> Option<super::nodes::Comment<'tree>> {
        {
            [5u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Comment<
                        'tree,
                    > as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
            .next()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for HighlightsMatch<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(HighlightsMatch))
            .field("match_", &self.match_)
            .finish()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
for HighlightsMatch<'cursor, 'tree> {
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Highlights
    }
    #[inline]
    fn tree(&self) -> &'tree type_sitter_lib::tree_sitter_wrapper::Tree {
        self.tree
    }
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.match_
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.match_
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsCapture<'cursor, 'tree> {
    ///Try to interpret this capture as a `string.special.key` ([type_sitter_lib::UntypedNamedNode])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(_) @string.special.key
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string_special_key(
        &self,
    ) -> Option<&type_sitter_lib::UntypedNamedNode<'tree>> {
        match self {
            Self::StringSpecialKey { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    ///Try to interpret this capture as a `string` ([super::nodes::String])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string) @string
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string(&self) -> Option<&super::nodes::String<'tree>> {
        match self {
            Self::String { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    ///Try to interpret this capture as a `number` ([super::nodes::Number])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(number) @number
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn number(&self) -> Option<&super::nodes::Number<'tree>> {
        match self {
            Self::Number { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    ///Try to interpret this capture as a `constant.builtin` ([anon_unions::ConstantBuiltin])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (null)
  (true)
  (false)
] @constant.builtin*/
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constant_builtin(&self) -> Option<&anon_unions::ConstantBuiltin<'tree>> {
        match self {
            Self::ConstantBuiltin { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    ///Try to interpret this capture as a `escape` ([super::nodes::EscapeSequence])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn escape(&self) -> Option<&super::nodes::EscapeSequence<'tree>> {
        match self {
            Self::Escape { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    ///Try to interpret this capture as a `comment` ([super::nodes::Comment])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn comment(&self) -> Option<&super::nodes::Comment<'tree>> {
        match self {
            Self::Comment { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for HighlightsCapture<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringSpecialKey { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(StringSpecialKey)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::String { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(String)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Number { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Number)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::ConstantBuiltin { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(ConstantBuiltin)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Escape { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Escape)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Comment { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Comment)),
                    )
                    .field("node", node)
                    .finish()
            }
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
    fn query(&self) -> &'cursor Self::Query {
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
                    index: 0usize,
                    name: "string.special.key",
                }
            }
            Self::String { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 1usize,
                    name: "string",
                }
            }
            Self::Number { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 2usize,
                    name: "number",
                }
            }
            Self::ConstantBuiltin { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 3usize,
                    name: "constant.builtin",
                }
            }
            Self::Escape { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 4usize,
                    name: "escape",
                }
            }
            Self::Comment { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 5usize,
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
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::StringSpecialKey { .. } => "string.special.key",
            Self::String { .. } => "string",
            Self::Number { .. } => "number",
            Self::ConstantBuiltin { .. } => "constant.builtin",
            Self::Escape { .. } => "escape",
            Self::Comment { .. } => "comment",
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::StringSpecialKey { .. } => 0usize,
            Self::String { .. } => 1usize,
            Self::Number { .. } => 2usize,
            Self::ConstantBuiltin { .. } => 3usize,
            Self::Escape { .. } => 4usize,
            Self::Comment { .. } => 5usize,
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::super::nodes::*;
    /**one of `{false | null | true}`:
- [False]
- [Null]
- [True]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstantBuiltin<'tree> {
        False(False<'tree>),
        Null(Null<'tree>),
        True(True<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ConstantBuiltin<'tree> {
        ///Returns the node if it is of kind `false` ([False]), otherwise returns None
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#false(self) -> Option<False<'tree>> {
            match self {
                Self::False(x) => Some(x),
                _ => None,
            }
        }
        ///Returns the node if it is of kind `null` ([Null]), otherwise returns None
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn null(self) -> Option<Null<'tree>> {
            match self {
                Self::Null(x) => Some(x),
                _ => None,
            }
        }
        ///Returns the node if it is of kind `true` ([True]), otherwise returns None
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#true(self) -> Option<True<'tree>> {
            match self {
                Self::True(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for ConstantBuiltin<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "false" => {
                    Ok(unsafe {
                        Self::False(
                            <False<
                                'tree,
                            > as type_sitter_lib::TypedNode<
                                'tree,
                            >>::from_node_unchecked(node),
                        )
                    })
                }
                "null" => {
                    Ok(unsafe {
                        Self::Null(
                            <Null<
                                'tree,
                            > as type_sitter_lib::TypedNode<
                                'tree,
                            >>::from_node_unchecked(node),
                        )
                    })
                }
                "true" => {
                    Ok(unsafe {
                        Self::True(
                            <True<
                                'tree,
                            > as type_sitter_lib::TypedNode<
                                'tree,
                            >>::from_node_unchecked(node),
                        )
                    })
                }
                _ => {
                    Err(type_sitter_lib::IncorrectKind {
                        node,
                        kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstantBuiltin<'tree> {
        const KIND: &'static str = "{false | null | true}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::False(x) => x.node(),
                Self::Null(x) => x.node(),
                Self::True(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::False(x) => x.node_mut(),
                Self::Null(x) => x.node_mut(),
                Self::True(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::False(x) => x.into_node(),
                Self::Null(x) => x.into_node(),
                Self::True(x) => x.into_node(),
            }
        }
    }
}
