#[allow(non_upper_case_globals)]
static __Tags__: once_cell::race::OnceBox<tree_sitter::Query> = once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Tags() -> tree_sitter::Query {
    let mut query = tree_sitter::Query::new(
            tree_sitter_rust::language(),
            "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n",
        )
        .expect(
            "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?",
        );
    query
}
/**Typed version of the query:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
pub type TagsMatches<'cursor, 'tree> = type_sitter_lib::TypedQueryMatches<
    'cursor,
    'tree,
    TagsMatch<'cursor, 'tree>,
>;
pub type TagsCaptures<'cursor, 'tree> = type_sitter_lib::TypedQueryCaptures<
    'cursor,
    'tree,
    TagsMatch<'cursor, 'tree>,
>;
/**A match returned by the query [Tags]:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[derive(Debug)]
pub struct TagsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
/**A capture returned by the query [Tags]:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[derive(Debug)]
pub enum TagsCapture<'cursor, 'tree> {
    ///A `name`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///struct_item @name
    ///```
    Name {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `definition.class`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.class*/
    ///```
    DefinitionClass {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `definition.method`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.method*/
    ///```
    DefinitionMethod {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `definition.function`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.function*/
    ///```
    DefinitionFunction {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `definition.interface`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.interface*/
    ///```
    DefinitionInterface {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `definition.module`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.module*/
    ///```
    DefinitionModule {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `definition.macro`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.macro*/
    ///```
    DefinitionMacro {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `reference.call`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @reference.call*/
    ///```
    ReferenceCall {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    ///A `reference.implementation`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @reference.implementation*/
    ///```
    ReferenceImplementation {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Tags {
    type Match<'cursor, 'tree> = TagsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree> = TagsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n"
    }
    fn query(&self) -> &'static tree_sitter::Query {
        __Tags__.get_or_init(__Mk__Tags)
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
                Self::Capture::Name {
                    node: <super::nodes::StructItem<
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
                Self::Capture::DefinitionClass {
                    node: <super::nodes::StructItem<
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
                Self::Capture::DefinitionMethod {
                    node: <super::nodes::StructItem<
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
                Self::Capture::DefinitionFunction {
                    node: <super::nodes::StructItem<
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
                Self::Capture::DefinitionInterface {
                    node: <super::nodes::StructItem<
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
                Self::Capture::DefinitionModule {
                    node: <super::nodes::StructItem<
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
            6usize => {
                Self::Capture::DefinitionMacro {
                    node: <super::nodes::StructItem<
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
            7usize => {
                Self::Capture::ReferenceCall {
                    node: <super::nodes::StructItem<
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
            8usize => {
                Self::Capture::ReferenceImplementation {
                    node: <super::nodes::StructItem<
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
impl<'cursor, 'tree> TagsMatch<'cursor, 'tree> {
    ///Returns an iterator over the nodes captured by `name`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///struct_item @name
    ///```
    #[inline]
    pub fn name(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.class`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.class*/
    ///```
    #[inline]
    pub fn definition_class(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.method`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.method*/
    ///```
    #[inline]
    pub fn definition_method(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.function`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.function*/
    ///```
    #[inline]
    pub fn definition_function(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.interface`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.interface*/
    ///```
    #[inline]
    pub fn definition_interface(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.module`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.module*/
    ///```
    #[inline]
    pub fn definition_module(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.macro`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.macro*/
    ///```
    #[inline]
    pub fn definition_macro(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `reference.call`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @reference.call*/
    ///```
    #[inline]
    pub fn reference_call(&self) -> Option<super::nodes::StructItem<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `reference.implementation`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @reference.implementation*/
    ///```
    #[inline]
    pub fn reference_implementation(&self) -> Option<super::nodes::StructItem<'tree>> {
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
for TagsMatch<'cursor, 'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Tags
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
impl<'cursor, 'tree> TagsCapture<'cursor, 'tree> {
    ///Try to interpret this capture as a `name`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///struct_item @name
    ///```
    #[inline]
    pub fn name(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::Name { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `definition.class`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.class*/
    ///```
    #[inline]
    pub fn definition_class(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionClass { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `definition.method`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.method*/
    ///```
    #[inline]
    pub fn definition_method(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionMethod { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `definition.function`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.function*/
    ///```
    #[inline]
    pub fn definition_function(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionFunction { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `definition.interface`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.interface*/
    ///```
    #[inline]
    pub fn definition_interface(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionInterface { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `definition.module`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.module*/
    ///```
    #[inline]
    pub fn definition_module(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionModule { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `definition.macro`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.macro*/
    ///```
    #[inline]
    pub fn definition_macro(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionMacro { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `reference.call`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @reference.call*/
    ///```
    #[inline]
    pub fn reference_call(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::ReferenceCall { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `reference.implementation`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @reference.implementation*/
    ///```
    #[inline]
    pub fn reference_implementation(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::ReferenceImplementation { node, .. } => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for TagsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::Name { node, .. } => {
                Self::Name {
                    node: *node,
                    match_: None,
                }
            }
            Self::DefinitionClass { node, .. } => {
                Self::DefinitionClass {
                    node: *node,
                    match_: None,
                }
            }
            Self::DefinitionMethod { node, .. } => {
                Self::DefinitionMethod {
                    node: *node,
                    match_: None,
                }
            }
            Self::DefinitionFunction { node, .. } => {
                Self::DefinitionFunction {
                    node: *node,
                    match_: None,
                }
            }
            Self::DefinitionInterface { node, .. } => {
                Self::DefinitionInterface {
                    node: *node,
                    match_: None,
                }
            }
            Self::DefinitionModule { node, .. } => {
                Self::DefinitionModule {
                    node: *node,
                    match_: None,
                }
            }
            Self::DefinitionMacro { node, .. } => {
                Self::DefinitionMacro {
                    node: *node,
                    match_: None,
                }
            }
            Self::ReferenceCall { node, .. } => {
                Self::ReferenceCall {
                    node: *node,
                    match_: None,
                }
            }
            Self::ReferenceImplementation { node, .. } => {
                Self::ReferenceImplementation {
                    node: *node,
                    match_: None,
                }
            }
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
for TagsCapture<'cursor, 'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Tags
    }
    #[inline]
    fn match_(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Name { match_, .. } => match_.as_ref(),
            Self::DefinitionClass { match_, .. } => match_.as_ref(),
            Self::DefinitionMethod { match_, .. } => match_.as_ref(),
            Self::DefinitionFunction { match_, .. } => match_.as_ref(),
            Self::DefinitionInterface { match_, .. } => match_.as_ref(),
            Self::DefinitionModule { match_, .. } => match_.as_ref(),
            Self::DefinitionMacro { match_, .. } => match_.as_ref(),
            Self::ReferenceCall { match_, .. } => match_.as_ref(),
            Self::ReferenceImplementation { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Name { match_, .. } => match_,
            Self::DefinitionClass { match_, .. } => match_,
            Self::DefinitionMethod { match_, .. } => match_,
            Self::DefinitionFunction { match_, .. } => match_,
            Self::DefinitionInterface { match_, .. } => match_,
            Self::DefinitionModule { match_, .. } => match_,
            Self::DefinitionMacro { match_, .. } => match_,
            Self::ReferenceCall { match_, .. } => match_,
            Self::ReferenceImplementation { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(
        &self,
    ) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "name",
                }
            }
            Self::DefinitionClass { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "definition.class",
                }
            }
            Self::DefinitionMethod { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "definition.method",
                }
            }
            Self::DefinitionFunction { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "definition.function",
                }
            }
            Self::DefinitionInterface { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "definition.interface",
                }
            }
            Self::DefinitionModule { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "definition.module",
                }
            }
            Self::DefinitionMacro { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "definition.macro",
                }
            }
            Self::ReferenceCall { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "reference.call",
                }
            }
            Self::ReferenceImplementation { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "reference.implementation",
                }
            }
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => node.node(),
            Self::DefinitionClass { node, .. } => node.node(),
            Self::DefinitionMethod { node, .. } => node.node(),
            Self::DefinitionFunction { node, .. } => node.node(),
            Self::DefinitionInterface { node, .. } => node.node(),
            Self::DefinitionModule { node, .. } => node.node(),
            Self::DefinitionMacro { node, .. } => node.node(),
            Self::ReferenceCall { node, .. } => node.node(),
            Self::ReferenceImplementation { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => node.node_mut(),
            Self::DefinitionClass { node, .. } => node.node_mut(),
            Self::DefinitionMethod { node, .. } => node.node_mut(),
            Self::DefinitionFunction { node, .. } => node.node_mut(),
            Self::DefinitionInterface { node, .. } => node.node_mut(),
            Self::DefinitionModule { node, .. } => node.node_mut(),
            Self::DefinitionMacro { node, .. } => node.node_mut(),
            Self::ReferenceCall { node, .. } => node.node_mut(),
            Self::ReferenceImplementation { node, .. } => node.node_mut(),
        }
    }
}
#[allow(non_upper_case_globals)]
static __Highlights__: once_cell::race::OnceBox<tree_sitter::Query> = once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Highlights() -> tree_sitter::Query {
    let mut query = tree_sitter::Query::new(
            tree_sitter_rust::language(),
            "; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n",
        )
        .expect(
            "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?",
        );
    query
}
/**Typed version of the query:

```sexp
; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume other uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

; Other identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

(line_comment) @comment
(block_comment) @comment

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

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
; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume other uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

; Other identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

(line_comment) @comment
(block_comment) @comment

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
#[derive(Debug)]
pub struct HighlightsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
/**A capture returned by the query [Highlights]:

```sexp
; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume other uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

; Other identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

(line_comment) @comment
(block_comment) @comment

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
#[derive(Debug)]
pub enum HighlightsCapture<'cursor, 'tree> {
    ///A `constant`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///(identifier) @constant
    ///```
    Constant {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `type`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @type*/
    ///```
    Type {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `constructor`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///(identifier) @constructor
    ///```
    Constructor {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `function`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///call_expression @function
    ///```
    Function {
        node: super::nodes::CallExpression<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `function.method`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///field_expression @function.method
    ///```
    FunctionMethod {
        node: super::nodes::FieldExpression<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `function.macro`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///macro_invocation @function.macro
    ///```
    FunctionMacro {
        node: super::nodes::MacroInvocation<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `type.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @type.builtin*/
    ///```
    TypeBuiltin {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `property`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @property*/
    ///```
    Property {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `comment`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @comment*/
    ///```
    Comment {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `punctuation.bracket`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @punctuation.bracket*/
    ///```
    PunctuationBracket {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `punctuation.delimiter`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @punctuation.delimiter*/
    ///```
    PunctuationDelimiter {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `variable.parameter`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///parameter @variable.parameter
    ///```
    VariableParameter {
        node: super::nodes::Parameter<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `label`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///lifetime @label
    ///```
    Label {
        node: super::nodes::Lifetime<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `keyword`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @keyword*/
    ///```
    Keyword {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `variable.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @variable.builtin*/
    ///```
    VariableBuiltin {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `string`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @string*/
    ///```
    String {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `constant.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @constant.builtin*/
    ///```
    ConstantBuiltin {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `escape`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @escape*/
    ///```
    Escape {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `attribute`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @attribute*/
    ///```
    Attribute {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    ///A `operator`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @operator*/
    ///```
    Operator {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Highlights {
    type Match<'cursor, 'tree> = HighlightsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree> = HighlightsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n"
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
                Self::Capture::Constant {
                    node: <super::nodes::Identifier<
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
                Self::Capture::Type {
                    node: <super::nodes::Identifier<
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
                Self::Capture::Constructor {
                    node: <super::nodes::Identifier<
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
                Self::Capture::Function {
                    node: <super::nodes::CallExpression<
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
                Self::Capture::FunctionMethod {
                    node: <super::nodes::FieldExpression<
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
                Self::Capture::FunctionMacro {
                    node: <super::nodes::MacroInvocation<
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
            6usize => {
                Self::Capture::TypeBuiltin {
                    node: <super::nodes::Identifier<
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
            7usize => {
                Self::Capture::Property {
                    node: <super::nodes::Identifier<
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
            8usize => {
                Self::Capture::Comment {
                    node: <super::nodes::Identifier<
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
            9usize => {
                Self::Capture::PunctuationBracket {
                    node: <super::nodes::Identifier<
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
            10usize => {
                Self::Capture::PunctuationDelimiter {
                    node: <super::nodes::Identifier<
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
            11usize => {
                Self::Capture::VariableParameter {
                    node: <super::nodes::Parameter<
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
            12usize => {
                Self::Capture::Label {
                    node: <super::nodes::Lifetime<
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
            13usize => {
                Self::Capture::Keyword {
                    node: <super::nodes::Identifier<
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
            14usize => {
                Self::Capture::VariableBuiltin {
                    node: <super::nodes::Identifier<
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
            15usize => {
                Self::Capture::String {
                    node: <super::nodes::Identifier<
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
            16usize => {
                Self::Capture::ConstantBuiltin {
                    node: <super::nodes::Identifier<
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
            17usize => {
                Self::Capture::Escape {
                    node: <super::nodes::Identifier<
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
            18usize => {
                Self::Capture::Attribute {
                    node: <super::nodes::Identifier<
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
            19usize => {
                Self::Capture::Operator {
                    node: <super::nodes::Identifier<
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
    ///Returns an iterator over the nodes captured by `constant`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///(identifier) @constant
    ///```
    #[inline]
    pub fn constant(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `type`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @type*/
    ///```
    #[inline]
    pub fn r#type(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `constructor`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///(identifier) @constructor
    ///```
    #[inline]
    pub fn constructor(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///call_expression @function
    ///```
    #[inline]
    pub fn function(&self) -> Option<super::nodes::CallExpression<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function.method`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///field_expression @function.method
    ///```
    #[inline]
    pub fn function_method(&self) -> Option<super::nodes::FieldExpression<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function.macro`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///macro_invocation @function.macro
    ///```
    #[inline]
    pub fn function_macro(&self) -> Option<super::nodes::MacroInvocation<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `type.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @type.builtin*/
    ///```
    #[inline]
    pub fn type_builtin(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `property`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @property*/
    ///```
    #[inline]
    pub fn property(&self) -> Option<super::nodes::Identifier<'tree>> {
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
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @comment*/
    ///```
    #[inline]
    pub fn comment(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `punctuation.bracket`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @punctuation.bracket*/
    ///```
    #[inline]
    pub fn punctuation_bracket(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `punctuation.delimiter`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @punctuation.delimiter*/
    ///```
    #[inline]
    pub fn punctuation_delimiter(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `variable.parameter`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///parameter @variable.parameter
    ///```
    #[inline]
    pub fn variable_parameter(&self) -> Option<super::nodes::Parameter<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `label`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///lifetime @label
    ///```
    #[inline]
    pub fn label(&self) -> Option<super::nodes::Lifetime<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `keyword`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @keyword*/
    ///```
    #[inline]
    pub fn keyword(&self) -> impl Iterator<Item = super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
    }
    ///Returns an iterator over the nodes captured by `variable.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @variable.builtin*/
    ///```
    #[inline]
    pub fn variable_builtin(&self) -> Option<super::nodes::Identifier<'tree>> {
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
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @string*/
    ///```
    #[inline]
    pub fn string(&self) -> Option<super::nodes::Identifier<'tree>> {
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
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @constant.builtin*/
    ///```
    #[inline]
    pub fn constant_builtin(&self) -> Option<super::nodes::Identifier<'tree>> {
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
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @escape*/
    ///```
    #[inline]
    pub fn escape(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `attribute`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @attribute*/
    ///```
    #[inline]
    pub fn attribute(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `operator`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @operator*/
    ///```
    #[inline]
    pub fn operator(&self) -> Option<super::nodes::Identifier<'tree>> {
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
    ///Try to interpret this capture as a `constant`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///(identifier) @constant
    ///```
    #[inline]
    pub fn constant(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Constant { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `type`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @type*/
    ///```
    #[inline]
    pub fn r#type(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Type { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `constructor`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///(identifier) @constructor
    ///```
    #[inline]
    pub fn constructor(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Constructor { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `function`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///call_expression @function
    ///```
    #[inline]
    pub fn function(&self) -> Option<&super::nodes::CallExpression<'tree>> {
        match self {
            Self::Function { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `function.method`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///field_expression @function.method
    ///```
    #[inline]
    pub fn function_method(&self) -> Option<&super::nodes::FieldExpression<'tree>> {
        match self {
            Self::FunctionMethod { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `function.macro`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///macro_invocation @function.macro
    ///```
    #[inline]
    pub fn function_macro(&self) -> Option<&super::nodes::MacroInvocation<'tree>> {
        match self {
            Self::FunctionMacro { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `type.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @type.builtin*/
    ///```
    #[inline]
    pub fn type_builtin(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::TypeBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `property`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @property*/
    ///```
    #[inline]
    pub fn property(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Property { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `comment`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @comment*/
    ///```
    #[inline]
    pub fn comment(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Comment { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `punctuation.bracket`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @punctuation.bracket*/
    ///```
    #[inline]
    pub fn punctuation_bracket(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::PunctuationBracket { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `punctuation.delimiter`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @punctuation.delimiter*/
    ///```
    #[inline]
    pub fn punctuation_delimiter(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::PunctuationDelimiter { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `variable.parameter`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///parameter @variable.parameter
    ///```
    #[inline]
    pub fn variable_parameter(&self) -> Option<&super::nodes::Parameter<'tree>> {
        match self {
            Self::VariableParameter { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `label`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///lifetime @label
    ///```
    #[inline]
    pub fn label(&self) -> Option<&super::nodes::Lifetime<'tree>> {
        match self {
            Self::Label { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `keyword`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @keyword*/
    ///```
    #[inline]
    pub fn keyword(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Keyword { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `variable.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @variable.builtin*/
    ///```
    #[inline]
    pub fn variable_builtin(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::VariableBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `string`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @string*/
    ///```
    #[inline]
    pub fn string(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::String { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `constant.builtin`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @constant.builtin*/
    ///```
    #[inline]
    pub fn constant_builtin(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::ConstantBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `escape`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @escape*/
    ///```
    #[inline]
    pub fn escape(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Escape { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `attribute`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @attribute*/
    ///```
    #[inline]
    pub fn attribute(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Attribute { node, .. } => Some(node),
            _ => None,
        }
    }
    ///Try to interpret this capture as a `operator`
    ///
    ///The full capture including pattern is:
    ///```sexp
    /**((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'")) @operator*/
    ///```
    #[inline]
    pub fn operator(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Operator { node, .. } => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for HighlightsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::Constant { node, .. } => {
                Self::Constant {
                    node: *node,
                    match_: None,
                }
            }
            Self::Type { node, .. } => {
                Self::Type {
                    node: *node,
                    match_: None,
                }
            }
            Self::Constructor { node, .. } => {
                Self::Constructor {
                    node: *node,
                    match_: None,
                }
            }
            Self::Function { node, .. } => {
                Self::Function {
                    node: *node,
                    match_: None,
                }
            }
            Self::FunctionMethod { node, .. } => {
                Self::FunctionMethod {
                    node: *node,
                    match_: None,
                }
            }
            Self::FunctionMacro { node, .. } => {
                Self::FunctionMacro {
                    node: *node,
                    match_: None,
                }
            }
            Self::TypeBuiltin { node, .. } => {
                Self::TypeBuiltin {
                    node: *node,
                    match_: None,
                }
            }
            Self::Property { node, .. } => {
                Self::Property {
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
            Self::PunctuationBracket { node, .. } => {
                Self::PunctuationBracket {
                    node: *node,
                    match_: None,
                }
            }
            Self::PunctuationDelimiter { node, .. } => {
                Self::PunctuationDelimiter {
                    node: *node,
                    match_: None,
                }
            }
            Self::VariableParameter { node, .. } => {
                Self::VariableParameter {
                    node: *node,
                    match_: None,
                }
            }
            Self::Label { node, .. } => {
                Self::Label {
                    node: *node,
                    match_: None,
                }
            }
            Self::Keyword { node, .. } => {
                Self::Keyword {
                    node: *node,
                    match_: None,
                }
            }
            Self::VariableBuiltin { node, .. } => {
                Self::VariableBuiltin {
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
            Self::Attribute { node, .. } => {
                Self::Attribute {
                    node: *node,
                    match_: None,
                }
            }
            Self::Operator { node, .. } => {
                Self::Operator {
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
            Self::Constant { match_, .. } => match_.as_ref(),
            Self::Type { match_, .. } => match_.as_ref(),
            Self::Constructor { match_, .. } => match_.as_ref(),
            Self::Function { match_, .. } => match_.as_ref(),
            Self::FunctionMethod { match_, .. } => match_.as_ref(),
            Self::FunctionMacro { match_, .. } => match_.as_ref(),
            Self::TypeBuiltin { match_, .. } => match_.as_ref(),
            Self::Property { match_, .. } => match_.as_ref(),
            Self::Comment { match_, .. } => match_.as_ref(),
            Self::PunctuationBracket { match_, .. } => match_.as_ref(),
            Self::PunctuationDelimiter { match_, .. } => match_.as_ref(),
            Self::VariableParameter { match_, .. } => match_.as_ref(),
            Self::Label { match_, .. } => match_.as_ref(),
            Self::Keyword { match_, .. } => match_.as_ref(),
            Self::VariableBuiltin { match_, .. } => match_.as_ref(),
            Self::String { match_, .. } => match_.as_ref(),
            Self::ConstantBuiltin { match_, .. } => match_.as_ref(),
            Self::Escape { match_, .. } => match_.as_ref(),
            Self::Attribute { match_, .. } => match_.as_ref(),
            Self::Operator { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Constant { match_, .. } => match_,
            Self::Type { match_, .. } => match_,
            Self::Constructor { match_, .. } => match_,
            Self::Function { match_, .. } => match_,
            Self::FunctionMethod { match_, .. } => match_,
            Self::FunctionMacro { match_, .. } => match_,
            Self::TypeBuiltin { match_, .. } => match_,
            Self::Property { match_, .. } => match_,
            Self::Comment { match_, .. } => match_,
            Self::PunctuationBracket { match_, .. } => match_,
            Self::PunctuationDelimiter { match_, .. } => match_,
            Self::VariableParameter { match_, .. } => match_,
            Self::Label { match_, .. } => match_,
            Self::Keyword { match_, .. } => match_,
            Self::VariableBuiltin { match_, .. } => match_,
            Self::String { match_, .. } => match_,
            Self::ConstantBuiltin { match_, .. } => match_,
            Self::Escape { match_, .. } => match_,
            Self::Attribute { match_, .. } => match_,
            Self::Operator { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(
        &self,
    ) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "constant",
                }
            }
            Self::Type { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "type",
                }
            }
            Self::Constructor { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "constructor",
                }
            }
            Self::Function { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "function",
                }
            }
            Self::FunctionMethod { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "function.method",
                }
            }
            Self::FunctionMacro { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "function.macro",
                }
            }
            Self::TypeBuiltin { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "type.builtin",
                }
            }
            Self::Property { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "property",
                }
            }
            Self::Comment { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "comment",
                }
            }
            Self::PunctuationBracket { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "punctuation.bracket",
                }
            }
            Self::PunctuationDelimiter { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "punctuation.delimiter",
                }
            }
            Self::VariableParameter { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "variable.parameter",
                }
            }
            Self::Label { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "label",
                }
            }
            Self::Keyword { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "keyword",
                }
            }
            Self::VariableBuiltin { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "variable.builtin",
                }
            }
            Self::String { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "string",
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
            Self::Attribute { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "attribute",
                }
            }
            Self::Operator { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "operator",
                }
            }
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => node.node(),
            Self::Type { node, .. } => node.node(),
            Self::Constructor { node, .. } => node.node(),
            Self::Function { node, .. } => node.node(),
            Self::FunctionMethod { node, .. } => node.node(),
            Self::FunctionMacro { node, .. } => node.node(),
            Self::TypeBuiltin { node, .. } => node.node(),
            Self::Property { node, .. } => node.node(),
            Self::Comment { node, .. } => node.node(),
            Self::PunctuationBracket { node, .. } => node.node(),
            Self::PunctuationDelimiter { node, .. } => node.node(),
            Self::VariableParameter { node, .. } => node.node(),
            Self::Label { node, .. } => node.node(),
            Self::Keyword { node, .. } => node.node(),
            Self::VariableBuiltin { node, .. } => node.node(),
            Self::String { node, .. } => node.node(),
            Self::ConstantBuiltin { node, .. } => node.node(),
            Self::Escape { node, .. } => node.node(),
            Self::Attribute { node, .. } => node.node(),
            Self::Operator { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => node.node_mut(),
            Self::Type { node, .. } => node.node_mut(),
            Self::Constructor { node, .. } => node.node_mut(),
            Self::Function { node, .. } => node.node_mut(),
            Self::FunctionMethod { node, .. } => node.node_mut(),
            Self::FunctionMacro { node, .. } => node.node_mut(),
            Self::TypeBuiltin { node, .. } => node.node_mut(),
            Self::Property { node, .. } => node.node_mut(),
            Self::Comment { node, .. } => node.node_mut(),
            Self::PunctuationBracket { node, .. } => node.node_mut(),
            Self::PunctuationDelimiter { node, .. } => node.node_mut(),
            Self::VariableParameter { node, .. } => node.node_mut(),
            Self::Label { node, .. } => node.node_mut(),
            Self::Keyword { node, .. } => node.node_mut(),
            Self::VariableBuiltin { node, .. } => node.node_mut(),
            Self::String { node, .. } => node.node_mut(),
            Self::ConstantBuiltin { node, .. } => node.node_mut(),
            Self::Escape { node, .. } => node.node_mut(),
            Self::Attribute { node, .. } => node.node_mut(),
            Self::Operator { node, .. } => node.node_mut(),
        }
    }
}
#[allow(non_upper_case_globals)]
static __Injections__: once_cell::race::OnceBox<tree_sitter::Query> = once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Injections() -> tree_sitter::Query {
    let mut query = tree_sitter::Query::new(
            tree_sitter_rust::language(),
            "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n",
        )
        .expect(
            "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?",
        );
    query
}
/**Typed version of the query:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Injections;
pub type InjectionsMatches<'cursor, 'tree> = type_sitter_lib::TypedQueryMatches<
    'cursor,
    'tree,
    InjectionsMatch<'cursor, 'tree>,
>;
pub type InjectionsCaptures<'cursor, 'tree> = type_sitter_lib::TypedQueryCaptures<
    'cursor,
    'tree,
    InjectionsMatch<'cursor, 'tree>,
>;
/**A match returned by the query [Injections]:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[derive(Debug)]
pub struct InjectionsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
/**A capture returned by the query [Injections]:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[derive(Debug)]
pub enum InjectionsCapture<'cursor, 'tree> {
    ///A `injection.content`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///macro_invocation @injection.content
    ///```
    InjectionContent {
        node: super::nodes::MacroInvocation<'tree>,
        match_: Option<InjectionsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Injections {
    type Match<'cursor, 'tree> = InjectionsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree> = InjectionsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n"
    }
    fn query(&self) -> &'static tree_sitter::Query {
        __Injections__.get_or_init(__Mk__Injections)
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
                Self::Capture::InjectionContent {
                    node: <super::nodes::MacroInvocation<
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
impl<'cursor, 'tree> InjectionsMatch<'cursor, 'tree> {
    ///Returns an iterator over the nodes captured by `injection.content`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///macro_invocation @injection.content
    ///```
    #[inline]
    pub fn injection_content(&self) -> super::nodes::MacroInvocation<'tree> {
        let result = {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
            .next()
            .expect("one quantifier returned nothing");
        debug_assert!(
            { unsafe { self.nodes_for_capture_ix(capture_idx)
            .map(tree_sitter_lib::TypedNode:: < 'tree > ::from_unchecked) } } .next()
            .is_none(), "one quantifier returned more than one item"
        );
        result
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
for InjectionsMatch<'cursor, 'tree> {
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Injections
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
impl<'cursor, 'tree> InjectionsCapture<'cursor, 'tree> {
    ///Try to interpret this capture as a `injection.content`
    ///
    ///The full capture including pattern is:
    ///```sexp
    ///macro_invocation @injection.content
    ///```
    #[inline]
    pub fn injection_content(&self) -> Option<&super::nodes::MacroInvocation<'tree>> {
        match self {
            Self::InjectionContent { node, .. } => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for InjectionsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::InjectionContent { node, .. } => {
                Self::InjectionContent {
                    node: *node,
                    match_: None,
                }
            }
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
for InjectionsCapture<'cursor, 'tree> {
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Injections
    }
    #[inline]
    fn match_(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::InjectionContent { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::InjectionContent { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(
        &self,
    ) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    name: "injection.content",
                }
            }
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => node.node_mut(),
        }
    }
}
