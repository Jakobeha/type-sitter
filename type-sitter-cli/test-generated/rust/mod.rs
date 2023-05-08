# [doc = concat ! ("Typed node `" , "_declaration_statement" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum DeclarationStatement<'tree> {
    AssociatedType(AssociatedType<'tree>),
    AttributeItem(AttributeItem<'tree>),
    ConstItem(ConstItem<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    EnumItem(EnumItem<'tree>),
    ExternCrateDeclaration(ExternCrateDeclaration<'tree>),
    ForeignModItem(ForeignModItem<'tree>),
    FunctionItem(FunctionItem<'tree>),
    FunctionSignatureItem(FunctionSignatureItem<'tree>),
    ImplItem(ImplItem<'tree>),
    InnerAttributeItem(InnerAttributeItem<'tree>),
    LetDeclaration(LetDeclaration<'tree>),
    MacroDefinition(MacroDefinition<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    ModItem(ModItem<'tree>),
    StaticItem(StaticItem<'tree>),
    StructItem(StructItem<'tree>),
    TraitItem(TraitItem<'tree>),
    TypeItem(TypeItem<'tree>),
    UnionItem(UnionItem<'tree>),
    UseDeclaration(UseDeclaration<'tree>),
}
#[automatically_derived]
impl<'tree> DeclarationStatement<'tree> {
    # [doc = concat ! ("Returns the node if it is of kind `" , "associated_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn associated_type(self) -> Option<AssociatedType<'tree>> {
        match self {
            Self::AssociatedType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
        match self {
            Self::AttributeItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "const_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_item(self) -> Option<ConstItem<'tree>> {
        match self {
            Self::ConstItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "empty_statement" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn empty_statement(self) -> Option<EmptyStatement<'tree>> {
        match self {
            Self::EmptyStatement(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "enum_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn enum_item(self) -> Option<EnumItem<'tree>> {
        match self {
            Self::EnumItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "extern_crate_declaration" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn extern_crate_declaration(self) -> Option<ExternCrateDeclaration<'tree>> {
        match self {
            Self::ExternCrateDeclaration(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "foreign_mod_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn foreign_mod_item(self) -> Option<ForeignModItem<'tree>> {
        match self {
            Self::ForeignModItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "function_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_item(self) -> Option<FunctionItem<'tree>> {
        match self {
            Self::FunctionItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "function_signature_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_signature_item(self) -> Option<FunctionSignatureItem<'tree>> {
        match self {
            Self::FunctionSignatureItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "impl_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn impl_item(self) -> Option<ImplItem<'tree>> {
        match self {
            Self::ImplItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "inner_attribute_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn inner_attribute_item(self) -> Option<InnerAttributeItem<'tree>> {
        match self {
            Self::InnerAttributeItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "let_declaration" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn let_declaration(self) -> Option<LetDeclaration<'tree>> {
        match self {
            Self::LetDeclaration(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "macro_definition" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_definition(self) -> Option<MacroDefinition<'tree>> {
        match self {
            Self::MacroDefinition(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "macro_invocation" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "mod_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn mod_item(self) -> Option<ModItem<'tree>> {
        match self {
            Self::ModItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "static_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn static_item(self) -> Option<StaticItem<'tree>> {
        match self {
            Self::StaticItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "struct_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_item(self) -> Option<StructItem<'tree>> {
        match self {
            Self::StructItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "trait_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn trait_item(self) -> Option<TraitItem<'tree>> {
        match self {
            Self::TraitItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "type_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_item(self) -> Option<TypeItem<'tree>> {
        match self {
            Self::TypeItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "union_item" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn union_item(self) -> Option<UnionItem<'tree>> {
        match self {
            Self::UnionItem(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "use_declaration" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn use_declaration(self) -> Option<UseDeclaration<'tree>> {
        match self {
            Self::UseDeclaration(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DeclarationStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "associated_type" => Ok(unsafe {
                Self::AssociatedType(<AssociatedType<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "attribute_item" => Ok(unsafe {
                Self::AttributeItem(<AttributeItem<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "const_item" => {
                Ok(unsafe {
                    Self :: ConstItem (< ConstItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "empty_statement" => Ok(unsafe {
                Self::EmptyStatement(<EmptyStatement<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "enum_item" => Ok(unsafe {
                Self::EnumItem(
                    <EnumItem<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                        node,
                    ),
                )
            }),
            "extern_crate_declaration" => Ok(unsafe {
                Self :: ExternCrateDeclaration (< ExternCrateDeclaration < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "foreign_mod_item" => Ok(unsafe {
                Self::ForeignModItem(<ForeignModItem<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "function_item" => {
                Ok(unsafe {
                    Self :: FunctionItem (< FunctionItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "function_signature_item" => Ok(unsafe {
                Self :: FunctionSignatureItem (< FunctionSignatureItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "impl_item" => Ok(unsafe {
                Self::ImplItem(
                    <ImplItem<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                        node,
                    ),
                )
            }),
            "inner_attribute_item" => Ok(unsafe {
                Self :: InnerAttributeItem (< InnerAttributeItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "let_declaration" => Ok(unsafe {
                Self::LetDeclaration(<LetDeclaration<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "macro_definition" => Ok(unsafe {
                Self::MacroDefinition(<MacroDefinition<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "macro_invocation" => Ok(unsafe {
                Self::MacroInvocation(<MacroInvocation<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "mod_item" => Ok(unsafe {
                Self::ModItem(
                    <ModItem<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                        node,
                    ),
                )
            }),
            "static_item" => {
                Ok(unsafe {
                    Self :: StaticItem (< StaticItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "struct_item" => {
                Ok(unsafe {
                    Self :: StructItem (< StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "trait_item" => {
                Ok(unsafe {
                    Self :: TraitItem (< TraitItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "type_item" => Ok(unsafe {
                Self::TypeItem(
                    <TypeItem<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                        node,
                    ),
                )
            }),
            "union_item" => {
                Ok(unsafe {
                    Self :: UnionItem (< UnionItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "use_declaration" => Ok(unsafe {
                Self::UseDeclaration(<UseDeclaration<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DeclarationStatement<'tree> {
    const KIND: &'static str = "_declaration_statement";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::AssociatedType(x) => x.node(),
            Self::AttributeItem(x) => x.node(),
            Self::ConstItem(x) => x.node(),
            Self::EmptyStatement(x) => x.node(),
            Self::EnumItem(x) => x.node(),
            Self::ExternCrateDeclaration(x) => x.node(),
            Self::ForeignModItem(x) => x.node(),
            Self::FunctionItem(x) => x.node(),
            Self::FunctionSignatureItem(x) => x.node(),
            Self::ImplItem(x) => x.node(),
            Self::InnerAttributeItem(x) => x.node(),
            Self::LetDeclaration(x) => x.node(),
            Self::MacroDefinition(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::ModItem(x) => x.node(),
            Self::StaticItem(x) => x.node(),
            Self::StructItem(x) => x.node(),
            Self::TraitItem(x) => x.node(),
            Self::TypeItem(x) => x.node(),
            Self::UnionItem(x) => x.node(),
            Self::UseDeclaration(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        match self {
            Self::AssociatedType(x) => x.node_mut(),
            Self::AttributeItem(x) => x.node_mut(),
            Self::ConstItem(x) => x.node_mut(),
            Self::EmptyStatement(x) => x.node_mut(),
            Self::EnumItem(x) => x.node_mut(),
            Self::ExternCrateDeclaration(x) => x.node_mut(),
            Self::ForeignModItem(x) => x.node_mut(),
            Self::FunctionItem(x) => x.node_mut(),
            Self::FunctionSignatureItem(x) => x.node_mut(),
            Self::ImplItem(x) => x.node_mut(),
            Self::InnerAttributeItem(x) => x.node_mut(),
            Self::LetDeclaration(x) => x.node_mut(),
            Self::MacroDefinition(x) => x.node_mut(),
            Self::MacroInvocation(x) => x.node_mut(),
            Self::ModItem(x) => x.node_mut(),
            Self::StaticItem(x) => x.node_mut(),
            Self::StructItem(x) => x.node_mut(),
            Self::TraitItem(x) => x.node_mut(),
            Self::TypeItem(x) => x.node_mut(),
            Self::UnionItem(x) => x.node_mut(),
            Self::UseDeclaration(x) => x.node_mut(),
        }
    }
}
# [doc = concat ! ("Typed node `" , "_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Expression<'tree> {
    Literal(Literal<'tree>),
    ArrayExpression(ArrayExpression<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    AsyncBlock(AsyncBlock<'tree>),
    AwaitExpression(AwaitExpression<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BreakExpression(BreakExpression<'tree>),
    CallExpression(CallExpression<'tree>),
    ClosureExpression(ClosureExpression<'tree>),
    CompoundAssignmentExpr(CompoundAssignmentExpr<'tree>),
    ConstBlock(ConstBlock<'tree>),
    ContinueExpression(ContinueExpression<'tree>),
    FieldExpression(FieldExpression<'tree>),
    ForExpression(ForExpression<'tree>),
    GenericFunction(GenericFunction<'tree>),
    Identifier(Identifier<'tree>),
    IfExpression(IfExpression<'tree>),
    IndexExpression(IndexExpression<'tree>),
    LoopExpression(LoopExpression<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    MatchExpression(MatchExpression<'tree>),
    Metavariable(Metavariable<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    RangeExpression(RangeExpression<'tree>),
    ReferenceExpression(ReferenceExpression<'tree>),
    ReturnExpression(ReturnExpression<'tree>),
    ScopedIdentifier(ScopedIdentifier<'tree>),
    _Self(_Self<'tree>),
    StructExpression(StructExpression<'tree>),
    TryExpression(TryExpression<'tree>),
    TupleExpression(TupleExpression<'tree>),
    TypeCastExpression(TypeCastExpression<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnitExpression(UnitExpression<'tree>),
    UnsafeBlock(UnsafeBlock<'tree>),
    WhileExpression(WhileExpression<'tree>),
    YieldExpression(YieldExpression<'tree>),
}
#[automatically_derived]
impl<'tree> Expression<'tree> {
    # [doc = concat ! ("Returns the node if it is of kind `" , "_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn literal(self) -> Option<Literal<'tree>> {
        match self {
            Self::Literal(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "array_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array_expression(self) -> Option<ArrayExpression<'tree>> {
        match self {
            Self::ArrayExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "assignment_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn assignment_expression(self) -> Option<AssignmentExpression<'tree>> {
        match self {
            Self::AssignmentExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "async_block" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn async_block(self) -> Option<AsyncBlock<'tree>> {
        match self {
            Self::AsyncBlock(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "await_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn await_expression(self) -> Option<AwaitExpression<'tree>> {
        match self {
            Self::AwaitExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "binary_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn binary_expression(self) -> Option<BinaryExpression<'tree>> {
        match self {
            Self::BinaryExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn block(self) -> Option<Block<'tree>> {
        match self {
            Self::Block(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "break_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn break_expression(self) -> Option<BreakExpression<'tree>> {
        match self {
            Self::BreakExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "call_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn call_expression(self) -> Option<CallExpression<'tree>> {
        match self {
            Self::CallExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "closure_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn closure_expression(self) -> Option<ClosureExpression<'tree>> {
        match self {
            Self::ClosureExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "compound_assignment_expr" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn compound_assignment_expr(self) -> Option<CompoundAssignmentExpr<'tree>> {
        match self {
            Self::CompoundAssignmentExpr(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "const_block" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_block(self) -> Option<ConstBlock<'tree>> {
        match self {
            Self::ConstBlock(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "continue_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn continue_expression(self) -> Option<ContinueExpression<'tree>> {
        match self {
            Self::ContinueExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "field_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn field_expression(self) -> Option<FieldExpression<'tree>> {
        match self {
            Self::FieldExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "for_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn for_expression(self) -> Option<ForExpression<'tree>> {
        match self {
            Self::ForExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "generic_function" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn generic_function(self) -> Option<GenericFunction<'tree>> {
        match self {
            Self::GenericFunction(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        match self {
            Self::Identifier(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "if_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn if_expression(self) -> Option<IfExpression<'tree>> {
        match self {
            Self::IfExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "index_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn index_expression(self) -> Option<IndexExpression<'tree>> {
        match self {
            Self::IndexExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "loop_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn loop_expression(self) -> Option<LoopExpression<'tree>> {
        match self {
            Self::LoopExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "macro_invocation" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "match_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn match_expression(self) -> Option<MatchExpression<'tree>> {
        match self {
            Self::MatchExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn metavariable(self) -> Option<Metavariable<'tree>> {
        match self {
            Self::Metavariable(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "parenthesized_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn parenthesized_expression(self) -> Option<ParenthesizedExpression<'tree>> {
        match self {
            Self::ParenthesizedExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "range_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn range_expression(self) -> Option<RangeExpression<'tree>> {
        match self {
            Self::RangeExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "reference_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_expression(self) -> Option<ReferenceExpression<'tree>> {
        match self {
            Self::ReferenceExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "return_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn return_expression(self) -> Option<ReturnExpression<'tree>> {
        match self {
            Self::ReturnExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
        match self {
            Self::ScopedIdentifier(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn _self(self) -> Option<_Self<'tree>> {
        match self {
            Self::_Self(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "struct_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_expression(self) -> Option<StructExpression<'tree>> {
        match self {
            Self::StructExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "try_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn try_expression(self) -> Option<TryExpression<'tree>> {
        match self {
            Self::TryExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "tuple_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_expression(self) -> Option<TupleExpression<'tree>> {
        match self {
            Self::TupleExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "type_cast_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_cast_expression(self) -> Option<TypeCastExpression<'tree>> {
        match self {
            Self::TypeCastExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "unary_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unary_expression(self) -> Option<UnaryExpression<'tree>> {
        match self {
            Self::UnaryExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "unit_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unit_expression(self) -> Option<UnitExpression<'tree>> {
        match self {
            Self::UnitExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "unsafe_block" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unsafe_block(self) -> Option<UnsafeBlock<'tree>> {
        match self {
            Self::UnsafeBlock(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "while_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn while_expression(self) -> Option<WhileExpression<'tree>> {
        match self {
            Self::WhileExpression(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "yield_expression" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn yield_expression(self) -> Option<YieldExpression<'tree>> {
        match self {
            Self::YieldExpression(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if let Ok(this) = <Literal<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::Literal(this));
        }
        if let Ok(this) = <ArrayExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ArrayExpression(this));
        }
        if let Ok(this) = <AssignmentExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::AssignmentExpression(this));
        }
        if let Ok(this) = <AsyncBlock<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::AsyncBlock(this));
        }
        if let Ok(this) = <AwaitExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::AwaitExpression(this));
        }
        if let Ok(this) = <BinaryExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::BinaryExpression(this));
        }
        if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::Block(this));
        }
        if let Ok(this) = <BreakExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::BreakExpression(this));
        }
        if let Ok(this) = <CallExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::CallExpression(this));
        }
        if let Ok(this) = <ClosureExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ClosureExpression(this));
        }
        if let Ok(this) = <CompoundAssignmentExpr<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::CompoundAssignmentExpr(this));
        }
        if let Ok(this) = <ConstBlock<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ConstBlock(this));
        }
        if let Ok(this) = <ContinueExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ContinueExpression(this));
        }
        if let Ok(this) = <FieldExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::FieldExpression(this));
        }
        if let Ok(this) = <ForExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ForExpression(this));
        }
        if let Ok(this) = <GenericFunction<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::GenericFunction(this));
        }
        if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::Identifier(this));
        }
        if let Ok(this) = <IfExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::IfExpression(this));
        }
        if let Ok(this) = <IndexExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::IndexExpression(this));
        }
        if let Ok(this) = <LoopExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::LoopExpression(this));
        }
        if let Ok(this) = <MacroInvocation<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::MacroInvocation(this));
        }
        if let Ok(this) = <MatchExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::MatchExpression(this));
        }
        if let Ok(this) = <Metavariable<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::Metavariable(this));
        }
        if let Ok(this) = <ParenthesizedExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ParenthesizedExpression(this));
        }
        if let Ok(this) = <RangeExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::RangeExpression(this));
        }
        if let Ok(this) = <ReferenceExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ReferenceExpression(this));
        }
        if let Ok(this) = <ReturnExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ReturnExpression(this));
        }
        if let Ok(this) = <ScopedIdentifier<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ScopedIdentifier(this));
        }
        if let Ok(this) = <_Self<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::_Self(this));
        }
        if let Ok(this) = <StructExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::StructExpression(this));
        }
        if let Ok(this) = <TryExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TryExpression(this));
        }
        if let Ok(this) = <TupleExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TupleExpression(this));
        }
        if let Ok(this) = <TypeCastExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TypeCastExpression(this));
        }
        if let Ok(this) = <UnaryExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::UnaryExpression(this));
        }
        if let Ok(this) = <UnitExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::UnitExpression(this));
        }
        if let Ok(this) = <UnsafeBlock<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::UnsafeBlock(this));
        }
        if let Ok(this) = <WhileExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::WhileExpression(this));
        }
        if let Ok(this) = <YieldExpression<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::YieldExpression(this));
        }
        Err(type_sitter_lib::IncorrectKind {
            node,
            kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
        })
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression<'tree> {
    const KIND: &'static str = "_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::Literal(x) => x.node(),
            Self::ArrayExpression(x) => x.node(),
            Self::AssignmentExpression(x) => x.node(),
            Self::AsyncBlock(x) => x.node(),
            Self::AwaitExpression(x) => x.node(),
            Self::BinaryExpression(x) => x.node(),
            Self::Block(x) => x.node(),
            Self::BreakExpression(x) => x.node(),
            Self::CallExpression(x) => x.node(),
            Self::ClosureExpression(x) => x.node(),
            Self::CompoundAssignmentExpr(x) => x.node(),
            Self::ConstBlock(x) => x.node(),
            Self::ContinueExpression(x) => x.node(),
            Self::FieldExpression(x) => x.node(),
            Self::ForExpression(x) => x.node(),
            Self::GenericFunction(x) => x.node(),
            Self::Identifier(x) => x.node(),
            Self::IfExpression(x) => x.node(),
            Self::IndexExpression(x) => x.node(),
            Self::LoopExpression(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::MatchExpression(x) => x.node(),
            Self::Metavariable(x) => x.node(),
            Self::ParenthesizedExpression(x) => x.node(),
            Self::RangeExpression(x) => x.node(),
            Self::ReferenceExpression(x) => x.node(),
            Self::ReturnExpression(x) => x.node(),
            Self::ScopedIdentifier(x) => x.node(),
            Self::_Self(x) => x.node(),
            Self::StructExpression(x) => x.node(),
            Self::TryExpression(x) => x.node(),
            Self::TupleExpression(x) => x.node(),
            Self::TypeCastExpression(x) => x.node(),
            Self::UnaryExpression(x) => x.node(),
            Self::UnitExpression(x) => x.node(),
            Self::UnsafeBlock(x) => x.node(),
            Self::WhileExpression(x) => x.node(),
            Self::YieldExpression(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        match self {
            Self::Literal(x) => x.node_mut(),
            Self::ArrayExpression(x) => x.node_mut(),
            Self::AssignmentExpression(x) => x.node_mut(),
            Self::AsyncBlock(x) => x.node_mut(),
            Self::AwaitExpression(x) => x.node_mut(),
            Self::BinaryExpression(x) => x.node_mut(),
            Self::Block(x) => x.node_mut(),
            Self::BreakExpression(x) => x.node_mut(),
            Self::CallExpression(x) => x.node_mut(),
            Self::ClosureExpression(x) => x.node_mut(),
            Self::CompoundAssignmentExpr(x) => x.node_mut(),
            Self::ConstBlock(x) => x.node_mut(),
            Self::ContinueExpression(x) => x.node_mut(),
            Self::FieldExpression(x) => x.node_mut(),
            Self::ForExpression(x) => x.node_mut(),
            Self::GenericFunction(x) => x.node_mut(),
            Self::Identifier(x) => x.node_mut(),
            Self::IfExpression(x) => x.node_mut(),
            Self::IndexExpression(x) => x.node_mut(),
            Self::LoopExpression(x) => x.node_mut(),
            Self::MacroInvocation(x) => x.node_mut(),
            Self::MatchExpression(x) => x.node_mut(),
            Self::Metavariable(x) => x.node_mut(),
            Self::ParenthesizedExpression(x) => x.node_mut(),
            Self::RangeExpression(x) => x.node_mut(),
            Self::ReferenceExpression(x) => x.node_mut(),
            Self::ReturnExpression(x) => x.node_mut(),
            Self::ScopedIdentifier(x) => x.node_mut(),
            Self::_Self(x) => x.node_mut(),
            Self::StructExpression(x) => x.node_mut(),
            Self::TryExpression(x) => x.node_mut(),
            Self::TupleExpression(x) => x.node_mut(),
            Self::TypeCastExpression(x) => x.node_mut(),
            Self::UnaryExpression(x) => x.node_mut(),
            Self::UnitExpression(x) => x.node_mut(),
            Self::UnsafeBlock(x) => x.node_mut(),
            Self::WhileExpression(x) => x.node_mut(),
            Self::YieldExpression(x) => x.node_mut(),
        }
    }
}
# [doc = concat ! ("Typed node `" , "_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Literal<'tree> {
    BooleanLiteral(BooleanLiteral<'tree>),
    CharLiteral(CharLiteral<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    StringLiteral(StringLiteral<'tree>),
}
#[automatically_derived]
impl<'tree> Literal<'tree> {
    # [doc = concat ! ("Returns the node if it is of kind `" , "boolean_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn boolean_literal(self) -> Option<BooleanLiteral<'tree>> {
        match self {
            Self::BooleanLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "char_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn char_literal(self) -> Option<CharLiteral<'tree>> {
        match self {
            Self::CharLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "float_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
        match self {
            Self::FloatLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "integer_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
        match self {
            Self::IntegerLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "raw_string_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
        match self {
            Self::RawStringLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "string_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string_literal(self) -> Option<StringLiteral<'tree>> {
        match self {
            Self::StringLiteral(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Literal<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "boolean_literal" => Ok(unsafe {
                Self::BooleanLiteral(<BooleanLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "char_literal" => {
                Ok(unsafe {
                    Self :: CharLiteral (< CharLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "float_literal" => {
                Ok(unsafe {
                    Self :: FloatLiteral (< FloatLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "integer_literal" => Ok(unsafe {
                Self::IntegerLiteral(<IntegerLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "raw_string_literal" => {
                Ok(unsafe {
                    Self :: RawStringLiteral (< RawStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "string_literal" => Ok(unsafe {
                Self::StringLiteral(<StringLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Literal<'tree> {
    const KIND: &'static str = "_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::BooleanLiteral(x) => x.node(),
            Self::CharLiteral(x) => x.node(),
            Self::FloatLiteral(x) => x.node(),
            Self::IntegerLiteral(x) => x.node(),
            Self::RawStringLiteral(x) => x.node(),
            Self::StringLiteral(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        match self {
            Self::BooleanLiteral(x) => x.node_mut(),
            Self::CharLiteral(x) => x.node_mut(),
            Self::FloatLiteral(x) => x.node_mut(),
            Self::IntegerLiteral(x) => x.node_mut(),
            Self::RawStringLiteral(x) => x.node_mut(),
            Self::StringLiteral(x) => x.node_mut(),
        }
    }
}
# [doc = concat ! ("Typed node `" , "_literal_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum LiteralPattern<'tree> {
    BooleanLiteral(BooleanLiteral<'tree>),
    CharLiteral(CharLiteral<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    NegativeLiteral(NegativeLiteral<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    StringLiteral(StringLiteral<'tree>),
}
#[automatically_derived]
impl<'tree> LiteralPattern<'tree> {
    # [doc = concat ! ("Returns the node if it is of kind `" , "boolean_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn boolean_literal(self) -> Option<BooleanLiteral<'tree>> {
        match self {
            Self::BooleanLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "char_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn char_literal(self) -> Option<CharLiteral<'tree>> {
        match self {
            Self::CharLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "float_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
        match self {
            Self::FloatLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "integer_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
        match self {
            Self::IntegerLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "negative_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn negative_literal(self) -> Option<NegativeLiteral<'tree>> {
        match self {
            Self::NegativeLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "raw_string_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
        match self {
            Self::RawStringLiteral(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "string_literal" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string_literal(self) -> Option<StringLiteral<'tree>> {
        match self {
            Self::StringLiteral(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LiteralPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "boolean_literal" => Ok(unsafe {
                Self::BooleanLiteral(<BooleanLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "char_literal" => {
                Ok(unsafe {
                    Self :: CharLiteral (< CharLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "float_literal" => {
                Ok(unsafe {
                    Self :: FloatLiteral (< FloatLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "integer_literal" => Ok(unsafe {
                Self::IntegerLiteral(<IntegerLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "negative_literal" => Ok(unsafe {
                Self::NegativeLiteral(<NegativeLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "raw_string_literal" => {
                Ok(unsafe {
                    Self :: RawStringLiteral (< RawStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "string_literal" => Ok(unsafe {
                Self::StringLiteral(<StringLiteral<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LiteralPattern<'tree> {
    const KIND: &'static str = "_literal_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::BooleanLiteral(x) => x.node(),
            Self::CharLiteral(x) => x.node(),
            Self::FloatLiteral(x) => x.node(),
            Self::IntegerLiteral(x) => x.node(),
            Self::NegativeLiteral(x) => x.node(),
            Self::RawStringLiteral(x) => x.node(),
            Self::StringLiteral(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        match self {
            Self::BooleanLiteral(x) => x.node_mut(),
            Self::CharLiteral(x) => x.node_mut(),
            Self::FloatLiteral(x) => x.node_mut(),
            Self::IntegerLiteral(x) => x.node_mut(),
            Self::NegativeLiteral(x) => x.node_mut(),
            Self::RawStringLiteral(x) => x.node_mut(),
            Self::StringLiteral(x) => x.node_mut(),
        }
    }
}
# [doc = concat ! ("Typed node `" , "_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Pattern<'tree> {
    __(symbols::__<'tree>),
    LiteralPattern(LiteralPattern<'tree>),
    CapturedPattern(CapturedPattern<'tree>),
    ConstBlock(ConstBlock<'tree>),
    Identifier(Identifier<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    MutPattern(MutPattern<'tree>),
    OrPattern(OrPattern<'tree>),
    RangePattern(RangePattern<'tree>),
    RefPattern(RefPattern<'tree>),
    ReferencePattern(ReferencePattern<'tree>),
    RemainingFieldPattern(RemainingFieldPattern<'tree>),
    ScopedIdentifier(ScopedIdentifier<'tree>),
    SlicePattern(SlicePattern<'tree>),
    StructPattern(StructPattern<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TupleStructPattern(TupleStructPattern<'tree>),
}
#[automatically_derived]
impl<'tree> Pattern<'tree> {
    # [doc = concat ! ("Returns the node if it is of kind `" , "_" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn __(self) -> Option<symbols::__<'tree>> {
        match self {
            Self::__(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "_literal_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn literal_pattern(self) -> Option<LiteralPattern<'tree>> {
        match self {
            Self::LiteralPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "captured_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn captured_pattern(self) -> Option<CapturedPattern<'tree>> {
        match self {
            Self::CapturedPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "const_block" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_block(self) -> Option<ConstBlock<'tree>> {
        match self {
            Self::ConstBlock(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        match self {
            Self::Identifier(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "macro_invocation" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "mut_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn mut_pattern(self) -> Option<MutPattern<'tree>> {
        match self {
            Self::MutPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "or_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn or_pattern(self) -> Option<OrPattern<'tree>> {
        match self {
            Self::OrPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "range_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn range_pattern(self) -> Option<RangePattern<'tree>> {
        match self {
            Self::RangePattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "ref_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn ref_pattern(self) -> Option<RefPattern<'tree>> {
        match self {
            Self::RefPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "reference_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_pattern(self) -> Option<ReferencePattern<'tree>> {
        match self {
            Self::ReferencePattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "remaining_field_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn remaining_field_pattern(self) -> Option<RemainingFieldPattern<'tree>> {
        match self {
            Self::RemainingFieldPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
        match self {
            Self::ScopedIdentifier(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "slice_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn slice_pattern(self) -> Option<SlicePattern<'tree>> {
        match self {
            Self::SlicePattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "struct_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_pattern(self) -> Option<StructPattern<'tree>> {
        match self {
            Self::StructPattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "tuple_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_pattern(self) -> Option<TuplePattern<'tree>> {
        match self {
            Self::TuplePattern(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "tuple_struct_pattern" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_struct_pattern(self) -> Option<TupleStructPattern<'tree>> {
        match self {
            Self::TupleStructPattern(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if let Ok(this) = <symbols::__<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::__(this));
        }
        if let Ok(this) = <LiteralPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::LiteralPattern(this));
        }
        if let Ok(this) = <CapturedPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::CapturedPattern(this));
        }
        if let Ok(this) = <ConstBlock<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ConstBlock(this));
        }
        if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::Identifier(this));
        }
        if let Ok(this) = <MacroInvocation<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::MacroInvocation(this));
        }
        if let Ok(this) = <MutPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::MutPattern(this));
        }
        if let Ok(this) = <OrPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::OrPattern(this));
        }
        if let Ok(this) = <RangePattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::RangePattern(this));
        }
        if let Ok(this) = <RefPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::RefPattern(this));
        }
        if let Ok(this) = <ReferencePattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ReferencePattern(this));
        }
        if let Ok(this) = <RemainingFieldPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::RemainingFieldPattern(this));
        }
        if let Ok(this) = <ScopedIdentifier<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ScopedIdentifier(this));
        }
        if let Ok(this) = <SlicePattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::SlicePattern(this));
        }
        if let Ok(this) = <StructPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::StructPattern(this));
        }
        if let Ok(this) = <TuplePattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TuplePattern(this));
        }
        if let Ok(this) = <TupleStructPattern<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TupleStructPattern(this));
        }
        Err(type_sitter_lib::IncorrectKind {
            node,
            kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
        })
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern<'tree> {
    const KIND: &'static str = "_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::__(x) => x.node(),
            Self::LiteralPattern(x) => x.node(),
            Self::CapturedPattern(x) => x.node(),
            Self::ConstBlock(x) => x.node(),
            Self::Identifier(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::MutPattern(x) => x.node(),
            Self::OrPattern(x) => x.node(),
            Self::RangePattern(x) => x.node(),
            Self::RefPattern(x) => x.node(),
            Self::ReferencePattern(x) => x.node(),
            Self::RemainingFieldPattern(x) => x.node(),
            Self::ScopedIdentifier(x) => x.node(),
            Self::SlicePattern(x) => x.node(),
            Self::StructPattern(x) => x.node(),
            Self::TuplePattern(x) => x.node(),
            Self::TupleStructPattern(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        match self {
            Self::__(x) => x.node_mut(),
            Self::LiteralPattern(x) => x.node_mut(),
            Self::CapturedPattern(x) => x.node_mut(),
            Self::ConstBlock(x) => x.node_mut(),
            Self::Identifier(x) => x.node_mut(),
            Self::MacroInvocation(x) => x.node_mut(),
            Self::MutPattern(x) => x.node_mut(),
            Self::OrPattern(x) => x.node_mut(),
            Self::RangePattern(x) => x.node_mut(),
            Self::RefPattern(x) => x.node_mut(),
            Self::ReferencePattern(x) => x.node_mut(),
            Self::RemainingFieldPattern(x) => x.node_mut(),
            Self::ScopedIdentifier(x) => x.node_mut(),
            Self::SlicePattern(x) => x.node_mut(),
            Self::StructPattern(x) => x.node_mut(),
            Self::TuplePattern(x) => x.node_mut(),
            Self::TupleStructPattern(x) => x.node_mut(),
        }
    }
}
# [doc = concat ! ("Typed node `" , "_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Type<'tree> {
    AbstractType(AbstractType<'tree>),
    ArrayType(ArrayType<'tree>),
    BoundedType(BoundedType<'tree>),
    DynamicType(DynamicType<'tree>),
    EmptyType(EmptyType<'tree>),
    FunctionType(FunctionType<'tree>),
    GenericType(GenericType<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    Metavariable(Metavariable<'tree>),
    PointerType(PointerType<'tree>),
    PrimitiveType(PrimitiveType<'tree>),
    ReferenceType(ReferenceType<'tree>),
    ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
    TupleType(TupleType<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    UnitType(UnitType<'tree>),
}
#[automatically_derived]
impl<'tree> Type<'tree> {
    # [doc = concat ! ("Returns the node if it is of kind `" , "abstract_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn abstract_type(self) -> Option<AbstractType<'tree>> {
        match self {
            Self::AbstractType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "array_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array_type(self) -> Option<ArrayType<'tree>> {
        match self {
            Self::ArrayType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "bounded_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn bounded_type(self) -> Option<BoundedType<'tree>> {
        match self {
            Self::BoundedType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "dynamic_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn dynamic_type(self) -> Option<DynamicType<'tree>> {
        match self {
            Self::DynamicType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "empty_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn empty_type(self) -> Option<EmptyType<'tree>> {
        match self {
            Self::EmptyType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "function_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_type(self) -> Option<FunctionType<'tree>> {
        match self {
            Self::FunctionType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn generic_type(self) -> Option<GenericType<'tree>> {
        match self {
            Self::GenericType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "macro_invocation" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn metavariable(self) -> Option<Metavariable<'tree>> {
        match self {
            Self::Metavariable(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "pointer_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn pointer_type(self) -> Option<PointerType<'tree>> {
        match self {
            Self::PointerType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "primitive_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn primitive_type(self) -> Option<PrimitiveType<'tree>> {
        match self {
            Self::PrimitiveType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "reference_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_type(self) -> Option<ReferenceType<'tree>> {
        match self {
            Self::ReferenceType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
        match self {
            Self::ScopedTypeIdentifier(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "tuple_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_type(self) -> Option<TupleType<'tree>> {
        match self {
            Self::TupleType(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
        match self {
            Self::TypeIdentifier(x) => Some(x),
            _ => None,
        }
    }
    # [doc = concat ! ("Returns the node if it is of kind `" , "unit_type" , "`, otherwise returns None")]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unit_type(self) -> Option<UnitType<'tree>> {
        match self {
            Self::UnitType(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Type<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "abstract_type" => {
                Ok(unsafe {
                    Self :: AbstractType (< AbstractType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "array_type" => {
                Ok(unsafe {
                    Self :: ArrayType (< ArrayType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "bounded_type" => {
                Ok(unsafe {
                    Self :: BoundedType (< BoundedType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "dynamic_type" => {
                Ok(unsafe {
                    Self :: DynamicType (< DynamicType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "empty_type" => {
                Ok(unsafe {
                    Self :: EmptyType (< EmptyType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "function_type" => {
                Ok(unsafe {
                    Self :: FunctionType (< FunctionType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "generic_type" => {
                Ok(unsafe {
                    Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "macro_invocation" => Ok(unsafe {
                Self::MacroInvocation(<MacroInvocation<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "metavariable" => {
                Ok(unsafe {
                    Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "pointer_type" => {
                Ok(unsafe {
                    Self :: PointerType (< PointerType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "primitive_type" => Ok(unsafe {
                Self::PrimitiveType(<PrimitiveType<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "reference_type" => Ok(unsafe {
                Self::ReferenceType(<ReferenceType<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "scoped_type_identifier" => Ok(unsafe {
                Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "tuple_type" => {
                Ok(unsafe {
                    Self :: TupleType (< TupleType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "type_identifier" => Ok(unsafe {
                Self::TypeIdentifier(<TypeIdentifier<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "unit_type" => Ok(unsafe {
                Self::UnitType(
                    <UnitType<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                        node,
                    ),
                )
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Type<'tree> {
    const KIND: &'static str = "_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::AbstractType(x) => x.node(),
            Self::ArrayType(x) => x.node(),
            Self::BoundedType(x) => x.node(),
            Self::DynamicType(x) => x.node(),
            Self::EmptyType(x) => x.node(),
            Self::FunctionType(x) => x.node(),
            Self::GenericType(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::Metavariable(x) => x.node(),
            Self::PointerType(x) => x.node(),
            Self::PrimitiveType(x) => x.node(),
            Self::ReferenceType(x) => x.node(),
            Self::ScopedTypeIdentifier(x) => x.node(),
            Self::TupleType(x) => x.node(),
            Self::TypeIdentifier(x) => x.node(),
            Self::UnitType(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        match self {
            Self::AbstractType(x) => x.node_mut(),
            Self::ArrayType(x) => x.node_mut(),
            Self::BoundedType(x) => x.node_mut(),
            Self::DynamicType(x) => x.node_mut(),
            Self::EmptyType(x) => x.node_mut(),
            Self::FunctionType(x) => x.node_mut(),
            Self::GenericType(x) => x.node_mut(),
            Self::MacroInvocation(x) => x.node_mut(),
            Self::Metavariable(x) => x.node_mut(),
            Self::PointerType(x) => x.node_mut(),
            Self::PrimitiveType(x) => x.node_mut(),
            Self::ReferenceType(x) => x.node_mut(),
            Self::ScopedTypeIdentifier(x) => x.node_mut(),
            Self::TupleType(x) => x.node_mut(),
            Self::TypeIdentifier(x) => x.node_mut(),
            Self::UnitType(x) => x.node_mut(),
        }
    }
}
# [doc = concat ! ("Typed node `" , "abstract_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AbstractType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AbstractType<'tree> {
    # [doc = concat ! ("Get the field `" , "trait" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier,
    > {
        self . 0 . child_by_field_name ("trait") . map (< anon_unions :: FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AbstractType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "abstract_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AbstractType<'tree> {
    const KIND: &'static str = "abstract_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "arguments" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Arguments<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Arguments<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_AttributeItem > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Arguments<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "arguments" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Arguments<'tree> {
    const KIND: &'static str = "arguments";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "array_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ArrayExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ArrayExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "length" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn length(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("length")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_AttributeItem > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ArrayExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ArrayExpression<'tree> {
    const KIND: &'static str = "array_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "array_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ArrayType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ArrayType<'tree> {
    # [doc = concat ! ("Get the field `" , "element" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn element(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("element") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "length" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn length(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("length")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ArrayType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ArrayType<'tree> {
    const KIND: &'static str = "array_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "assignment_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AssignmentExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AssignmentExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "left" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "right" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AssignmentExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "assignment_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AssignmentExpression<'tree> {
    const KIND: &'static str = "assignment_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "associated_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AssociatedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AssociatedType<'tree> {
    # [doc = concat ! ("Get the field `" , "bounds" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(&self) -> Option<type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>>> {
        self.0
            .child_by_field_name("bounds")
            .map(<TraitBounds<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AssociatedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "associated_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AssociatedType<'tree> {
    const KIND: &'static str = "associated_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "async_block" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AsyncBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AsyncBlock<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . named_child (0) . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AsyncBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "async_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AsyncBlock<'tree> {
    const KIND: &'static str = "async_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "attribute" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Attribute<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Attribute<'tree> {
    # [doc = concat ! ("Get the field `" , "arguments" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn arguments(&self) -> Option<type_sitter_lib::NodeResult<'tree, TokenTree<'tree>>> {
        self.0
            .child_by_field_name("arguments")
            .map(<TokenTree<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Attribute<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Attribute<'tree> {
    const KIND: &'static str = "attribute";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "attribute_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AttributeItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AttributeItem<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Attribute<'tree>> {
        self . 0 . named_child (0) . map (< Attribute < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "attribute_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem<'tree> {
    const KIND: &'static str = "attribute_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "await_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AwaitExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AwaitExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AwaitExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "await_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AwaitExpression<'tree> {
    const KIND: &'static str = "await_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "base_field_initializer" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BaseFieldInitializer<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BaseFieldInitializer<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BaseFieldInitializer<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "base_field_initializer" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BaseFieldInitializer<'tree> {
    const KIND: &'static str = "base_field_initializer";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "binary_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BinaryExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BinaryExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "left" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "operator" , "`")]
    #[allow(dead_code)]
    #[inline]    pub fn operator (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: NotEq_Mod_And_AndAnd_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr >{
        self . 0 . child_by_field_name ("operator") . map (< anon_unions :: NotEq_Mod_And_AndAnd_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "right" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BinaryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "binary_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BinaryExpression<'tree> {
    const KIND: &'static str = "binary_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "block" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Block<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Block<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DeclarationStatement_Expression_ExpressionStatement,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DeclarationStatement_Expression_ExpressionStatement,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DeclarationStatement_Expression_ExpressionStatement,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DeclarationStatement_Expression_ExpressionStatement,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Block<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Block<'tree> {
    const KIND: &'static str = "block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "boolean_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BooleanLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BooleanLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BooleanLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "boolean_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BooleanLiteral<'tree> {
    const KIND: &'static str = "boolean_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "bounded_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BoundedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BoundedType<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Type_Lifetime>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Type_Lifetime> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Type_Lifetime>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Type_Lifetime> as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BoundedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "bounded_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BoundedType<'tree> {
    const KIND: &'static str = "bounded_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "bracketed_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BracketedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BracketedType<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Type_QualifiedType> {
        self . 0 . named_child (0) . map (< anon_unions :: Type_QualifiedType as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BracketedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "bracketed_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BracketedType<'tree> {
    const KIND: &'static str = "bracketed_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "break_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BreakExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BreakExpression<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_LoopLabel>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_LoopLabel > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_LoopLabel>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_LoopLabel > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BreakExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "break_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BreakExpression<'tree> {
    const KIND: &'static str = "break_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "call_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CallExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CallExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "arguments" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn arguments(&self) -> type_sitter_lib::NodeResult<'tree, Arguments<'tree>> {
        self . 0 . child_by_field_name ("arguments") . map (< Arguments < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "function" , "`")]
    #[allow(dead_code)]
    #[inline]    pub fn function (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Literal_ArrayExpression_AssignmentExpression_AsyncBlock_AwaitExpression_BinaryExpression_Block_BreakExpression_CallExpression_ClosureExpression_CompoundAssignmentExpr_ConstBlock_ContinueExpression_FieldExpression_ForExpression_GenericFunction_Identifier_IfExpression_IndexExpression_LoopExpression_MacroInvocation_MatchExpression_Metavariable_ParenthesizedExpression_ReferenceExpression_ReturnExpression_ScopedIdentifier__Self_StructExpression_TryExpression_TupleExpression_TypeCastExpression_UnaryExpression_UnitExpression_UnsafeBlock_WhileExpression_YieldExpression >{
        self . 0 . child_by_field_name ("function") . map (< anon_unions :: Literal_ArrayExpression_AssignmentExpression_AsyncBlock_AwaitExpression_BinaryExpression_Block_BreakExpression_CallExpression_ClosureExpression_CompoundAssignmentExpr_ConstBlock_ContinueExpression_FieldExpression_ForExpression_GenericFunction_Identifier_IfExpression_IndexExpression_LoopExpression_MacroInvocation_MatchExpression_Metavariable_ParenthesizedExpression_ReferenceExpression_ReturnExpression_ScopedIdentifier__Self_StructExpression_TryExpression_TupleExpression_TypeCastExpression_UnaryExpression_UnitExpression_UnsafeBlock_WhileExpression_YieldExpression as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CallExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "call_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CallExpression<'tree> {
    const KIND: &'static str = "call_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "captured_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CapturedPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CapturedPattern<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CapturedPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "captured_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CapturedPattern<'tree> {
    const KIND: &'static str = "captured_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "closure_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ClosureExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ClosureExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, ClosureParameters<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< ClosureParameters < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "return_type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ClosureExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "closure_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ClosureExpression<'tree> {
    const KIND: &'static str = "closure_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "closure_parameters" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ClosureParameters<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ClosureParameters<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_Parameter>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_Parameter > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_Parameter>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_Parameter > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ClosureParameters<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "closure_parameters" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ClosureParameters<'tree> {
    const KIND: &'static str = "closure_parameters";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "compound_assignment_expr" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CompoundAssignmentExpr<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CompoundAssignmentExpr<'tree> {
    # [doc = concat ! ("Get the field `" , "left" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "operator" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn operator(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::ModEq_AndEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq,
    > {
        self . 0 . child_by_field_name ("operator") . map (< anon_unions :: ModEq_AndEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "right" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CompoundAssignmentExpr<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "compound_assignment_expr" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CompoundAssignmentExpr<'tree> {
    const KIND: &'static str = "compound_assignment_expr";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "const_block" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstBlock<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstBlock<'tree> {
    const KIND: &'static str = "const_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "const_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstItem<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_Identifier_Type_Expression,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_Identifier_Type_Expression,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_Identifier_Type_Expression,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_Identifier_Type_Expression,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstItem<'tree> {
    const KIND: &'static str = "const_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "const_parameter" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstParameter<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstParameter<'tree> {
    const KIND: &'static str = "const_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "constrained_type_parameter" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstrainedTypeParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstrainedTypeParameter<'tree> {
    # [doc = concat ! ("Get the field `" , "bounds" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(&self) -> type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>> {
        self . 0 . child_by_field_name ("bounds") . map (< TraitBounds < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "left" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Lifetime_TypeIdentifier> {
        self . 0 . child_by_field_name ("left") . map (< anon_unions :: Lifetime_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstrainedTypeParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "constrained_type_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstrainedTypeParameter<'tree> {
    const KIND: &'static str = "constrained_type_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "continue_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ContinueExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ContinueExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0
            .named_child(0)
            .map(<LoopLabel<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ContinueExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "continue_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ContinueExpression<'tree> {
    const KIND: &'static str = "continue_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "declaration_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DeclarationList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DeclarationList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, DeclarationStatement<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, DeclarationStatement<'tree>> as TryFrom<_>>::try_from(
                n,
            )
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, DeclarationStatement<'tree>>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, DeclarationStatement<'tree>> as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DeclarationList<'tree> {
    const KIND: &'static str = "declaration_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "dynamic_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DynamicType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DynamicType<'tree> {
    # [doc = concat ! ("Get the field `" , "trait" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier,
    > {
        self . 0 . child_by_field_name ("trait") . map (< anon_unions :: FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DynamicType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dynamic_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DynamicType<'tree> {
    const KIND: &'static str = "dynamic_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "else_clause" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ElseClause<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ElseClause<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Block_IfExpression> {
        self . 0 . named_child (0) . map (< anon_unions :: Block_IfExpression as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ElseClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "else_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ElseClause<'tree> {
    const KIND: &'static str = "else_clause";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "empty_statement" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EmptyStatement<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EmptyStatement<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EmptyStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EmptyStatement<'tree> {
    const KIND: &'static str = "empty_statement";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "empty_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EmptyType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EmptyType<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EmptyType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EmptyType<'tree> {
    const KIND: &'static str = "empty_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "enum_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EnumItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EnumItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, EnumVariantList<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< EnumVariantList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EnumItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EnumItem<'tree> {
    const KIND: &'static str = "enum_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "enum_variant" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EnumVariant<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EnumVariant<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::FieldDeclarationList_OrderedFieldDeclarationList,
        >,
    > {
        self.0.child_by_field_name("body").map(
            <anon_unions::FieldDeclarationList_OrderedFieldDeclarationList as TryFrom<_>>::try_from,
        )
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EnumVariant<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_variant" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EnumVariant<'tree> {
    const KIND: &'static str = "enum_variant";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "enum_variant_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EnumVariantList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EnumVariantList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_EnumVariant>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_EnumVariant> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_EnumVariant>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_EnumVariant> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EnumVariantList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_variant_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EnumVariantList<'tree> {
    const KIND: &'static str = "enum_variant_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "expression_statement" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExpressionStatement<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExpressionStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ExpressionStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExpressionStatement<'tree> {
    const KIND: &'static str = "expression_statement";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "extern_crate_declaration" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExternCrateDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExternCrateDeclaration<'tree> {
    # [doc = concat ! ("Get the field `" , "alias" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(&self) -> Option<type_sitter_lib::NodeResult<'tree, Identifier<'tree>>> {
        self.0
            .child_by_field_name("alias")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Crate_VisibilityModifier_Identifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_VisibilityModifier_Identifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Crate_VisibilityModifier_Identifier>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_VisibilityModifier_Identifier > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ExternCrateDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern_crate_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExternCrateDeclaration<'tree> {
    const KIND: &'static str = "extern_crate_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "extern_modifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExternModifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExternModifier<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, StringLiteral<'tree>>> {
        self.0
            .named_child(0)
            .map(<StringLiteral<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ExternModifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern_modifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExternModifier<'tree> {
    const KIND: &'static str = "extern_modifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_declaration" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldDeclaration<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< FieldIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::VisibilityModifier_FieldIdentifier_Type>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_FieldIdentifier_Type > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::VisibilityModifier_FieldIdentifier_Type>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_FieldIdentifier_Type > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldDeclaration<'tree> {
    const KIND: &'static str = "field_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_declaration_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldDeclarationList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldDeclarationList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_FieldDeclaration>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_FieldDeclaration > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_FieldDeclaration>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_FieldDeclaration > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldDeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldDeclarationList<'tree> {
    const KIND: &'static str = "field_declaration_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "field" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn field(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::FieldIdentifier_IntegerLiteral> {
        self . 0 . child_by_field_name ("field") . map (< anon_unions :: FieldIdentifier_IntegerLiteral as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldExpression<'tree> {
    const KIND: &'static str = "field_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_initializer" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldInitializer<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldInitializer<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< FieldIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_FieldIdentifier_Expression>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_FieldIdentifier_Expression > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_FieldIdentifier_Expression>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_FieldIdentifier_Expression > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldInitializer<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_initializer" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldInitializer<'tree> {
    const KIND: &'static str = "field_initializer";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_initializer_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldInitializerList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldInitializerList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldInitializerList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_initializer_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldInitializerList<'tree> {
    const KIND: &'static str = "field_initializer_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldPattern<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::FieldIdentifier_ShorthandFieldIdentifier>
    {
        self . 0 . child_by_field_name ("name") . map (< anon_unions :: FieldIdentifier_ShorthandFieldIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "pattern" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> Option<type_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0
            .child_by_field_name("pattern")
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldPattern<'tree> {
    const KIND: &'static str = "field_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "for_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "pattern" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self . 0 . child_by_field_name ("pattern") . map (< Pattern < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LoopLabel_Block_Pattern_Expression>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LoopLabel_Block_Pattern_Expression > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LoopLabel_Block_Pattern_Expression>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LoopLabel_Block_Pattern_Expression > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ForExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForExpression<'tree> {
    const KIND: &'static str = "for_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "for_lifetimes" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForLifetimes<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForLifetimes<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Lifetime<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Lifetime<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Lifetime<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Lifetime<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ForLifetimes<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_lifetimes" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForLifetimes<'tree> {
    const KIND: &'static str = "for_lifetimes";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "foreign_mod_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForeignModItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForeignModItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> Option<type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ExternModifier_VisibilityModifier_DeclarationList,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ExternModifier_VisibilityModifier_DeclarationList,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ExternModifier_VisibilityModifier_DeclarationList,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ExternModifier_VisibilityModifier_DeclarationList,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ForeignModItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "foreign_mod_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForeignModItem<'tree> {
    const KIND: &'static str = "foreign_mod_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "fragment_specifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FragmentSpecifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FragmentSpecifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FragmentSpecifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "fragment_specifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FragmentSpecifier<'tree> {
    const KIND: &'static str = "fragment_specifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "function_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Identifier_Metavariable> {
        self . 0 . child_by_field_name ("name") . map (< anon_unions :: Identifier_Metavariable as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< Parameters < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "return_type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionItem<'tree> {
    const KIND: &'static str = "function_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "function_modifiers" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionModifiers<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionModifiers<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, ExternModifier<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, ExternModifier<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, ExternModifier<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, ExternModifier<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionModifiers<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_modifiers" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionModifiers<'tree> {
    const KIND: &'static str = "function_modifiers";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "function_signature_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionSignatureItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionSignatureItem<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Identifier_Metavariable> {
        self . 0 . child_by_field_name ("name") . map (< anon_unions :: Identifier_Metavariable as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< Parameters < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "return_type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionSignatureItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_signature_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionSignatureItem<'tree> {
    const KIND: &'static str = "function_signature_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "function_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionType<'tree> {
    # [doc = concat ! ("Get the field `" , "parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< Parameters < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "return_type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "trait" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::ScopedTypeIdentifier_TypeIdentifier>>
    {
        self.0
            .child_by_field_name("trait")
            .map(<anon_unions::ScopedTypeIdentifier_TypeIdentifier as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionType<'tree> {
    const KIND: &'static str = "function_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "generic_function" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericFunction<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericFunction<'tree> {
    # [doc = concat ! ("Get the field `" , "function" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn function(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::FieldExpression_Identifier_ScopedIdentifier>
    {
        self . 0 . child_by_field_name ("function") . map (< anon_unions :: FieldExpression_Identifier_ScopedIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_arguments" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(&self) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self . 0 . child_by_field_name ("type_arguments") . map (< TypeArguments < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GenericFunction<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_function" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericFunction<'tree> {
    const KIND: &'static str = "generic_function";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "generic_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericType<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::ScopedIdentifier_ScopedTypeIdentifier_TypeIdentifier,
    > {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: ScopedIdentifier_ScopedTypeIdentifier_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_arguments" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(&self) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self . 0 . child_by_field_name ("type_arguments") . map (< TypeArguments < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GenericType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericType<'tree> {
    const KIND: &'static str = "generic_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "generic_type_with_turbofish" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericTypeWithTurbofish<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericTypeWithTurbofish<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::ScopedIdentifier_TypeIdentifier> {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: ScopedIdentifier_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_arguments" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(&self) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self . 0 . child_by_field_name ("type_arguments") . map (< TypeArguments < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GenericTypeWithTurbofish<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type_with_turbofish" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericTypeWithTurbofish<'tree> {
    const KIND: &'static str = "generic_type_with_turbofish";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "higher_ranked_trait_bound" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct HigherRankedTraitBound<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> HigherRankedTraitBound<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(&self) -> type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>> {
        self . 0 . child_by_field_name ("type_parameters") . map (< TypeParameters < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for HigherRankedTraitBound<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "higher_ranked_trait_bound" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for HigherRankedTraitBound<'tree> {
    const KIND: &'static str = "higher_ranked_trait_bound";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "if_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IfExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IfExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "alternative" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alternative(&self) -> Option<type_sitter_lib::NodeResult<'tree, ElseClause<'tree>>> {
        self.0
            .child_by_field_name("alternative")
            .map(<ElseClause<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "condition" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Expression_LetChain_LetCondition> {
        self . 0 . child_by_field_name ("condition") . map (< anon_unions :: Expression_LetChain_LetCondition as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "consequence" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn consequence(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("consequence") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IfExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "if_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IfExpression<'tree> {
    const KIND: &'static str = "if_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "impl_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImplItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImplItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> Option<type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "trait" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::GenericType_ScopedTypeIdentifier_TypeIdentifier,
        >,
    > {
        self.0.child_by_field_name("trait").map(
            <anon_unions::GenericType_ScopedTypeIdentifier_TypeIdentifier as TryFrom<_>>::try_from,
        )
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ImplItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "impl_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImplItem<'tree> {
    const KIND: &'static str = "impl_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "index_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IndexExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IndexExpression<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, Expression<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Expression<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Expression<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IndexExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "index_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IndexExpression<'tree> {
    const KIND: &'static str = "index_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "inner_attribute_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct InnerAttributeItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> InnerAttributeItem<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Attribute<'tree>> {
        self . 0 . named_child (0) . map (< Attribute < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for InnerAttributeItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "inner_attribute_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for InnerAttributeItem<'tree> {
    const KIND: &'static str = "inner_attribute_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "let_chain" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LetChain<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LetChain<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_LetCondition>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_LetCondition > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_LetCondition>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_LetCondition > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LetChain<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_chain" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LetChain<'tree> {
    const KIND: &'static str = "let_chain";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "let_condition" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LetCondition<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LetCondition<'tree> {
    # [doc = concat ! ("Get the field `" , "pattern" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self . 0 . child_by_field_name ("pattern") . map (< Pattern < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LetCondition<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_condition" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LetCondition<'tree> {
    const KIND: &'static str = "let_condition";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "let_declaration" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LetDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LetDeclaration<'tree> {
    # [doc = concat ! ("Get the field `" , "alternative" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alternative(&self) -> Option<type_sitter_lib::NodeResult<'tree, Block<'tree>>> {
        self.0
            .child_by_field_name("alternative")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "pattern" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self . 0 . child_by_field_name ("pattern") . map (< Pattern < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_Block_Pattern_Type_Expression,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_Block_Pattern_Type_Expression,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_Block_Pattern_Type_Expression,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_Block_Pattern_Type_Expression,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LetDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LetDeclaration<'tree> {
    const KIND: &'static str = "let_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "lifetime" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Lifetime<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Lifetime<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . named_child (0) . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lifetime<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "lifetime" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Lifetime<'tree> {
    const KIND: &'static str = "lifetime";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "loop_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LoopExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LoopExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LoopLabel_Block>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::LoopLabel_Block> as TryFrom<_>>::try_from(
                n,
            )
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LoopLabel_Block>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::LoopLabel_Block> as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LoopExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LoopExpression<'tree> {
    const KIND: &'static str = "loop_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "loop_label" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LoopLabel<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LoopLabel<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . named_child (0) . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LoopLabel<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop_label" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LoopLabel<'tree> {
    const KIND: &'static str = "loop_label";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "macro_definition" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MacroDefinition<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MacroDefinition<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MacroRule_Identifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MacroRule_Identifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MacroRule_Identifier>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MacroRule_Identifier > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroDefinition<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_definition" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroDefinition<'tree> {
    const KIND: &'static str = "macro_definition";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "macro_invocation" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MacroInvocation<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MacroInvocation<'tree> {
    # [doc = concat ! ("Get the field `" , "macro" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#macro(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Identifier_ScopedIdentifier> {
        self . 0 . child_by_field_name ("macro") . map (< anon_unions :: Identifier_ScopedIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::TokenTree_Identifier_ScopedIdentifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: TokenTree_Identifier_ScopedIdentifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::TokenTree_Identifier_ScopedIdentifier>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: TokenTree_Identifier_ScopedIdentifier > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroInvocation<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_invocation" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroInvocation<'tree> {
    const KIND: &'static str = "macro_invocation";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "macro_rule" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MacroRule<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MacroRule<'tree> {
    # [doc = concat ! ("Get the field `" , "left" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, TokenTreePattern<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< TokenTreePattern < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "right" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, TokenTree<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< TokenTree < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroRule<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_rule" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroRule<'tree> {
    const KIND: &'static str = "macro_rule";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "match_arm" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchArm<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchArm<'tree> {
    # [doc = concat ! ("Get the field `" , "pattern" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, MatchPattern<'tree>> {
        self . 0 . child_by_field_name ("pattern") . map (< MatchPattern < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_MatchPattern_Expression>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_MatchPattern_Expression > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_MatchPattern_Expression>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_MatchPattern_Expression > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchArm<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_arm" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchArm<'tree> {
    const KIND: &'static str = "match_arm";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "match_block" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, MatchArm<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, MatchArm<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, MatchArm<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, MatchArm<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchBlock<'tree> {
    const KIND: &'static str = "match_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "match_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, MatchBlock<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< MatchBlock < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchExpression<'tree> {
    const KIND: &'static str = "match_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "match_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchPattern<'tree> {
    # [doc = concat ! ("Get the field `" , "condition" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::Expression_LetChain_LetCondition>>
    {
        self.0
            .child_by_field_name("condition")
            .map(<anon_unions::Expression_LetChain_LetCondition as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_Expression_LetChain_LetCondition>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_Expression_LetChain_LetCondition > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_Expression_LetChain_LetCondition>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_Expression_LetChain_LetCondition > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchPattern<'tree> {
    const KIND: &'static str = "match_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "mod_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ModItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ModItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> Option<type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_DeclarationList_Identifier,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_DeclarationList_Identifier,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_DeclarationList_Identifier,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_DeclarationList_Identifier,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ModItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mod_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ModItem<'tree> {
    const KIND: &'static str = "mod_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "mut_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MutPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MutPattern<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_MutableSpecifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_MutableSpecifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_MutableSpecifier>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_MutableSpecifier> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mut_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MutPattern<'tree> {
    const KIND: &'static str = "mut_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "negative_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct NegativeLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> NegativeLiteral<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::FloatLiteral_IntegerLiteral> {
        self . 0 . named_child (0) . map (< anon_unions :: FloatLiteral_IntegerLiteral as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for NegativeLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "negative_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for NegativeLiteral<'tree> {
    const KIND: &'static str = "negative_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "optional_type_parameter" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OptionalTypeParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OptionalTypeParameter<'tree> {
    # [doc = concat ! ("Get the field `" , "default_type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn default_type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("default_type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::ConstrainedTypeParameter_TypeIdentifier>
    {
        self . 0 . child_by_field_name ("name") . map (< anon_unions :: ConstrainedTypeParameter_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OptionalTypeParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "optional_type_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OptionalTypeParameter<'tree> {
    const KIND: &'static str = "optional_type_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "or_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OrPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OrPattern<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "or_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OrPattern<'tree> {
    const KIND: &'static str = "or_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "ordered_field_declaration_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OrderedFieldDeclarationList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OrderedFieldDeclarationList<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn types<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>,
    > + 'a {
        self.0
            .children_by_field_name("type", c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_VisibilityModifier_Type>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_VisibilityModifier_Type > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_VisibilityModifier_Type>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_VisibilityModifier_Type > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrderedFieldDeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ordered_field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OrderedFieldDeclarationList<'tree> {
    const KIND: &'static str = "ordered_field_declaration_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "parameter" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Parameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Parameter<'tree> {
    # [doc = concat ! ("Get the field `" , "pattern" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Pattern__Self> {
        self . 0 . child_by_field_name ("pattern") . map (< anon_unions :: Pattern__Self as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Pattern__Self_Type>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MutableSpecifier_Pattern__Self_Type > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Pattern__Self_Type>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MutableSpecifier_Pattern__Self_Type > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Parameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Parameter<'tree> {
    const KIND: &'static str = "parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "parameters" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Parameters<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Parameters<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_AttributeItem_Parameter_SelfParameter_VariadicParameter,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_AttributeItem_Parameter_SelfParameter_VariadicParameter,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_AttributeItem_Parameter_SelfParameter_VariadicParameter,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_AttributeItem_Parameter_SelfParameter_VariadicParameter,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Parameters<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameters" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Parameters<'tree> {
    const KIND: &'static str = "parameters";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "parenthesized_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ParenthesizedExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ParenthesizedExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ParenthesizedExpression<'tree> {
    const KIND: &'static str = "parenthesized_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "pointer_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PointerType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PointerType<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Type>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MutableSpecifier_Type > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Type>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MutableSpecifier_Type > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PointerType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pointer_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PointerType<'tree> {
    const KIND: &'static str = "pointer_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "qualified_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct QualifiedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> QualifiedType<'tree> {
    # [doc = concat ! ("Get the field `" , "alias" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("alias") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for QualifiedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "qualified_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for QualifiedType<'tree> {
    const KIND: &'static str = "qualified_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "range_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RangeExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RangeExpression<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, Expression<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Expression<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Expression<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RangeExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RangeExpression<'tree> {
    const KIND: &'static str = "range_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "range_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RangePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RangePattern<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RangePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RangePattern<'tree> {
    const KIND: &'static str = "range_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "ref_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RefPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RefPattern<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self . 0 . named_child (0) . map (< Pattern < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RefPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ref_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RefPattern<'tree> {
    const KIND: &'static str = "ref_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "reference_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReferenceExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReferenceExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Expression>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Expression> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MutableSpecifier_Expression>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: MutableSpecifier_Expression > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReferenceExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferenceExpression<'tree> {
    const KIND: &'static str = "reference_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "reference_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReferencePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReferencePattern<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_MutableSpecifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_MutableSpecifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_MutableSpecifier>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_MutableSpecifier> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReferencePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferencePattern<'tree> {
    const KIND: &'static str = "reference_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "reference_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReferenceType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReferenceType<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Lifetime_MutableSpecifier_Type>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Lifetime_MutableSpecifier_Type > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Lifetime_MutableSpecifier_Type>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Lifetime_MutableSpecifier_Type > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReferenceType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferenceType<'tree> {
    const KIND: &'static str = "reference_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "remaining_field_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RemainingFieldPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RemainingFieldPattern<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RemainingFieldPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "remaining_field_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RemainingFieldPattern<'tree> {
    const KIND: &'static str = "remaining_field_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "removed_trait_bound" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RemovedTraitBound<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RemovedTraitBound<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . named_child (0) . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RemovedTraitBound<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "removed_trait_bound" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RemovedTraitBound<'tree> {
    const KIND: &'static str = "removed_trait_bound";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "return_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReturnExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReturnExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReturnExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReturnExpression<'tree> {
    const KIND: &'static str = "return_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "scoped_identifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ScopedIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ScopedIdentifier<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "path" , "`")]
    #[allow(dead_code)]
    #[inline]    pub fn path (& self) -> Option < type_sitter_lib :: NodeResult < 'tree , anon_unions :: BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super > >{
        self . 0 . child_by_field_name ("path") . map (< anon_unions :: BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedIdentifier<'tree> {
    const KIND: &'static str = "scoped_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "scoped_type_identifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ScopedTypeIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ScopedTypeIdentifier<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "path" , "`")]
    #[allow(dead_code)]
    #[inline]    pub fn path (& self) -> Option < type_sitter_lib :: NodeResult < 'tree , anon_unions :: BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super > >{
        self . 0 . child_by_field_name ("path") . map (< anon_unions :: BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedTypeIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_type_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedTypeIdentifier<'tree> {
    const KIND: &'static str = "scoped_type_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "scoped_use_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ScopedUseList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ScopedUseList<'tree> {
    # [doc = concat ! ("Get the field `" , "list" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn list(&self) -> type_sitter_lib::NodeResult<'tree, UseList<'tree>> {
        self . 0 . child_by_field_name ("list") . map (< UseList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "path" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super,
        >,
    > {
        self.0.child_by_field_name("path").map(
            <anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedUseList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_use_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedUseList<'tree> {
    const KIND: &'static str = "scoped_use_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "self_parameter" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SelfParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SelfParameter<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Lifetime_MutableSpecifier__Self>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Lifetime_MutableSpecifier__Self > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Lifetime_MutableSpecifier__Self>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Lifetime_MutableSpecifier__Self > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SelfParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "self_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SelfParameter<'tree> {
    const KIND: &'static str = "self_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "shorthand_field_initializer" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ShorthandFieldInitializer<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ShorthandFieldInitializer<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_Identifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AttributeItem_Identifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_Identifier>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::AttributeItem_Identifier> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ShorthandFieldInitializer<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "shorthand_field_initializer" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ShorthandFieldInitializer<'tree> {
    const KIND: &'static str = "shorthand_field_initializer";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "slice_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SlicePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SlicePattern<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SlicePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "slice_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SlicePattern<'tree> {
    const KIND: &'static str = "slice_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "source_file" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SourceFile<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SourceFile<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::DeclarationStatement_ExpressionStatement>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: DeclarationStatement_ExpressionStatement > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::DeclarationStatement_ExpressionStatement>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: DeclarationStatement_ExpressionStatement > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SourceFile<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "source_file" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SourceFile<'tree> {
    const KIND: &'static str = "source_file";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "static_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StaticItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StaticItem<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_VisibilityModifier_Identifier_Type_Expression,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_VisibilityModifier_Identifier_Type_Expression,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_VisibilityModifier_Identifier_Type_Expression,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::MutableSpecifier_VisibilityModifier_Identifier_Type_Expression,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StaticItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "static_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StaticItem<'tree> {
    const KIND: &'static str = "static_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "string_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StringLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StringLiteral<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StringLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StringLiteral<'tree> {
    const KIND: &'static str = "string_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "struct_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, FieldInitializerList<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< FieldInitializerList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::GenericTypeWithTurbofish_ScopedTypeIdentifier_TypeIdentifier,
    > {
        self . 0 . child_by_field_name ("name") . map (< anon_unions :: GenericTypeWithTurbofish_ScopedTypeIdentifier_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StructExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructExpression<'tree> {
    const KIND: &'static str = "struct_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "struct_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::FieldDeclarationList_OrderedFieldDeclarationList,
        >,
    > {
        self.0.child_by_field_name("body").map(
            <anon_unions::FieldDeclarationList_OrderedFieldDeclarationList as TryFrom<_>>::try_from,
        )
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StructItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructItem<'tree> {
    const KIND: &'static str = "struct_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "struct_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructPattern<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::ScopedTypeIdentifier_TypeIdentifier> {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: ScopedTypeIdentifier_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StructPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructPattern<'tree> {
    const KIND: &'static str = "struct_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "token_binding_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenBindingPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenBindingPattern<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Metavariable<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Metavariable < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, FragmentSpecifier<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< FragmentSpecifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenBindingPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_binding_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenBindingPattern<'tree> {
    const KIND: &'static str = "token_binding_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "token_repetition" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenRepetition<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenRepetition<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenRepetition<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_repetition" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenRepetition<'tree> {
    const KIND: &'static str = "token_repetition";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "token_repetition_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenRepetitionPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenRepetitionPattern<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenRepetitionPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_repetition_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenRepetitionPattern<'tree> {
    const KIND: &'static str = "token_repetition_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "token_tree" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenTree<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenTree<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenTree<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_tree" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenTree<'tree> {
    const KIND: &'static str = "token_tree";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "token_tree_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenTreePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenTreePattern<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenTreePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_tree_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenTreePattern<'tree> {
    const KIND: &'static str = "token_tree_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "trait_bounds" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TraitBounds<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TraitBounds<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TraitBounds<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait_bounds" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TraitBounds<'tree> {
    const KIND: &'static str = "trait_bounds";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "trait_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TraitItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TraitItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< DeclarationList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "bounds" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(&self) -> Option<type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>>> {
        self.0
            .child_by_field_name("bounds")
            .map(<TraitBounds<'tree> as TryFrom<_>>::try_from)
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TraitItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TraitItem<'tree> {
    const KIND: &'static str = "trait_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "try_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TryExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TryExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "try_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TryExpression<'tree> {
    const KIND: &'static str = "try_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "tuple_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TupleExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TupleExpression<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_AttributeItem > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_AttributeItem> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TupleExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TupleExpression<'tree> {
    const KIND: &'static str = "tuple_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "tuple_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TuplePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TuplePattern<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TuplePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TuplePattern<'tree> {
    const KIND: &'static str = "tuple_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "tuple_struct_pattern" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TupleStructPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TupleStructPattern<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Identifier_ScopedIdentifier> {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: Identifier_ScopedIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_Identifier_ScopedIdentifier>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_Identifier_ScopedIdentifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Pattern_Identifier_ScopedIdentifier>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Pattern_Identifier_ScopedIdentifier > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TupleStructPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_struct_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TupleStructPattern<'tree> {
    const KIND: &'static str = "tuple_struct_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "tuple_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TupleType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TupleType<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TupleType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TupleType<'tree> {
    const KIND: &'static str = "tuple_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "type_arguments" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeArguments<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeArguments<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Literal_Type_Block_Lifetime_TypeBinding>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Type_Block_Lifetime_TypeBinding > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Literal_Type_Block_Lifetime_TypeBinding>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Literal_Type_Block_Lifetime_TypeBinding > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeArguments<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_arguments" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeArguments<'tree> {
    const KIND: &'static str = "type_arguments";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "type_binding" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeBinding<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeBinding<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_arguments" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>>> {
        self.0
            .child_by_field_name("type_arguments")
            .map(<TypeArguments<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeBinding<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_binding" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeBinding<'tree> {
    const KIND: &'static str = "type_binding";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "type_cast_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeCastExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeCastExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "value" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeCastExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_cast_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeCastExpression<'tree> {
    const KIND: &'static str = "type_cast_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "type_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeItem<'tree> {
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_TypeIdentifier_Type_TypeParameters,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_TypeIdentifier_Type_TypeParameters,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_TypeIdentifier_Type_TypeParameters,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::VisibilityModifier_TypeIdentifier_Type_TypeParameters,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeItem<'tree> {
    const KIND: &'static str = "type_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "type_parameters" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeParameters<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeParameters<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeParameters<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_parameters" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeParameters<'tree> {
    const KIND: &'static str = "type_parameters";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "unary_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnaryExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnaryExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnaryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unary_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnaryExpression<'tree> {
    const KIND: &'static str = "unary_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "union_item" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnionItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnionItem<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, FieldDeclarationList<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< FieldDeclarationList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "name" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "type_parameters" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnionItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "union_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnionItem<'tree> {
    const KIND: &'static str = "union_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "unit_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnitExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnitExpression<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnitExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unit_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnitExpression<'tree> {
    const KIND: &'static str = "unit_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "unit_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnitType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnitType<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnitType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unit_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnitType<'tree> {
    const KIND: &'static str = "unit_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "unsafe_block" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnsafeBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnsafeBlock<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . named_child (0) . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnsafeBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unsafe_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnsafeBlock<'tree> {
    const KIND: &'static str = "unsafe_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "use_as_clause" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseAsClause<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseAsClause<'tree> {
    # [doc = concat ! ("Get the field `" , "alias" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("alias") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "path" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super,
    > {
        self . 0 . child_by_field_name ("path") . map (< anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseAsClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_as_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseAsClause<'tree> {
    const KIND: &'static str = "use_as_clause";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "use_declaration" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseDeclaration<'tree> {
    # [doc = concat ! ("Get the field `" , "argument" , "`")]
    #[allow(dead_code)]
    #[inline]    pub fn argument (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard >{
        self . 0 . child_by_field_name ("argument") . map (< anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseDeclaration<'tree> {
    const KIND: &'static str = "use_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "use_list" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseList<'tree> {
    const KIND: &'static str = "use_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "use_wildcard" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseWildcard<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseWildcard<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super,
        >,
    > {
        self.0.named_child(0).map(
            <anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseWildcard<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_wildcard" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseWildcard<'tree> {
    const KIND: &'static str = "use_wildcard";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "variadic_parameter" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VariadicParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VariadicParameter<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for VariadicParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "variadic_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VariadicParameter<'tree> {
    const KIND: &'static str = "variadic_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "visibility_modifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VisibilityModifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VisibilityModifier<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super,
        >,
    > {
        self.0.named_child(0).map(
            <anon_unions::Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for VisibilityModifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "visibility_modifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VisibilityModifier<'tree> {
    const KIND: &'static str = "visibility_modifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "where_clause" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WhereClause<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WhereClause<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, WherePredicate<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, WherePredicate<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, WherePredicate<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, WherePredicate<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WhereClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WhereClause<'tree> {
    const KIND: &'static str = "where_clause";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "where_predicate" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WherePredicate<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WherePredicate<'tree> {
    # [doc = concat ! ("Get the field `" , "bounds" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(&self) -> type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>> {
        self . 0 . child_by_field_name ("bounds") . map (< TraitBounds < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "left" , "`")]
    #[allow(dead_code)]
    #[inline]    pub fn left (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: ArrayType_GenericType_HigherRankedTraitBound_Lifetime_PointerType_PrimitiveType_ReferenceType_ScopedTypeIdentifier_TupleType_TypeIdentifier >{
        self . 0 . child_by_field_name ("left") . map (< anon_unions :: ArrayType_GenericType_HigherRankedTraitBound_Lifetime_PointerType_PrimitiveType_ReferenceType_ScopedTypeIdentifier_TupleType_TypeIdentifier as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WherePredicate<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where_predicate" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WherePredicate<'tree> {
    const KIND: &'static str = "where_predicate";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "while_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WhileExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WhileExpression<'tree> {
    # [doc = concat ! ("Get the field `" , "body" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    # [doc = concat ! ("Get the field `" , "condition" , "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Expression_LetChain_LetCondition> {
        self . 0 . child_by_field_name ("condition") . map (< anon_unions :: Expression_LetChain_LetCondition as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::LoopLabel_Block_Expression_LetChain_LetCondition,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::LoopLabel_Block_Expression_LetChain_LetCondition,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::LoopLabel_Block_Expression_LetChain_LetCondition,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::LoopLabel_Block_Expression_LetChain_LetCondition,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WhileExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "while_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WhileExpression<'tree> {
    const KIND: &'static str = "while_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "yield_expression" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct YieldExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> YieldExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for YieldExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "yield_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for YieldExpression<'tree> {
    const KIND: &'static str = "yield_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "block_comment" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BlockComment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BlockComment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BlockComment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block_comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BlockComment<'tree> {
    const KIND: &'static str = "block_comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "char_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CharLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CharLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CharLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "char_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CharLiteral<'tree> {
    const KIND: &'static str = "char_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "crate" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Crate<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Crate<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Crate<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "crate" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Crate<'tree> {
    const KIND: &'static str = "crate";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "escape_sequence" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EscapeSequence<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EscapeSequence<'tree> {
    const KIND: &'static str = "escape_sequence";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "field_identifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldIdentifier<'tree> {
    const KIND: &'static str = "field_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "float_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FloatLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FloatLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FloatLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "float_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FloatLiteral<'tree> {
    const KIND: &'static str = "float_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "identifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Identifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier<'tree> {
    const KIND: &'static str = "identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "integer_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IntegerLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IntegerLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IntegerLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "integer_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IntegerLiteral<'tree> {
    const KIND: &'static str = "integer_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "line_comment" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LineComment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LineComment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LineComment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "line_comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LineComment<'tree> {
    const KIND: &'static str = "line_comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "metavariable" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Metavariable<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Metavariable<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Metavariable<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "metavariable" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Metavariable<'tree> {
    const KIND: &'static str = "metavariable";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "mutable_specifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MutableSpecifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MutableSpecifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutableSpecifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mutable_specifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MutableSpecifier<'tree> {
    const KIND: &'static str = "mutable_specifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "primitive_type" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PrimitiveType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PrimitiveType<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PrimitiveType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "primitive_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PrimitiveType<'tree> {
    const KIND: &'static str = "primitive_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "raw_string_literal" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RawStringLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RawStringLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RawStringLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "raw_string_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RawStringLiteral<'tree> {
    const KIND: &'static str = "raw_string_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "self" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct _Self<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> _Self<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for _Self<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "self" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for _Self<'tree> {
    const KIND: &'static str = "self";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "shorthand_field_identifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ShorthandFieldIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ShorthandFieldIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ShorthandFieldIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "shorthand_field_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ShorthandFieldIdentifier<'tree> {
    const KIND: &'static str = "shorthand_field_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "super" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Super<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Super<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Super<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "super" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Super<'tree> {
    const KIND: &'static str = "super";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
# [doc = concat ! ("Typed node `" , "type_identifier" , "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeIdentifier<'tree> {
    const KIND: &'static str = "type_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    # [doc = concat ! ("Typed node `" , "as" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct As<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> As<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for As<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "as" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for As<'tree> {
        const KIND: &'static str = "as";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "async" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Async<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Async<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Async<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "async" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Async<'tree> {
        const KIND: &'static str = "async";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "await" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Await<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Await<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Await<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "await" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Await<'tree> {
        const KIND: &'static str = "await";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "block" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Block<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Block<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Block<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "block" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Block<'tree> {
        const KIND: &'static str = "block";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "break" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Break<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Break<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Break<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "break" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Break<'tree> {
        const KIND: &'static str = "break";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "const" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Const<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Const<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Const<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "const" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Const<'tree> {
        const KIND: &'static str = "const";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "continue" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Continue<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Continue<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Continue<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "continue" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Continue<'tree> {
        const KIND: &'static str = "continue";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "default" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Default<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Default<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Default<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "default" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Default<'tree> {
        const KIND: &'static str = "default";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "dyn" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Dyn<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Dyn<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dyn<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "dyn" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Dyn<'tree> {
        const KIND: &'static str = "dyn";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "else" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Else<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Else<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Else<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "else" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Else<'tree> {
        const KIND: &'static str = "else";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "enum" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Enum<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Enum<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Enum<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "enum" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Enum<'tree> {
        const KIND: &'static str = "enum";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "expr" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Expr<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Expr<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expr<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "expr" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expr<'tree> {
        const KIND: &'static str = "expr";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "extern" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Extern<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Extern<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Extern<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "extern" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Extern<'tree> {
        const KIND: &'static str = "extern";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "false" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct False<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> False<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for False<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "false" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for False<'tree> {
        const KIND: &'static str = "false";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "fn" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Fn<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Fn<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Fn<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "fn" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Fn<'tree> {
        const KIND: &'static str = "fn";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "for" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct For<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> For<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for For<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "for" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for For<'tree> {
        const KIND: &'static str = "for";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "ident" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Ident<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Ident<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Ident<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "ident" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Ident<'tree> {
        const KIND: &'static str = "ident";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "if" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct If<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> If<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for If<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "if" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for If<'tree> {
        const KIND: &'static str = "if";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "impl" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Impl<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Impl<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Impl<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "impl" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Impl<'tree> {
        const KIND: &'static str = "impl";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "in" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct In<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> In<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for In<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "in" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for In<'tree> {
        const KIND: &'static str = "in";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "item" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Item<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Item<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Item<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "item" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Item<'tree> {
        const KIND: &'static str = "item";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "let" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Let<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Let<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Let<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "let" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Let<'tree> {
        const KIND: &'static str = "let";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "lifetime" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Lifetime<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Lifetime<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lifetime<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "lifetime" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lifetime<'tree> {
        const KIND: &'static str = "lifetime";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "literal" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Literal<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Literal<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Literal<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "literal" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Literal<'tree> {
        const KIND: &'static str = "literal";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "loop" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Loop<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Loop<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Loop<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "loop" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Loop<'tree> {
        const KIND: &'static str = "loop";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "match" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Match<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Match<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Match<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "match" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Match<'tree> {
        const KIND: &'static str = "match";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "meta" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Meta<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Meta<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Meta<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "meta" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Meta<'tree> {
        const KIND: &'static str = "meta";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "mod" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mod<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mod<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "mod" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mod<'tree> {
        const KIND: &'static str = "mod";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "move" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Move<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Move<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Move<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "move" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Move<'tree> {
        const KIND: &'static str = "move";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "pat" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Pat<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Pat<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pat<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "pat" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pat<'tree> {
        const KIND: &'static str = "pat";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "path" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Path<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Path<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Path<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "path" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Path<'tree> {
        const KIND: &'static str = "path";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "pub" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Pub<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Pub<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pub<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "pub" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pub<'tree> {
        const KIND: &'static str = "pub";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "ref" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Ref<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Ref<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Ref<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "ref" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Ref<'tree> {
        const KIND: &'static str = "ref";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "return" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Return<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Return<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Return<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "return" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Return<'tree> {
        const KIND: &'static str = "return";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "static" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Static<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Static<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Static<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "static" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Static<'tree> {
        const KIND: &'static str = "static";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "stmt" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Stmt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Stmt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Stmt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "stmt" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Stmt<'tree> {
        const KIND: &'static str = "stmt";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "struct" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Struct<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Struct<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Struct<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "struct" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Struct<'tree> {
        const KIND: &'static str = "struct";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "trait" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Trait<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Trait<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Trait<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "trait" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Trait<'tree> {
        const KIND: &'static str = "trait";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "true" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct True<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> True<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for True<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "true" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for True<'tree> {
        const KIND: &'static str = "true";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "tt" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Tt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Tt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Tt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "tt" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Tt<'tree> {
        const KIND: &'static str = "tt";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "ty" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Ty<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Ty<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Ty<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "ty" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Ty<'tree> {
        const KIND: &'static str = "ty";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "type" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Type<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Type<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "type" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Type<'tree> {
        const KIND: &'static str = "type";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "union" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Union<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Union<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Union<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "union" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Union<'tree> {
        const KIND: &'static str = "union";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "unsafe" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Unsafe<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Unsafe<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Unsafe<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "unsafe" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Unsafe<'tree> {
        const KIND: &'static str = "unsafe";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "use" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Use<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Use<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Use<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "use" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Use<'tree> {
        const KIND: &'static str = "use";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "vis" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Vis<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Vis<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Vis<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "vis" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Vis<'tree> {
        const KIND: &'static str = "vis";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "where" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Where<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Where<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Where<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "where" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Where<'tree> {
        const KIND: &'static str = "where";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "while" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct While<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> While<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for While<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "while" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for While<'tree> {
        const KIND: &'static str = "while";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "yield" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Yield<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Yield<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Yield<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "yield" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Yield<'tree> {
        const KIND: &'static str = "yield";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    # [doc = concat ! ("Typed node `" , "!" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Not<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not<'tree> {
        const KIND: &'static str = "!";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "!=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct NotEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> NotEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for NotEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for NotEq<'tree> {
        const KIND: &'static str = "!=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "\"" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DoubleQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DoubleQuote<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\"" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DoubleQuote<'tree> {
        const KIND: &'static str = "\"";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "#" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Hash<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Hash<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Hash<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "#" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Hash<'tree> {
        const KIND: &'static str = "#";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "$" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Dollar<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Dollar<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dollar<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "$" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Dollar<'tree> {
        const KIND: &'static str = "$";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "%" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mod<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mod<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "%" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mod<'tree> {
        const KIND: &'static str = "%";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "%=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct ModEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> ModEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ModEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "%=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ModEq<'tree> {
        const KIND: &'static str = "%=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "&" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for And<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for And<'tree> {
        const KIND: &'static str = "&";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "&&" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AndAnd<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AndAnd<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AndAnd<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&&" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AndAnd<'tree> {
        const KIND: &'static str = "&&";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "&=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AndEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AndEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AndEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AndEq<'tree> {
        const KIND: &'static str = "&=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "'" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Quote<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Quote<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Quote<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "'" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Quote<'tree> {
        const KIND: &'static str = "'";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "(" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LParen<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LParen<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen<'tree> {
        const KIND: &'static str = "(";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ")" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RParen<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RParen<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RParen<'tree> {
        const KIND: &'static str = ")";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "*" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mul<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mul<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mul<'tree> {
        const KIND: &'static str = "*";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "*=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct MulEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> MulEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MulEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "*=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MulEq<'tree> {
        const KIND: &'static str = "*=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "+" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Add<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add<'tree> {
        const KIND: &'static str = "+";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "+=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AddEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AddEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AddEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "+=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AddEq<'tree> {
        const KIND: &'static str = "+=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "," , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Comma<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Comma<'tree> {
        const KIND: &'static str = ",";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "-" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub<'tree> {
        const KIND: &'static str = "-";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "-=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct SubEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> SubEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SubEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "-=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for SubEq<'tree> {
        const KIND: &'static str = "-=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "->" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct SubGt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> SubGt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SubGt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "->" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for SubGt<'tree> {
        const KIND: &'static str = "->";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "." , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Dot<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Dot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Dot<'tree> {
        const KIND: &'static str = ".";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ".." , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DotDot<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DotDot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DotDot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ".." {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDot<'tree> {
        const KIND: &'static str = "..";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "..." , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DotDotDot<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DotDotDot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DotDotDot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "..." {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDotDot<'tree> {
        const KIND: &'static str = "...";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "..=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DotDotEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DotDotEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DotDotEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "..=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDotEq<'tree> {
        const KIND: &'static str = "..=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "/" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Div<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Div<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Div<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Div<'tree> {
        const KIND: &'static str = "/";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "/=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DivEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DivEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DivEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "/=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DivEq<'tree> {
        const KIND: &'static str = "/=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ":" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Colon<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Colon<'tree> {
        const KIND: &'static str = ":";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "::" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct ColonColon<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> ColonColon<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ColonColon<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "::" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ColonColon<'tree> {
        const KIND: &'static str = "::";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ";" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Semicolon<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Semicolon<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Semicolon<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ";" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Semicolon<'tree> {
        const KIND: &'static str = ";";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "<" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Lt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Lt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lt<'tree> {
        const KIND: &'static str = "<";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "<<" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtLt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtLt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LtLt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<<" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtLt<'tree> {
        const KIND: &'static str = "<<";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "<<=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtLtEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtLtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LtLtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<<=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtLtEq<'tree> {
        const KIND: &'static str = "<<=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "<=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtEq<'tree> {
        const KIND: &'static str = "<=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Eq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Eq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Eq<'tree> {
        const KIND: &'static str = "=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "==" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct EqEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> EqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EqEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for EqEq<'tree> {
        const KIND: &'static str = "==";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "=>" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct EqGt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> EqGt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EqGt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "=>" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for EqGt<'tree> {
        const KIND: &'static str = "=>";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ">" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Gt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Gt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Gt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Gt<'tree> {
        const KIND: &'static str = ">";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ">=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct GtEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> GtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for GtEq<'tree> {
        const KIND: &'static str = ">=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ">>" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct GtGt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> GtGt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GtGt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">>" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for GtGt<'tree> {
        const KIND: &'static str = ">>";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , ">>=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct GtGtEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> GtGtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GtGtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">>=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for GtGtEq<'tree> {
        const KIND: &'static str = ">>=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "?" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Question<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Question<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Question<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "?" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Question<'tree> {
        const KIND: &'static str = "?";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "@" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct At<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> At<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for At<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "@" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for At<'tree> {
        const KIND: &'static str = "@";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "[" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LBracket<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LBracket<'tree> {
        const KIND: &'static str = "[";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "]" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RBracket<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RBracket<'tree> {
        const KIND: &'static str = "]";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "^" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitXor<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitXor<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BitXor<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "^" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXor<'tree> {
        const KIND: &'static str = "^";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "^=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitXorEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitXorEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BitXorEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "^=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXorEq<'tree> {
        const KIND: &'static str = "^=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "_" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct __<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> __<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for __<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "_" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for __<'tree> {
        const KIND: &'static str = "_";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "macro_rules!" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct MacroRulesNot<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> MacroRulesNot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroRulesNot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "macro_rules!" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroRulesNot<'tree> {
        const KIND: &'static str = "macro_rules!";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "{" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LBrace<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LBrace<'tree> {
        const KIND: &'static str = "{";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "|" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Or<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Or<'tree> {
        const KIND: &'static str = "|";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "|=" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct OrEq<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> OrEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "|=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for OrEq<'tree> {
        const KIND: &'static str = "|=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "||" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct OrOr<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> OrOr<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrOr<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "||" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for OrOr<'tree> {
        const KIND: &'static str = "||";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    # [doc = concat ! ("Typed node `" , "}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RBrace<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RBrace<'tree> {
        const KIND: &'static str = "}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    # [doc = concat ! ("one of `" , "{function_type | generic_type | scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        FunctionType(FunctionType<'tree>),
        GenericType(GenericType<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "function_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn function_type(self) -> Option<FunctionType<'tree>> {
            match self {
                Self::FunctionType(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn generic_type(self) -> Option<GenericType<'tree>> {
            match self {
                Self::GenericType(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "function_type" => Ok(unsafe {
                    Self :: FunctionType (< FunctionType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "generic_type" => Ok(unsafe {
                    Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_type_identifier" => Ok(unsafe {
                    Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for FunctionType_GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        const KIND: &'static str =
            "{function_type | generic_type | scoped_type_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FunctionType(x) => x.node(),
                Self::GenericType(x) => x.node(),
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FunctionType(x) => x.node_mut(),
                Self::GenericType(x) => x.node_mut(),
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_expression | attribute_item}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_AttributeItem<'tree> {
        Expression(Expression<'tree>),
        AttributeItem(AttributeItem<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_AttributeItem<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expression_AttributeItem<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <AttributeItem<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::AttributeItem(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_AttributeItem<'tree> {
        const KIND: &'static str = "{_expression | attribute_item}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::AttributeItem(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::AttributeItem(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{crate | identifier | metavariable | scoped_identifier | self | super | token_tree | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression<'tree> {
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
        TokenTree(TokenTree<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn crate_(self) -> Option<Crate<'tree>> {
            match self {
                Self::Crate(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn metavariable(self) -> Option<Metavariable<'tree>> {
            match self {
                Self::Metavariable(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn super_(self) -> Option<Super<'tree>> {
            match self {
                Self::Super(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "token_tree" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn token_tree(self) -> Option<TokenTree<'tree>> {
            match self {
                Self::TokenTree(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Crate<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Crate(this));
            }
            if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Metavariable<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Metavariable(this));
            }
            if let Ok(this) = <ScopedIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ScopedIdentifier(this));
            }
            if let Ok(this) = <_Self<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::_Self(this));
            }
            if let Ok(this) = <Super<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Super(this));
            }
            if let Ok(this) = <TokenTree<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TokenTree(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super_TokenTree_Expression<'tree>
    {
        const KIND : & 'static str = "{crate | identifier | metavariable | scoped_identifier | self | super | token_tree | _expression}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Crate(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Metavariable(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
                Self::_Self(x) => x.node(),
                Self::Super(x) => x.node(),
                Self::TokenTree(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Crate(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Metavariable(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
                Self::Super(x) => x.node_mut(),
                Self::TokenTree(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{!= | % | & | && | * | + | - | / | < | << | <= | == | > | >= | >> | ^ | | | ||}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NotEq_Mod_And_AndAnd_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr<
        'tree,
    > {
        NotEq(symbols::NotEq<'tree>),
        Mod(symbols::Mod<'tree>),
        And(symbols::And<'tree>),
        AndAnd(symbols::AndAnd<'tree>),
        Mul(symbols::Mul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        Div(symbols::Div<'tree>),
        Lt(symbols::Lt<'tree>),
        LtLt(symbols::LtLt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
        GtGt(symbols::GtGt<'tree>),
        BitXor(symbols::BitXor<'tree>),
        Or(symbols::Or<'tree>),
        OrOr(symbols::OrOr<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        NotEq_Mod_And_AndAnd_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr<'tree>
    {
        # [doc = concat ! ("Returns the node if it is of kind `" , "!=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn not_eq(self) -> Option<symbols::NotEq<'tree>> {
            match self {
                Self::NotEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "%" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#mod(self) -> Option<symbols::Mod<'tree>> {
            match self {
                Self::Mod(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "&" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and(self) -> Option<symbols::And<'tree>> {
            match self {
                Self::And(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "&&" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and_and(self) -> Option<symbols::AndAnd<'tree>> {
            match self {
                Self::AndAnd(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "*" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mul(self) -> Option<symbols::Mul<'tree>> {
            match self {
                Self::Mul(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "+" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add(self) -> Option<symbols::Add<'tree>> {
            match self {
                Self::Add(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "-" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub(self) -> Option<symbols::Sub<'tree>> {
            match self {
                Self::Sub(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "/" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn div(self) -> Option<symbols::Div<'tree>> {
            match self {
                Self::Div(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "<" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt(self) -> Option<symbols::Lt<'tree>> {
            match self {
                Self::Lt(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "<<" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_lt(self) -> Option<symbols::LtLt<'tree>> {
            match self {
                Self::LtLt(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "<=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_eq(self) -> Option<symbols::LtEq<'tree>> {
            match self {
                Self::LtEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "==" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn eq_eq(self) -> Option<symbols::EqEq<'tree>> {
            match self {
                Self::EqEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , ">" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt(self) -> Option<symbols::Gt<'tree>> {
            match self {
                Self::Gt(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , ">=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt_eq(self) -> Option<symbols::GtEq<'tree>> {
            match self {
                Self::GtEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , ">>" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt_gt(self) -> Option<symbols::GtGt<'tree>> {
            match self {
                Self::GtGt(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "^" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn bit_xor(self) -> Option<symbols::BitXor<'tree>> {
            match self {
                Self::BitXor(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "|" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn or(self) -> Option<symbols::Or<'tree>> {
            match self {
                Self::Or(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "||" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn or_or(self) -> Option<symbols::OrOr<'tree>> {
            match self {
                Self::OrOr(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for NotEq_Mod_And_AndAnd_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr<
            'tree,
        >
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "!=" => Ok(unsafe {
                    Self::NotEq(<symbols::NotEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "%" => Ok(unsafe {
                    Self :: Mod (< symbols :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "&" => Ok(unsafe {
                    Self :: And (< symbols :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "&&" => Ok(unsafe {
                    Self::AndAnd(<symbols::AndAnd<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "*" => Ok(unsafe {
                    Self :: Mul (< symbols :: Mul < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "+" => Ok(unsafe {
                    Self :: Add (< symbols :: Add < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "-" => Ok(unsafe {
                    Self :: Sub (< symbols :: Sub < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "/" => Ok(unsafe {
                    Self :: Div (< symbols :: Div < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "<" => Ok(unsafe {
                    Self :: Lt (< symbols :: Lt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "<<" => Ok(unsafe {
                    Self::LtLt(<symbols::LtLt<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "<=" => Ok(unsafe {
                    Self::LtEq(<symbols::LtEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "==" => Ok(unsafe {
                    Self::EqEq(<symbols::EqEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ">" => Ok(unsafe {
                    Self :: Gt (< symbols :: Gt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                ">=" => Ok(unsafe {
                    Self::GtEq(<symbols::GtEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ">>" => Ok(unsafe {
                    Self::GtGt(<symbols::GtGt<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "^" => Ok(unsafe {
                    Self::BitXor(<symbols::BitXor<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "|" => Ok(unsafe {
                    Self :: Or (< symbols :: Or < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "||" => Ok(unsafe {
                    Self::OrOr(<symbols::OrOr<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for NotEq_Mod_And_AndAnd_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr<
            'tree,
        >
    {
        const KIND: &'static str =
            "{!= | % | & | && | * | + | - | / | < | << | <= | == | > | >= | >> | ^ | | | ||}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => x.node(),
                Self::Mod(x) => x.node(),
                Self::And(x) => x.node(),
                Self::AndAnd(x) => x.node(),
                Self::Mul(x) => x.node(),
                Self::Add(x) => x.node(),
                Self::Sub(x) => x.node(),
                Self::Div(x) => x.node(),
                Self::Lt(x) => x.node(),
                Self::LtLt(x) => x.node(),
                Self::LtEq(x) => x.node(),
                Self::EqEq(x) => x.node(),
                Self::Gt(x) => x.node(),
                Self::GtEq(x) => x.node(),
                Self::GtGt(x) => x.node(),
                Self::BitXor(x) => x.node(),
                Self::Or(x) => x.node(),
                Self::OrOr(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => x.node_mut(),
                Self::Mod(x) => x.node_mut(),
                Self::And(x) => x.node_mut(),
                Self::AndAnd(x) => x.node_mut(),
                Self::Mul(x) => x.node_mut(),
                Self::Add(x) => x.node_mut(),
                Self::Sub(x) => x.node_mut(),
                Self::Div(x) => x.node_mut(),
                Self::Lt(x) => x.node_mut(),
                Self::LtLt(x) => x.node_mut(),
                Self::LtEq(x) => x.node_mut(),
                Self::EqEq(x) => x.node_mut(),
                Self::Gt(x) => x.node_mut(),
                Self::GtEq(x) => x.node_mut(),
                Self::GtGt(x) => x.node_mut(),
                Self::BitXor(x) => x.node_mut(),
                Self::Or(x) => x.node_mut(),
                Self::OrOr(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_declaration_statement | _expression | expression_statement}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DeclarationStatement_Expression_ExpressionStatement<'tree> {
        DeclarationStatement(DeclarationStatement<'tree>),
        Expression(Expression<'tree>),
        ExpressionStatement(ExpressionStatement<'tree>),
    }
    #[automatically_derived]
    impl<'tree> DeclarationStatement_Expression_ExpressionStatement<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_declaration_statement" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn declaration_statement(self) -> Option<DeclarationStatement<'tree>> {
            match self {
                Self::DeclarationStatement(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "expression_statement" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression_statement(self) -> Option<ExpressionStatement<'tree>> {
            match self {
                Self::ExpressionStatement(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for DeclarationStatement_Expression_ExpressionStatement<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <DeclarationStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::DeclarationStatement(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ExpressionStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ExpressionStatement(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for DeclarationStatement_Expression_ExpressionStatement<'tree>
    {
        const KIND: &'static str = "{_declaration_statement | _expression | expression_statement}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::DeclarationStatement(x) => x.node(),
                Self::Expression(x) => x.node(),
                Self::ExpressionStatement(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::DeclarationStatement(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
                Self::ExpressionStatement(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_type | lifetime}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type_Lifetime<'tree> {
        Type(Type<'tree>),
        Lifetime(Lifetime<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type_Lifetime<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lifetime(self) -> Option<Lifetime<'tree>> {
            match self {
                Self::Lifetime(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Type_Lifetime<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <Lifetime<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Lifetime(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Type_Lifetime<'tree> {
        const KIND: &'static str = "{_type | lifetime}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node(),
                Self::Lifetime(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node_mut(),
                Self::Lifetime(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_type | qualified_type}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type_QualifiedType<'tree> {
        Type(Type<'tree>),
        QualifiedType(QualifiedType<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type_QualifiedType<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "qualified_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn qualified_type(self) -> Option<QualifiedType<'tree>> {
            match self {
                Self::QualifiedType(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Type_QualifiedType<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <QualifiedType<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::QualifiedType(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Type_QualifiedType<'tree> {
        const KIND: &'static str = "{_type | qualified_type}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node(),
                Self::QualifiedType(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node_mut(),
                Self::QualifiedType(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_expression | loop_label}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_LoopLabel<'tree> {
        Expression(Expression<'tree>),
        LoopLabel(LoopLabel<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_LoopLabel<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "loop_label" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn loop_label(self) -> Option<LoopLabel<'tree>> {
            match self {
                Self::LoopLabel(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expression_LoopLabel<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <LoopLabel<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LoopLabel(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_LoopLabel<'tree> {
        const KIND: &'static str = "{_expression | loop_label}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::LoopLabel(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::LoopLabel(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_literal | array_expression | assignment_expression | async_block | await_expression | binary_expression | block | break_expression | call_expression | closure_expression | compound_assignment_expr | const_block | continue_expression | field_expression | for_expression | generic_function | identifier | if_expression | index_expression | loop_expression | macro_invocation | match_expression | metavariable | parenthesized_expression | reference_expression | return_expression | scoped_identifier | self | struct_expression | try_expression | tuple_expression | type_cast_expression | unary_expression | unit_expression | unsafe_block | while_expression | yield_expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Literal_ArrayExpression_AssignmentExpression_AsyncBlock_AwaitExpression_BinaryExpression_Block_BreakExpression_CallExpression_ClosureExpression_CompoundAssignmentExpr_ConstBlock_ContinueExpression_FieldExpression_ForExpression_GenericFunction_Identifier_IfExpression_IndexExpression_LoopExpression_MacroInvocation_MatchExpression_Metavariable_ParenthesizedExpression_ReferenceExpression_ReturnExpression_ScopedIdentifier__Self_StructExpression_TryExpression_TupleExpression_TypeCastExpression_UnaryExpression_UnitExpression_UnsafeBlock_WhileExpression_YieldExpression<
        'tree,
    > {
        Literal(Literal<'tree>),
        ArrayExpression(ArrayExpression<'tree>),
        AssignmentExpression(AssignmentExpression<'tree>),
        AsyncBlock(AsyncBlock<'tree>),
        AwaitExpression(AwaitExpression<'tree>),
        BinaryExpression(BinaryExpression<'tree>),
        Block(Block<'tree>),
        BreakExpression(BreakExpression<'tree>),
        CallExpression(CallExpression<'tree>),
        ClosureExpression(ClosureExpression<'tree>),
        CompoundAssignmentExpr(CompoundAssignmentExpr<'tree>),
        ConstBlock(ConstBlock<'tree>),
        ContinueExpression(ContinueExpression<'tree>),
        FieldExpression(FieldExpression<'tree>),
        ForExpression(ForExpression<'tree>),
        GenericFunction(GenericFunction<'tree>),
        Identifier(Identifier<'tree>),
        IfExpression(IfExpression<'tree>),
        IndexExpression(IndexExpression<'tree>),
        LoopExpression(LoopExpression<'tree>),
        MacroInvocation(MacroInvocation<'tree>),
        MatchExpression(MatchExpression<'tree>),
        Metavariable(Metavariable<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        ReferenceExpression(ReferenceExpression<'tree>),
        ReturnExpression(ReturnExpression<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        _Self(_Self<'tree>),
        StructExpression(StructExpression<'tree>),
        TryExpression(TryExpression<'tree>),
        TupleExpression(TupleExpression<'tree>),
        TypeCastExpression(TypeCastExpression<'tree>),
        UnaryExpression(UnaryExpression<'tree>),
        UnitExpression(UnitExpression<'tree>),
        UnsafeBlock(UnsafeBlock<'tree>),
        WhileExpression(WhileExpression<'tree>),
        YieldExpression(YieldExpression<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Literal_ArrayExpression_AssignmentExpression_AsyncBlock_AwaitExpression_BinaryExpression_Block_BreakExpression_CallExpression_ClosureExpression_CompoundAssignmentExpr_ConstBlock_ContinueExpression_FieldExpression_ForExpression_GenericFunction_Identifier_IfExpression_IndexExpression_LoopExpression_MacroInvocation_MatchExpression_Metavariable_ParenthesizedExpression_ReferenceExpression_ReturnExpression_ScopedIdentifier__Self_StructExpression_TryExpression_TupleExpression_TypeCastExpression_UnaryExpression_UnitExpression_UnsafeBlock_WhileExpression_YieldExpression < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "_literal" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn literal (self) -> Option < Literal < 'tree > > { match self { Self :: Literal (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "array_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn array_expression (self) -> Option < ArrayExpression < 'tree > > { match self { Self :: ArrayExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "assignment_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn assignment_expression (self) -> Option < AssignmentExpression < 'tree > > { match self { Self :: AssignmentExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "async_block" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn async_block (self) -> Option < AsyncBlock < 'tree > > { match self { Self :: AsyncBlock (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "await_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn await_expression (self) -> Option < AwaitExpression < 'tree > > { match self { Self :: AwaitExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "binary_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn binary_expression (self) -> Option < BinaryExpression < 'tree > > { match self { Self :: BinaryExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn block (self) -> Option < Block < 'tree > > { match self { Self :: Block (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "break_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn break_expression (self) -> Option < BreakExpression < 'tree > > { match self { Self :: BreakExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "call_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn call_expression (self) -> Option < CallExpression < 'tree > > { match self { Self :: CallExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "closure_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn closure_expression (self) -> Option < ClosureExpression < 'tree > > { match self { Self :: ClosureExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "compound_assignment_expr" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn compound_assignment_expr (self) -> Option < CompoundAssignmentExpr < 'tree > > { match self { Self :: CompoundAssignmentExpr (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "const_block" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn const_block (self) -> Option < ConstBlock < 'tree > > { match self { Self :: ConstBlock (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "continue_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn continue_expression (self) -> Option < ContinueExpression < 'tree > > { match self { Self :: ContinueExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "field_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn field_expression (self) -> Option < FieldExpression < 'tree > > { match self { Self :: FieldExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "for_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn for_expression (self) -> Option < ForExpression < 'tree > > { match self { Self :: ForExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "generic_function" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn generic_function (self) -> Option < GenericFunction < 'tree > > { match self { Self :: GenericFunction (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "if_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn if_expression (self) -> Option < IfExpression < 'tree > > { match self { Self :: IfExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "index_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn index_expression (self) -> Option < IndexExpression < 'tree > > { match self { Self :: IndexExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "loop_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn loop_expression (self) -> Option < LoopExpression < 'tree > > { match self { Self :: LoopExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "macro_invocation" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn macro_invocation (self) -> Option < MacroInvocation < 'tree > > { match self { Self :: MacroInvocation (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "match_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn match_expression (self) -> Option < MatchExpression < 'tree > > { match self { Self :: MatchExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "parenthesized_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn parenthesized_expression (self) -> Option < ParenthesizedExpression < 'tree > > { match self { Self :: ParenthesizedExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "reference_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn reference_expression (self) -> Option < ReferenceExpression < 'tree > > { match self { Self :: ReferenceExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "return_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn return_expression (self) -> Option < ReturnExpression < 'tree > > { match self { Self :: ReturnExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_identifier (self) -> Option < ScopedIdentifier < 'tree > > { match self { Self :: ScopedIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn _self (self) -> Option < _Self < 'tree > > { match self { Self :: _Self (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "struct_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn struct_expression (self) -> Option < StructExpression < 'tree > > { match self { Self :: StructExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "try_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn try_expression (self) -> Option < TryExpression < 'tree > > { match self { Self :: TryExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "tuple_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn tuple_expression (self) -> Option < TupleExpression < 'tree > > { match self { Self :: TupleExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_cast_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_cast_expression (self) -> Option < TypeCastExpression < 'tree > > { match self { Self :: TypeCastExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "unary_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn unary_expression (self) -> Option < UnaryExpression < 'tree > > { match self { Self :: UnaryExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "unit_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn unit_expression (self) -> Option < UnitExpression < 'tree > > { match self { Self :: UnitExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "unsafe_block" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn unsafe_block (self) -> Option < UnsafeBlock < 'tree > > { match self { Self :: UnsafeBlock (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "while_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn while_expression (self) -> Option < WhileExpression < 'tree > > { match self { Self :: WhileExpression (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "yield_expression" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn yield_expression (self) -> Option < YieldExpression < 'tree > > { match self { Self :: YieldExpression (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Literal_ArrayExpression_AssignmentExpression_AsyncBlock_AwaitExpression_BinaryExpression_Block_BreakExpression_CallExpression_ClosureExpression_CompoundAssignmentExpr_ConstBlock_ContinueExpression_FieldExpression_ForExpression_GenericFunction_Identifier_IfExpression_IndexExpression_LoopExpression_MacroInvocation_MatchExpression_Metavariable_ParenthesizedExpression_ReferenceExpression_ReturnExpression_ScopedIdentifier__Self_StructExpression_TryExpression_TupleExpression_TypeCastExpression_UnaryExpression_UnitExpression_UnsafeBlock_WhileExpression_YieldExpression < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < Literal < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Literal (this)) ; } if let Ok (this) = < ArrayExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ArrayExpression (this)) ; } if let Ok (this) = < AssignmentExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: AssignmentExpression (this)) ; } if let Ok (this) = < AsyncBlock < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: AsyncBlock (this)) ; } if let Ok (this) = < AwaitExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: AwaitExpression (this)) ; } if let Ok (this) = < BinaryExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: BinaryExpression (this)) ; } if let Ok (this) = < Block < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Block (this)) ; } if let Ok (this) = < BreakExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: BreakExpression (this)) ; } if let Ok (this) = < CallExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: CallExpression (this)) ; } if let Ok (this) = < ClosureExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ClosureExpression (this)) ; } if let Ok (this) = < CompoundAssignmentExpr < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: CompoundAssignmentExpr (this)) ; } if let Ok (this) = < ConstBlock < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ConstBlock (this)) ; } if let Ok (this) = < ContinueExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ContinueExpression (this)) ; } if let Ok (this) = < FieldExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: FieldExpression (this)) ; } if let Ok (this) = < ForExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ForExpression (this)) ; } if let Ok (this) = < GenericFunction < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: GenericFunction (this)) ; } if let Ok (this) = < Identifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Identifier (this)) ; } if let Ok (this) = < IfExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: IfExpression (this)) ; } if let Ok (this) = < IndexExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: IndexExpression (this)) ; } if let Ok (this) = < LoopExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: LoopExpression (this)) ; } if let Ok (this) = < MacroInvocation < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: MacroInvocation (this)) ; } if let Ok (this) = < MatchExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: MatchExpression (this)) ; } if let Ok (this) = < Metavariable < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Metavariable (this)) ; } if let Ok (this) = < ParenthesizedExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ParenthesizedExpression (this)) ; } if let Ok (this) = < ReferenceExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ReferenceExpression (this)) ; } if let Ok (this) = < ReturnExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ReturnExpression (this)) ; } if let Ok (this) = < ScopedIdentifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ScopedIdentifier (this)) ; } if let Ok (this) = < _Self < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: _Self (this)) ; } if let Ok (this) = < StructExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: StructExpression (this)) ; } if let Ok (this) = < TryExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TryExpression (this)) ; } if let Ok (this) = < TupleExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TupleExpression (this)) ; } if let Ok (this) = < TypeCastExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TypeCastExpression (this)) ; } if let Ok (this) = < UnaryExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: UnaryExpression (this)) ; } if let Ok (this) = < UnitExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: UnitExpression (this)) ; } if let Ok (this) = < UnsafeBlock < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: UnsafeBlock (this)) ; } if let Ok (this) = < WhileExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: WhileExpression (this)) ; } if let Ok (this) = < YieldExpression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: YieldExpression (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Literal_ArrayExpression_AssignmentExpression_AsyncBlock_AwaitExpression_BinaryExpression_Block_BreakExpression_CallExpression_ClosureExpression_CompoundAssignmentExpr_ConstBlock_ContinueExpression_FieldExpression_ForExpression_GenericFunction_Identifier_IfExpression_IndexExpression_LoopExpression_MacroInvocation_MatchExpression_Metavariable_ParenthesizedExpression_ReferenceExpression_ReturnExpression_ScopedIdentifier__Self_StructExpression_TryExpression_TupleExpression_TypeCastExpression_UnaryExpression_UnitExpression_UnsafeBlock_WhileExpression_YieldExpression < 'tree > { const KIND : & 'static str = "{_literal | array_expression | assignment_expression | async_block | await_expression | binary_expression | block | break_expression | call_expression | closure_expression | compound_assignment_expr | const_block | continue_expression | field_expression | for_expression | generic_function | identifier | if_expression | index_expression | loop_expression | macro_invocation | match_expression | metavariable | parenthesized_expression | reference_expression | return_expression | scoped_identifier | self | struct_expression | try_expression | tuple_expression | type_cast_expression | unary_expression | unit_expression | unsafe_block | while_expression | yield_expression}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Literal (x) => x . node () , Self :: ArrayExpression (x) => x . node () , Self :: AssignmentExpression (x) => x . node () , Self :: AsyncBlock (x) => x . node () , Self :: AwaitExpression (x) => x . node () , Self :: BinaryExpression (x) => x . node () , Self :: Block (x) => x . node () , Self :: BreakExpression (x) => x . node () , Self :: CallExpression (x) => x . node () , Self :: ClosureExpression (x) => x . node () , Self :: CompoundAssignmentExpr (x) => x . node () , Self :: ConstBlock (x) => x . node () , Self :: ContinueExpression (x) => x . node () , Self :: FieldExpression (x) => x . node () , Self :: ForExpression (x) => x . node () , Self :: GenericFunction (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: IfExpression (x) => x . node () , Self :: IndexExpression (x) => x . node () , Self :: LoopExpression (x) => x . node () , Self :: MacroInvocation (x) => x . node () , Self :: MatchExpression (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: ParenthesizedExpression (x) => x . node () , Self :: ReferenceExpression (x) => x . node () , Self :: ReturnExpression (x) => x . node () , Self :: ScopedIdentifier (x) => x . node () , Self :: _Self (x) => x . node () , Self :: StructExpression (x) => x . node () , Self :: TryExpression (x) => x . node () , Self :: TupleExpression (x) => x . node () , Self :: TypeCastExpression (x) => x . node () , Self :: UnaryExpression (x) => x . node () , Self :: UnitExpression (x) => x . node () , Self :: UnsafeBlock (x) => x . node () , Self :: WhileExpression (x) => x . node () , Self :: YieldExpression (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Literal (x) => x . node_mut () , Self :: ArrayExpression (x) => x . node_mut () , Self :: AssignmentExpression (x) => x . node_mut () , Self :: AsyncBlock (x) => x . node_mut () , Self :: AwaitExpression (x) => x . node_mut () , Self :: BinaryExpression (x) => x . node_mut () , Self :: Block (x) => x . node_mut () , Self :: BreakExpression (x) => x . node_mut () , Self :: CallExpression (x) => x . node_mut () , Self :: ClosureExpression (x) => x . node_mut () , Self :: CompoundAssignmentExpr (x) => x . node_mut () , Self :: ConstBlock (x) => x . node_mut () , Self :: ContinueExpression (x) => x . node_mut () , Self :: FieldExpression (x) => x . node_mut () , Self :: ForExpression (x) => x . node_mut () , Self :: GenericFunction (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: IfExpression (x) => x . node_mut () , Self :: IndexExpression (x) => x . node_mut () , Self :: LoopExpression (x) => x . node_mut () , Self :: MacroInvocation (x) => x . node_mut () , Self :: MatchExpression (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: ParenthesizedExpression (x) => x . node_mut () , Self :: ReferenceExpression (x) => x . node_mut () , Self :: ReturnExpression (x) => x . node_mut () , Self :: ScopedIdentifier (x) => x . node_mut () , Self :: _Self (x) => x . node_mut () , Self :: StructExpression (x) => x . node_mut () , Self :: TryExpression (x) => x . node_mut () , Self :: TupleExpression (x) => x . node_mut () , Self :: TypeCastExpression (x) => x . node_mut () , Self :: UnaryExpression (x) => x . node_mut () , Self :: UnitExpression (x) => x . node_mut () , Self :: UnsafeBlock (x) => x . node_mut () , Self :: WhileExpression (x) => x . node_mut () , Self :: YieldExpression (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{_pattern | parameter}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern_Parameter<'tree> {
        Pattern(Pattern<'tree>),
        Parameter(Parameter<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Pattern_Parameter<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "parameter" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn parameter(self) -> Option<Parameter<'tree>> {
            match self {
                Self::Parameter(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern_Parameter<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <Parameter<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Parameter(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern_Parameter<'tree> {
        const KIND: &'static str = "{_pattern | parameter}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node(),
                Self::Parameter(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node_mut(),
                Self::Parameter(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{%= | &= | *= | += | -= | /= | <<= | >>= | ^= | |=}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ModEq_AndEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree> {
        ModEq(symbols::ModEq<'tree>),
        AndEq(symbols::AndEq<'tree>),
        MulEq(symbols::MulEq<'tree>),
        AddEq(symbols::AddEq<'tree>),
        SubEq(symbols::SubEq<'tree>),
        DivEq(symbols::DivEq<'tree>),
        LtLtEq(symbols::LtLtEq<'tree>),
        GtGtEq(symbols::GtGtEq<'tree>),
        BitXorEq(symbols::BitXorEq<'tree>),
        OrEq(symbols::OrEq<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ModEq_AndEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "%=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mod_eq(self) -> Option<symbols::ModEq<'tree>> {
            match self {
                Self::ModEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "&=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and_eq(self) -> Option<symbols::AndEq<'tree>> {
            match self {
                Self::AndEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "*=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mul_eq(self) -> Option<symbols::MulEq<'tree>> {
            match self {
                Self::MulEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "+=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add_eq(self) -> Option<symbols::AddEq<'tree>> {
            match self {
                Self::AddEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "-=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub_eq(self) -> Option<symbols::SubEq<'tree>> {
            match self {
                Self::SubEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "/=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn div_eq(self) -> Option<symbols::DivEq<'tree>> {
            match self {
                Self::DivEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "<<=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_lt_eq(self) -> Option<symbols::LtLtEq<'tree>> {
            match self {
                Self::LtLtEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , ">>=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt_gt_eq(self) -> Option<symbols::GtGtEq<'tree>> {
            match self {
                Self::GtGtEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "^=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn bit_xor_eq(self) -> Option<symbols::BitXorEq<'tree>> {
            match self {
                Self::BitXorEq(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "|=" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn or_eq(self) -> Option<symbols::OrEq<'tree>> {
            match self {
                Self::OrEq(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for ModEq_AndEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "%=" => Ok(unsafe {
                    Self::ModEq(<symbols::ModEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "&=" => Ok(unsafe {
                    Self::AndEq(<symbols::AndEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "*=" => Ok(unsafe {
                    Self::MulEq(<symbols::MulEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "+=" => Ok(unsafe {
                    Self::AddEq(<symbols::AddEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "-=" => Ok(unsafe {
                    Self::SubEq(<symbols::SubEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "/=" => Ok(unsafe {
                    Self::DivEq(<symbols::DivEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "<<=" => Ok(unsafe {
                    Self::LtLtEq(<symbols::LtLtEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ">>=" => Ok(unsafe {
                    Self::GtGtEq(<symbols::GtGtEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "^=" => Ok(unsafe {
                    Self::BitXorEq(<symbols::BitXorEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "|=" => Ok(unsafe {
                    Self::OrEq(<symbols::OrEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ModEq_AndEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree>
    {
        const KIND: &'static str = "{%= | &= | *= | += | -= | /= | <<= | >>= | ^= | |=}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => x.node(),
                Self::AndEq(x) => x.node(),
                Self::MulEq(x) => x.node(),
                Self::AddEq(x) => x.node(),
                Self::SubEq(x) => x.node(),
                Self::DivEq(x) => x.node(),
                Self::LtLtEq(x) => x.node(),
                Self::GtGtEq(x) => x.node(),
                Self::BitXorEq(x) => x.node(),
                Self::OrEq(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => x.node_mut(),
                Self::AndEq(x) => x.node_mut(),
                Self::MulEq(x) => x.node_mut(),
                Self::AddEq(x) => x.node_mut(),
                Self::SubEq(x) => x.node_mut(),
                Self::DivEq(x) => x.node_mut(),
                Self::LtLtEq(x) => x.node_mut(),
                Self::GtGtEq(x) => x.node_mut(),
                Self::BitXorEq(x) => x.node_mut(),
                Self::OrEq(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | identifier | _type | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_Identifier_Type_Expression<'tree> {
        VisibilityModifier(VisibilityModifier<'tree>),
        Identifier(Identifier<'tree>),
        Type(Type<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> VisibilityModifier_Identifier_Type_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for VisibilityModifier_Identifier_Type_Expression<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <VisibilityModifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VisibilityModifier(this));
            }
            if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for VisibilityModifier_Identifier_Type_Expression<'tree>
    {
        const KIND: &'static str = "{visibility_modifier | identifier | _type | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{lifetime | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Lifetime_TypeIdentifier<'tree> {
        Lifetime(Lifetime<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Lifetime_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lifetime(self) -> Option<Lifetime<'tree>> {
            match self {
                Self::Lifetime(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lifetime_TypeIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "lifetime" => {
                    Ok(unsafe {
                        Self :: Lifetime (< Lifetime < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lifetime_TypeIdentifier<'tree> {
        const KIND: &'static str = "{lifetime | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Lifetime(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Lifetime(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{block | if_expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Block_IfExpression<'tree> {
        Block(Block<'tree>),
        IfExpression(IfExpression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Block_IfExpression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "if_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn if_expression(self) -> Option<IfExpression<'tree>> {
            match self {
                Self::IfExpression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Block_IfExpression<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "block" => {
                    Ok(unsafe {
                        Self :: Block (< Block < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "if_expression" => Ok(unsafe {
                    Self :: IfExpression (< IfExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Block_IfExpression<'tree> {
        const KIND: &'static str = "{block | if_expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Block(x) => x.node(),
                Self::IfExpression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Block(x) => x.node_mut(),
                Self::IfExpression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | where_clause | enum_variant_list | type_identifier | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters<'tree> {
        VisibilityModifier(VisibilityModifier<'tree>),
        WhereClause(WhereClause<'tree>),
        EnumVariantList(EnumVariantList<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl<'tree> VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn where_clause(self) -> Option<WhereClause<'tree>> {
            match self {
                Self::WhereClause(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "enum_variant_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn enum_variant_list(self) -> Option<EnumVariantList<'tree>> {
            match self {
                Self::EnumVariantList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_parameters(self) -> Option<TypeParameters<'tree>> {
            match self {
                Self::TypeParameters(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "visibility_modifier" => Ok(unsafe {
                    Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "where_clause" => Ok(unsafe {
                    Self :: WhereClause (< WhereClause < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "enum_variant_list" => {
                    Ok(unsafe {
                        Self :: EnumVariantList (< EnumVariantList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "type_parameters" => {
                    Ok(unsafe {
                        Self :: TypeParameters (< TypeParameters < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for VisibilityModifier_WhereClause_EnumVariantList_TypeIdentifier_TypeParameters<'tree>
    {
        const KIND : & 'static str = "{visibility_modifier | where_clause | enum_variant_list | type_identifier | type_parameters}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::WhereClause(x) => x.node(),
                Self::EnumVariantList(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
                Self::TypeParameters(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::WhereClause(x) => x.node_mut(),
                Self::EnumVariantList(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
                Self::TypeParameters(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | field_declaration_list | ordered_field_declaration_list | identifier | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression<
        'tree,
    > {
        VisibilityModifier(VisibilityModifier<'tree>),
        FieldDeclarationList(FieldDeclarationList<'tree>),
        OrderedFieldDeclarationList(OrderedFieldDeclarationList<'tree>),
        Identifier(Identifier<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression<
            'tree,
        >
    {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_declaration_list(self) -> Option<FieldDeclarationList<'tree>> {
            match self {
                Self::FieldDeclarationList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "ordered_field_declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn ordered_field_declaration_list(self) -> Option<OrderedFieldDeclarationList<'tree>> {
            match self {
                Self::OrderedFieldDeclarationList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < VisibilityModifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: VisibilityModifier (this)) ; } if let Ok (this) = < FieldDeclarationList < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: FieldDeclarationList (this)) ; } if let Ok (this) = < OrderedFieldDeclarationList < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: OrderedFieldDeclarationList (this)) ; } if let Ok (this) = < Identifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Identifier (this)) ; } if let Ok (this) = < Expression < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Expression (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for VisibilityModifier_FieldDeclarationList_OrderedFieldDeclarationList_Identifier_Expression < 'tree > { const KIND : & 'static str = "{visibility_modifier | field_declaration_list | ordered_field_declaration_list | identifier | _expression}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: VisibilityModifier (x) => x . node () , Self :: FieldDeclarationList (x) => x . node () , Self :: OrderedFieldDeclarationList (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Expression (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: VisibilityModifier (x) => x . node_mut () , Self :: FieldDeclarationList (x) => x . node_mut () , Self :: OrderedFieldDeclarationList (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Expression (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{field_declaration_list | ordered_field_declaration_list}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FieldDeclarationList_OrderedFieldDeclarationList<'tree> {
        FieldDeclarationList(FieldDeclarationList<'tree>),
        OrderedFieldDeclarationList(OrderedFieldDeclarationList<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FieldDeclarationList_OrderedFieldDeclarationList<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_declaration_list(self) -> Option<FieldDeclarationList<'tree>> {
            match self {
                Self::FieldDeclarationList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "ordered_field_declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn ordered_field_declaration_list(self) -> Option<OrderedFieldDeclarationList<'tree>> {
            match self {
                Self::OrderedFieldDeclarationList(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for FieldDeclarationList_OrderedFieldDeclarationList<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "field_declaration_list" => Ok(unsafe {
                    Self :: FieldDeclarationList (< FieldDeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "ordered_field_declaration_list" => Ok(unsafe {
                    Self :: OrderedFieldDeclarationList (< OrderedFieldDeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for FieldDeclarationList_OrderedFieldDeclarationList<'tree>
    {
        const KIND: &'static str = "{field_declaration_list | ordered_field_declaration_list}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FieldDeclarationList(x) => x.node(),
                Self::OrderedFieldDeclarationList(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FieldDeclarationList(x) => x.node_mut(),
                Self::OrderedFieldDeclarationList(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{attribute_item | enum_variant}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeItem_EnumVariant<'tree> {
        AttributeItem(AttributeItem<'tree>),
        EnumVariant(EnumVariant<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AttributeItem_EnumVariant<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "enum_variant" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn enum_variant(self) -> Option<EnumVariant<'tree>> {
            match self {
                Self::EnumVariant(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem_EnumVariant<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "attribute_item" => Ok(unsafe {
                    Self::AttributeItem(<AttributeItem<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "enum_variant" => Ok(unsafe {
                    Self :: EnumVariant (< EnumVariant < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem_EnumVariant<'tree> {
        const KIND: &'static str = "{attribute_item | enum_variant}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::EnumVariant(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::EnumVariant(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{crate | visibility_modifier | identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Crate_VisibilityModifier_Identifier<'tree> {
        Crate(Crate<'tree>),
        VisibilityModifier(VisibilityModifier<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Crate_VisibilityModifier_Identifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn crate_(self) -> Option<Crate<'tree>> {
            match self {
                Self::Crate(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Crate_VisibilityModifier_Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "crate" => {
                    Ok(unsafe {
                        Self :: Crate (< Crate < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "visibility_modifier" => Ok(unsafe {
                    Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Crate_VisibilityModifier_Identifier<'tree> {
        const KIND: &'static str = "{crate | visibility_modifier | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Crate(x) => x.node(),
                Self::VisibilityModifier(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Crate(x) => x.node_mut(),
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | field_identifier | _type}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_FieldIdentifier_Type<'tree> {
        VisibilityModifier(VisibilityModifier<'tree>),
        FieldIdentifier(FieldIdentifier<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> VisibilityModifier_FieldIdentifier_Type<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            match self {
                Self::FieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for VisibilityModifier_FieldIdentifier_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <VisibilityModifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VisibilityModifier(this));
            }
            if let Ok(this) = <FieldIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::FieldIdentifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for VisibilityModifier_FieldIdentifier_Type<'tree> {
        const KIND: &'static str = "{visibility_modifier | field_identifier | _type}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::FieldIdentifier(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::FieldIdentifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{attribute_item | field_declaration}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeItem_FieldDeclaration<'tree> {
        AttributeItem(AttributeItem<'tree>),
        FieldDeclaration(FieldDeclaration<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AttributeItem_FieldDeclaration<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_declaration" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_declaration(self) -> Option<FieldDeclaration<'tree>> {
            match self {
                Self::FieldDeclaration(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem_FieldDeclaration<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "attribute_item" => Ok(unsafe {
                    Self::AttributeItem(<AttributeItem<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "field_declaration" => Ok(unsafe {
                    Self :: FieldDeclaration (< FieldDeclaration < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem_FieldDeclaration<'tree> {
        const KIND: &'static str = "{attribute_item | field_declaration}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::FieldDeclaration(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::FieldDeclaration(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{field_identifier | integer_literal}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FieldIdentifier_IntegerLiteral<'tree> {
        FieldIdentifier(FieldIdentifier<'tree>),
        IntegerLiteral(IntegerLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FieldIdentifier_IntegerLiteral<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            match self {
                Self::FieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "integer_literal" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
            match self {
                Self::IntegerLiteral(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldIdentifier_IntegerLiteral<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "field_identifier" => {
                    Ok(unsafe {
                        Self :: FieldIdentifier (< FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "integer_literal" => {
                    Ok(unsafe {
                        Self :: IntegerLiteral (< IntegerLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldIdentifier_IntegerLiteral<'tree> {
        const KIND: &'static str = "{field_identifier | integer_literal}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.node(),
                Self::IntegerLiteral(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.node_mut(),
                Self::IntegerLiteral(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{attribute_item | field_identifier | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeItem_FieldIdentifier_Expression<'tree> {
        AttributeItem(AttributeItem<'tree>),
        FieldIdentifier(FieldIdentifier<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AttributeItem_FieldIdentifier_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            match self {
                Self::FieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem_FieldIdentifier_Expression<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <AttributeItem<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::AttributeItem(this));
            }
            if let Ok(this) = <FieldIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::FieldIdentifier(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem_FieldIdentifier_Expression<'tree> {
        const KIND: &'static str = "{attribute_item | field_identifier | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::FieldIdentifier(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::FieldIdentifier(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{base_field_initializer | field_initializer | shorthand_field_initializer}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer<'tree> {
        BaseFieldInitializer(BaseFieldInitializer<'tree>),
        FieldInitializer(FieldInitializer<'tree>),
        ShorthandFieldInitializer(ShorthandFieldInitializer<'tree>),
    }
    #[automatically_derived]
    impl<'tree> BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "base_field_initializer" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn base_field_initializer(self) -> Option<BaseFieldInitializer<'tree>> {
            match self {
                Self::BaseFieldInitializer(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_initializer" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_initializer(self) -> Option<FieldInitializer<'tree>> {
            match self {
                Self::FieldInitializer(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "shorthand_field_initializer" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn shorthand_field_initializer(self) -> Option<ShorthandFieldInitializer<'tree>> {
            match self {
                Self::ShorthandFieldInitializer(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "base_field_initializer" => Ok(unsafe {
                    Self :: BaseFieldInitializer (< BaseFieldInitializer < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "field_initializer" => Ok(unsafe {
                    Self :: FieldInitializer (< FieldInitializer < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "shorthand_field_initializer" => Ok(unsafe {
                    Self :: ShorthandFieldInitializer (< ShorthandFieldInitializer < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for BaseFieldInitializer_FieldInitializer_ShorthandFieldInitializer<'tree>
    {
        const KIND: &'static str =
            "{base_field_initializer | field_initializer | shorthand_field_initializer}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::BaseFieldInitializer(x) => x.node(),
                Self::FieldInitializer(x) => x.node(),
                Self::ShorthandFieldInitializer(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::BaseFieldInitializer(x) => x.node_mut(),
                Self::FieldInitializer(x) => x.node_mut(),
                Self::ShorthandFieldInitializer(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{mutable_specifier | field_identifier | shorthand_field_identifier | _pattern}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern<'tree> {
        MutableSpecifier(MutableSpecifier<'tree>),
        FieldIdentifier(FieldIdentifier<'tree>),
        ShorthandFieldIdentifier(ShorthandFieldIdentifier<'tree>),
        Pattern(Pattern<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            match self {
                Self::FieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "shorthand_field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn shorthand_field_identifier(self) -> Option<ShorthandFieldIdentifier<'tree>> {
            match self {
                Self::ShorthandFieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <FieldIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::FieldIdentifier(this));
            }
            if let Ok(this) = <ShorthandFieldIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ShorthandFieldIdentifier(this));
            }
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for MutableSpecifier_FieldIdentifier_ShorthandFieldIdentifier_Pattern<'tree>
    {
        const KIND: &'static str =
            "{mutable_specifier | field_identifier | shorthand_field_identifier | _pattern}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node(),
                Self::FieldIdentifier(x) => x.node(),
                Self::ShorthandFieldIdentifier(x) => x.node(),
                Self::Pattern(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::FieldIdentifier(x) => x.node_mut(),
                Self::ShorthandFieldIdentifier(x) => x.node_mut(),
                Self::Pattern(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{field_identifier | shorthand_field_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FieldIdentifier_ShorthandFieldIdentifier<'tree> {
        FieldIdentifier(FieldIdentifier<'tree>),
        ShorthandFieldIdentifier(ShorthandFieldIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FieldIdentifier_ShorthandFieldIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            match self {
                Self::FieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "shorthand_field_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn shorthand_field_identifier(self) -> Option<ShorthandFieldIdentifier<'tree>> {
            match self {
                Self::ShorthandFieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldIdentifier_ShorthandFieldIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "field_identifier" => {
                    Ok(unsafe {
                        Self :: FieldIdentifier (< FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "shorthand_field_identifier" => Ok(unsafe {
                    Self :: ShorthandFieldIdentifier (< ShorthandFieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldIdentifier_ShorthandFieldIdentifier<'tree> {
        const KIND: &'static str = "{field_identifier | shorthand_field_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.node(),
                Self::ShorthandFieldIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.node_mut(),
                Self::ShorthandFieldIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{loop_label | block | _pattern | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LoopLabel_Block_Pattern_Expression<'tree> {
        LoopLabel(LoopLabel<'tree>),
        Block(Block<'tree>),
        Pattern(Pattern<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LoopLabel_Block_Pattern_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "loop_label" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn loop_label(self) -> Option<LoopLabel<'tree>> {
            match self {
                Self::LoopLabel(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LoopLabel_Block_Pattern_Expression<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <LoopLabel<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LoopLabel(this));
            }
            if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Block(this));
            }
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LoopLabel_Block_Pattern_Expression<'tree> {
        const KIND: &'static str = "{loop_label | block | _pattern | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LoopLabel(x) => x.node(),
                Self::Block(x) => x.node(),
                Self::Pattern(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LoopLabel(x) => x.node_mut(),
                Self::Block(x) => x.node_mut(),
                Self::Pattern(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{extern_modifier | visibility_modifier | declaration_list}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ExternModifier_VisibilityModifier_DeclarationList<'tree> {
        ExternModifier(ExternModifier<'tree>),
        VisibilityModifier(VisibilityModifier<'tree>),
        DeclarationList(DeclarationList<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ExternModifier_VisibilityModifier_DeclarationList<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "extern_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn extern_modifier(self) -> Option<ExternModifier<'tree>> {
            match self {
                Self::ExternModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn declaration_list(self) -> Option<DeclarationList<'tree>> {
            match self {
                Self::DeclarationList(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for ExternModifier_VisibilityModifier_DeclarationList<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "extern_modifier" => {
                    Ok(unsafe {
                        Self :: ExternModifier (< ExternModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "visibility_modifier" => Ok(unsafe {
                    Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "declaration_list" => {
                    Ok(unsafe {
                        Self :: DeclarationList (< DeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ExternModifier_VisibilityModifier_DeclarationList<'tree>
    {
        const KIND: &'static str = "{extern_modifier | visibility_modifier | declaration_list}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ExternModifier(x) => x.node(),
                Self::VisibilityModifier(x) => x.node(),
                Self::DeclarationList(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ExternModifier(x) => x.node_mut(),
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::DeclarationList(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{function_modifiers | visibility_modifier | where_clause | block | identifier | metavariable | parameters | _type | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters<
        'tree,
    > {
        FunctionModifiers(FunctionModifiers<'tree>),
        VisibilityModifier(VisibilityModifier<'tree>),
        WhereClause(WhereClause<'tree>),
        Block(Block<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        Parameters(Parameters<'tree>),
        Type(Type<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "function_modifiers" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn function_modifiers (self) -> Option < FunctionModifiers < 'tree > > { match self { Self :: FunctionModifiers (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn visibility_modifier (self) -> Option < VisibilityModifier < 'tree > > { match self { Self :: VisibilityModifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn where_clause (self) -> Option < WhereClause < 'tree > > { match self { Self :: WhereClause (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn block (self) -> Option < Block < 'tree > > { match self { Self :: Block (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "parameters" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn parameters (self) -> Option < Parameters < 'tree > > { match self { Self :: Parameters (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn r#type (self) -> Option < Type < 'tree > > { match self { Self :: Type (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_parameters (self) -> Option < TypeParameters < 'tree > > { match self { Self :: TypeParameters (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < FunctionModifiers < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: FunctionModifiers (this)) ; } if let Ok (this) = < VisibilityModifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: VisibilityModifier (this)) ; } if let Ok (this) = < WhereClause < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: WhereClause (this)) ; } if let Ok (this) = < Block < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Block (this)) ; } if let Ok (this) = < Identifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Identifier (this)) ; } if let Ok (this) = < Metavariable < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Metavariable (this)) ; } if let Ok (this) = < Parameters < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Parameters (this)) ; } if let Ok (this) = < Type < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Type (this)) ; } if let Ok (this) = < TypeParameters < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TypeParameters (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for FunctionModifiers_VisibilityModifier_WhereClause_Block_Identifier_Metavariable_Parameters_Type_TypeParameters < 'tree > { const KIND : & 'static str = "{function_modifiers | visibility_modifier | where_clause | block | identifier | metavariable | parameters | _type | type_parameters}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: FunctionModifiers (x) => x . node () , Self :: VisibilityModifier (x) => x . node () , Self :: WhereClause (x) => x . node () , Self :: Block (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: Parameters (x) => x . node () , Self :: Type (x) => x . node () , Self :: TypeParameters (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: FunctionModifiers (x) => x . node_mut () , Self :: VisibilityModifier (x) => x . node_mut () , Self :: WhereClause (x) => x . node_mut () , Self :: Block (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: Parameters (x) => x . node_mut () , Self :: Type (x) => x . node_mut () , Self :: TypeParameters (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{identifier | metavariable}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_Metavariable<'tree> {
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Identifier_Metavariable<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn metavariable(self) -> Option<Metavariable<'tree>> {
            match self {
                Self::Metavariable(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier_Metavariable<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "metavariable" => Ok(unsafe {
                    Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier_Metavariable<'tree> {
        const KIND: &'static str = "{identifier | metavariable}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node(),
                Self::Metavariable(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node_mut(),
                Self::Metavariable(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{function_modifiers | visibility_modifier | where_clause | identifier | metavariable | parameters | _type | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters<
        'tree,
    > {
        FunctionModifiers(FunctionModifiers<'tree>),
        VisibilityModifier(VisibilityModifier<'tree>),
        WhereClause(WhereClause<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        Parameters(Parameters<'tree>),
        Type(Type<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "function_modifiers" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn function_modifiers (self) -> Option < FunctionModifiers < 'tree > > { match self { Self :: FunctionModifiers (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn visibility_modifier (self) -> Option < VisibilityModifier < 'tree > > { match self { Self :: VisibilityModifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn where_clause (self) -> Option < WhereClause < 'tree > > { match self { Self :: WhereClause (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "parameters" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn parameters (self) -> Option < Parameters < 'tree > > { match self { Self :: Parameters (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn r#type (self) -> Option < Type < 'tree > > { match self { Self :: Type (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_parameters (self) -> Option < TypeParameters < 'tree > > { match self { Self :: TypeParameters (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < FunctionModifiers < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: FunctionModifiers (this)) ; } if let Ok (this) = < VisibilityModifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: VisibilityModifier (this)) ; } if let Ok (this) = < WhereClause < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: WhereClause (this)) ; } if let Ok (this) = < Identifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Identifier (this)) ; } if let Ok (this) = < Metavariable < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Metavariable (this)) ; } if let Ok (this) = < Parameters < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Parameters (this)) ; } if let Ok (this) = < Type < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Type (this)) ; } if let Ok (this) = < TypeParameters < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TypeParameters (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for FunctionModifiers_VisibilityModifier_WhereClause_Identifier_Metavariable_Parameters_Type_TypeParameters < 'tree > { const KIND : & 'static str = "{function_modifiers | visibility_modifier | where_clause | identifier | metavariable | parameters | _type | type_parameters}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: FunctionModifiers (x) => x . node () , Self :: VisibilityModifier (x) => x . node () , Self :: WhereClause (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: Parameters (x) => x . node () , Self :: Type (x) => x . node () , Self :: TypeParameters (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: FunctionModifiers (x) => x . node_mut () , Self :: VisibilityModifier (x) => x . node_mut () , Self :: WhereClause (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: Parameters (x) => x . node_mut () , Self :: Type (x) => x . node_mut () , Self :: TypeParameters (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{for_lifetimes | function_modifiers | parameters | _type | scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier<
        'tree,
    > {
        ForLifetimes(ForLifetimes<'tree>),
        FunctionModifiers(FunctionModifiers<'tree>),
        Parameters(Parameters<'tree>),
        Type(Type<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        # [doc = concat ! ("Returns the node if it is of kind `" , "for_lifetimes" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn for_lifetimes(self) -> Option<ForLifetimes<'tree>> {
            match self {
                Self::ForLifetimes(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "function_modifiers" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn function_modifiers(self) -> Option<FunctionModifiers<'tree>> {
            match self {
                Self::FunctionModifiers(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "parameters" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn parameters(self) -> Option<Parameters<'tree>> {
            match self {
                Self::Parameters(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier<
            'tree,
        >
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <ForLifetimes<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ForLifetimes(this));
            }
            if let Ok(this) = <FunctionModifiers<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::FunctionModifiers(this));
            }
            if let Ok(this) = <Parameters<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Parameters(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <ScopedTypeIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ScopedTypeIdentifier(this));
            }
            if let Ok(this) = <TypeIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TypeIdentifier(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ForLifetimes_FunctionModifiers_Parameters_Type_ScopedTypeIdentifier_TypeIdentifier<
            'tree,
        >
    {
        const KIND : & 'static str = "{for_lifetimes | function_modifiers | parameters | _type | scoped_type_identifier | type_identifier}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ForLifetimes(x) => x.node(),
                Self::FunctionModifiers(x) => x.node(),
                Self::Parameters(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ForLifetimes(x) => x.node_mut(),
                Self::FunctionModifiers(x) => x.node_mut(),
                Self::Parameters(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ScopedTypeIdentifier_TypeIdentifier<'tree> {
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ScopedTypeIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedTypeIdentifier_TypeIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "scoped_type_identifier" => Ok(unsafe {
                    Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedTypeIdentifier_TypeIdentifier<'tree> {
        const KIND: &'static str = "{scoped_type_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{field_expression | identifier | scoped_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FieldExpression_Identifier_ScopedIdentifier<'tree> {
        FieldExpression(FieldExpression<'tree>),
        Identifier(Identifier<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FieldExpression_Identifier_ScopedIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_expression(self) -> Option<FieldExpression<'tree>> {
            match self {
                Self::FieldExpression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for FieldExpression_Identifier_ScopedIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "field_expression" => {
                    Ok(unsafe {
                        Self :: FieldExpression (< FieldExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for FieldExpression_Identifier_ScopedIdentifier<'tree>
    {
        const KIND: &'static str = "{field_expression | identifier | scoped_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FieldExpression(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FieldExpression(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{scoped_identifier | scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ScopedIdentifier_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        ScopedIdentifier(ScopedIdentifier<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ScopedIdentifier_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for ScopedIdentifier_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_type_identifier" => Ok(unsafe {
                    Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ScopedIdentifier_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        const KIND: &'static str = "{scoped_identifier | scoped_type_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ScopedIdentifier(x) => x.node(),
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ScopedIdentifier(x) => x.node_mut(),
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{scoped_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ScopedIdentifier_TypeIdentifier<'tree> {
        ScopedIdentifier(ScopedIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ScopedIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedIdentifier_TypeIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedIdentifier_TypeIdentifier<'tree> {
        const KIND: &'static str = "{scoped_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ScopedIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ScopedIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_expression | let_chain | let_condition}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_LetChain_LetCondition<'tree> {
        Expression(Expression<'tree>),
        LetChain(LetChain<'tree>),
        LetCondition(LetCondition<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_LetChain_LetCondition<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_chain" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_chain(self) -> Option<LetChain<'tree>> {
            match self {
                Self::LetChain(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_condition" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_condition(self) -> Option<LetCondition<'tree>> {
            match self {
                Self::LetCondition(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expression_LetChain_LetCondition<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <LetChain<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetChain(this));
            }
            if let Ok(this) = <LetCondition<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetCondition(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_LetChain_LetCondition<'tree> {
        const KIND: &'static str = "{_expression | let_chain | let_condition}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::LetChain(x) => x.node(),
                Self::LetCondition(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::LetChain(x) => x.node_mut(),
                Self::LetCondition(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{where_clause | declaration_list | generic_type | scoped_type_identifier | type_identifier | _type | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters<
        'tree,
    > {
        WhereClause(WhereClause<'tree>),
        DeclarationList(DeclarationList<'tree>),
        GenericType(GenericType<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
        Type(Type<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn where_clause (self) -> Option < WhereClause < 'tree > > { match self { Self :: WhereClause (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "declaration_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn declaration_list (self) -> Option < DeclarationList < 'tree > > { match self { Self :: DeclarationList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn generic_type (self) -> Option < GenericType < 'tree > > { match self { Self :: GenericType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_type_identifier (self) -> Option < ScopedTypeIdentifier < 'tree > > { match self { Self :: ScopedTypeIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_identifier (self) -> Option < TypeIdentifier < 'tree > > { match self { Self :: TypeIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn r#type (self) -> Option < Type < 'tree > > { match self { Self :: Type (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_parameters (self) -> Option < TypeParameters < 'tree > > { match self { Self :: TypeParameters (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < WhereClause < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: WhereClause (this)) ; } if let Ok (this) = < DeclarationList < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: DeclarationList (this)) ; } if let Ok (this) = < GenericType < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: GenericType (this)) ; } if let Ok (this) = < ScopedTypeIdentifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: ScopedTypeIdentifier (this)) ; } if let Ok (this) = < TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TypeIdentifier (this)) ; } if let Ok (this) = < Type < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Type (this)) ; } if let Ok (this) = < TypeParameters < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TypeParameters (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for WhereClause_DeclarationList_GenericType_ScopedTypeIdentifier_TypeIdentifier_Type_TypeParameters < 'tree > { const KIND : & 'static str = "{where_clause | declaration_list | generic_type | scoped_type_identifier | type_identifier | _type | type_parameters}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: WhereClause (x) => x . node () , Self :: DeclarationList (x) => x . node () , Self :: GenericType (x) => x . node () , Self :: ScopedTypeIdentifier (x) => x . node () , Self :: TypeIdentifier (x) => x . node () , Self :: Type (x) => x . node () , Self :: TypeParameters (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: WhereClause (x) => x . node_mut () , Self :: DeclarationList (x) => x . node_mut () , Self :: GenericType (x) => x . node_mut () , Self :: ScopedTypeIdentifier (x) => x . node_mut () , Self :: TypeIdentifier (x) => x . node_mut () , Self :: Type (x) => x . node_mut () , Self :: TypeParameters (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{generic_type | scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        GenericType(GenericType<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn generic_type(self) -> Option<GenericType<'tree>> {
            match self {
                Self::GenericType(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "generic_type" => Ok(unsafe {
                    Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_type_identifier" => Ok(unsafe {
                    Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for GenericType_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        const KIND: &'static str = "{generic_type | scoped_type_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::GenericType(x) => x.node(),
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::GenericType(x) => x.node_mut(),
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_expression | let_condition}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_LetCondition<'tree> {
        Expression(Expression<'tree>),
        LetCondition(LetCondition<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_LetCondition<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_condition" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_condition(self) -> Option<LetCondition<'tree>> {
            match self {
                Self::LetCondition(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expression_LetCondition<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <LetCondition<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetCondition(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_LetCondition<'tree> {
        const KIND: &'static str = "{_expression | let_condition}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::LetCondition(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::LetCondition(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{mutable_specifier | block | _pattern | _type | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MutableSpecifier_Block_Pattern_Type_Expression<'tree> {
        MutableSpecifier(MutableSpecifier<'tree>),
        Block(Block<'tree>),
        Pattern(Pattern<'tree>),
        Type(Type<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MutableSpecifier_Block_Pattern_Type_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for MutableSpecifier_Block_Pattern_Type_Expression<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Block(this));
            }
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for MutableSpecifier_Block_Pattern_Type_Expression<'tree>
    {
        const KIND: &'static str = "{mutable_specifier | block | _pattern | _type | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node(),
                Self::Block(x) => x.node(),
                Self::Pattern(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::Block(x) => x.node_mut(),
                Self::Pattern(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{loop_label | block}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LoopLabel_Block<'tree> {
        LoopLabel(LoopLabel<'tree>),
        Block(Block<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LoopLabel_Block<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "loop_label" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn loop_label(self) -> Option<LoopLabel<'tree>> {
            match self {
                Self::LoopLabel(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LoopLabel_Block<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "loop_label" => Ok(unsafe {
                    Self :: LoopLabel (< LoopLabel < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "block" => {
                    Ok(unsafe {
                        Self :: Block (< Block < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LoopLabel_Block<'tree> {
        const KIND: &'static str = "{loop_label | block}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LoopLabel(x) => x.node(),
                Self::Block(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LoopLabel(x) => x.node_mut(),
                Self::Block(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{macro_rule | identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MacroRule_Identifier<'tree> {
        MacroRule(MacroRule<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MacroRule_Identifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "macro_rule" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn macro_rule(self) -> Option<MacroRule<'tree>> {
            match self {
                Self::MacroRule(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroRule_Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "macro_rule" => Ok(unsafe {
                    Self :: MacroRule (< MacroRule < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroRule_Identifier<'tree> {
        const KIND: &'static str = "{macro_rule | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MacroRule(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MacroRule(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{token_tree | identifier | scoped_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum TokenTree_Identifier_ScopedIdentifier<'tree> {
        TokenTree(TokenTree<'tree>),
        Identifier(Identifier<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> TokenTree_Identifier_ScopedIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "token_tree" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn token_tree(self) -> Option<TokenTree<'tree>> {
            match self {
                Self::TokenTree(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenTree_Identifier_ScopedIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "token_tree" => Ok(unsafe {
                    Self :: TokenTree (< TokenTree < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenTree_Identifier_ScopedIdentifier<'tree> {
        const KIND: &'static str = "{token_tree | identifier | scoped_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::TokenTree(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::TokenTree(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{identifier | scoped_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_ScopedIdentifier<'tree> {
        Identifier(Identifier<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Identifier_ScopedIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier_ScopedIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier_ScopedIdentifier<'tree> {
        const KIND: &'static str = "{identifier | scoped_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{attribute_item | match_pattern | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeItem_MatchPattern_Expression<'tree> {
        AttributeItem(AttributeItem<'tree>),
        MatchPattern(MatchPattern<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AttributeItem_MatchPattern_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "match_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn match_pattern(self) -> Option<MatchPattern<'tree>> {
            match self {
                Self::MatchPattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem_MatchPattern_Expression<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <AttributeItem<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::AttributeItem(this));
            }
            if let Ok(this) = <MatchPattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MatchPattern(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem_MatchPattern_Expression<'tree> {
        const KIND: &'static str = "{attribute_item | match_pattern | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::MatchPattern(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::MatchPattern(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_pattern | _expression | let_chain | let_condition}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern_Expression_LetChain_LetCondition<'tree> {
        Pattern(Pattern<'tree>),
        Expression(Expression<'tree>),
        LetChain(LetChain<'tree>),
        LetCondition(LetCondition<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Pattern_Expression_LetChain_LetCondition<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_chain" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_chain(self) -> Option<LetChain<'tree>> {
            match self {
                Self::LetChain(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_condition" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_condition(self) -> Option<LetCondition<'tree>> {
            match self {
                Self::LetCondition(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern_Expression_LetChain_LetCondition<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <LetChain<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetChain(this));
            }
            if let Ok(this) = <LetCondition<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetCondition(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern_Expression_LetChain_LetCondition<'tree> {
        const KIND: &'static str = "{_pattern | _expression | let_chain | let_condition}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node(),
                Self::Expression(x) => x.node(),
                Self::LetChain(x) => x.node(),
                Self::LetCondition(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
                Self::LetChain(x) => x.node_mut(),
                Self::LetCondition(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | declaration_list | identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_DeclarationList_Identifier<'tree> {
        VisibilityModifier(VisibilityModifier<'tree>),
        DeclarationList(DeclarationList<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> VisibilityModifier_DeclarationList_Identifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn declaration_list(self) -> Option<DeclarationList<'tree>> {
            match self {
                Self::DeclarationList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for VisibilityModifier_DeclarationList_Identifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "visibility_modifier" => Ok(unsafe {
                    Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "declaration_list" => {
                    Ok(unsafe {
                        Self :: DeclarationList (< DeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for VisibilityModifier_DeclarationList_Identifier<'tree>
    {
        const KIND: &'static str = "{visibility_modifier | declaration_list | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::DeclarationList(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::DeclarationList(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_pattern | mutable_specifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern_MutableSpecifier<'tree> {
        Pattern(Pattern<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Pattern_MutableSpecifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern_MutableSpecifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern_MutableSpecifier<'tree> {
        const KIND: &'static str = "{_pattern | mutable_specifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node(),
                Self::MutableSpecifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node_mut(),
                Self::MutableSpecifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{float_literal | integer_literal}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FloatLiteral_IntegerLiteral<'tree> {
        FloatLiteral(FloatLiteral<'tree>),
        IntegerLiteral(IntegerLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FloatLiteral_IntegerLiteral<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "float_literal" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
            match self {
                Self::FloatLiteral(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "integer_literal" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
            match self {
                Self::IntegerLiteral(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FloatLiteral_IntegerLiteral<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "float_literal" => Ok(unsafe {
                    Self :: FloatLiteral (< FloatLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "integer_literal" => {
                    Ok(unsafe {
                        Self :: IntegerLiteral (< IntegerLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for FloatLiteral_IntegerLiteral<'tree> {
        const KIND: &'static str = "{float_literal | integer_literal}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FloatLiteral(x) => x.node(),
                Self::IntegerLiteral(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FloatLiteral(x) => x.node_mut(),
                Self::IntegerLiteral(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{constrained_type_parameter | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstrainedTypeParameter_TypeIdentifier<'tree> {
        ConstrainedTypeParameter(ConstrainedTypeParameter<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ConstrainedTypeParameter_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "constrained_type_parameter" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn constrained_type_parameter(self) -> Option<ConstrainedTypeParameter<'tree>> {
            match self {
                Self::ConstrainedTypeParameter(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstrainedTypeParameter_TypeIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "constrained_type_parameter" => Ok(unsafe {
                    Self :: ConstrainedTypeParameter (< ConstrainedTypeParameter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstrainedTypeParameter_TypeIdentifier<'tree> {
        const KIND: &'static str = "{constrained_type_parameter | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ConstrainedTypeParameter(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ConstrainedTypeParameter(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{attribute_item | visibility_modifier | _type}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeItem_VisibilityModifier_Type<'tree> {
        AttributeItem(AttributeItem<'tree>),
        VisibilityModifier(VisibilityModifier<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AttributeItem_VisibilityModifier_Type<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem_VisibilityModifier_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <AttributeItem<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::AttributeItem(this));
            }
            if let Ok(this) = <VisibilityModifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VisibilityModifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem_VisibilityModifier_Type<'tree> {
        const KIND: &'static str = "{attribute_item | visibility_modifier | _type}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::VisibilityModifier(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{mutable_specifier | _pattern | self | _type}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MutableSpecifier_Pattern__Self_Type<'tree> {
        MutableSpecifier(MutableSpecifier<'tree>),
        Pattern(Pattern<'tree>),
        _Self(_Self<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MutableSpecifier_Pattern__Self_Type<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutableSpecifier_Pattern__Self_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <_Self<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::_Self(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MutableSpecifier_Pattern__Self_Type<'tree> {
        const KIND: &'static str = "{mutable_specifier | _pattern | self | _type}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node(),
                Self::Pattern(x) => x.node(),
                Self::_Self(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::Pattern(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_pattern | self}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern__Self<'tree> {
        Pattern(Pattern<'tree>),
        _Self(_Self<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Pattern__Self<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern__Self<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <_Self<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::_Self(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern__Self<'tree> {
        const KIND: &'static str = "{_pattern | self}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node(),
                Self::_Self(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_type | attribute_item | parameter | self_parameter | variadic_parameter}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type_AttributeItem_Parameter_SelfParameter_VariadicParameter<'tree> {
        Type(Type<'tree>),
        AttributeItem(AttributeItem<'tree>),
        Parameter(Parameter<'tree>),
        SelfParameter(SelfParameter<'tree>),
        VariadicParameter(VariadicParameter<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type_AttributeItem_Parameter_SelfParameter_VariadicParameter<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "parameter" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn parameter(self) -> Option<Parameter<'tree>> {
            match self {
                Self::Parameter(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self_parameter" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn self_parameter(self) -> Option<SelfParameter<'tree>> {
            match self {
                Self::SelfParameter(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "variadic_parameter" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn variadic_parameter(self) -> Option<VariadicParameter<'tree>> {
            match self {
                Self::VariadicParameter(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Type_AttributeItem_Parameter_SelfParameter_VariadicParameter<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <AttributeItem<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::AttributeItem(this));
            }
            if let Ok(this) = <Parameter<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Parameter(this));
            }
            if let Ok(this) = <SelfParameter<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::SelfParameter(this));
            }
            if let Ok(this) = <VariadicParameter<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VariadicParameter(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Type_AttributeItem_Parameter_SelfParameter_VariadicParameter<'tree>
    {
        const KIND: &'static str =
            "{_type | attribute_item | parameter | self_parameter | variadic_parameter}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node(),
                Self::AttributeItem(x) => x.node(),
                Self::Parameter(x) => x.node(),
                Self::SelfParameter(x) => x.node(),
                Self::VariadicParameter(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node_mut(),
                Self::AttributeItem(x) => x.node_mut(),
                Self::Parameter(x) => x.node_mut(),
                Self::SelfParameter(x) => x.node_mut(),
                Self::VariadicParameter(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{mutable_specifier | _type}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MutableSpecifier_Type<'tree> {
        MutableSpecifier(MutableSpecifier<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MutableSpecifier_Type<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutableSpecifier_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MutableSpecifier_Type<'tree> {
        const KIND: &'static str = "{mutable_specifier | _type}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_literal_pattern | crate | identifier | metavariable | scoped_identifier | self | super}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree> {
        LiteralPattern(LiteralPattern<'tree>),
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_literal_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn literal_pattern(self) -> Option<LiteralPattern<'tree>> {
            match self {
                Self::LiteralPattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn crate_(self) -> Option<Crate<'tree>> {
            match self {
                Self::Crate(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn metavariable(self) -> Option<Metavariable<'tree>> {
            match self {
                Self::Metavariable(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn super_(self) -> Option<Super<'tree>> {
            match self {
                Self::Super(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <LiteralPattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LiteralPattern(this));
            }
            if let Ok(this) = <Crate<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Crate(this));
            }
            if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Metavariable<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Metavariable(this));
            }
            if let Ok(this) = <ScopedIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ScopedIdentifier(this));
            }
            if let Ok(this) = <_Self<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::_Self(this));
            }
            if let Ok(this) = <Super<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Super(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for LiteralPattern_Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree>
    {
        const KIND : & 'static str = "{_literal_pattern | crate | identifier | metavariable | scoped_identifier | self | super}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LiteralPattern(x) => x.node(),
                Self::Crate(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Metavariable(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
                Self::_Self(x) => x.node(),
                Self::Super(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LiteralPattern(x) => x.node_mut(),
                Self::Crate(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Metavariable(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
                Self::Super(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{mutable_specifier | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MutableSpecifier_Expression<'tree> {
        MutableSpecifier(MutableSpecifier<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MutableSpecifier_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutableSpecifier_Expression<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MutableSpecifier_Expression<'tree> {
        const KIND: &'static str = "{mutable_specifier | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{lifetime | mutable_specifier | _type}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Lifetime_MutableSpecifier_Type<'tree> {
        Lifetime(Lifetime<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Lifetime_MutableSpecifier_Type<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lifetime(self) -> Option<Lifetime<'tree>> {
            match self {
                Self::Lifetime(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lifetime_MutableSpecifier_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Lifetime<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Lifetime(this));
            }
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lifetime_MutableSpecifier_Type<'tree> {
        const KIND: &'static str = "{lifetime | mutable_specifier | _type}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Lifetime(x) => x.node(),
                Self::MutableSpecifier(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Lifetime(x) => x.node_mut(),
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{bracketed_type | crate | generic_type | identifier | metavariable | scoped_identifier | self | super}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super<
        'tree,
    > {
        BracketedType(BracketedType<'tree>),
        Crate(Crate<'tree>),
        GenericType(GenericType<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree>
    {
        # [doc = concat ! ("Returns the node if it is of kind `" , "bracketed_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn bracketed_type(self) -> Option<BracketedType<'tree>> {
            match self {
                Self::BracketedType(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn crate_(self) -> Option<Crate<'tree>> {
            match self {
                Self::Crate(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn generic_type(self) -> Option<GenericType<'tree>> {
            match self {
                Self::GenericType(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn metavariable(self) -> Option<Metavariable<'tree>> {
            match self {
                Self::Metavariable(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn super_(self) -> Option<Super<'tree>> {
            match self {
                Self::Super(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super<
            'tree,
        >
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "bracketed_type" => Ok(unsafe {
                    Self::BracketedType(<BracketedType<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "crate" => {
                    Ok(unsafe {
                        Self :: Crate (< Crate < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "generic_type" => Ok(unsafe {
                    Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "metavariable" => Ok(unsafe {
                    Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "self" => {
                    Ok(unsafe {
                        Self :: _Self (< _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "super" => {
                    Ok(unsafe {
                        Self :: Super (< Super < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for BracketedType_Crate_GenericType_Identifier_Metavariable_ScopedIdentifier__Self_Super<
            'tree,
        >
    {
        const KIND : & 'static str = "{bracketed_type | crate | generic_type | identifier | metavariable | scoped_identifier | self | super}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::BracketedType(x) => x.node(),
                Self::Crate(x) => x.node(),
                Self::GenericType(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Metavariable(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
                Self::_Self(x) => x.node(),
                Self::Super(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::BracketedType(x) => x.node_mut(),
                Self::Crate(x) => x.node_mut(),
                Self::GenericType(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Metavariable(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
                Self::Super(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{crate | identifier | metavariable | scoped_identifier | self | super}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree> {
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn crate_(self) -> Option<Crate<'tree>> {
            match self {
                Self::Crate(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn metavariable(self) -> Option<Metavariable<'tree>> {
            match self {
                Self::Metavariable(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn super_(self) -> Option<Super<'tree>> {
            match self {
                Self::Super(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "crate" => {
                    Ok(unsafe {
                        Self :: Crate (< Crate < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "metavariable" => Ok(unsafe {
                    Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_identifier" => Ok(unsafe {
                    Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "self" => {
                    Ok(unsafe {
                        Self :: _Self (< _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "super" => {
                    Ok(unsafe {
                        Self :: Super (< Super < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Crate_Identifier_Metavariable_ScopedIdentifier__Self_Super<'tree>
    {
        const KIND: &'static str =
            "{crate | identifier | metavariable | scoped_identifier | self | super}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Crate(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Metavariable(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
                Self::_Self(x) => x.node(),
                Self::Super(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Crate(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Metavariable(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
                Self::Super(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{lifetime | mutable_specifier | self}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Lifetime_MutableSpecifier__Self<'tree> {
        Lifetime(Lifetime<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
        _Self(_Self<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Lifetime_MutableSpecifier__Self<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lifetime(self) -> Option<Lifetime<'tree>> {
            match self {
                Self::Lifetime(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lifetime_MutableSpecifier__Self<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "lifetime" => {
                    Ok(unsafe {
                        Self :: Lifetime (< Lifetime < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "mutable_specifier" => Ok(unsafe {
                    Self :: MutableSpecifier (< MutableSpecifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "self" => {
                    Ok(unsafe {
                        Self :: _Self (< _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lifetime_MutableSpecifier__Self<'tree> {
        const KIND: &'static str = "{lifetime | mutable_specifier | self}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Lifetime(x) => x.node(),
                Self::MutableSpecifier(x) => x.node(),
                Self::_Self(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Lifetime(x) => x.node_mut(),
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{attribute_item | identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeItem_Identifier<'tree> {
        AttributeItem(AttributeItem<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AttributeItem_Identifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "attribute_item" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem_Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "attribute_item" => Ok(unsafe {
                    Self::AttributeItem(<AttributeItem<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem_Identifier<'tree> {
        const KIND: &'static str = "{attribute_item | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_declaration_statement | expression_statement}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DeclarationStatement_ExpressionStatement<'tree> {
        DeclarationStatement(DeclarationStatement<'tree>),
        ExpressionStatement(ExpressionStatement<'tree>),
    }
    #[automatically_derived]
    impl<'tree> DeclarationStatement_ExpressionStatement<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_declaration_statement" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn declaration_statement(self) -> Option<DeclarationStatement<'tree>> {
            match self {
                Self::DeclarationStatement(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "expression_statement" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression_statement(self) -> Option<ExpressionStatement<'tree>> {
            match self {
                Self::ExpressionStatement(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DeclarationStatement_ExpressionStatement<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <DeclarationStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::DeclarationStatement(this));
            }
            if let Ok(this) = <ExpressionStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ExpressionStatement(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DeclarationStatement_ExpressionStatement<'tree> {
        const KIND: &'static str = "{_declaration_statement | expression_statement}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::DeclarationStatement(x) => x.node(),
                Self::ExpressionStatement(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::DeclarationStatement(x) => x.node_mut(),
                Self::ExpressionStatement(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{mutable_specifier | visibility_modifier | identifier | _type | _expression}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MutableSpecifier_VisibilityModifier_Identifier_Type_Expression<'tree> {
        MutableSpecifier(MutableSpecifier<'tree>),
        VisibilityModifier(VisibilityModifier<'tree>),
        Identifier(Identifier<'tree>),
        Type(Type<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MutableSpecifier_VisibilityModifier_Identifier_Type_Expression<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for MutableSpecifier_VisibilityModifier_Identifier_Type_Expression<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <MutableSpecifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MutableSpecifier(this));
            }
            if let Ok(this) = <VisibilityModifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VisibilityModifier(this));
            }
            if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for MutableSpecifier_VisibilityModifier_Identifier_Type_Expression<'tree>
    {
        const KIND: &'static str =
            "{mutable_specifier | visibility_modifier | identifier | _type | _expression}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node(),
                Self::VisibilityModifier(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{generic_type_with_turbofish | scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum GenericTypeWithTurbofish_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        GenericTypeWithTurbofish(GenericTypeWithTurbofish<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> GenericTypeWithTurbofish_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type_with_turbofish" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn generic_type_with_turbofish(self) -> Option<GenericTypeWithTurbofish<'tree>> {
            match self {
                Self::GenericTypeWithTurbofish(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for GenericTypeWithTurbofish_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "generic_type_with_turbofish" => Ok(unsafe {
                    Self :: GenericTypeWithTurbofish (< GenericTypeWithTurbofish < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_type_identifier" => Ok(unsafe {
                    Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for GenericTypeWithTurbofish_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        const KIND: &'static str =
            "{generic_type_with_turbofish | scoped_type_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::GenericTypeWithTurbofish(x) => x.node(),
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::GenericTypeWithTurbofish(x) => x.node_mut(),
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | where_clause | field_declaration_list | ordered_field_declaration_list | type_identifier | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters<
        'tree,
    > {
        VisibilityModifier(VisibilityModifier<'tree>),
        WhereClause(WhereClause<'tree>),
        FieldDeclarationList(FieldDeclarationList<'tree>),
        OrderedFieldDeclarationList(OrderedFieldDeclarationList<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn visibility_modifier (self) -> Option < VisibilityModifier < 'tree > > { match self { Self :: VisibilityModifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn where_clause (self) -> Option < WhereClause < 'tree > > { match self { Self :: WhereClause (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "field_declaration_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn field_declaration_list (self) -> Option < FieldDeclarationList < 'tree > > { match self { Self :: FieldDeclarationList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "ordered_field_declaration_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn ordered_field_declaration_list (self) -> Option < OrderedFieldDeclarationList < 'tree > > { match self { Self :: OrderedFieldDeclarationList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_identifier (self) -> Option < TypeIdentifier < 'tree > > { match self { Self :: TypeIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_parameters (self) -> Option < TypeParameters < 'tree > > { match self { Self :: TypeParameters (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "visibility_modifier" => Ok (unsafe { Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "where_clause" => Ok (unsafe { Self :: WhereClause (< WhereClause < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "field_declaration_list" => Ok (unsafe { Self :: FieldDeclarationList (< FieldDeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "ordered_field_declaration_list" => Ok (unsafe { Self :: OrderedFieldDeclarationList (< OrderedFieldDeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "type_identifier" => Ok (unsafe { Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "type_parameters" => Ok (unsafe { Self :: TypeParameters (< TypeParameters < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for VisibilityModifier_WhereClause_FieldDeclarationList_OrderedFieldDeclarationList_TypeIdentifier_TypeParameters < 'tree > { const KIND : & 'static str = "{visibility_modifier | where_clause | field_declaration_list | ordered_field_declaration_list | type_identifier | type_parameters}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: VisibilityModifier (x) => x . node () , Self :: WhereClause (x) => x . node () , Self :: FieldDeclarationList (x) => x . node () , Self :: OrderedFieldDeclarationList (x) => x . node () , Self :: TypeIdentifier (x) => x . node () , Self :: TypeParameters (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: VisibilityModifier (x) => x . node_mut () , Self :: WhereClause (x) => x . node_mut () , Self :: FieldDeclarationList (x) => x . node_mut () , Self :: OrderedFieldDeclarationList (x) => x . node_mut () , Self :: TypeIdentifier (x) => x . node_mut () , Self :: TypeParameters (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{field_pattern | remaining_field_pattern | scoped_type_identifier | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        FieldPattern(FieldPattern<'tree>),
        RemainingFieldPattern(RemainingFieldPattern<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_pattern(self) -> Option<FieldPattern<'tree>> {
            match self {
                Self::FieldPattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "remaining_field_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn remaining_field_pattern(self) -> Option<RemainingFieldPattern<'tree>> {
            match self {
                Self::RemainingFieldPattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
            match self {
                Self::ScopedTypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "field_pattern" => Ok(unsafe {
                    Self :: FieldPattern (< FieldPattern < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "remaining_field_pattern" => Ok(unsafe {
                    Self :: RemainingFieldPattern (< RemainingFieldPattern < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "scoped_type_identifier" => Ok(unsafe {
                    Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for FieldPattern_RemainingFieldPattern_ScopedTypeIdentifier_TypeIdentifier<'tree>
    {
        const KIND: &'static str =
            "{field_pattern | remaining_field_pattern | scoped_type_identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::FieldPattern(x) => x.node(),
                Self::RemainingFieldPattern(x) => x.node(),
                Self::ScopedTypeIdentifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::FieldPattern(x) => x.node_mut(),
                Self::RemainingFieldPattern(x) => x.node_mut(),
                Self::ScopedTypeIdentifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_literal | crate | identifier | metavariable | mutable_specifier | primitive_type | self | super | token_repetition | token_tree}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree<
        'tree,
    > {
        Literal(Literal<'tree>),
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
        TokenRepetition(TokenRepetition<'tree>),
        TokenTree(TokenTree<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "_literal" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn literal (self) -> Option < Literal < 'tree > > { match self { Self :: Literal (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn crate_ (self) -> Option < Crate < 'tree > > { match self { Self :: Crate (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn mutable_specifier (self) -> Option < MutableSpecifier < 'tree > > { match self { Self :: MutableSpecifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "primitive_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn primitive_type (self) -> Option < PrimitiveType < 'tree > > { match self { Self :: PrimitiveType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn _self (self) -> Option < _Self < 'tree > > { match self { Self :: _Self (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn super_ (self) -> Option < Super < 'tree > > { match self { Self :: Super (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "token_repetition" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn token_repetition (self) -> Option < TokenRepetition < 'tree > > { match self { Self :: TokenRepetition (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "token_tree" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn token_tree (self) -> Option < TokenTree < 'tree > > { match self { Self :: TokenTree (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < Literal < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Literal (this)) ; } if let Ok (this) = < Crate < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Crate (this)) ; } if let Ok (this) = < Identifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Identifier (this)) ; } if let Ok (this) = < Metavariable < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Metavariable (this)) ; } if let Ok (this) = < MutableSpecifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: MutableSpecifier (this)) ; } if let Ok (this) = < PrimitiveType < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: PrimitiveType (this)) ; } if let Ok (this) = < _Self < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: _Self (this)) ; } if let Ok (this) = < Super < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Super (this)) ; } if let Ok (this) = < TokenRepetition < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TokenRepetition (this)) ; } if let Ok (this) = < TokenTree < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TokenTree (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenRepetition_TokenTree < 'tree > { const KIND : & 'static str = "{_literal | crate | identifier | metavariable | mutable_specifier | primitive_type | self | super | token_repetition | token_tree}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Literal (x) => x . node () , Self :: Crate (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: MutableSpecifier (x) => x . node () , Self :: PrimitiveType (x) => x . node () , Self :: _Self (x) => x . node () , Self :: Super (x) => x . node () , Self :: TokenRepetition (x) => x . node () , Self :: TokenTree (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Literal (x) => x . node_mut () , Self :: Crate (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: MutableSpecifier (x) => x . node_mut () , Self :: PrimitiveType (x) => x . node_mut () , Self :: _Self (x) => x . node_mut () , Self :: Super (x) => x . node_mut () , Self :: TokenRepetition (x) => x . node_mut () , Self :: TokenTree (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{_literal | crate | identifier | metavariable | mutable_specifier | primitive_type | self | super | token_binding_pattern | token_repetition_pattern | token_tree_pattern}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern<
        'tree,
    > {
        Literal(Literal<'tree>),
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
        TokenBindingPattern(TokenBindingPattern<'tree>),
        TokenRepetitionPattern(TokenRepetitionPattern<'tree>),
        TokenTreePattern(TokenTreePattern<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "_literal" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn literal (self) -> Option < Literal < 'tree > > { match self { Self :: Literal (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn crate_ (self) -> Option < Crate < 'tree > > { match self { Self :: Crate (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "mutable_specifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn mutable_specifier (self) -> Option < MutableSpecifier < 'tree > > { match self { Self :: MutableSpecifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "primitive_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn primitive_type (self) -> Option < PrimitiveType < 'tree > > { match self { Self :: PrimitiveType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn _self (self) -> Option < _Self < 'tree > > { match self { Self :: _Self (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn super_ (self) -> Option < Super < 'tree > > { match self { Self :: Super (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "token_binding_pattern" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn token_binding_pattern (self) -> Option < TokenBindingPattern < 'tree > > { match self { Self :: TokenBindingPattern (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "token_repetition_pattern" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn token_repetition_pattern (self) -> Option < TokenRepetitionPattern < 'tree > > { match self { Self :: TokenRepetitionPattern (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "token_tree_pattern" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn token_tree_pattern (self) -> Option < TokenTreePattern < 'tree > > { match self { Self :: TokenTreePattern (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { if let Ok (this) = < Literal < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Literal (this)) ; } if let Ok (this) = < Crate < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Crate (this)) ; } if let Ok (this) = < Identifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Identifier (this)) ; } if let Ok (this) = < Metavariable < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Metavariable (this)) ; } if let Ok (this) = < MutableSpecifier < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: MutableSpecifier (this)) ; } if let Ok (this) = < PrimitiveType < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: PrimitiveType (this)) ; } if let Ok (this) = < _Self < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: _Self (this)) ; } if let Ok (this) = < Super < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: Super (this)) ; } if let Ok (this) = < TokenBindingPattern < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TokenBindingPattern (this)) ; } if let Ok (this) = < TokenRepetitionPattern < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TokenRepetitionPattern (this)) ; } if let Ok (this) = < TokenTreePattern < 'tree > as TryFrom < _ >> :: try_from (node) { return Ok (Self :: TokenTreePattern (this)) ; } Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Literal_Crate_Identifier_Metavariable_MutableSpecifier_PrimitiveType__Self_Super_TokenBindingPattern_TokenRepetitionPattern_TokenTreePattern < 'tree > { const KIND : & 'static str = "{_literal | crate | identifier | metavariable | mutable_specifier | primitive_type | self | super | token_binding_pattern | token_repetition_pattern | token_tree_pattern}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Literal (x) => x . node () , Self :: Crate (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: MutableSpecifier (x) => x . node () , Self :: PrimitiveType (x) => x . node () , Self :: _Self (x) => x . node () , Self :: Super (x) => x . node () , Self :: TokenBindingPattern (x) => x . node () , Self :: TokenRepetitionPattern (x) => x . node () , Self :: TokenTreePattern (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Literal (x) => x . node_mut () , Self :: Crate (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: MutableSpecifier (x) => x . node_mut () , Self :: PrimitiveType (x) => x . node_mut () , Self :: _Self (x) => x . node_mut () , Self :: Super (x) => x . node_mut () , Self :: TokenBindingPattern (x) => x . node_mut () , Self :: TokenRepetitionPattern (x) => x . node_mut () , Self :: TokenTreePattern (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{_type | higher_ranked_trait_bound | lifetime | removed_trait_bound}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound<'tree> {
        Type(Type<'tree>),
        HigherRankedTraitBound(HigherRankedTraitBound<'tree>),
        Lifetime(Lifetime<'tree>),
        RemovedTraitBound(RemovedTraitBound<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "higher_ranked_trait_bound" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn higher_ranked_trait_bound(self) -> Option<HigherRankedTraitBound<'tree>> {
            match self {
                Self::HigherRankedTraitBound(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lifetime(self) -> Option<Lifetime<'tree>> {
            match self {
                Self::Lifetime(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "removed_trait_bound" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn removed_trait_bound(self) -> Option<RemovedTraitBound<'tree>> {
            match self {
                Self::RemovedTraitBound(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <HigherRankedTraitBound<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::HigherRankedTraitBound(this));
            }
            if let Ok(this) = <Lifetime<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Lifetime(this));
            }
            if let Ok(this) = <RemovedTraitBound<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::RemovedTraitBound(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Type_HigherRankedTraitBound_Lifetime_RemovedTraitBound<'tree>
    {
        const KIND: &'static str =
            "{_type | higher_ranked_trait_bound | lifetime | removed_trait_bound}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node(),
                Self::HigherRankedTraitBound(x) => x.node(),
                Self::Lifetime(x) => x.node(),
                Self::RemovedTraitBound(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node_mut(),
                Self::HigherRankedTraitBound(x) => x.node_mut(),
                Self::Lifetime(x) => x.node_mut(),
                Self::RemovedTraitBound(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | where_clause | declaration_list | trait_bounds | type_identifier | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters<
        'tree,
    > {
        VisibilityModifier(VisibilityModifier<'tree>),
        WhereClause(WhereClause<'tree>),
        DeclarationList(DeclarationList<'tree>),
        TraitBounds(TraitBounds<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters<
            'tree,
        >
    {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn where_clause(self) -> Option<WhereClause<'tree>> {
            match self {
                Self::WhereClause(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn declaration_list(self) -> Option<DeclarationList<'tree>> {
            match self {
                Self::DeclarationList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "trait_bounds" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn trait_bounds(self) -> Option<TraitBounds<'tree>> {
            match self {
                Self::TraitBounds(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_parameters(self) -> Option<TypeParameters<'tree>> {
            match self {
                Self::TypeParameters(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters<
            'tree,
        >
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "visibility_modifier" => Ok(unsafe {
                    Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "where_clause" => Ok(unsafe {
                    Self :: WhereClause (< WhereClause < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "declaration_list" => {
                    Ok(unsafe {
                        Self :: DeclarationList (< DeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "trait_bounds" => Ok(unsafe {
                    Self :: TraitBounds (< TraitBounds < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "type_parameters" => {
                    Ok(unsafe {
                        Self :: TypeParameters (< TypeParameters < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for VisibilityModifier_WhereClause_DeclarationList_TraitBounds_TypeIdentifier_TypeParameters<
            'tree,
        >
    {
        const KIND : & 'static str = "{visibility_modifier | where_clause | declaration_list | trait_bounds | type_identifier | type_parameters}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::WhereClause(x) => x.node(),
                Self::DeclarationList(x) => x.node(),
                Self::TraitBounds(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
                Self::TypeParameters(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::WhereClause(x) => x.node_mut(),
                Self::DeclarationList(x) => x.node_mut(),
                Self::TraitBounds(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
                Self::TypeParameters(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_pattern | identifier | scoped_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern_Identifier_ScopedIdentifier<'tree> {
        Pattern(Pattern<'tree>),
        Identifier(Identifier<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Pattern_Identifier_ScopedIdentifier<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_pattern" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn pattern(self) -> Option<Pattern<'tree>> {
            match self {
                Self::Pattern(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
            match self {
                Self::ScopedIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern_Identifier_ScopedIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Pattern<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <Identifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <ScopedIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ScopedIdentifier(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern_Identifier_ScopedIdentifier<'tree> {
        const KIND: &'static str = "{_pattern | identifier | scoped_identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::ScopedIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::ScopedIdentifier(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{_literal | _type | block | lifetime | type_binding}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Literal_Type_Block_Lifetime_TypeBinding<'tree> {
        Literal(Literal<'tree>),
        Type(Type<'tree>),
        Block(Block<'tree>),
        Lifetime(Lifetime<'tree>),
        TypeBinding(TypeBinding<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Literal_Type_Block_Lifetime_TypeBinding<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "_literal" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn literal(self) -> Option<Literal<'tree>> {
            match self {
                Self::Literal(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lifetime(self) -> Option<Lifetime<'tree>> {
            match self {
                Self::Lifetime(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_binding" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_binding(self) -> Option<TypeBinding<'tree>> {
            match self {
                Self::TypeBinding(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Literal_Type_Block_Lifetime_TypeBinding<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Literal<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Literal(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Block(this));
            }
            if let Ok(this) = <Lifetime<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Lifetime(this));
            }
            if let Ok(this) = <TypeBinding<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TypeBinding(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Literal_Type_Block_Lifetime_TypeBinding<'tree> {
        const KIND: &'static str = "{_literal | _type | block | lifetime | type_binding}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Literal(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::Block(x) => x.node(),
                Self::Lifetime(x) => x.node(),
                Self::TypeBinding(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Literal(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::Block(x) => x.node_mut(),
                Self::Lifetime(x) => x.node_mut(),
                Self::TypeBinding(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | type_identifier | _type | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_TypeIdentifier_Type_TypeParameters<'tree> {
        VisibilityModifier(VisibilityModifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
        Type(Type<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl<'tree> VisibilityModifier_TypeIdentifier_Type_TypeParameters<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_type" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_parameters(self) -> Option<TypeParameters<'tree>> {
            match self {
                Self::TypeParameters(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for VisibilityModifier_TypeIdentifier_Type_TypeParameters<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <VisibilityModifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VisibilityModifier(this));
            }
            if let Ok(this) = <TypeIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TypeIdentifier(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <TypeParameters<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TypeParameters(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for VisibilityModifier_TypeIdentifier_Type_TypeParameters<'tree>
    {
        const KIND: &'static str =
            "{visibility_modifier | type_identifier | _type | type_parameters}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::TypeParameters(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::TypeParameters(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{const_parameter | constrained_type_parameter | lifetime | metavariable | optional_type_parameter | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier<
        'tree,
    > {
        ConstParameter(ConstParameter<'tree>),
        ConstrainedTypeParameter(ConstrainedTypeParameter<'tree>),
        Lifetime(Lifetime<'tree>),
        Metavariable(Metavariable<'tree>),
        OptionalTypeParameter(OptionalTypeParameter<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "const_parameter" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn const_parameter (self) -> Option < ConstParameter < 'tree > > { match self { Self :: ConstParameter (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "constrained_type_parameter" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn constrained_type_parameter (self) -> Option < ConstrainedTypeParameter < 'tree > > { match self { Self :: ConstrainedTypeParameter (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn lifetime (self) -> Option < Lifetime < 'tree > > { match self { Self :: Lifetime (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "optional_type_parameter" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn optional_type_parameter (self) -> Option < OptionalTypeParameter < 'tree > > { match self { Self :: OptionalTypeParameter (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_identifier (self) -> Option < TypeIdentifier < 'tree > > { match self { Self :: TypeIdentifier (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "const_parameter" => Ok (unsafe { Self :: ConstParameter (< ConstParameter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "constrained_type_parameter" => Ok (unsafe { Self :: ConstrainedTypeParameter (< ConstrainedTypeParameter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "lifetime" => Ok (unsafe { Self :: Lifetime (< Lifetime < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "metavariable" => Ok (unsafe { Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "optional_type_parameter" => Ok (unsafe { Self :: OptionalTypeParameter (< OptionalTypeParameter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "type_identifier" => Ok (unsafe { Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for ConstParameter_ConstrainedTypeParameter_Lifetime_Metavariable_OptionalTypeParameter_TypeIdentifier < 'tree > { const KIND : & 'static str = "{const_parameter | constrained_type_parameter | lifetime | metavariable | optional_type_parameter | type_identifier}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: ConstParameter (x) => x . node () , Self :: ConstrainedTypeParameter (x) => x . node () , Self :: Lifetime (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: OptionalTypeParameter (x) => x . node () , Self :: TypeIdentifier (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: ConstParameter (x) => x . node_mut () , Self :: ConstrainedTypeParameter (x) => x . node_mut () , Self :: Lifetime (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: OptionalTypeParameter (x) => x . node_mut () , Self :: TypeIdentifier (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{visibility_modifier | where_clause | field_declaration_list | type_identifier | type_parameters}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters<
        'tree,
    > {
        VisibilityModifier(VisibilityModifier<'tree>),
        WhereClause(WhereClause<'tree>),
        FieldDeclarationList(FieldDeclarationList<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
        TypeParameters(TypeParameters<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters<'tree>
    {
        # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn visibility_modifier(self) -> Option<VisibilityModifier<'tree>> {
            match self {
                Self::VisibilityModifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "where_clause" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn where_clause(self) -> Option<WhereClause<'tree>> {
            match self {
                Self::WhereClause(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "field_declaration_list" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_declaration_list(self) -> Option<FieldDeclarationList<'tree>> {
            match self {
                Self::FieldDeclarationList(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "type_parameters" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_parameters(self) -> Option<TypeParameters<'tree>> {
            match self {
                Self::TypeParameters(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "visibility_modifier" => Ok(unsafe {
                    Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "where_clause" => Ok(unsafe {
                    Self :: WhereClause (< WhereClause < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "field_declaration_list" => Ok(unsafe {
                    Self :: FieldDeclarationList (< FieldDeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "type_parameters" => {
                    Ok(unsafe {
                        Self :: TypeParameters (< TypeParameters < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for VisibilityModifier_WhereClause_FieldDeclarationList_TypeIdentifier_TypeParameters<'tree>
    {
        const KIND : & 'static str = "{visibility_modifier | where_clause | field_declaration_list | type_identifier | type_parameters}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node(),
                Self::WhereClause(x) => x.node(),
                Self::FieldDeclarationList(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
                Self::TypeParameters(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::VisibilityModifier(x) => x.node_mut(),
                Self::WhereClause(x) => x.node_mut(),
                Self::FieldDeclarationList(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
                Self::TypeParameters(x) => x.node_mut(),
            }
        }
    }
    # [doc = concat ! ("one of `" , "{visibility_modifier | crate | identifier | metavariable | scoped_identifier | scoped_use_list | self | super | use_as_clause | use_list | use_wildcard}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard<
        'tree,
    > {
        VisibilityModifier(VisibilityModifier<'tree>),
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        ScopedUseList(ScopedUseList<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
        UseAsClause(UseAsClause<'tree>),
        UseList(UseList<'tree>),
        UseWildcard(UseWildcard<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "visibility_modifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn visibility_modifier (self) -> Option < VisibilityModifier < 'tree > > { match self { Self :: VisibilityModifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn crate_ (self) -> Option < Crate < 'tree > > { match self { Self :: Crate (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_identifier (self) -> Option < ScopedIdentifier < 'tree > > { match self { Self :: ScopedIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_use_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_use_list (self) -> Option < ScopedUseList < 'tree > > { match self { Self :: ScopedUseList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn _self (self) -> Option < _Self < 'tree > > { match self { Self :: _Self (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn super_ (self) -> Option < Super < 'tree > > { match self { Self :: Super (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "use_as_clause" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn use_as_clause (self) -> Option < UseAsClause < 'tree > > { match self { Self :: UseAsClause (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "use_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn use_list (self) -> Option < UseList < 'tree > > { match self { Self :: UseList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "use_wildcard" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn use_wildcard (self) -> Option < UseWildcard < 'tree > > { match self { Self :: UseWildcard (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "visibility_modifier" => Ok (unsafe { Self :: VisibilityModifier (< VisibilityModifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "crate" => Ok (unsafe { Self :: Crate (< Crate < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "metavariable" => Ok (unsafe { Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "scoped_identifier" => Ok (unsafe { Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "scoped_use_list" => Ok (unsafe { Self :: ScopedUseList (< ScopedUseList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "self" => Ok (unsafe { Self :: _Self (< _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "super" => Ok (unsafe { Self :: Super (< Super < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "use_as_clause" => Ok (unsafe { Self :: UseAsClause (< UseAsClause < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "use_list" => Ok (unsafe { Self :: UseList (< UseList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "use_wildcard" => Ok (unsafe { Self :: UseWildcard (< UseWildcard < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for VisibilityModifier_Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard < 'tree > { const KIND : & 'static str = "{visibility_modifier | crate | identifier | metavariable | scoped_identifier | scoped_use_list | self | super | use_as_clause | use_list | use_wildcard}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: VisibilityModifier (x) => x . node () , Self :: Crate (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: ScopedIdentifier (x) => x . node () , Self :: ScopedUseList (x) => x . node () , Self :: _Self (x) => x . node () , Self :: Super (x) => x . node () , Self :: UseAsClause (x) => x . node () , Self :: UseList (x) => x . node () , Self :: UseWildcard (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: VisibilityModifier (x) => x . node_mut () , Self :: Crate (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: ScopedIdentifier (x) => x . node_mut () , Self :: ScopedUseList (x) => x . node_mut () , Self :: _Self (x) => x . node_mut () , Self :: Super (x) => x . node_mut () , Self :: UseAsClause (x) => x . node_mut () , Self :: UseList (x) => x . node_mut () , Self :: UseWildcard (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{crate | identifier | metavariable | scoped_identifier | scoped_use_list | self | super | use_as_clause | use_list | use_wildcard}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard<
        'tree,
    > {
        Crate(Crate<'tree>),
        Identifier(Identifier<'tree>),
        Metavariable(Metavariable<'tree>),
        ScopedIdentifier(ScopedIdentifier<'tree>),
        ScopedUseList(ScopedUseList<'tree>),
        _Self(_Self<'tree>),
        Super(Super<'tree>),
        UseAsClause(UseAsClause<'tree>),
        UseList(UseList<'tree>),
        UseWildcard(UseWildcard<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "crate" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn crate_ (self) -> Option < Crate < 'tree > > { match self { Self :: Crate (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "metavariable" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn metavariable (self) -> Option < Metavariable < 'tree > > { match self { Self :: Metavariable (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_identifier (self) -> Option < ScopedIdentifier < 'tree > > { match self { Self :: ScopedIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_use_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_use_list (self) -> Option < ScopedUseList < 'tree > > { match self { Self :: ScopedUseList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "self" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn _self (self) -> Option < _Self < 'tree > > { match self { Self :: _Self (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "super" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn super_ (self) -> Option < Super < 'tree > > { match self { Self :: Super (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "use_as_clause" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn use_as_clause (self) -> Option < UseAsClause < 'tree > > { match self { Self :: UseAsClause (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "use_list" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn use_list (self) -> Option < UseList < 'tree > > { match self { Self :: UseList (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "use_wildcard" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn use_wildcard (self) -> Option < UseWildcard < 'tree > > { match self { Self :: UseWildcard (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "crate" => Ok (unsafe { Self :: Crate (< Crate < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "metavariable" => Ok (unsafe { Self :: Metavariable (< Metavariable < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "scoped_identifier" => Ok (unsafe { Self :: ScopedIdentifier (< ScopedIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "scoped_use_list" => Ok (unsafe { Self :: ScopedUseList (< ScopedUseList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "self" => Ok (unsafe { Self :: _Self (< _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "super" => Ok (unsafe { Self :: Super (< Super < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "use_as_clause" => Ok (unsafe { Self :: UseAsClause (< UseAsClause < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "use_list" => Ok (unsafe { Self :: UseList (< UseList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "use_wildcard" => Ok (unsafe { Self :: UseWildcard (< UseWildcard < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Crate_Identifier_Metavariable_ScopedIdentifier_ScopedUseList__Self_Super_UseAsClause_UseList_UseWildcard < 'tree > { const KIND : & 'static str = "{crate | identifier | metavariable | scoped_identifier | scoped_use_list | self | super | use_as_clause | use_list | use_wildcard}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Crate (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: Metavariable (x) => x . node () , Self :: ScopedIdentifier (x) => x . node () , Self :: ScopedUseList (x) => x . node () , Self :: _Self (x) => x . node () , Self :: Super (x) => x . node () , Self :: UseAsClause (x) => x . node () , Self :: UseList (x) => x . node () , Self :: UseWildcard (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Crate (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: Metavariable (x) => x . node_mut () , Self :: ScopedIdentifier (x) => x . node_mut () , Self :: ScopedUseList (x) => x . node_mut () , Self :: _Self (x) => x . node_mut () , Self :: Super (x) => x . node_mut () , Self :: UseAsClause (x) => x . node_mut () , Self :: UseList (x) => x . node_mut () , Self :: UseWildcard (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{array_type | generic_type | higher_ranked_trait_bound | lifetime | pointer_type | primitive_type | reference_type | scoped_type_identifier | tuple_type | type_identifier}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ArrayType_GenericType_HigherRankedTraitBound_Lifetime_PointerType_PrimitiveType_ReferenceType_ScopedTypeIdentifier_TupleType_TypeIdentifier<
        'tree,
    > {
        ArrayType(ArrayType<'tree>),
        GenericType(GenericType<'tree>),
        HigherRankedTraitBound(HigherRankedTraitBound<'tree>),
        Lifetime(Lifetime<'tree>),
        PointerType(PointerType<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
        ReferenceType(ReferenceType<'tree>),
        ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
        TupleType(TupleType<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > ArrayType_GenericType_HigherRankedTraitBound_Lifetime_PointerType_PrimitiveType_ReferenceType_ScopedTypeIdentifier_TupleType_TypeIdentifier < 'tree > { # [doc = concat ! ("Returns the node if it is of kind `" , "array_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn array_type (self) -> Option < ArrayType < 'tree > > { match self { Self :: ArrayType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "generic_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn generic_type (self) -> Option < GenericType < 'tree > > { match self { Self :: GenericType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "higher_ranked_trait_bound" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn higher_ranked_trait_bound (self) -> Option < HigherRankedTraitBound < 'tree > > { match self { Self :: HigherRankedTraitBound (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "lifetime" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn lifetime (self) -> Option < Lifetime < 'tree > > { match self { Self :: Lifetime (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "pointer_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn pointer_type (self) -> Option < PointerType < 'tree > > { match self { Self :: PointerType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "primitive_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn primitive_type (self) -> Option < PrimitiveType < 'tree > > { match self { Self :: PrimitiveType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "reference_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn reference_type (self) -> Option < ReferenceType < 'tree > > { match self { Self :: ReferenceType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "scoped_type_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn scoped_type_identifier (self) -> Option < ScopedTypeIdentifier < 'tree > > { match self { Self :: ScopedTypeIdentifier (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "tuple_type" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn tuple_type (self) -> Option < TupleType < 'tree > > { match self { Self :: TupleType (x) => Some (x) , _ => None , } } # [doc = concat ! ("Returns the node if it is of kind `" , "type_identifier" , "`, otherwise returns None")] # [inline] # [allow (unused , non_snake_case)] pub fn type_identifier (self) -> Option < TypeIdentifier < 'tree > > { match self { Self :: TypeIdentifier (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for ArrayType_GenericType_HigherRankedTraitBound_Lifetime_PointerType_PrimitiveType_ReferenceType_ScopedTypeIdentifier_TupleType_TypeIdentifier < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "array_type" => Ok (unsafe { Self :: ArrayType (< ArrayType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "generic_type" => Ok (unsafe { Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "higher_ranked_trait_bound" => Ok (unsafe { Self :: HigherRankedTraitBound (< HigherRankedTraitBound < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "lifetime" => Ok (unsafe { Self :: Lifetime (< Lifetime < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "pointer_type" => Ok (unsafe { Self :: PointerType (< PointerType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "primitive_type" => Ok (unsafe { Self :: PrimitiveType (< PrimitiveType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "reference_type" => Ok (unsafe { Self :: ReferenceType (< ReferenceType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "scoped_type_identifier" => Ok (unsafe { Self :: ScopedTypeIdentifier (< ScopedTypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "tuple_type" => Ok (unsafe { Self :: TupleType (< TupleType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "type_identifier" => Ok (unsafe { Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for ArrayType_GenericType_HigherRankedTraitBound_Lifetime_PointerType_PrimitiveType_ReferenceType_ScopedTypeIdentifier_TupleType_TypeIdentifier < 'tree > { const KIND : & 'static str = "{array_type | generic_type | higher_ranked_trait_bound | lifetime | pointer_type | primitive_type | reference_type | scoped_type_identifier | tuple_type | type_identifier}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: ArrayType (x) => x . node () , Self :: GenericType (x) => x . node () , Self :: HigherRankedTraitBound (x) => x . node () , Self :: Lifetime (x) => x . node () , Self :: PointerType (x) => x . node () , Self :: PrimitiveType (x) => x . node () , Self :: ReferenceType (x) => x . node () , Self :: ScopedTypeIdentifier (x) => x . node () , Self :: TupleType (x) => x . node () , Self :: TypeIdentifier (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: ArrayType (x) => x . node_mut () , Self :: GenericType (x) => x . node_mut () , Self :: HigherRankedTraitBound (x) => x . node_mut () , Self :: Lifetime (x) => x . node_mut () , Self :: PointerType (x) => x . node_mut () , Self :: PrimitiveType (x) => x . node_mut () , Self :: ReferenceType (x) => x . node_mut () , Self :: ScopedTypeIdentifier (x) => x . node_mut () , Self :: TupleType (x) => x . node_mut () , Self :: TypeIdentifier (x) => x . node_mut () , } } }
    # [doc = concat ! ("one of `" , "{loop_label | block | _expression | let_chain | let_condition}" , "`")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LoopLabel_Block_Expression_LetChain_LetCondition<'tree> {
        LoopLabel(LoopLabel<'tree>),
        Block(Block<'tree>),
        Expression(Expression<'tree>),
        LetChain(LetChain<'tree>),
        LetCondition(LetCondition<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LoopLabel_Block_Expression_LetChain_LetCondition<'tree> {
        # [doc = concat ! ("Returns the node if it is of kind `" , "loop_label" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn loop_label(self) -> Option<LoopLabel<'tree>> {
            match self {
                Self::LoopLabel(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "block" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "_expression" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_chain" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_chain(self) -> Option<LetChain<'tree>> {
            match self {
                Self::LetChain(x) => Some(x),
                _ => None,
            }
        }
        # [doc = concat ! ("Returns the node if it is of kind `" , "let_condition" , "`, otherwise returns None")]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn let_condition(self) -> Option<LetCondition<'tree>> {
            match self {
                Self::LetCondition(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for LoopLabel_Block_Expression_LetChain_LetCondition<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <LoopLabel<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LoopLabel(this));
            }
            if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Block(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <LetChain<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetChain(this));
            }
            if let Ok(this) = <LetCondition<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LetCondition(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for LoopLabel_Block_Expression_LetChain_LetCondition<'tree>
    {
        const KIND: &'static str = "{loop_label | block | _expression | let_chain | let_condition}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LoopLabel(x) => x.node(),
                Self::Block(x) => x.node(),
                Self::Expression(x) => x.node(),
                Self::LetChain(x) => x.node(),
                Self::LetCondition(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LoopLabel(x) => x.node_mut(),
                Self::Block(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
                Self::LetChain(x) => x.node_mut(),
                Self::LetCondition(x) => x.node_mut(),
            }
        }
    }
}
