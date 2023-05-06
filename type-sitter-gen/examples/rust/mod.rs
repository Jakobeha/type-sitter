#[doc = concat!("Typed node `", "_declaration_statement", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
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
impl<'tree> TryFrom<TSNode<'tree>> for DeclarationStatement<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "associated_type" => Ok(Self::AssociatedType(AssociatedType(node))),
            "attribute_item" => Ok(Self::AttributeItem(AttributeItem(node))),
            "const_item" => Ok(Self::ConstItem(ConstItem(node))),
            "empty_statement" => Ok(Self::EmptyStatement(EmptyStatement(node))),
            "enum_item" => Ok(Self::EnumItem(EnumItem(node))),
            "extern_crate_declaration" => {
                Ok(Self::ExternCrateDeclaration(ExternCrateDeclaration(node)))
            }
            "foreign_mod_item" => Ok(Self::ForeignModItem(ForeignModItem(node))),
            "function_item" => Ok(Self::FunctionItem(FunctionItem(node))),
            "function_signature_item" => {
                Ok(Self::FunctionSignatureItem(FunctionSignatureItem(node)))
            }
            "impl_item" => Ok(Self::ImplItem(ImplItem(node))),
            "inner_attribute_item" => {
                Ok(Self::InnerAttributeItem(InnerAttributeItem(node)))
            }
            "let_declaration" => Ok(Self::LetDeclaration(LetDeclaration(node))),
            "macro_definition" => Ok(Self::MacroDefinition(MacroDefinition(node))),
            "macro_invocation" => Ok(Self::MacroInvocation(MacroInvocation(node))),
            "mod_item" => Ok(Self::ModItem(ModItem(node))),
            "static_item" => Ok(Self::StaticItem(StaticItem(node))),
            "struct_item" => Ok(Self::StructItem(StructItem(node))),
            "trait_item" => Ok(Self::TraitItem(TraitItem(node))),
            "type_item" => Ok(Self::TypeItem(TypeItem(node))),
            "union_item" => Ok(Self::UnionItem(UnionItem(node))),
            "use_declaration" => Ok(Self::UseDeclaration(UseDeclaration(node))),
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_declaration_statement",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DeclarationStatement<'tree> {
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
}
#[doc = concat!("Typed node `", "_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
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
impl<'tree> TryFrom<TSNode<'tree>> for Expression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "_literal" => Ok(Self::Literal(Literal(node))),
            "array_expression" => Ok(Self::ArrayExpression(ArrayExpression(node))),
            "assignment_expression" => {
                Ok(Self::AssignmentExpression(AssignmentExpression(node)))
            }
            "async_block" => Ok(Self::AsyncBlock(AsyncBlock(node))),
            "await_expression" => Ok(Self::AwaitExpression(AwaitExpression(node))),
            "binary_expression" => Ok(Self::BinaryExpression(BinaryExpression(node))),
            "block" => Ok(Self::Block(Block(node))),
            "break_expression" => Ok(Self::BreakExpression(BreakExpression(node))),
            "call_expression" => Ok(Self::CallExpression(CallExpression(node))),
            "closure_expression" => Ok(Self::ClosureExpression(ClosureExpression(node))),
            "compound_assignment_expr" => {
                Ok(Self::CompoundAssignmentExpr(CompoundAssignmentExpr(node)))
            }
            "const_block" => Ok(Self::ConstBlock(ConstBlock(node))),
            "continue_expression" => {
                Ok(Self::ContinueExpression(ContinueExpression(node)))
            }
            "field_expression" => Ok(Self::FieldExpression(FieldExpression(node))),
            "for_expression" => Ok(Self::ForExpression(ForExpression(node))),
            "generic_function" => Ok(Self::GenericFunction(GenericFunction(node))),
            "identifier" => Ok(Self::Identifier(Identifier(node))),
            "if_expression" => Ok(Self::IfExpression(IfExpression(node))),
            "index_expression" => Ok(Self::IndexExpression(IndexExpression(node))),
            "loop_expression" => Ok(Self::LoopExpression(LoopExpression(node))),
            "macro_invocation" => Ok(Self::MacroInvocation(MacroInvocation(node))),
            "match_expression" => Ok(Self::MatchExpression(MatchExpression(node))),
            "metavariable" => Ok(Self::Metavariable(Metavariable(node))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(ParenthesizedExpression(node)))
            }
            "range_expression" => Ok(Self::RangeExpression(RangeExpression(node))),
            "reference_expression" => {
                Ok(Self::ReferenceExpression(ReferenceExpression(node)))
            }
            "return_expression" => Ok(Self::ReturnExpression(ReturnExpression(node))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(ScopedIdentifier(node))),
            "self" => Ok(Self::_Self(_Self(node))),
            "struct_expression" => Ok(Self::StructExpression(StructExpression(node))),
            "try_expression" => Ok(Self::TryExpression(TryExpression(node))),
            "tuple_expression" => Ok(Self::TupleExpression(TupleExpression(node))),
            "type_cast_expression" => {
                Ok(Self::TypeCastExpression(TypeCastExpression(node)))
            }
            "unary_expression" => Ok(Self::UnaryExpression(UnaryExpression(node))),
            "unit_expression" => Ok(Self::UnitExpression(UnitExpression(node))),
            "unsafe_block" => Ok(Self::UnsafeBlock(UnsafeBlock(node))),
            "while_expression" => Ok(Self::WhileExpression(WhileExpression(node))),
            "yield_expression" => Ok(Self::YieldExpression(YieldExpression(node))),
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_expression",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Expression<'tree> {
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
}
#[doc = concat!("Typed node `", "_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub enum Literal<'tree> {
    BooleanLiteral(BooleanLiteral<'tree>),
    CharLiteral(CharLiteral<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    StringLiteral(StringLiteral<'tree>),
}
impl<'tree> TryFrom<TSNode<'tree>> for Literal<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(BooleanLiteral(node))),
            "char_literal" => Ok(Self::CharLiteral(CharLiteral(node))),
            "float_literal" => Ok(Self::FloatLiteral(FloatLiteral(node))),
            "integer_literal" => Ok(Self::IntegerLiteral(IntegerLiteral(node))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(RawStringLiteral(node))),
            "string_literal" => Ok(Self::StringLiteral(StringLiteral(node))),
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_literal",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Literal<'tree> {
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
}
#[doc = concat!("Typed node `", "_literal_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub enum LiteralPattern<'tree> {
    BooleanLiteral(BooleanLiteral<'tree>),
    CharLiteral(CharLiteral<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    NegativeLiteral(NegativeLiteral<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    StringLiteral(StringLiteral<'tree>),
}
impl<'tree> TryFrom<TSNode<'tree>> for LiteralPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(BooleanLiteral(node))),
            "char_literal" => Ok(Self::CharLiteral(CharLiteral(node))),
            "float_literal" => Ok(Self::FloatLiteral(FloatLiteral(node))),
            "integer_literal" => Ok(Self::IntegerLiteral(IntegerLiteral(node))),
            "negative_literal" => Ok(Self::NegativeLiteral(NegativeLiteral(node))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(RawStringLiteral(node))),
            "string_literal" => Ok(Self::StringLiteral(StringLiteral(node))),
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_literal_pattern",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LiteralPattern<'tree> {
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
}
#[doc = concat!("Typed node `", "_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub enum Pattern<'tree> {
    __(__<'tree>),
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
impl<'tree> TryFrom<TSNode<'tree>> for Pattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "_" => Ok(Self::__(__(node))),
            "_literal_pattern" => Ok(Self::LiteralPattern(LiteralPattern(node))),
            "captured_pattern" => Ok(Self::CapturedPattern(CapturedPattern(node))),
            "const_block" => Ok(Self::ConstBlock(ConstBlock(node))),
            "identifier" => Ok(Self::Identifier(Identifier(node))),
            "macro_invocation" => Ok(Self::MacroInvocation(MacroInvocation(node))),
            "mut_pattern" => Ok(Self::MutPattern(MutPattern(node))),
            "or_pattern" => Ok(Self::OrPattern(OrPattern(node))),
            "range_pattern" => Ok(Self::RangePattern(RangePattern(node))),
            "ref_pattern" => Ok(Self::RefPattern(RefPattern(node))),
            "reference_pattern" => Ok(Self::ReferencePattern(ReferencePattern(node))),
            "remaining_field_pattern" => {
                Ok(Self::RemainingFieldPattern(RemainingFieldPattern(node)))
            }
            "scoped_identifier" => Ok(Self::ScopedIdentifier(ScopedIdentifier(node))),
            "slice_pattern" => Ok(Self::SlicePattern(SlicePattern(node))),
            "struct_pattern" => Ok(Self::StructPattern(StructPattern(node))),
            "tuple_pattern" => Ok(Self::TuplePattern(TuplePattern(node))),
            "tuple_struct_pattern" => {
                Ok(Self::TupleStructPattern(TupleStructPattern(node)))
            }
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_pattern",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Pattern<'tree> {
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
}
#[doc = concat!("Typed node `", "_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
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
impl<'tree> TryFrom<TSNode<'tree>> for Type<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "abstract_type" => Ok(Self::AbstractType(AbstractType(node))),
            "array_type" => Ok(Self::ArrayType(ArrayType(node))),
            "bounded_type" => Ok(Self::BoundedType(BoundedType(node))),
            "dynamic_type" => Ok(Self::DynamicType(DynamicType(node))),
            "empty_type" => Ok(Self::EmptyType(EmptyType(node))),
            "function_type" => Ok(Self::FunctionType(FunctionType(node))),
            "generic_type" => Ok(Self::GenericType(GenericType(node))),
            "macro_invocation" => Ok(Self::MacroInvocation(MacroInvocation(node))),
            "metavariable" => Ok(Self::Metavariable(Metavariable(node))),
            "pointer_type" => Ok(Self::PointerType(PointerType(node))),
            "primitive_type" => Ok(Self::PrimitiveType(PrimitiveType(node))),
            "reference_type" => Ok(Self::ReferenceType(ReferenceType(node))),
            "scoped_type_identifier" => {
                Ok(Self::ScopedTypeIdentifier(ScopedTypeIdentifier(node)))
            }
            "tuple_type" => Ok(Self::TupleType(TupleType(node))),
            "type_identifier" => Ok(Self::TypeIdentifier(TypeIdentifier(node))),
            "unit_type" => Ok(Self::UnitType(UnitType(node))),
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_type",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Type<'tree> {
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
}
#[doc = concat!("Typed node `", "abstract_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AbstractType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AbstractType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "abstract_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "abstract_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AbstractType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AbstractType<'tree> {
    #[doc = concat!("Get the field `", "trait", "`")]
    pub fn r#trait(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either4<
            FunctionType<'tree>,
            GenericType<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("trait")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "arguments", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Arguments<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Arguments<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "arguments" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "arguments",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Arguments<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Arguments<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "array_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ArrayExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ArrayExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "array_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ArrayExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ArrayExpression<'tree> {
    #[doc = concat!("Get the field `", "length", "`")]
    pub fn length(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("length")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "array_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ArrayType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ArrayType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "array_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ArrayType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ArrayType<'tree> {
    #[doc = concat!("Get the field `", "element", "`")]
    pub fn element(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("element")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "length", "`")]
    pub fn length(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("length")
    }
}
#[doc = concat!("Typed node `", "assignment_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AssignmentExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AssignmentExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "assignment_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "assignment_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AssignmentExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AssignmentExpression<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    pub fn left(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("left")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    pub fn right(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("right")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "associated_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AssociatedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AssociatedType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "associated_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "associated_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AssociatedType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AssociatedType<'tree> {
    #[doc = concat!("Get the field `", "bounds", "`")]
    pub fn bounds(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TraitBounds<'tree>>> {
        self.0.child_by_field_name("bounds")
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
}
#[doc = concat!("Typed node `", "async_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AsyncBlock<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AsyncBlock<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "async_block" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "async_block",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AsyncBlock<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AsyncBlock<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "attribute", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Attribute<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Attribute<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "attribute",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Attribute<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Attribute<'tree> {
    #[doc = concat!("Get the field `", "arguments", "`")]
    pub fn arguments(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TokenTree<'tree>>> {
        self.0.child_by_field_name("arguments")
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("value")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either6<
            Crate<'tree>,
            Identifier<'tree>,
            Metavariable<'tree>,
            ScopedIdentifier<'tree>,
            _Self<'tree>,
            _Super<'tree>,
        >,
    > {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "attribute_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AttributeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AttributeItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "attribute_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "attribute_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AttributeItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AttributeItem<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Attribute<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "await_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AwaitExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AwaitExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "await_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "await_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AwaitExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AwaitExpression<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "base_field_initializer", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BaseFieldInitializer<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BaseFieldInitializer<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "base_field_initializer" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "base_field_initializer",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BaseFieldInitializer<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BaseFieldInitializer<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "binary_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BinaryExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BinaryExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "binary_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "binary_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BinaryExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BinaryExpression<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    pub fn left(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("left")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "operator", "`")]
    pub fn operator(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either18<
            NotEq<'tree>,
            Mod<'tree>,
            And<'tree>,
            AndAnd<'tree>,
            Mul<'tree>,
            Add<'tree>,
            Sub<'tree>,
            Div<'tree>,
            Lt<'tree>,
            LtLt<'tree>,
            LtEq<'tree>,
            EqEq<'tree>,
            Gt<'tree>,
            GtEq<'tree>,
            GtGt<'tree>,
            BitXor<'tree>,
            Or<'tree>,
            OrOr<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("operator")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    pub fn right(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("right")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Block<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Block<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "block",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Block<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Block<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                DeclarationStatement<'tree>,
                Expression<'tree>,
                ExpressionStatement<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    DeclarationStatement<'tree>,
                    Expression<'tree>,
                    ExpressionStatement<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                DeclarationStatement<'tree>,
                Expression<'tree>,
                ExpressionStatement<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    DeclarationStatement<'tree>,
                    Expression<'tree>,
                    ExpressionStatement<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "boolean_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BooleanLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BooleanLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "boolean_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "boolean_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BooleanLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BooleanLiteral<'tree> {}
#[doc = concat!("Typed node `", "bounded_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BoundedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BoundedType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "bounded_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "bounded_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BoundedType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BoundedType<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Type<'tree>, Lifetime<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Type<'tree>,
                    Lifetime<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Type<'tree>, Lifetime<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Type<'tree>,
                    Lifetime<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "bracketed_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BracketedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BracketedType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "bracketed_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "bracketed_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BracketedType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BracketedType<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Type<'tree>, QualifiedType<'tree>>,
    > {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "break_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BreakExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BreakExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "break_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "break_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BreakExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BreakExpression<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, LoopLabel<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    LoopLabel<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, LoopLabel<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    LoopLabel<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "call_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct CallExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for CallExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "call_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "call_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for CallExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> CallExpression<'tree> {
    #[doc = concat!("Get the field `", "arguments", "`")]
    pub fn arguments(&self) -> tree_sitter_lib::NodeResult<'tree, Arguments<'tree>> {
        self.0
            .child_by_field_name("arguments")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "function", "`")]
    pub fn function(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::Either![
            Literal < 'tree >, ArrayExpression < 'tree >, AssignmentExpression < 'tree >,
            AsyncBlock < 'tree >, AwaitExpression < 'tree >, BinaryExpression < 'tree >,
            Block < 'tree >, BreakExpression < 'tree >, CallExpression < 'tree >,
            ClosureExpression < 'tree >, CompoundAssignmentExpr < 'tree >, ConstBlock <
            'tree >, ContinueExpression < 'tree >, FieldExpression < 'tree >,
            ForExpression < 'tree >, GenericFunction < 'tree >, Identifier < 'tree >,
            IfExpression < 'tree >, IndexExpression < 'tree >, LoopExpression < 'tree >,
            MacroInvocation < 'tree >, MatchExpression < 'tree >, Metavariable < 'tree >,
            ParenthesizedExpression < 'tree >, ReferenceExpression < 'tree >,
            ReturnExpression < 'tree >, ScopedIdentifier < 'tree >, _Self < 'tree >,
            StructExpression < 'tree >, TryExpression < 'tree >, TupleExpression < 'tree
            >, TypeCastExpression < 'tree >, UnaryExpression < 'tree >, UnitExpression <
            'tree >, UnsafeBlock < 'tree >, WhileExpression < 'tree >, YieldExpression <
            'tree >
        ],
    > {
        self.0
            .child_by_field_name("function")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "captured_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct CapturedPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for CapturedPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "captured_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "captured_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for CapturedPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> CapturedPattern<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.children(&mut c).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.child(i).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "closure_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ClosureExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ClosureExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "closure_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "closure_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ClosureExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ClosureExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "parameters", "`")]
    pub fn parameters(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, ClosureParameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    pub fn return_type(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child_by_field_name("return_type")
    }
}
#[doc = concat!("Typed node `", "closure_parameters", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ClosureParameters<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ClosureParameters<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "closure_parameters" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "closure_parameters",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ClosureParameters<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ClosureParameters<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Pattern<'tree>, Parameter<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Pattern<'tree>,
                    Parameter<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Pattern<'tree>, Parameter<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Pattern<'tree>,
                    Parameter<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "compound_assignment_expr", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct CompoundAssignmentExpr<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for CompoundAssignmentExpr<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "compound_assignment_expr" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "compound_assignment_expr",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for CompoundAssignmentExpr<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> CompoundAssignmentExpr<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    pub fn left(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("left")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "operator", "`")]
    pub fn operator(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either10<
            ModEq<'tree>,
            AndEq<'tree>,
            MulEq<'tree>,
            AddEq<'tree>,
            SubEq<'tree>,
            DivEq<'tree>,
            LtLtEq<'tree>,
            GtGtEq<'tree>,
            BitXorEq<'tree>,
            OrEq<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("operator")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    pub fn right(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("right")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "const_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ConstBlock<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ConstBlock<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_block" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "const_block",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ConstBlock<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ConstBlock<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "const_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ConstItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ConstItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "const_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ConstItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ConstItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("value")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "const_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ConstParameter<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ConstParameter<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_parameter" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "const_parameter",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ConstParameter<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ConstParameter<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "constrained_type_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ConstrainedTypeParameter<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ConstrainedTypeParameter<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "constrained_type_parameter" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "constrained_type_parameter",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ConstrainedTypeParameter<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ConstrainedTypeParameter<'tree> {
    #[doc = concat!("Get the field `", "bounds", "`")]
    pub fn bounds(&self) -> tree_sitter_lib::NodeResult<'tree, TraitBounds<'tree>> {
        self.0
            .child_by_field_name("bounds")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "left", "`")]
    pub fn left(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Lifetime<'tree>, TypeIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("left")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "continue_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ContinueExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ContinueExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "continue_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "continue_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ContinueExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ContinueExpression<'tree> {
    ///Get the node's child
    pub fn child(&self) -> Option<tree_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "declaration_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DeclarationList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DeclarationList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "declaration_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "declaration_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DeclarationList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DeclarationList<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<'tree, DeclarationStatement<'tree>>,
    > {
        self.0
            .children(&mut c)
            .map(<DeclarationStatement<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, DeclarationStatement<'tree>>> {
        self.0.child(i).map(<DeclarationStatement<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "dynamic_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DynamicType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DynamicType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dynamic_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "dynamic_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DynamicType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DynamicType<'tree> {
    #[doc = concat!("Get the field `", "trait", "`")]
    pub fn r#trait(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either4<
            FunctionType<'tree>,
            GenericType<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("trait")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "else_clause", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ElseClause<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ElseClause<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "else_clause" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "else_clause",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ElseClause<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ElseClause<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Block<'tree>, IfExpression<'tree>>,
    > {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "empty_statement", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EmptyStatement<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EmptyStatement<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_statement" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "empty_statement",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EmptyStatement<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EmptyStatement<'tree> {}
#[doc = concat!("Typed node `", "empty_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EmptyType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EmptyType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "empty_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EmptyType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EmptyType<'tree> {}
#[doc = concat!("Typed node `", "enum_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EnumItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EnumItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "enum_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EnumItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EnumItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, EnumVariantList<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "enum_variant", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EnumVariant<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EnumVariant<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_variant" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "enum_variant",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EnumVariant<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EnumVariant<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                FieldDeclarationList<'tree>,
                OrderedFieldDeclarationList<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("body")
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("value")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "enum_variant_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EnumVariantList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EnumVariantList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_variant_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "enum_variant_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EnumVariantList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EnumVariantList<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<AttributeItem<'tree>, EnumVariant<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    EnumVariant<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<AttributeItem<'tree>, EnumVariant<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    EnumVariant<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "expression_statement", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ExpressionStatement<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ExpressionStatement<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_statement" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "expression_statement",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ExpressionStatement<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ExpressionStatement<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "extern_crate_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ExternCrateDeclaration<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ExternCrateDeclaration<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern_crate_declaration" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "extern_crate_declaration",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ExternCrateDeclaration<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ExternCrateDeclaration<'tree> {
    #[doc = concat!("Get the field `", "alias", "`")]
    pub fn alias(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Identifier<'tree>>> {
        self.0.child_by_field_name("alias")
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Crate<'tree>, VisibilityModifier<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Crate<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Crate<'tree>, VisibilityModifier<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Crate<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "extern_modifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ExternModifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ExternModifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern_modifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "extern_modifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ExternModifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ExternModifier<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, StringLiteral<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "field_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldDeclaration<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldDeclaration<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_declaration",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldDeclaration<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldDeclaration<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "field_declaration_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldDeclarationList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldDeclarationList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_declaration_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldDeclarationList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldDeclarationList<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                AttributeItem<'tree>,
                FieldDeclaration<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    FieldDeclaration<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                AttributeItem<'tree>,
                FieldDeclaration<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    FieldDeclaration<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "field_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldExpression<'tree> {
    #[doc = concat!("Get the field `", "field", "`")]
    pub fn field(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<FieldIdentifier<'tree>, IntegerLiteral<'tree>>,
    > {
        self.0
            .child_by_field_name("field")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "field_initializer", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldInitializer<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldInitializer<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_initializer" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_initializer",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldInitializer<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldInitializer<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, AttributeItem<'tree>>> {
        self.0.children(&mut c).map(<AttributeItem<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, AttributeItem<'tree>>> {
        self.0.child(i).map(<AttributeItem<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "field_initializer_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldInitializerList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldInitializerList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_initializer_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_initializer_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldInitializerList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldInitializerList<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                BaseFieldInitializer<'tree>,
                FieldInitializer<'tree>,
                ShorthandFieldInitializer<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    BaseFieldInitializer<'tree>,
                    FieldInitializer<'tree>,
                    ShorthandFieldInitializer<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                BaseFieldInitializer<'tree>,
                FieldInitializer<'tree>,
                ShorthandFieldInitializer<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    BaseFieldInitializer<'tree>,
                    FieldInitializer<'tree>,
                    ShorthandFieldInitializer<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "field_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldPattern<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<
            FieldIdentifier<'tree>,
            ShorthandFieldIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "pattern", "`")]
    pub fn pattern(&self) -> Option<tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.child_by_field_name("pattern")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "for_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ForExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ForExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "for_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ForExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ForExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "pattern", "`")]
    pub fn pattern(&self) -> tree_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(&self) -> Option<tree_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "for_lifetimes", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ForLifetimes<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ForLifetimes<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_lifetimes" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "for_lifetimes",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ForLifetimes<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ForLifetimes<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Lifetime<'tree>>> {
        self.0.children(&mut c).map(<Lifetime<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Lifetime<'tree>>> {
        self.0.child(i).map(<Lifetime<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "foreign_mod_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ForeignModItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ForeignModItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "foreign_mod_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "foreign_mod_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ForeignModItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ForeignModItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0.child_by_field_name("body")
    }
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                ExternModifier<'tree>,
                VisibilityModifier<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    ExternModifier<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                ExternModifier<'tree>,
                VisibilityModifier<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    ExternModifier<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "fragment_specifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FragmentSpecifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FragmentSpecifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "fragment_specifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "fragment_specifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FragmentSpecifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FragmentSpecifier<'tree> {}
#[doc = concat!("Typed node `", "function_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FunctionItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FunctionItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "function_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FunctionItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FunctionItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Identifier<'tree>, Metavariable<'tree>>,
    > {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "parameters", "`")]
    pub fn parameters(&self) -> tree_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    pub fn return_type(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child_by_field_name("return_type")
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                FunctionModifiers<'tree>,
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                FunctionModifiers<'tree>,
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "function_modifiers", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FunctionModifiers<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FunctionModifiers<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_modifiers" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "function_modifiers",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FunctionModifiers<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FunctionModifiers<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<'tree, ExternModifier<'tree>>,
    > {
        self.0.children(&mut c).map(<ExternModifier<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, ExternModifier<'tree>>> {
        self.0.child(i).map(<ExternModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "function_signature_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FunctionSignatureItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FunctionSignatureItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_signature_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "function_signature_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FunctionSignatureItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FunctionSignatureItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Identifier<'tree>, Metavariable<'tree>>,
    > {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "parameters", "`")]
    pub fn parameters(&self) -> tree_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    pub fn return_type(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child_by_field_name("return_type")
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                FunctionModifiers<'tree>,
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                FunctionModifiers<'tree>,
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "function_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FunctionType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FunctionType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "function_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FunctionType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FunctionType<'tree> {
    #[doc = concat!("Get the field `", "parameters", "`")]
    pub fn parameters(&self) -> tree_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    pub fn return_type(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child_by_field_name("return_type")
    }
    #[doc = concat!("Get the field `", "trait", "`")]
    pub fn r#trait(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                ScopedTypeIdentifier<'tree>,
                TypeIdentifier<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("trait")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                ForLifetimes<'tree>,
                FunctionModifiers<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    ForLifetimes<'tree>,
                    FunctionModifiers<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                ForLifetimes<'tree>,
                FunctionModifiers<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    ForLifetimes<'tree>,
                    FunctionModifiers<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "generic_function", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct GenericFunction<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for GenericFunction<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_function" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "generic_function",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for GenericFunction<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> GenericFunction<'tree> {
    #[doc = concat!("Get the field `", "function", "`")]
    pub fn function(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either3<
            FieldExpression<'tree>,
            Identifier<'tree>,
            ScopedIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("function")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    pub fn type_arguments(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self.0
            .child_by_field_name("type_arguments")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "generic_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct GenericType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for GenericType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "generic_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for GenericType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> GenericType<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either3<
            ScopedIdentifier<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    pub fn type_arguments(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self.0
            .child_by_field_name("type_arguments")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "generic_type_with_turbofish", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct GenericTypeWithTurbofish<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for GenericTypeWithTurbofish<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type_with_turbofish" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "generic_type_with_turbofish",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for GenericTypeWithTurbofish<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> GenericTypeWithTurbofish<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<
            ScopedIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    pub fn type_arguments(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self.0
            .child_by_field_name("type_arguments")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "higher_ranked_trait_bound", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct HigherRankedTraitBound<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for HigherRankedTraitBound<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "higher_ranked_trait_bound" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "higher_ranked_trait_bound",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for HigherRankedTraitBound<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> HigherRankedTraitBound<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>> {
        self.0
            .child_by_field_name("type_parameters")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "if_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IfExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IfExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "if_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "if_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IfExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IfExpression<'tree> {
    #[doc = concat!("Get the field `", "alternative", "`")]
    pub fn alternative(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, ElseClause<'tree>>> {
        self.0.child_by_field_name("alternative")
    }
    #[doc = concat!("Get the field `", "condition", "`")]
    pub fn condition(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either3<
            Expression<'tree>,
            LetChain<'tree>,
            LetCondition<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("condition")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "consequence", "`")]
    pub fn consequence(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("consequence")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "impl_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ImplItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ImplItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "impl_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "impl_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ImplItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ImplItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0.child_by_field_name("body")
    }
    #[doc = concat!("Get the field `", "trait", "`")]
    pub fn r#trait(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                GenericType<'tree>,
                ScopedTypeIdentifier<'tree>,
                TypeIdentifier<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("trait")
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, WhereClause<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "index_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IndexExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IndexExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "index_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "index_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IndexExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IndexExpression<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.children(&mut c).map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child(i).map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "inner_attribute_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct InnerAttributeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for InnerAttributeItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "inner_attribute_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "inner_attribute_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for InnerAttributeItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> InnerAttributeItem<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Attribute<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "let_chain", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LetChain<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LetChain<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_chain" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "let_chain",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LetChain<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LetChain<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, LetCondition<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    LetCondition<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, LetCondition<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    LetCondition<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "let_condition", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LetCondition<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LetCondition<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_condition" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "let_condition",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LetCondition<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LetCondition<'tree> {
    #[doc = concat!("Get the field `", "pattern", "`")]
    pub fn pattern(&self) -> tree_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "let_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LetDeclaration<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LetDeclaration<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_declaration" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "let_declaration",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LetDeclaration<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LetDeclaration<'tree> {
    #[doc = concat!("Get the field `", "alternative", "`")]
    pub fn alternative(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Block<'tree>>> {
        self.0.child_by_field_name("alternative")
    }
    #[doc = concat!("Get the field `", "pattern", "`")]
    pub fn pattern(&self) -> tree_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> Option<tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child_by_field_name("type")
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("value")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "lifetime", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Lifetime<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Lifetime<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "lifetime" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "lifetime",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Lifetime<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Lifetime<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "loop_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LoopExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LoopExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "loop_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LoopExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LoopExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(&self) -> Option<tree_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "loop_label", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LoopLabel<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LoopLabel<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop_label" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "loop_label",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LoopLabel<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LoopLabel<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "macro_definition", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MacroDefinition<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MacroDefinition<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_definition" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "macro_definition",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MacroDefinition<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MacroDefinition<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, MacroRule<'tree>>> {
        self.0.children(&mut c).map(<MacroRule<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MacroRule<'tree>>> {
        self.0.child(i).map(<MacroRule<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "macro_invocation", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MacroInvocation<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MacroInvocation<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_invocation" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "macro_invocation",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MacroInvocation<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MacroInvocation<'tree> {
    #[doc = concat!("Get the field `", "macro", "`")]
    pub fn r#macro(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Identifier<'tree>, ScopedIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("macro")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, TokenTree<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "macro_rule", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MacroRule<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MacroRule<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_rule" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "macro_rule",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MacroRule<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MacroRule<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    pub fn left(&self) -> tree_sitter_lib::NodeResult<'tree, TokenTreePattern<'tree>> {
        self.0
            .child_by_field_name("left")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    pub fn right(&self) -> tree_sitter_lib::NodeResult<'tree, TokenTree<'tree>> {
        self.0
            .child_by_field_name("right")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "match_arm", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MatchArm<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MatchArm<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_arm" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "match_arm",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MatchArm<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MatchArm<'tree> {
    #[doc = concat!("Get the field `", "pattern", "`")]
    pub fn pattern(&self) -> tree_sitter_lib::NodeResult<'tree, MatchPattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, AttributeItem<'tree>>> {
        self.0.children(&mut c).map(<AttributeItem<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, AttributeItem<'tree>>> {
        self.0.child(i).map(<AttributeItem<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "match_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MatchBlock<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MatchBlock<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_block" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "match_block",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MatchBlock<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MatchBlock<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, MatchArm<'tree>>> {
        self.0.children(&mut c).map(<MatchArm<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MatchArm<'tree>>> {
        self.0.child(i).map(<MatchArm<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "match_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MatchExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MatchExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "match_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MatchExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MatchExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, MatchBlock<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "match_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MatchPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MatchPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "match_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MatchPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MatchPattern<'tree> {
    #[doc = concat!("Get the field `", "condition", "`")]
    pub fn condition(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                Expression<'tree>,
                LetChain<'tree>,
                LetCondition<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("condition")
    }
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "mod_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ModItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ModItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mod_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "mod_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ModItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ModItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0.child_by_field_name("body")
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "mut_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MutPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MutPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mut_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "mut_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MutPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MutPattern<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Pattern<'tree>,
                    MutableSpecifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Pattern<'tree>,
                    MutableSpecifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "negative_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct NegativeLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for NegativeLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "negative_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "negative_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for NegativeLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> NegativeLiteral<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<FloatLiteral<'tree>, IntegerLiteral<'tree>>,
    > {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "optional_type_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct OptionalTypeParameter<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for OptionalTypeParameter<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "optional_type_parameter" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "optional_type_parameter",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for OptionalTypeParameter<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> OptionalTypeParameter<'tree> {
    #[doc = concat!("Get the field `", "default_type", "`")]
    pub fn default_type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("default_type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<
            ConstrainedTypeParameter<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "or_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct OrPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for OrPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "or_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "or_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for OrPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> OrPattern<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.children(&mut c).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.child(i).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "ordered_field_declaration_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct OrderedFieldDeclarationList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for OrderedFieldDeclarationList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ordered_field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "ordered_field_declaration_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for OrderedFieldDeclarationList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> OrderedFieldDeclarationList<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn types(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .children_by_field_name("type", &mut c)
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                AttributeItem<'tree>,
                VisibilityModifier<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                AttributeItem<'tree>,
                VisibilityModifier<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Parameter<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Parameter<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameter" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "parameter",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Parameter<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Parameter<'tree> {
    #[doc = concat!("Get the field `", "pattern", "`")]
    pub fn pattern(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Pattern<'tree>, _Self<'tree>>,
    > {
        self.0
            .child_by_field_name("pattern")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "parameters", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Parameters<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Parameters<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameters" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "parameters",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Parameters<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Parameters<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either5<
                Type<'tree>,
                AttributeItem<'tree>,
                Parameter<'tree>,
                SelfParameter<'tree>,
                VariadicParameter<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either5<
                    Type<'tree>,
                    AttributeItem<'tree>,
                    Parameter<'tree>,
                    SelfParameter<'tree>,
                    VariadicParameter<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either5<
                Type<'tree>,
                AttributeItem<'tree>,
                Parameter<'tree>,
                SelfParameter<'tree>,
                VariadicParameter<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either5<
                    Type<'tree>,
                    AttributeItem<'tree>,
                    Parameter<'tree>,
                    SelfParameter<'tree>,
                    VariadicParameter<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "parenthesized_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ParenthesizedExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ParenthesizedExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "parenthesized_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ParenthesizedExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ParenthesizedExpression<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "pointer_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct PointerType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for PointerType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pointer_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "pointer_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for PointerType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> PointerType<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "qualified_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct QualifiedType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for QualifiedType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "qualified_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "qualified_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for QualifiedType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> QualifiedType<'tree> {
    #[doc = concat!("Get the field `", "alias", "`")]
    pub fn alias(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("alias")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "range_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RangeExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RangeExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "range_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RangeExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RangeExpression<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.children(&mut c).map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child(i).map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "range_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RangePattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RangePattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "range_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RangePattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RangePattern<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either7<
                LiteralPattern<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either7<
                    LiteralPattern<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either7<
                LiteralPattern<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either7<
                    LiteralPattern<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "ref_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RefPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RefPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ref_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "ref_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RefPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RefPattern<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "reference_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ReferenceExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ReferenceExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "reference_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ReferenceExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ReferenceExpression<'tree> {
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "reference_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ReferencePattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ReferencePattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "reference_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ReferencePattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ReferencePattern<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Pattern<'tree>,
                    MutableSpecifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Pattern<'tree>,
                    MutableSpecifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "reference_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ReferenceType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ReferenceType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "reference_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ReferenceType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ReferenceType<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Lifetime<'tree>, MutableSpecifier<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Lifetime<'tree>,
                    MutableSpecifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Lifetime<'tree>, MutableSpecifier<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Lifetime<'tree>,
                    MutableSpecifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "remaining_field_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RemainingFieldPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RemainingFieldPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "remaining_field_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "remaining_field_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RemainingFieldPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RemainingFieldPattern<'tree> {}
#[doc = concat!("Typed node `", "removed_trait_bound", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RemovedTraitBound<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RemovedTraitBound<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "removed_trait_bound" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "removed_trait_bound",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RemovedTraitBound<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RemovedTraitBound<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "return_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ReturnExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ReturnExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "return_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ReturnExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ReturnExpression<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "scoped_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ScopedIdentifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ScopedIdentifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_identifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "scoped_identifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ScopedIdentifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ScopedIdentifier<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    pub fn path(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either8<
                BracketedType<'tree>,
                Crate<'tree>,
                GenericType<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("path")
    }
}
#[doc = concat!("Typed node `", "scoped_type_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ScopedTypeIdentifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ScopedTypeIdentifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_type_identifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "scoped_type_identifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ScopedTypeIdentifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ScopedTypeIdentifier<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    pub fn path(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either8<
                BracketedType<'tree>,
                Crate<'tree>,
                GenericType<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("path")
    }
}
#[doc = concat!("Typed node `", "scoped_use_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ScopedUseList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ScopedUseList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_use_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "scoped_use_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ScopedUseList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ScopedUseList<'tree> {
    #[doc = concat!("Get the field `", "list", "`")]
    pub fn list(&self) -> tree_sitter_lib::NodeResult<'tree, UseList<'tree>> {
        self.0
            .child_by_field_name("list")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    pub fn path(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either6<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("path")
    }
}
#[doc = concat!("Typed node `", "self_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct SelfParameter<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for SelfParameter<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "self_parameter" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "self_parameter",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for SelfParameter<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> SelfParameter<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                Lifetime<'tree>,
                MutableSpecifier<'tree>,
                _Self<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    Lifetime<'tree>,
                    MutableSpecifier<'tree>,
                    _Self<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either3<
                Lifetime<'tree>,
                MutableSpecifier<'tree>,
                _Self<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either3<
                    Lifetime<'tree>,
                    MutableSpecifier<'tree>,
                    _Self<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "shorthand_field_initializer", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ShorthandFieldInitializer<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ShorthandFieldInitializer<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "shorthand_field_initializer" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "shorthand_field_initializer",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ShorthandFieldInitializer<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ShorthandFieldInitializer<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<AttributeItem<'tree>, Identifier<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    Identifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<AttributeItem<'tree>, Identifier<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    AttributeItem<'tree>,
                    Identifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "slice_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct SlicePattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for SlicePattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "slice_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "slice_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for SlicePattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> SlicePattern<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.children(&mut c).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.child(i).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "source_file", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct SourceFile<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for SourceFile<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "source_file" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "source_file",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for SourceFile<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> SourceFile<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                DeclarationStatement<'tree>,
                ExpressionStatement<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    DeclarationStatement<'tree>,
                    ExpressionStatement<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                DeclarationStatement<'tree>,
                ExpressionStatement<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    DeclarationStatement<'tree>,
                    ExpressionStatement<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "static_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct StaticItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for StaticItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "static_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "static_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for StaticItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> StaticItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child_by_field_name("value")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                MutableSpecifier<'tree>,
                VisibilityModifier<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    MutableSpecifier<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                MutableSpecifier<'tree>,
                VisibilityModifier<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    MutableSpecifier<'tree>,
                    VisibilityModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "string_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct StringLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for StringLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "string_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for StringLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> StringLiteral<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<'tree, EscapeSequence<'tree>>,
    > {
        self.0.children(&mut c).map(<EscapeSequence<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, EscapeSequence<'tree>>> {
        self.0.child(i).map(<EscapeSequence<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "struct_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct StructExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for StructExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "struct_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for StructExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> StructExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, FieldInitializerList<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either3<
            GenericTypeWithTurbofish<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "struct_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct StructItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for StructItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "struct_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for StructItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> StructItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                FieldDeclarationList<'tree>,
                OrderedFieldDeclarationList<'tree>,
            >,
        >,
    > {
        self.0.child_by_field_name("body")
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "struct_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct StructPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for StructPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "struct_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for StructPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> StructPattern<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                FieldPattern<'tree>,
                RemainingFieldPattern<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    FieldPattern<'tree>,
                    RemainingFieldPattern<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                FieldPattern<'tree>,
                RemainingFieldPattern<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    FieldPattern<'tree>,
                    RemainingFieldPattern<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "token_binding_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TokenBindingPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TokenBindingPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_binding_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "token_binding_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TokenBindingPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TokenBindingPattern<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, Metavariable<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, FragmentSpecifier<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "token_repetition", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TokenRepetition<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TokenRepetition<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_repetition" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "token_repetition",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TokenRepetition<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TokenRepetition<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either10<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenRepetition<'tree>,
                TokenTree<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either10<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenRepetition<'tree>,
                TokenTree<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "token_repetition_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TokenRepetitionPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TokenRepetitionPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_repetition_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "token_repetition_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TokenRepetitionPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TokenRepetitionPattern<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either11<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenBindingPattern<'tree>,
                TokenRepetitionPattern<'tree>,
                TokenTreePattern<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either11<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenBindingPattern<'tree>,
                TokenRepetitionPattern<'tree>,
                TokenTreePattern<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "token_tree", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TokenTree<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TokenTree<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_tree" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "token_tree",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TokenTree<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TokenTree<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either10<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenRepetition<'tree>,
                TokenTree<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either10<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenRepetition<'tree>,
                TokenTree<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "token_tree_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TokenTreePattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TokenTreePattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_tree_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "token_tree_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TokenTreePattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TokenTreePattern<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either11<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenBindingPattern<'tree>,
                TokenRepetitionPattern<'tree>,
                TokenTreePattern<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either11<
                Literal<'tree>,
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                MutableSpecifier<'tree>,
                PrimitiveType<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                TokenBindingPattern<'tree>,
                TokenRepetitionPattern<'tree>,
                TokenTreePattern<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "trait_bounds", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TraitBounds<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TraitBounds<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait_bounds" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "trait_bounds",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TraitBounds<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TraitBounds<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either4<
                Type<'tree>,
                HigherRankedTraitBound<'tree>,
                Lifetime<'tree>,
                RemovedTraitBound<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either4<
                    Type<'tree>,
                    HigherRankedTraitBound<'tree>,
                    Lifetime<'tree>,
                    RemovedTraitBound<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either4<
                Type<'tree>,
                HigherRankedTraitBound<'tree>,
                Lifetime<'tree>,
                RemovedTraitBound<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either4<
                    Type<'tree>,
                    HigherRankedTraitBound<'tree>,
                    Lifetime<'tree>,
                    RemovedTraitBound<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "trait_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TraitItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TraitItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "trait_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TraitItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TraitItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, DeclarationList<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "bounds", "`")]
    pub fn bounds(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TraitBounds<'tree>>> {
        self.0.child_by_field_name("bounds")
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "try_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TryExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TryExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "try_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "try_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TryExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TryExpression<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "tuple_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TupleExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TupleExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "tuple_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TupleExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TupleExpression<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<Expression<'tree>, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    Expression<'tree>,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "tuple_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TuplePattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TuplePattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "tuple_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TuplePattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TuplePattern<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.children(&mut c).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.child(i).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "tuple_struct_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TupleStructPattern<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TupleStructPattern<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_struct_pattern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "tuple_struct_pattern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TupleStructPattern<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TupleStructPattern<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Identifier<'tree>, ScopedIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.children(&mut c).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0.child(i).map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "tuple_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TupleType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TupleType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "tuple_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TupleType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TupleType<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.children(&mut c).map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child(i).map(<Type<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "type_arguments", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TypeArguments<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TypeArguments<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_arguments" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type_arguments",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TypeArguments<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TypeArguments<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either5<
                Literal<'tree>,
                Type<'tree>,
                Block<'tree>,
                Lifetime<'tree>,
                TypeBinding<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either5<
                    Literal<'tree>,
                    Type<'tree>,
                    Block<'tree>,
                    Lifetime<'tree>,
                    TypeBinding<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either5<
                Literal<'tree>,
                Type<'tree>,
                Block<'tree>,
                Lifetime<'tree>,
                TypeBinding<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either5<
                    Literal<'tree>,
                    Type<'tree>,
                    Block<'tree>,
                    Lifetime<'tree>,
                    TypeBinding<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "type_binding", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TypeBinding<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TypeBinding<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_binding" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type_binding",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TypeBinding<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TypeBinding<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    pub fn type_arguments(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeArguments<'tree>>> {
        self.0.child_by_field_name("type_arguments")
    }
}
#[doc = concat!("Typed node `", "type_cast_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TypeCastExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TypeCastExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_cast_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type_cast_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TypeCastExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TypeCastExpression<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "type_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TypeItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TypeItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TypeItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TypeItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    pub fn r#type(&self) -> tree_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "type_parameters", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TypeParameters<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TypeParameters<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_parameters" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type_parameters",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TypeParameters<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TypeParameters<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either6<
                ConstParameter<'tree>,
                ConstrainedTypeParameter<'tree>,
                Lifetime<'tree>,
                Metavariable<'tree>,
                OptionalTypeParameter<'tree>,
                TypeIdentifier<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either6<
                    ConstParameter<'tree>,
                    ConstrainedTypeParameter<'tree>,
                    Lifetime<'tree>,
                    Metavariable<'tree>,
                    OptionalTypeParameter<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either6<
                ConstParameter<'tree>,
                ConstrainedTypeParameter<'tree>,
                Lifetime<'tree>,
                Metavariable<'tree>,
                OptionalTypeParameter<'tree>,
                TypeIdentifier<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either6<
                    ConstParameter<'tree>,
                    ConstrainedTypeParameter<'tree>,
                    Lifetime<'tree>,
                    Metavariable<'tree>,
                    OptionalTypeParameter<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "unary_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UnaryExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UnaryExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unary_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "unary_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UnaryExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UnaryExpression<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "union_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UnionItem<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UnionItem<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "union_item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "union_item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UnionItem<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UnionItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(
        &self,
    ) -> tree_sitter_lib::NodeResult<'tree, FieldDeclarationList<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    pub fn name(&self) -> tree_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    pub fn type_parameters(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0.child_by_field_name("type_parameters")
    }
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either2<
                VisibilityModifier<'tree>,
                WhereClause<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either2<
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "unit_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UnitExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UnitExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unit_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "unit_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UnitExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UnitExpression<'tree> {}
#[doc = concat!("Typed node `", "unit_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UnitType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UnitType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unit_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "unit_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UnitType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UnitType<'tree> {}
#[doc = concat!("Typed node `", "unsafe_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UnsafeBlock<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UnsafeBlock<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unsafe_block" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "unsafe_block",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UnsafeBlock<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UnsafeBlock<'tree> {
    ///Get the node's child
    pub fn child(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child(0)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "use_as_clause", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UseAsClause<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UseAsClause<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_as_clause" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "use_as_clause",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UseAsClause<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UseAsClause<'tree> {
    #[doc = concat!("Get the field `", "alias", "`")]
    pub fn alias(&self) -> tree_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("alias")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    pub fn path(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either6<
            Crate<'tree>,
            Identifier<'tree>,
            Metavariable<'tree>,
            ScopedIdentifier<'tree>,
            _Self<'tree>,
            _Super<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("path")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "use_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UseDeclaration<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UseDeclaration<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_declaration" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "use_declaration",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UseDeclaration<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UseDeclaration<'tree> {
    #[doc = concat!("Get the field `", "argument", "`")]
    pub fn argument(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either10<
            Crate<'tree>,
            Identifier<'tree>,
            Metavariable<'tree>,
            ScopedIdentifier<'tree>,
            ScopedUseList<'tree>,
            _Self<'tree>,
            _Super<'tree>,
            UseAsClause<'tree>,
            UseList<'tree>,
            UseWildcard<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("argument")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "use_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UseList<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UseList<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_list" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "use_list",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UseList<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UseList<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either10<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                ScopedUseList<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                UseAsClause<'tree>,
                UseList<'tree>,
                UseWildcard<'tree>,
            >,
        >,
    > {
        self.0
            .children(&mut c)
            .map(
                <tree_sitter_lib::either_n::Either10<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    ScopedUseList<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    UseAsClause<'tree>,
                    UseList<'tree>,
                    UseWildcard<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either10<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                ScopedUseList<'tree>,
                _Self<'tree>,
                _Super<'tree>,
                UseAsClause<'tree>,
                UseList<'tree>,
                UseWildcard<'tree>,
            >,
        >,
    > {
        self.0
            .child(i)
            .map(
                <tree_sitter_lib::either_n::Either10<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    ScopedUseList<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    UseAsClause<'tree>,
                    UseList<'tree>,
                    UseWildcard<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[doc = concat!("Typed node `", "use_wildcard", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UseWildcard<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UseWildcard<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_wildcard" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "use_wildcard",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UseWildcard<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UseWildcard<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either6<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "variadic_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct VariadicParameter<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for VariadicParameter<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "variadic_parameter" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "variadic_parameter",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for VariadicParameter<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> VariadicParameter<'tree> {}
#[doc = concat!("Typed node `", "visibility_modifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct VisibilityModifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for VisibilityModifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "visibility_modifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "visibility_modifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for VisibilityModifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> VisibilityModifier<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<
        tree_sitter_lib::NodeResult<
            'tree,
            tree_sitter_lib::either_n::Either6<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "where_clause", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct WhereClause<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for WhereClause<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where_clause" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "where_clause",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for WhereClause<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> WhereClause<'tree> {
    ///Get the node's children
    ///This is guaranteed to return at least one child
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = tree_sitter_lib::NodeResult<'tree, WherePredicate<'tree>>,
    > {
        self.0.children(&mut c).map(<WherePredicate<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, WherePredicate<'tree>>> {
        self.0.child(i).map(<WherePredicate<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "where_predicate", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct WherePredicate<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for WherePredicate<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where_predicate" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "where_predicate",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for WherePredicate<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> WherePredicate<'tree> {
    #[doc = concat!("Get the field `", "bounds", "`")]
    pub fn bounds(&self) -> tree_sitter_lib::NodeResult<'tree, TraitBounds<'tree>> {
        self.0
            .child_by_field_name("bounds")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "left", "`")]
    pub fn left(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either10<
            ArrayType<'tree>,
            GenericType<'tree>,
            HigherRankedTraitBound<'tree>,
            Lifetime<'tree>,
            PointerType<'tree>,
            PrimitiveType<'tree>,
            ReferenceType<'tree>,
            ScopedTypeIdentifier<'tree>,
            TupleType<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("left")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "while_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct WhileExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for WhileExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "while_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "while_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for WhileExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> WhileExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    pub fn body(&self) -> tree_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "condition", "`")]
    pub fn condition(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either3<
            Expression<'tree>,
            LetChain<'tree>,
            LetCondition<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("condition")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's child
    pub fn child(&self) -> Option<tree_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "yield_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct YieldExpression<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for YieldExpression<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "yield_expression" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "yield_expression",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for YieldExpression<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> YieldExpression<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "!", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Not<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Not<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "!" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "!",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Not<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Not<'tree> {}
#[doc = concat!("Typed node `", "!=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct NotEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for NotEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "!=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "!=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for NotEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> NotEq<'tree> {}
#[doc = concat!("Typed node `", "\"", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DoubleQuote<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DoubleQuote<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "\"" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "\"",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DoubleQuote<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DoubleQuote<'tree> {}
#[doc = concat!("Typed node `", "#", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Hash<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Hash<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "#" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "#",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Hash<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Hash<'tree> {}
#[doc = concat!("Typed node `", "$", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Dollar<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Dollar<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "$" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "$",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Dollar<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Dollar<'tree> {}
#[doc = concat!("Typed node `", "%", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Mod<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Mod<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "%" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "%",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Mod<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Mod<'tree> {}
#[doc = concat!("Typed node `", "%=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ModEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ModEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "%=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "%=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ModEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ModEq<'tree> {}
#[doc = concat!("Typed node `", "&", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct And<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for And<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "&" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "&",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for And<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> And<'tree> {}
#[doc = concat!("Typed node `", "&&", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AndAnd<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AndAnd<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "&&" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "&&",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AndAnd<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AndAnd<'tree> {}
#[doc = concat!("Typed node `", "&=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AndEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AndEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "&=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "&=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AndEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AndEq<'tree> {}
#[doc = concat!("Typed node `", "'", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Quote<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Quote<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "'" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "'",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Quote<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Quote<'tree> {}
#[doc = concat!("Typed node `", "(", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LParen<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LParen<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "(" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "(",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LParen<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LParen<'tree> {}
#[doc = concat!("Typed node `", ")", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RParen<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RParen<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ")" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ")",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RParen<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RParen<'tree> {}
#[doc = concat!("Typed node `", "*", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Mul<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Mul<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "*" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "*",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Mul<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Mul<'tree> {}
#[doc = concat!("Typed node `", "*=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MulEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MulEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "*=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "*=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MulEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MulEq<'tree> {}
#[doc = concat!("Typed node `", "+", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Add<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Add<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "+" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "+",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Add<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Add<'tree> {}
#[doc = concat!("Typed node `", "+=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AddEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AddEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "+=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "+=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AddEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AddEq<'tree> {}
#[doc = concat!("Typed node `", ",", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Comma<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Comma<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "," {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ",",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Comma<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Comma<'tree> {}
#[doc = concat!("Typed node `", "-", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Sub<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Sub<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "-" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "-",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Sub<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Sub<'tree> {}
#[doc = concat!("Typed node `", "-=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct SubEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for SubEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "-=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "-=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for SubEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> SubEq<'tree> {}
#[doc = concat!("Typed node `", "->", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct SubGt<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for SubGt<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "->" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "->",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for SubGt<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> SubGt<'tree> {}
#[doc = concat!("Typed node `", ".", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Dot<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Dot<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "." {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ".",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Dot<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Dot<'tree> {}
#[doc = concat!("Typed node `", "..", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DotDot<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DotDot<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ".." {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "..",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DotDot<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DotDot<'tree> {}
#[doc = concat!("Typed node `", "...", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DotDotDot<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DotDotDot<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "..." {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "...",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DotDotDot<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DotDotDot<'tree> {}
#[doc = concat!("Typed node `", "..=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DotDotEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DotDotEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "..=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "..=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DotDotEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DotDotEq<'tree> {}
#[doc = concat!("Typed node `", "/", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Div<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Div<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "/" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "/",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Div<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Div<'tree> {}
#[doc = concat!("Typed node `", "/=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DivEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DivEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "/=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "/=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DivEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DivEq<'tree> {}
#[doc = concat!("Typed node `", ":", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Colon<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Colon<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ":" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ":",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Colon<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Colon<'tree> {}
#[doc = concat!("Typed node `", "::", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ColonColon<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ColonColon<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "::" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "::",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ColonColon<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ColonColon<'tree> {}
#[doc = concat!("Typed node `", ";", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Semicolon<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Semicolon<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ";" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ";",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Semicolon<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Semicolon<'tree> {}
#[doc = concat!("Typed node `", "<", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Lt<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Lt<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "<",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Lt<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Lt<'tree> {}
#[doc = concat!("Typed node `", "<<", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LtLt<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LtLt<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<<" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "<<",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LtLt<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LtLt<'tree> {}
#[doc = concat!("Typed node `", "<<=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LtLtEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LtLtEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<<=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "<<=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LtLtEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LtLtEq<'tree> {}
#[doc = concat!("Typed node `", "<=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LtEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LtEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "<=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LtEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LtEq<'tree> {}
#[doc = concat!("Typed node `", "=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Eq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Eq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Eq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Eq<'tree> {}
#[doc = concat!("Typed node `", "==", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EqEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EqEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "==" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "==",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EqEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EqEq<'tree> {}
#[doc = concat!("Typed node `", "=>", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EqGt<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EqGt<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "=>" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "=>",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EqGt<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EqGt<'tree> {}
#[doc = concat!("Typed node `", ">", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Gt<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Gt<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ">",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Gt<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Gt<'tree> {}
#[doc = concat!("Typed node `", ">=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct GtEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for GtEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ">=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for GtEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> GtEq<'tree> {}
#[doc = concat!("Typed node `", ">>", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct GtGt<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for GtGt<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">>" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ">>",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for GtGt<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> GtGt<'tree> {}
#[doc = concat!("Typed node `", ">>=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct GtGtEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for GtGtEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">>=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: ">>=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for GtGtEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> GtGtEq<'tree> {}
#[doc = concat!("Typed node `", "?", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Question<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Question<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "?" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "?",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Question<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Question<'tree> {}
#[doc = concat!("Typed node `", "@", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct At<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for At<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "@" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "@",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for At<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> At<'tree> {}
#[doc = concat!("Typed node `", "[", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LBracket<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LBracket<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "[" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "[",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LBracket<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LBracket<'tree> {}
#[doc = concat!("Typed node `", "]", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RBracket<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RBracket<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "]" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "]",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RBracket<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RBracket<'tree> {}
#[doc = concat!("Typed node `", "^", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BitXor<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BitXor<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "^" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "^",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BitXor<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BitXor<'tree> {}
#[doc = concat!("Typed node `", "^=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BitXorEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BitXorEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "^=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "^=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BitXorEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BitXorEq<'tree> {}
#[doc = concat!("Typed node `", "_", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct __<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for __<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "_" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "_",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for __<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> __<'tree> {}
#[doc = concat!("Typed node `", "as", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AS<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AS<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "as" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "as",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AS<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AS<'tree> {}
#[doc = concat!("Typed node `", "async", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ASYNC<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ASYNC<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "async" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "async",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ASYNC<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ASYNC<'tree> {}
#[doc = concat!("Typed node `", "await", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct AWAIT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for AWAIT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "await" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "await",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for AWAIT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> AWAIT<'tree> {}
#[doc = concat!("Typed node `", "block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BLOCK<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BLOCK<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "block",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BLOCK<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BLOCK<'tree> {}
#[doc = concat!("Typed node `", "block_comment", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BlockComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BlockComment<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block_comment" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "block_comment",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BlockComment<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BlockComment<'tree> {}
#[doc = concat!("Typed node `", "break", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct BREAK<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for BREAK<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "break" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "break",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for BREAK<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> BREAK<'tree> {}
#[doc = concat!("Typed node `", "char_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct CharLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for CharLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "char_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "char_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for CharLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> CharLiteral<'tree> {}
#[doc = concat!("Typed node `", "const", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct CONST<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for CONST<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "const",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for CONST<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> CONST<'tree> {}
#[doc = concat!("Typed node `", "continue", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct CONTINUE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for CONTINUE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "continue" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "continue",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for CONTINUE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> CONTINUE<'tree> {}
#[doc = concat!("Typed node `", "crate", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Crate<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Crate<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "crate" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "crate",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Crate<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Crate<'tree> {}
#[doc = concat!("Typed node `", "default", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DEFAULT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DEFAULT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "default" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "default",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DEFAULT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DEFAULT<'tree> {}
#[doc = concat!("Typed node `", "dyn", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct DYN<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for DYN<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dyn" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "dyn",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for DYN<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> DYN<'tree> {}
#[doc = concat!("Typed node `", "else", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ELSE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ELSE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "else" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "else",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ELSE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ELSE<'tree> {}
#[doc = concat!("Typed node `", "enum", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ENUM<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ENUM<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "enum",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ENUM<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ENUM<'tree> {}
#[doc = concat!("Typed node `", "escape_sequence", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EscapeSequence<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EscapeSequence<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "escape_sequence",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EscapeSequence<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EscapeSequence<'tree> {}
#[doc = concat!("Typed node `", "expr", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EXPR<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EXPR<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expr" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "expr",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EXPR<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EXPR<'tree> {}
#[doc = concat!("Typed node `", "extern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct EXTERN<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for EXTERN<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "extern",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for EXTERN<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> EXTERN<'tree> {}
#[doc = concat!("Typed node `", "false", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FALSE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FALSE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "false" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "false",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FALSE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FALSE<'tree> {}
#[doc = concat!("Typed node `", "field_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FieldIdentifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FieldIdentifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_identifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "field_identifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FieldIdentifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FieldIdentifier<'tree> {}
#[doc = concat!("Typed node `", "float_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FloatLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FloatLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "float_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "float_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FloatLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FloatLiteral<'tree> {}
#[doc = concat!("Typed node `", "fn", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FN<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FN<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "fn" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "fn",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FN<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FN<'tree> {}
#[doc = concat!("Typed node `", "for", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct FOR<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for FOR<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "for",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for FOR<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> FOR<'tree> {}
#[doc = concat!("Typed node `", "ident", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IDENT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IDENT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ident" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "ident",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IDENT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IDENT<'tree> {}
#[doc = concat!("Typed node `", "identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Identifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Identifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "identifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Identifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Identifier<'tree> {}
#[doc = concat!("Typed node `", "if", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IF<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IF<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "if" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "if",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IF<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IF<'tree> {}
#[doc = concat!("Typed node `", "impl", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IMPL<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IMPL<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "impl" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "impl",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IMPL<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IMPL<'tree> {}
#[doc = concat!("Typed node `", "in", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IN<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IN<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "in" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "in",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IN<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IN<'tree> {}
#[doc = concat!("Typed node `", "integer_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct IntegerLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for IntegerLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "integer_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "integer_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for IntegerLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> IntegerLiteral<'tree> {}
#[doc = concat!("Typed node `", "item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ITEM<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ITEM<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "item" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "item",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ITEM<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ITEM<'tree> {}
#[doc = concat!("Typed node `", "let", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LET<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LET<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "let",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LET<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LET<'tree> {}
#[doc = concat!("Typed node `", "lifetime", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LIFETIME<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LIFETIME<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "lifetime" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "lifetime",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LIFETIME<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LIFETIME<'tree> {}
#[doc = concat!("Typed node `", "line_comment", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LineComment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LineComment<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "line_comment" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "line_comment",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LineComment<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LineComment<'tree> {}
#[doc = concat!("Typed node `", "literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LITERAL<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LITERAL<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LITERAL<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LITERAL<'tree> {}
#[doc = concat!("Typed node `", "loop", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LOOP<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LOOP<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "loop",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LOOP<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LOOP<'tree> {}
#[doc = concat!("Typed node `", "macro_rules!", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MACROU5FRULESNot<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MACROU5FRULESNot<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_rules!" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "macro_rules!",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MACROU5FRULESNot<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MACROU5FRULESNot<'tree> {}
#[doc = concat!("Typed node `", "match", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MATCH<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MATCH<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "match",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MATCH<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MATCH<'tree> {}
#[doc = concat!("Typed node `", "meta", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct META<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for META<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "meta" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "meta",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for META<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> META<'tree> {}
#[doc = concat!("Typed node `", "metavariable", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Metavariable<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Metavariable<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "metavariable" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "metavariable",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Metavariable<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Metavariable<'tree> {}
#[doc = concat!("Typed node `", "mod", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MOD<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MOD<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mod" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "mod",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MOD<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MOD<'tree> {}
#[doc = concat!("Typed node `", "move", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MOVE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MOVE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "move" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "move",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MOVE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MOVE<'tree> {}
#[doc = concat!("Typed node `", "mutable_specifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct MutableSpecifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for MutableSpecifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mutable_specifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "mutable_specifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for MutableSpecifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> MutableSpecifier<'tree> {}
#[doc = concat!("Typed node `", "pat", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct PAT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for PAT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pat" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "pat",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for PAT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> PAT<'tree> {}
#[doc = concat!("Typed node `", "path", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct PATH<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for PATH<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "path" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "path",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for PATH<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> PATH<'tree> {}
#[doc = concat!("Typed node `", "primitive_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct PrimitiveType<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for PrimitiveType<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "primitive_type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "primitive_type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for PrimitiveType<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> PrimitiveType<'tree> {}
#[doc = concat!("Typed node `", "pub", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct PUB<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for PUB<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pub" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "pub",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for PUB<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> PUB<'tree> {}
#[doc = concat!("Typed node `", "raw_string_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RawStringLiteral<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RawStringLiteral<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "raw_string_literal" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "raw_string_literal",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RawStringLiteral<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RawStringLiteral<'tree> {}
#[doc = concat!("Typed node `", "ref", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct REF<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for REF<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ref" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "ref",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for REF<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> REF<'tree> {}
#[doc = concat!("Typed node `", "return", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RETURN<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RETURN<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "return",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RETURN<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RETURN<'tree> {}
#[doc = concat!("Typed node `", "self", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct _Self<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for _Self<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "self" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "self",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for _Self<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> _Self<'tree> {}
#[doc = concat!("Typed node `", "shorthand_field_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct ShorthandFieldIdentifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for ShorthandFieldIdentifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "shorthand_field_identifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "shorthand_field_identifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for ShorthandFieldIdentifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> ShorthandFieldIdentifier<'tree> {}
#[doc = concat!("Typed node `", "static", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct STATIC<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for STATIC<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "static" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "static",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for STATIC<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> STATIC<'tree> {}
#[doc = concat!("Typed node `", "stmt", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct STMT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for STMT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "stmt" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "stmt",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for STMT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> STMT<'tree> {}
#[doc = concat!("Typed node `", "struct", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct STRUCT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for STRUCT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "struct",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for STRUCT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> STRUCT<'tree> {}
#[doc = concat!("Typed node `", "super", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct _Super<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for _Super<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "super" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "super",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for _Super<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> _Super<'tree> {}
#[doc = concat!("Typed node `", "trait", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TRAIT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TRAIT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "trait",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TRAIT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TRAIT<'tree> {}
#[doc = concat!("Typed node `", "true", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TRUE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TRUE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "true" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "true",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TRUE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TRUE<'tree> {}
#[doc = concat!("Typed node `", "tt", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TT<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TT<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tt" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "tt",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TT<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TT<'tree> {}
#[doc = concat!("Typed node `", "ty", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TY<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TY<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ty" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "ty",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TY<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TY<'tree> {}
#[doc = concat!("Typed node `", "type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TYPE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TYPE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TYPE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TYPE<'tree> {}
#[doc = concat!("Typed node `", "type_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct TypeIdentifier<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for TypeIdentifier<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_identifier" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "type_identifier",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for TypeIdentifier<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> TypeIdentifier<'tree> {}
#[doc = concat!("Typed node `", "union", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UNION<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UNION<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "union" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "union",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UNION<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UNION<'tree> {}
#[doc = concat!("Typed node `", "unsafe", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct UNSAFE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for UNSAFE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unsafe" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "unsafe",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for UNSAFE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> UNSAFE<'tree> {}
#[doc = concat!("Typed node `", "use", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct USE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for USE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "use",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for USE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> USE<'tree> {}
#[doc = concat!("Typed node `", "vis", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct VIS<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for VIS<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "vis" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "vis",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for VIS<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> VIS<'tree> {}
#[doc = concat!("Typed node `", "where", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct WHERE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for WHERE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "where",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for WHERE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> WHERE<'tree> {}
#[doc = concat!("Typed node `", "while", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct WHILE<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for WHILE<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "while" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "while",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for WHILE<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> WHILE<'tree> {}
#[doc = concat!("Typed node `", "yield", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct YIELD<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for YIELD<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "yield" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "yield",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for YIELD<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> YIELD<'tree> {}
#[doc = concat!("Typed node `", "{", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct LBrace<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for LBrace<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "{" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "{",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for LBrace<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> LBrace<'tree> {}
#[doc = concat!("Typed node `", "|", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Or<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Or<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "|" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "|",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Or<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Or<'tree> {}
#[doc = concat!("Typed node `", "|=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct OrEq<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for OrEq<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "|=" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "|=",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for OrEq<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> OrEq<'tree> {}
#[doc = concat!("Typed node `", "||", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct OrOr<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for OrOr<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "||" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "||",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for OrOr<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> OrOr<'tree> {}
#[doc = concat!("Typed node `", "}", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct RBrace<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for RBrace<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "}" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "}",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for RBrace<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> RBrace<'tree> {}
