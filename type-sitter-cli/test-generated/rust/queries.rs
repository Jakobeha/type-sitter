#[allow(non_upper_case_globals)]
static __Tags__: once_cell::race::OnceBox<tree_sitter::Query> = once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Tags() -> tree_sitter::Query {
    let mut query = tree_sitter :: Query :: new (tree_sitter_rust :: language () , "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ;
    query
}
#[doc = "Typed version of the query:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
pub type TagsMatches<'cursor, 'tree, Text> =
    type_sitter_lib::TypedQueryMatches<'cursor, 'tree, TagsMatch<'cursor, 'tree>, Text>;
pub type TagsCaptures<'cursor, 'tree, Text> =
    type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, TagsMatch<'cursor, 'tree>, Text>;
#[doc = "A match returned by the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[derive(Debug)]
pub struct TagsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
}
#[doc = "A capture returned by the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[derive(Debug)]
pub enum TagsCapture<'cursor, 'tree> {
    #[doc = "A `name`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "struct_item @name"]
    #[doc = "```"]
    Name {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.class`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    DefinitionClass {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.method"]
    #[doc = "```"]
    DefinitionMethod {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.function"]
    #[doc = "```"]
    DefinitionFunction {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.interface`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    DefinitionInterface {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.module`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.module"]
    #[doc = "```"]
    DefinitionModule {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.macro"]
    #[doc = "```"]
    DefinitionMacro {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `reference.call`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.call"]
    #[doc = "```"]
    ReferenceCall {
        node: super::nodes::StructItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `reference.implementation`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.implementation"]
    #[doc = "```"]
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
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_ }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index as usize { 0usize => Self :: Capture :: Name { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 1usize => Self :: Capture :: DefinitionClass { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 2usize => Self :: Capture :: DefinitionMethod { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 3usize => Self :: Capture :: DefinitionFunction { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 4usize => Self :: Capture :: DefinitionInterface { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 5usize => Self :: Capture :: DefinitionModule { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 6usize => Self :: Capture :: DefinitionMacro { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 7usize => Self :: Capture :: ReferenceCall { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 8usize => Self :: Capture :: ReferenceImplementation { node : < super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TagsMatch<'cursor, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `name`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "struct_item @name"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `definition.class`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `definition.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.method"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `definition.function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.function"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `definition.interface`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `definition.module`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.module"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `definition.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.macro"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `reference.call`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.call"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `reference.implementation`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.implementation"]
    #[doc = "```"]
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
    for TagsMatch<'cursor, 'tree>
{
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Tags
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
    #[doc = "Try to interpret this capture as a `name`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "struct_item @name"]
    #[doc = "```"]
    #[inline]
    pub fn name(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::Name { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.class`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    #[inline]
    pub fn definition_class(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionClass { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.method"]
    #[doc = "```"]
    #[inline]
    pub fn definition_method(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionMethod { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.function"]
    #[doc = "```"]
    #[inline]
    pub fn definition_function(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionFunction { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.interface`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    #[inline]
    pub fn definition_interface(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionInterface { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.module`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.module"]
    #[doc = "```"]
    #[inline]
    pub fn definition_module(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionModule { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.macro"]
    #[doc = "```"]
    #[inline]
    pub fn definition_macro(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionMacro { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `reference.call`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.call"]
    #[doc = "```"]
    #[inline]
    pub fn reference_call(&self) -> Option<&super::nodes::StructItem<'tree>> {
        match self {
            Self::ReferenceCall { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `reference.implementation`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.implementation"]
    #[doc = "```"]
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
            Self::Name { node, .. } => Self::Name {
                node: *node,
                match_: None,
            },
            Self::DefinitionClass { node, .. } => Self::DefinitionClass {
                node: *node,
                match_: None,
            },
            Self::DefinitionMethod { node, .. } => Self::DefinitionMethod {
                node: *node,
                match_: None,
            },
            Self::DefinitionFunction { node, .. } => Self::DefinitionFunction {
                node: *node,
                match_: None,
            },
            Self::DefinitionInterface { node, .. } => Self::DefinitionInterface {
                node: *node,
                match_: None,
            },
            Self::DefinitionModule { node, .. } => Self::DefinitionModule {
                node: *node,
                match_: None,
            },
            Self::DefinitionMacro { node, .. } => Self::DefinitionMacro {
                node: *node,
                match_: None,
            },
            Self::ReferenceCall { node, .. } => Self::ReferenceCall {
                node: *node,
                match_: None,
            },
            Self::ReferenceImplementation { node, .. } => Self::ReferenceImplementation {
                node: *node,
                match_: None,
            },
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
    for TagsCapture<'cursor, 'tree>
{
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
    fn to_raw(&self) -> tree_sitter::QueryCapture<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => tree_sitter::QueryCapture {
                index: 0usize as u32,
                node: *node.node(),
            },
            Self::DefinitionClass { node, .. } => tree_sitter::QueryCapture {
                index: 1usize as u32,
                node: *node.node(),
            },
            Self::DefinitionMethod { node, .. } => tree_sitter::QueryCapture {
                index: 2usize as u32,
                node: *node.node(),
            },
            Self::DefinitionFunction { node, .. } => tree_sitter::QueryCapture {
                index: 3usize as u32,
                node: *node.node(),
            },
            Self::DefinitionInterface { node, .. } => tree_sitter::QueryCapture {
                index: 4usize as u32,
                node: *node.node(),
            },
            Self::DefinitionModule { node, .. } => tree_sitter::QueryCapture {
                index: 5usize as u32,
                node: *node.node(),
            },
            Self::DefinitionMacro { node, .. } => tree_sitter::QueryCapture {
                index: 6usize as u32,
                node: *node.node(),
            },
            Self::ReferenceCall { node, .. } => tree_sitter::QueryCapture {
                index: 7usize as u32,
                node: *node.node(),
            },
            Self::ReferenceImplementation { node, .. } => tree_sitter::QueryCapture {
                index: 8usize as u32,
                node: *node.node(),
            },
        }
    }
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
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
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
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
static __Highlights__: once_cell::race::OnceBox<tree_sitter::Query> =
    once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Highlights() -> tree_sitter::Query {
    let mut query = tree_sitter :: Query :: new (tree_sitter_rust :: language () , "; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ;
    query
}
#[doc = "Typed version of the query:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
pub type HighlightsMatches<'cursor, 'tree, Text> =
    type_sitter_lib::TypedQueryMatches<'cursor, 'tree, HighlightsMatch<'cursor, 'tree>, Text>;
pub type HighlightsCaptures<'cursor, 'tree, Text> =
    type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, HighlightsMatch<'cursor, 'tree>, Text>;
#[doc = "A match returned by the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[derive(Debug)]
pub struct HighlightsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
}
#[doc = "A capture returned by the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[derive(Debug)]
pub enum HighlightsCapture<'cursor, 'tree> {
    #[doc = "A `constant`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    Constant {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `type`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type"]
    #[doc = "```"]
    Type {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `constructor`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "```"]
    Constructor {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "call_expression @function"]
    #[doc = "```"]
    Function {
        node: super::nodes::CallExpression<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `function.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "field_expression @function.method"]
    #[doc = "```"]
    FunctionMethod {
        node: super::nodes::FieldExpression<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `function.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @function.macro"]
    #[doc = "```"]
    FunctionMacro {
        node: super::nodes::MacroInvocation<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `type.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type.builtin"]
    #[doc = "```"]
    TypeBuiltin {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `property`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @property"]
    #[doc = "```"]
    Property {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `comment`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @comment"]
    #[doc = "```"]
    Comment {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `punctuation.bracket`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.bracket"]
    #[doc = "```"]
    PunctuationBracket {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `punctuation.delimiter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.delimiter"]
    #[doc = "```"]
    PunctuationDelimiter {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `variable.parameter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "parameter @variable.parameter"]
    #[doc = "```"]
    VariableParameter {
        node: super::nodes::Parameter<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `label`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "lifetime @label"]
    #[doc = "```"]
    Label {
        node: super::nodes::Lifetime<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `keyword`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @keyword"]
    #[doc = "```"]
    Keyword {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `variable.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @variable.builtin"]
    #[doc = "```"]
    VariableBuiltin {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `string`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @string"]
    #[doc = "```"]
    String {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `constant.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @constant.builtin"]
    #[doc = "```"]
    ConstantBuiltin {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `escape`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @escape"]
    #[doc = "```"]
    Escape {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `attribute`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @attribute"]
    #[doc = "```"]
    Attribute {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `operator`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @operator"]
    #[doc = "```"]
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
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_ }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index as usize { 0usize => Self :: Capture :: Constant { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 1usize => Self :: Capture :: Type { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 2usize => Self :: Capture :: Constructor { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 3usize => Self :: Capture :: Function { node : < super :: nodes :: CallExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 4usize => Self :: Capture :: FunctionMethod { node : < super :: nodes :: FieldExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 5usize => Self :: Capture :: FunctionMacro { node : < super :: nodes :: MacroInvocation < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 6usize => Self :: Capture :: TypeBuiltin { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 7usize => Self :: Capture :: Property { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 8usize => Self :: Capture :: Comment { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 9usize => Self :: Capture :: PunctuationBracket { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 10usize => Self :: Capture :: PunctuationDelimiter { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 11usize => Self :: Capture :: VariableParameter { node : < super :: nodes :: Parameter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 12usize => Self :: Capture :: Label { node : < super :: nodes :: Lifetime < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 13usize => Self :: Capture :: Keyword { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 14usize => Self :: Capture :: VariableBuiltin { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 15usize => Self :: Capture :: String { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 16usize => Self :: Capture :: ConstantBuiltin { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 17usize => Self :: Capture :: Escape { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 18usize => Self :: Capture :: Attribute { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , 19usize => Self :: Capture :: Operator { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node) , match_ } , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsMatch<'cursor, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `constant`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `type`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `constructor`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "call_expression @function"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `function.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "field_expression @function.method"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `function.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @function.macro"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `type.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type.builtin"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `property`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @property"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `comment`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @comment"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `punctuation.bracket`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.bracket"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `punctuation.delimiter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.delimiter"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `variable.parameter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "parameter @variable.parameter"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `label`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "lifetime @label"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `keyword`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @keyword"]
    #[doc = "```"]
    #[inline]
    pub fn keyword(&self) -> impl Iterator<Item = super::nodes::Identifier<'tree>> {
        {
            unsafe {
                self.nodes_for_capture_ix(capture_idx)
                    .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
            }
        }
    }
    #[doc = "Returns an iterator over the nodes captured by `variable.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @variable.builtin"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `string`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @string"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `constant.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @constant.builtin"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `escape`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @escape"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `attribute`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @attribute"]
    #[doc = "```"]
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
    #[doc = "Returns an iterator over the nodes captured by `operator`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @operator"]
    #[doc = "```"]
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
    for HighlightsMatch<'cursor, 'tree>
{
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Highlights
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
    #[doc = "Try to interpret this capture as a `constant`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    #[inline]
    pub fn constant(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Constant { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `type`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type"]
    #[doc = "```"]
    #[inline]
    pub fn r#type(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Type { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `constructor`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "```"]
    #[inline]
    pub fn constructor(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Constructor { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "call_expression @function"]
    #[doc = "```"]
    #[inline]
    pub fn function(&self) -> Option<&super::nodes::CallExpression<'tree>> {
        match self {
            Self::Function { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `function.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "field_expression @function.method"]
    #[doc = "```"]
    #[inline]
    pub fn function_method(&self) -> Option<&super::nodes::FieldExpression<'tree>> {
        match self {
            Self::FunctionMethod { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `function.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @function.macro"]
    #[doc = "```"]
    #[inline]
    pub fn function_macro(&self) -> Option<&super::nodes::MacroInvocation<'tree>> {
        match self {
            Self::FunctionMacro { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `type.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn type_builtin(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::TypeBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `property`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @property"]
    #[doc = "```"]
    #[inline]
    pub fn property(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Property { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `comment`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @comment"]
    #[doc = "```"]
    #[inline]
    pub fn comment(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Comment { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `punctuation.bracket`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.bracket"]
    #[doc = "```"]
    #[inline]
    pub fn punctuation_bracket(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::PunctuationBracket { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `punctuation.delimiter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.delimiter"]
    #[doc = "```"]
    #[inline]
    pub fn punctuation_delimiter(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::PunctuationDelimiter { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `variable.parameter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "parameter @variable.parameter"]
    #[doc = "```"]
    #[inline]
    pub fn variable_parameter(&self) -> Option<&super::nodes::Parameter<'tree>> {
        match self {
            Self::VariableParameter { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `label`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "lifetime @label"]
    #[doc = "```"]
    #[inline]
    pub fn label(&self) -> Option<&super::nodes::Lifetime<'tree>> {
        match self {
            Self::Label { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `keyword`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @keyword"]
    #[doc = "```"]
    #[inline]
    pub fn keyword(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Keyword { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `variable.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @variable.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn variable_builtin(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::VariableBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `string`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @string"]
    #[doc = "```"]
    #[inline]
    pub fn string(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::String { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `constant.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @constant.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn constant_builtin(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::ConstantBuiltin { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `escape`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @escape"]
    #[doc = "```"]
    #[inline]
    pub fn escape(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Escape { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `attribute`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @attribute"]
    #[doc = "```"]
    #[inline]
    pub fn attribute(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Attribute { node, .. } => Some(node),
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `operator`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @operator"]
    #[doc = "```"]
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
            Self::Constant { node, .. } => Self::Constant {
                node: *node,
                match_: None,
            },
            Self::Type { node, .. } => Self::Type {
                node: *node,
                match_: None,
            },
            Self::Constructor { node, .. } => Self::Constructor {
                node: *node,
                match_: None,
            },
            Self::Function { node, .. } => Self::Function {
                node: *node,
                match_: None,
            },
            Self::FunctionMethod { node, .. } => Self::FunctionMethod {
                node: *node,
                match_: None,
            },
            Self::FunctionMacro { node, .. } => Self::FunctionMacro {
                node: *node,
                match_: None,
            },
            Self::TypeBuiltin { node, .. } => Self::TypeBuiltin {
                node: *node,
                match_: None,
            },
            Self::Property { node, .. } => Self::Property {
                node: *node,
                match_: None,
            },
            Self::Comment { node, .. } => Self::Comment {
                node: *node,
                match_: None,
            },
            Self::PunctuationBracket { node, .. } => Self::PunctuationBracket {
                node: *node,
                match_: None,
            },
            Self::PunctuationDelimiter { node, .. } => Self::PunctuationDelimiter {
                node: *node,
                match_: None,
            },
            Self::VariableParameter { node, .. } => Self::VariableParameter {
                node: *node,
                match_: None,
            },
            Self::Label { node, .. } => Self::Label {
                node: *node,
                match_: None,
            },
            Self::Keyword { node, .. } => Self::Keyword {
                node: *node,
                match_: None,
            },
            Self::VariableBuiltin { node, .. } => Self::VariableBuiltin {
                node: *node,
                match_: None,
            },
            Self::String { node, .. } => Self::String {
                node: *node,
                match_: None,
            },
            Self::ConstantBuiltin { node, .. } => Self::ConstantBuiltin {
                node: *node,
                match_: None,
            },
            Self::Escape { node, .. } => Self::Escape {
                node: *node,
                match_: None,
            },
            Self::Attribute { node, .. } => Self::Attribute {
                node: *node,
                match_: None,
            },
            Self::Operator { node, .. } => Self::Operator {
                node: *node,
                match_: None,
            },
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
    for HighlightsCapture<'cursor, 'tree>
{
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
    fn to_raw(&self) -> tree_sitter::QueryCapture<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => tree_sitter::QueryCapture {
                index: 0usize as u32,
                node: *node.node(),
            },
            Self::Type { node, .. } => tree_sitter::QueryCapture {
                index: 1usize as u32,
                node: *node.node(),
            },
            Self::Constructor { node, .. } => tree_sitter::QueryCapture {
                index: 2usize as u32,
                node: *node.node(),
            },
            Self::Function { node, .. } => tree_sitter::QueryCapture {
                index: 3usize as u32,
                node: *node.node(),
            },
            Self::FunctionMethod { node, .. } => tree_sitter::QueryCapture {
                index: 4usize as u32,
                node: *node.node(),
            },
            Self::FunctionMacro { node, .. } => tree_sitter::QueryCapture {
                index: 5usize as u32,
                node: *node.node(),
            },
            Self::TypeBuiltin { node, .. } => tree_sitter::QueryCapture {
                index: 6usize as u32,
                node: *node.node(),
            },
            Self::Property { node, .. } => tree_sitter::QueryCapture {
                index: 7usize as u32,
                node: *node.node(),
            },
            Self::Comment { node, .. } => tree_sitter::QueryCapture {
                index: 8usize as u32,
                node: *node.node(),
            },
            Self::PunctuationBracket { node, .. } => tree_sitter::QueryCapture {
                index: 9usize as u32,
                node: *node.node(),
            },
            Self::PunctuationDelimiter { node, .. } => tree_sitter::QueryCapture {
                index: 10usize as u32,
                node: *node.node(),
            },
            Self::VariableParameter { node, .. } => tree_sitter::QueryCapture {
                index: 11usize as u32,
                node: *node.node(),
            },
            Self::Label { node, .. } => tree_sitter::QueryCapture {
                index: 12usize as u32,
                node: *node.node(),
            },
            Self::Keyword { node, .. } => tree_sitter::QueryCapture {
                index: 13usize as u32,
                node: *node.node(),
            },
            Self::VariableBuiltin { node, .. } => tree_sitter::QueryCapture {
                index: 14usize as u32,
                node: *node.node(),
            },
            Self::String { node, .. } => tree_sitter::QueryCapture {
                index: 15usize as u32,
                node: *node.node(),
            },
            Self::ConstantBuiltin { node, .. } => tree_sitter::QueryCapture {
                index: 16usize as u32,
                node: *node.node(),
            },
            Self::Escape { node, .. } => tree_sitter::QueryCapture {
                index: 17usize as u32,
                node: *node.node(),
            },
            Self::Attribute { node, .. } => tree_sitter::QueryCapture {
                index: 18usize as u32,
                node: *node.node(),
            },
            Self::Operator { node, .. } => tree_sitter::QueryCapture {
                index: 19usize as u32,
                node: *node.node(),
            },
        }
    }
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
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
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
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
static __Injections__: once_cell::race::OnceBox<tree_sitter::Query> =
    once_cell::race::OnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Injections() -> tree_sitter::Query {
    let mut query = tree_sitter :: Query :: new (tree_sitter_rust :: language () , "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ;
    query
}
#[doc = "Typed version of the query:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Injections;
pub type InjectionsMatches<'cursor, 'tree, Text> =
    type_sitter_lib::TypedQueryMatches<'cursor, 'tree, InjectionsMatch<'cursor, 'tree>, Text>;
pub type InjectionsCaptures<'cursor, 'tree, Text> =
    type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, InjectionsMatch<'cursor, 'tree>, Text>;
#[doc = "A match returned by the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[derive(Debug)]
pub struct InjectionsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
}
#[doc = "A capture returned by the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[derive(Debug)]
pub enum InjectionsCapture<'cursor, 'tree> {
    #[doc = "A `injection.content`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @injection.content"]
    #[doc = "```"]
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
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_ }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture.index as usize {
            0usize => {
                Self::Capture::InjectionContent {
                    node: <super::nodes::MacroInvocation<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_unchecked(capture.node),
                    match_,
                }
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> InjectionsMatch<'cursor, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `injection.content`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @injection.content"]
    #[doc = "```"]
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
            {
                unsafe {
                    self.nodes_for_capture_ix(capture_idx)
                        .map(tree_sitter_lib::TypedNode::<'tree>::from_unchecked)
                }
            }
            .next()
            .is_none(),
            "one quantifier returned more than one item"
        );
        result
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
    for InjectionsMatch<'cursor, 'tree>
{
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'static Self::Query {
        &Injections
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
    #[doc = "Try to interpret this capture as a `injection.content`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @injection.content"]
    #[doc = "```"]
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
            Self::InjectionContent { node, .. } => Self::InjectionContent {
                node: *node,
                match_: None,
            },
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
    for InjectionsCapture<'cursor, 'tree>
{
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
    fn to_raw(&self) -> tree_sitter::QueryCapture<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => tree_sitter::QueryCapture {
                index: 0usize as u32,
                node: *node.node(),
            },
        }
    }
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => node.node_mut(),
        }
    }
}
