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
pub type TagsMatches<'a, 'tree, T> = TypedQueryMatches<'a, 'tree, T, TagsMatch<'a, 'tree>>;
pub type TagsCaptures<'a, 'tree, T> = TypedQueryCaptures<'a, 'tree, T, TagsMatch<'a, 'tree>>;
#[doc = "A match returned by the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[derive(Debug)]
pub struct TagsMatch<'cursor, 'tree>(tree_sitter::QueryMatch<'cursor, 'tree>);
#[doc = "A capture returned by the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[derive(Debug, Clone, Copy)]
pub enum TagsCapture<'cursor, 'tree> {
    #[doc = "A `name`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "struct_item @name"]
    #[doc = "```"]
    Name(super::nodes::StructItem<'tree>),
    #[doc = "A `definition.class`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    DefinitionClass(super::nodes::StructItem<'tree>),
    #[doc = "A `definition.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.method"]
    #[doc = "```"]
    DefinitionMethod(super::nodes::StructItem<'tree>),
    #[doc = "A `definition.function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.function"]
    #[doc = "```"]
    DefinitionFunction(super::nodes::StructItem<'tree>),
    #[doc = "A `definition.interface`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    DefinitionInterface(super::nodes::StructItem<'tree>),
    #[doc = "A `definition.module`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.module"]
    #[doc = "```"]
    DefinitionModule(super::nodes::StructItem<'tree>),
    #[doc = "A `definition.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.macro"]
    #[doc = "```"]
    DefinitionMacro(super::nodes::StructItem<'tree>),
    #[doc = "A `reference.call`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.call"]
    #[doc = "```"]
    ReferenceCall(super::nodes::StructItem<'tree>),
    #[doc = "A `reference.implementation`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @reference.implementation"]
    #[doc = "```"]
    ReferenceImplementation(super::nodes::StructItem<'tree>),
}
#[automatically_derived]
impl TypedQuery for Tags {
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
        Self::Match(match_)
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'cursor, 'tree>,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index { 0usize => Name (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 1usize => DefinitionClass (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 2usize => DefinitionMethod (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 3usize => DefinitionFunction (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 4usize => DefinitionInterface (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 5usize => DefinitionModule (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 6usize => DefinitionMacro (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 7usize => ReferenceCall (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 8usize => ReferenceImplementation (< super :: nodes :: StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , }
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
            }
        }
        .next()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TypedQueryMatch<'cursor, 'tree> for TagsMatch<'cursor, 'tree> {
    type Query = Tags;
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.0
    }
    #[inline]
    fn query(&self) -> &Self::Query {
        Tags
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TagsMatch<'cursor, 'tree> {
    #[doc = "Try to interpret this capture as a `name`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "struct_item @name"]
    #[doc = "```"]
    #[inline]
    pub fn name(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::Name(node) => Some(node),
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
    pub fn definition_class(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionClass(node) => Some(node),
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
    pub fn definition_method(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionMethod(node) => Some(node),
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
    pub fn definition_function(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionFunction(node) => Some(node),
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
    pub fn definition_interface(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionInterface(node) => Some(node),
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
    pub fn definition_module(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionModule(node) => Some(node),
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
    pub fn definition_macro(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::DefinitionMacro(node) => Some(node),
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
    pub fn reference_call(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::ReferenceCall(node) => Some(node),
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
    pub fn reference_implementation(&self) -> Option<super::nodes::StructItem<'tree>> {
        match self {
            Self::ReferenceImplementation(node) => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TypedQueryCapture<'cursor, 'tree> for TagsCapture<'cursor, 'tree> {
    type Query = Tags;
    #[inline]
    fn to_raw(self) -> tree_sitter::QueryCapture<'cursor, 'tree> {
        match self {
            Name(node) => tree_sitter::QueryCapture {
                index: 0usize,
                node: *node.node(),
            },
            DefinitionClass(node) => tree_sitter::QueryCapture {
                index: 1usize,
                node: *node.node(),
            },
            DefinitionMethod(node) => tree_sitter::QueryCapture {
                index: 2usize,
                node: *node.node(),
            },
            DefinitionFunction(node) => tree_sitter::QueryCapture {
                index: 3usize,
                node: *node.node(),
            },
            DefinitionInterface(node) => tree_sitter::QueryCapture {
                index: 4usize,
                node: *node.node(),
            },
            DefinitionModule(node) => tree_sitter::QueryCapture {
                index: 5usize,
                node: *node.node(),
            },
            DefinitionMacro(node) => tree_sitter::QueryCapture {
                index: 6usize,
                node: *node.node(),
            },
            ReferenceCall(node) => tree_sitter::QueryCapture {
                index: 7usize,
                node: *node.node(),
            },
            ReferenceImplementation(node) => tree_sitter::QueryCapture {
                index: 8usize,
                node: *node.node(),
            },
        }
    }
    #[inline]
    fn query(&self) -> &Self::Query {
        Tags
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
pub type HighlightsMatches<'a, 'tree, T> =
    TypedQueryMatches<'a, 'tree, T, HighlightsMatch<'a, 'tree>>;
pub type HighlightsCaptures<'a, 'tree, T> =
    TypedQueryCaptures<'a, 'tree, T, HighlightsMatch<'a, 'tree>>;
#[doc = "A match returned by the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[derive(Debug)]
pub struct HighlightsMatch<'cursor, 'tree>(tree_sitter::QueryMatch<'cursor, 'tree>);
#[doc = "A capture returned by the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[derive(Debug, Clone, Copy)]
pub enum HighlightsCapture<'cursor, 'tree> {
    #[doc = "A `constant`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    Constant(super::nodes::Identifier<'tree>),
    #[doc = "A `type`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type"]
    #[doc = "```"]
    Type(super::nodes::Identifier<'tree>),
    #[doc = "A `constructor`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "```"]
    Constructor(super::nodes::Identifier<'tree>),
    #[doc = "A `function`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "call_expression @function"]
    #[doc = "```"]
    Function(super::nodes::CallExpression<'tree>),
    #[doc = "A `function.method`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "field_expression @function.method"]
    #[doc = "```"]
    FunctionMethod(super::nodes::FieldExpression<'tree>),
    #[doc = "A `function.macro`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @function.macro"]
    #[doc = "```"]
    FunctionMacro(super::nodes::MacroInvocation<'tree>),
    #[doc = "A `type.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @type.builtin"]
    #[doc = "```"]
    TypeBuiltin(super::nodes::Identifier<'tree>),
    #[doc = "A `property`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @property"]
    #[doc = "```"]
    Property(super::nodes::Identifier<'tree>),
    #[doc = "A `comment`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @comment"]
    #[doc = "```"]
    Comment(super::nodes::Identifier<'tree>),
    #[doc = "A `punctuation.bracket`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.bracket"]
    #[doc = "```"]
    PunctuationBracket(super::nodes::Identifier<'tree>),
    #[doc = "A `punctuation.delimiter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @punctuation.delimiter"]
    #[doc = "```"]
    PunctuationDelimiter(super::nodes::Identifier<'tree>),
    #[doc = "A `variable.parameter`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "parameter @variable.parameter"]
    #[doc = "```"]
    VariableParameter(super::nodes::Parameter<'tree>),
    #[doc = "A `label`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "lifetime @label"]
    #[doc = "```"]
    Label(super::nodes::Lifetime<'tree>),
    #[doc = "A `keyword`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @keyword"]
    #[doc = "```"]
    Keyword(super::nodes::Identifier<'tree>),
    #[doc = "A `variable.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @variable.builtin"]
    #[doc = "```"]
    VariableBuiltin(super::nodes::Identifier<'tree>),
    #[doc = "A `string`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @string"]
    #[doc = "```"]
    String(super::nodes::Identifier<'tree>),
    #[doc = "A `constant.builtin`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @constant.builtin"]
    #[doc = "```"]
    ConstantBuiltin(super::nodes::Identifier<'tree>),
    #[doc = "A `escape`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @escape"]
    #[doc = "```"]
    Escape(super::nodes::Identifier<'tree>),
    #[doc = "A `attribute`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @attribute"]
    #[doc = "```"]
    Attribute(super::nodes::Identifier<'tree>),
    #[doc = "A `operator`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\")) @operator"]
    #[doc = "```"]
    Operator(super::nodes::Identifier<'tree>),
}
#[automatically_derived]
impl TypedQuery for Highlights {
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
        Self::Match(match_)
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'cursor, 'tree>,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index { 0usize => Constant (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 1usize => Type (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 2usize => Constructor (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 3usize => Function (< super :: nodes :: CallExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 4usize => FunctionMethod (< super :: nodes :: FieldExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 5usize => FunctionMacro (< super :: nodes :: MacroInvocation < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 6usize => TypeBuiltin (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 7usize => Property (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 8usize => Comment (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 9usize => PunctuationBracket (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 10usize => PunctuationDelimiter (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 11usize => VariableParameter (< super :: nodes :: Parameter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 12usize => Label (< super :: nodes :: Lifetime < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 13usize => Keyword (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 14usize => VariableBuiltin (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 15usize => String (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 16usize => ConstantBuiltin (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 17usize => Escape (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 18usize => Attribute (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , 19usize => Operator (< super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , }
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
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
                    .map(TypedNode::from_unchecked)
            }
        }
        .next()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TypedQueryMatch<'cursor, 'tree> for HighlightsMatch<'cursor, 'tree> {
    type Query = Highlights;
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.0
    }
    #[inline]
    fn query(&self) -> &Self::Query {
        Highlights
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsMatch<'cursor, 'tree> {
    #[doc = "Try to interpret this capture as a `constant`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    #[inline]
    pub fn constant(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Constant(node) => Some(node),
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
    pub fn r#type(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Type(node) => Some(node),
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
    pub fn constructor(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Constructor(node) => Some(node),
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
    pub fn function(&self) -> Option<super::nodes::CallExpression<'tree>> {
        match self {
            Self::Function(node) => Some(node),
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
    pub fn function_method(&self) -> Option<super::nodes::FieldExpression<'tree>> {
        match self {
            Self::FunctionMethod(node) => Some(node),
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
    pub fn function_macro(&self) -> Option<super::nodes::MacroInvocation<'tree>> {
        match self {
            Self::FunctionMacro(node) => Some(node),
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
    pub fn type_builtin(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::TypeBuiltin(node) => Some(node),
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
    pub fn property(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Property(node) => Some(node),
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
    pub fn comment(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Comment(node) => Some(node),
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
    pub fn punctuation_bracket(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::PunctuationBracket(node) => Some(node),
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
    pub fn punctuation_delimiter(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::PunctuationDelimiter(node) => Some(node),
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
    pub fn variable_parameter(&self) -> Option<super::nodes::Parameter<'tree>> {
        match self {
            Self::VariableParameter(node) => Some(node),
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
    pub fn label(&self) -> Option<super::nodes::Lifetime<'tree>> {
        match self {
            Self::Label(node) => Some(node),
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
    pub fn keyword(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Keyword(node) => Some(node),
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
    pub fn variable_builtin(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::VariableBuiltin(node) => Some(node),
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
    pub fn string(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::String(node) => Some(node),
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
    pub fn constant_builtin(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::ConstantBuiltin(node) => Some(node),
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
    pub fn escape(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Escape(node) => Some(node),
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
    pub fn attribute(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Attribute(node) => Some(node),
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
    pub fn operator(&self) -> Option<super::nodes::Identifier<'tree>> {
        match self {
            Self::Operator(node) => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TypedQueryCapture<'cursor, 'tree> for HighlightsCapture<'cursor, 'tree> {
    type Query = Highlights;
    #[inline]
    fn to_raw(self) -> tree_sitter::QueryCapture<'cursor, 'tree> {
        match self {
            Constant(node) => tree_sitter::QueryCapture {
                index: 0usize,
                node: *node.node(),
            },
            Type(node) => tree_sitter::QueryCapture {
                index: 1usize,
                node: *node.node(),
            },
            Constructor(node) => tree_sitter::QueryCapture {
                index: 2usize,
                node: *node.node(),
            },
            Function(node) => tree_sitter::QueryCapture {
                index: 3usize,
                node: *node.node(),
            },
            FunctionMethod(node) => tree_sitter::QueryCapture {
                index: 4usize,
                node: *node.node(),
            },
            FunctionMacro(node) => tree_sitter::QueryCapture {
                index: 5usize,
                node: *node.node(),
            },
            TypeBuiltin(node) => tree_sitter::QueryCapture {
                index: 6usize,
                node: *node.node(),
            },
            Property(node) => tree_sitter::QueryCapture {
                index: 7usize,
                node: *node.node(),
            },
            Comment(node) => tree_sitter::QueryCapture {
                index: 8usize,
                node: *node.node(),
            },
            PunctuationBracket(node) => tree_sitter::QueryCapture {
                index: 9usize,
                node: *node.node(),
            },
            PunctuationDelimiter(node) => tree_sitter::QueryCapture {
                index: 10usize,
                node: *node.node(),
            },
            VariableParameter(node) => tree_sitter::QueryCapture {
                index: 11usize,
                node: *node.node(),
            },
            Label(node) => tree_sitter::QueryCapture {
                index: 12usize,
                node: *node.node(),
            },
            Keyword(node) => tree_sitter::QueryCapture {
                index: 13usize,
                node: *node.node(),
            },
            VariableBuiltin(node) => tree_sitter::QueryCapture {
                index: 14usize,
                node: *node.node(),
            },
            String(node) => tree_sitter::QueryCapture {
                index: 15usize,
                node: *node.node(),
            },
            ConstantBuiltin(node) => tree_sitter::QueryCapture {
                index: 16usize,
                node: *node.node(),
            },
            Escape(node) => tree_sitter::QueryCapture {
                index: 17usize,
                node: *node.node(),
            },
            Attribute(node) => tree_sitter::QueryCapture {
                index: 18usize,
                node: *node.node(),
            },
            Operator(node) => tree_sitter::QueryCapture {
                index: 19usize,
                node: *node.node(),
            },
        }
    }
    #[inline]
    fn query(&self) -> &Self::Query {
        Highlights
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
pub type InjectionsMatches<'a, 'tree, T> =
    TypedQueryMatches<'a, 'tree, T, InjectionsMatch<'a, 'tree>>;
pub type InjectionsCaptures<'a, 'tree, T> =
    TypedQueryCaptures<'a, 'tree, T, InjectionsMatch<'a, 'tree>>;
#[doc = "A match returned by the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[derive(Debug)]
pub struct InjectionsMatch<'cursor, 'tree>(tree_sitter::QueryMatch<'cursor, 'tree>);
#[doc = "A capture returned by the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[derive(Debug, Clone, Copy)]
pub enum InjectionsCapture<'cursor, 'tree> {
    #[doc = "A `injection.content`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @injection.content"]
    #[doc = "```"]
    InjectionContent(super::nodes::MacroInvocation<'tree>),
}
#[automatically_derived]
impl TypedQuery for Injections {
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
        Self::Match(match_)
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
    ) -> Self::Capture<'tree> {
        match capture . index { 0usize => InjectionContent (< super :: nodes :: MacroInvocation < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_unchecked (capture . node)) , }
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
                    .map(TypedNode::from_unchecked)
            }
        }
        .next()
        .expect("one quantifier returned nothing");
        debug_assert!(
            {
                unsafe {
                    self.nodes_for_capture_ix(capture_idx)
                        .map(TypedNode::from_unchecked)
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
impl<'cursor, 'tree> TypedQueryMatch<'cursor, 'tree> for InjectionsMatch<'cursor, 'tree> {
    type Query = Injections;
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.0
    }
    #[inline]
    fn query(&self) -> &Self::Query {
        Injections
    }
}
#[automatically_derived]
impl<'cursor, 'tree> InjectionsMatch<'cursor, 'tree> {
    #[doc = "Try to interpret this capture as a `injection.content`"]
    #[doc = ""]
    #[doc = "The full capture including pattern is:"]
    #[doc = "```sexp"]
    #[doc = "macro_invocation @injection.content"]
    #[doc = "```"]
    #[inline]
    pub fn injection_content(&self) -> Option<super::nodes::MacroInvocation<'tree>> {
        match self {
            Self::InjectionContent(node) => Some(node),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TypedQueryCapture<'cursor, 'tree> for InjectionsCapture<'cursor, 'tree> {
    type Query = Injections;
    #[inline]
    fn to_raw(self) -> tree_sitter::QueryCapture<'cursor, 'tree> {
        match self {
            InjectionContent(node) => tree_sitter::QueryCapture {
                index: 0usize,
                node: *node.node(),
            },
        }
    }
    #[inline]
    fn query(&self) -> &Self::Query {
        Injections
    }
}
