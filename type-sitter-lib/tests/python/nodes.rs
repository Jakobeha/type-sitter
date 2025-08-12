/**Typed node `_compound_statement`

This node type has subtypes:

- `class_definition` ([`ClassDefinition`])
- `decorated_definition` ([`DecoratedDefinition`])
- `for_statement` ([`ForStatement`])
- `function_definition` ([`FunctionDefinition`])
- `if_statement` ([`IfStatement`])
- `match_statement` ([`MatchStatement`])
- `try_statement` ([`TryStatement`])
- `while_statement` ([`WhileStatement`])
- `with_statement` ([`WithStatement`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum CompoundStatement<'tree> {
    ClassDefinition(ClassDefinition<'tree>),
    DecoratedDefinition(DecoratedDefinition<'tree>),
    ForStatement(ForStatement<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    IfStatement(IfStatement<'tree>),
    MatchStatement(MatchStatement<'tree>),
    TryStatement(TryStatement<'tree>),
    WhileStatement(WhileStatement<'tree>),
    WithStatement(WithStatement<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> CompoundStatement<'tree> {
    ///Returns the node if it is of type `class_definition` ([`ClassDefinition`]), otherwise returns `None`
    #[inline]
    pub fn as_class_definition(self) -> ::std::option::Option<ClassDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ClassDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `decorated_definition` ([`DecoratedDefinition`]), otherwise returns `None`
    #[inline]
    pub fn as_decorated_definition(
        self,
    ) -> ::std::option::Option<DecoratedDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DecoratedDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `for_statement` ([`ForStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_for_statement(self) -> ::std::option::Option<ForStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ForStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `function_definition` ([`FunctionDefinition`]), otherwise returns `None`
    #[inline]
    pub fn as_function_definition(
        self,
    ) -> ::std::option::Option<FunctionDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `if_statement` ([`IfStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_if_statement(self) -> ::std::option::Option<IfStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::IfStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `match_statement` ([`MatchStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_match_statement(self) -> ::std::option::Option<MatchStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::MatchStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `try_statement` ([`TryStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_try_statement(self) -> ::std::option::Option<TryStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TryStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `while_statement` ([`WhileStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_while_statement(self) -> ::std::option::Option<WhileStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::WhileStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `with_statement` ([`WithStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_with_statement(self) -> ::std::option::Option<WithStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::WithStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for CompoundStatement<'tree> {
    type WithLifetime<'a> = CompoundStatement<'a>;
    const KIND: &'static str = "_compound_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if let Ok(this) = <ClassDefinition<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ClassDefinition(this));
        }
        if let Ok(this) = <DecoratedDefinition<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::DecoratedDefinition(this));
        }
        if let Ok(this) = <ForStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ForStatement(this));
        }
        if let Ok(this) = <FunctionDefinition<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::FunctionDefinition(this));
        }
        if let Ok(this) = <IfStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::IfStatement(this));
        }
        if let Ok(this) = <MatchStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::MatchStatement(this));
        }
        if let Ok(this) = <TryStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::TryStatement(this));
        }
        if let Ok(this) = <WhileStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::WhileStatement(this));
        }
        if let Ok(this) = <WithStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::WithStatement(this));
        }
        Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::ClassDefinition(x) => ::type_sitter_lib::Node::raw(x),
            Self::DecoratedDefinition(x) => ::type_sitter_lib::Node::raw(x),
            Self::ForStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::FunctionDefinition(x) => ::type_sitter_lib::Node::raw(x),
            Self::IfStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::MatchStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::TryStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::WhileStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::WithStatement(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::ClassDefinition(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::DecoratedDefinition(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ForStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::FunctionDefinition(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::IfStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::MatchStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::TryStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::WhileStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::WithStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        match self {
            Self::ClassDefinition(x) => x.into_raw(),
            Self::DecoratedDefinition(x) => x.into_raw(),
            Self::ForStatement(x) => x.into_raw(),
            Self::FunctionDefinition(x) => x.into_raw(),
            Self::IfStatement(x) => x.into_raw(),
            Self::MatchStatement(x) => x.into_raw(),
            Self::TryStatement(x) => x.into_raw(),
            Self::WhileStatement(x) => x.into_raw(),
            Self::WithStatement(x) => x.into_raw(),
        }
    }
}
/**Typed node `_simple_statement`

This node type has subtypes:

- `assert_statement` ([`AssertStatement`])
- `break_statement` ([`BreakStatement`])
- `continue_statement` ([`ContinueStatement`])
- `delete_statement` ([`DeleteStatement`])
- `exec_statement` ([`ExecStatement`])
- `expression_statement` ([`ExpressionStatement`])
- `future_import_statement` ([`FutureImportStatement`])
- `global_statement` ([`GlobalStatement`])
- `import_from_statement` ([`ImportFromStatement`])
- `import_statement` ([`ImportStatement`])
- `nonlocal_statement` ([`NonlocalStatement`])
- `pass_statement` ([`PassStatement`])
- `print_statement` ([`PrintStatement`])
- `raise_statement` ([`RaiseStatement`])
- `return_statement` ([`ReturnStatement`])
- `type_alias_statement` ([`TypeAliasStatement`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum SimpleStatement<'tree> {
    AssertStatement(AssertStatement<'tree>),
    BreakStatement(BreakStatement<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DeleteStatement(DeleteStatement<'tree>),
    ExecStatement(ExecStatement<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    FutureImportStatement(FutureImportStatement<'tree>),
    GlobalStatement(GlobalStatement<'tree>),
    ImportFromStatement(ImportFromStatement<'tree>),
    ImportStatement(ImportStatement<'tree>),
    NonlocalStatement(NonlocalStatement<'tree>),
    PassStatement(PassStatement<'tree>),
    PrintStatement(PrintStatement<'tree>),
    RaiseStatement(RaiseStatement<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    TypeAliasStatement(TypeAliasStatement<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> SimpleStatement<'tree> {
    ///Returns the node if it is of type `assert_statement` ([`AssertStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_assert_statement(self) -> ::std::option::Option<AssertStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::AssertStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `break_statement` ([`BreakStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_break_statement(self) -> ::std::option::Option<BreakStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BreakStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `continue_statement` ([`ContinueStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_continue_statement(
        self,
    ) -> ::std::option::Option<ContinueStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ContinueStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `delete_statement` ([`DeleteStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_delete_statement(self) -> ::std::option::Option<DeleteStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DeleteStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `exec_statement` ([`ExecStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_exec_statement(self) -> ::std::option::Option<ExecStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ExecStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `expression_statement` ([`ExpressionStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_expression_statement(
        self,
    ) -> ::std::option::Option<ExpressionStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ExpressionStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `future_import_statement` ([`FutureImportStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_future_import_statement(
        self,
    ) -> ::std::option::Option<FutureImportStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FutureImportStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `global_statement` ([`GlobalStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_global_statement(self) -> ::std::option::Option<GlobalStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::GlobalStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `import_from_statement` ([`ImportFromStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_import_from_statement(
        self,
    ) -> ::std::option::Option<ImportFromStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ImportFromStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `import_statement` ([`ImportStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_import_statement(self) -> ::std::option::Option<ImportStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ImportStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `nonlocal_statement` ([`NonlocalStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_nonlocal_statement(
        self,
    ) -> ::std::option::Option<NonlocalStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::NonlocalStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `pass_statement` ([`PassStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_pass_statement(self) -> ::std::option::Option<PassStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PassStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `print_statement` ([`PrintStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_print_statement(self) -> ::std::option::Option<PrintStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PrintStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `raise_statement` ([`RaiseStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_raise_statement(self) -> ::std::option::Option<RaiseStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::RaiseStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `return_statement` ([`ReturnStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_return_statement(self) -> ::std::option::Option<ReturnStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ReturnStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `type_alias_statement` ([`TypeAliasStatement`]), otherwise returns `None`
    #[inline]
    pub fn as_type_alias_statement(
        self,
    ) -> ::std::option::Option<TypeAliasStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypeAliasStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for SimpleStatement<'tree> {
    type WithLifetime<'a> = SimpleStatement<'a>;
    const KIND: &'static str = "_simple_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if let Ok(this) = <AssertStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::AssertStatement(this));
        }
        if let Ok(this) = <BreakStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::BreakStatement(this));
        }
        if let Ok(this) = <ContinueStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ContinueStatement(this));
        }
        if let Ok(this) = <DeleteStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::DeleteStatement(this));
        }
        if let Ok(this) = <ExecStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ExecStatement(this));
        }
        if let Ok(this) = <ExpressionStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ExpressionStatement(this));
        }
        if let Ok(this) = <FutureImportStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::FutureImportStatement(this));
        }
        if let Ok(this) = <GlobalStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::GlobalStatement(this));
        }
        if let Ok(this) = <ImportFromStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ImportFromStatement(this));
        }
        if let Ok(this) = <ImportStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ImportStatement(this));
        }
        if let Ok(this) = <NonlocalStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::NonlocalStatement(this));
        }
        if let Ok(this) = <PassStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::PassStatement(this));
        }
        if let Ok(this) = <PrintStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::PrintStatement(this));
        }
        if let Ok(this) = <RaiseStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::RaiseStatement(this));
        }
        if let Ok(this) = <ReturnStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ReturnStatement(this));
        }
        if let Ok(this) = <TypeAliasStatement<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::TypeAliasStatement(this));
        }
        Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::AssertStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::BreakStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::ContinueStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::DeleteStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::ExecStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::ExpressionStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::FutureImportStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::GlobalStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::ImportFromStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::ImportStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::NonlocalStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::PassStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::PrintStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::RaiseStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::ReturnStatement(x) => ::type_sitter_lib::Node::raw(x),
            Self::TypeAliasStatement(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::AssertStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::BreakStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ContinueStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::DeleteStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ExecStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ExpressionStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::FutureImportStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::GlobalStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ImportFromStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ImportStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::NonlocalStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::PassStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::PrintStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::RaiseStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ReturnStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::TypeAliasStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        match self {
            Self::AssertStatement(x) => x.into_raw(),
            Self::BreakStatement(x) => x.into_raw(),
            Self::ContinueStatement(x) => x.into_raw(),
            Self::DeleteStatement(x) => x.into_raw(),
            Self::ExecStatement(x) => x.into_raw(),
            Self::ExpressionStatement(x) => x.into_raw(),
            Self::FutureImportStatement(x) => x.into_raw(),
            Self::GlobalStatement(x) => x.into_raw(),
            Self::ImportFromStatement(x) => x.into_raw(),
            Self::ImportStatement(x) => x.into_raw(),
            Self::NonlocalStatement(x) => x.into_raw(),
            Self::PassStatement(x) => x.into_raw(),
            Self::PrintStatement(x) => x.into_raw(),
            Self::RaiseStatement(x) => x.into_raw(),
            Self::ReturnStatement(x) => x.into_raw(),
            Self::TypeAliasStatement(x) => x.into_raw(),
        }
    }
}
/**Typed node `aliased_import`

This node has these fields:

- `alias`: `identifier` ([`Identifier`])
- `name`: `dotted_name` ([`DottedName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AliasedImport<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AliasedImport<'tree> {
    /**Get the field `alias`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn alias(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("alias")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `name`.

This child has type `dotted_name` ([`DottedName`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, DottedName<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(<DottedName<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for AliasedImport<'tree> {
    type WithLifetime<'a> = AliasedImport<'a>;
    const KIND: &'static str = "aliased_import";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "aliased_import" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "aliased_import");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `argument_list`

This node has named children of type `{dictionary_splat | expression | keyword_argument | list_splat | parenthesized_expression}*`:

- [`DictionarySplat`]
- [`Expression`]
- [`KeywordArgument`]
- [`ListSplat`]
- [`ParenthesizedExpression`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArgumentList<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArgumentList<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ArgumentList<'tree> {
    type Child = anon_unions::DictionarySplat_Expression_KeywordArgument_ListSplat_ParenthesizedExpression<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ArgumentList<'tree> {
    type WithLifetime<'a> = ArgumentList<'a>;
    const KIND: &'static str = "argument_list";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "argument_list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "argument_list");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `as_pattern`

This node has these fields:

- `alias`: `as_pattern_target?` ([`AsPatternTarget`])

And additional named children of type `{case_pattern | expression | identifier}+`:

- [`CasePattern`]
- [`Expression`]
- [`Identifier`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AsPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AsPattern<'tree> {
    /**Get the optional field `alias`.

This child has type `as_pattern_target?` ([`AsPatternTarget`])*/
    #[inline]
    pub fn alias(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, AsPatternTarget<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("alias")
            .map(
                <AsPatternTarget<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{case_pattern | expression | identifier}+`:

- [`CasePattern`]
- [`Expression`]
- [`Identifier`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::CasePattern_Expression_Identifier<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::CasePattern_Expression_Identifier<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for AsPattern<'tree> {
    type WithLifetime<'a> = AsPattern<'a>;
    const KIND: &'static str = "as_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "as_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "as_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `as_pattern_target`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AsPatternTarget<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AsPatternTarget<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for AsPatternTarget<'tree> {
    type WithLifetime<'a> = AsPatternTarget<'a>;
    const KIND: &'static str = "as_pattern_target";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "as_pattern_target" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "as_pattern_target");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `assert_statement`

This node has named children of type `expression+` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AssertStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AssertStatement<'tree> {
    /**Get the node's not-extra named children.

These children have type `expression+` ([`Expression`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for AssertStatement<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for AssertStatement<'tree> {
    type WithLifetime<'a> = AssertStatement<'a>;
    const KIND: &'static str = "assert_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "assert_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "assert_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `assignment`

This node has these fields:

- `left`: `{pattern | pattern_list}` ([`Pattern`] | [`PatternList`])
- `right`: `{assignment | augmented_assignment | expression | expression_list | pattern_list | yield}?` ([`Assignment`] | [`AugmentedAssignment`] | [`Expression`] | [`ExpressionList`] | [`PatternList`] | [`Yield`])
- `type`: `type?` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Assignment<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Assignment<'tree> {
    /**Get the field `left`.

This child has type `{pattern | pattern_list}`:

- [`Pattern`]
- [`PatternList`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::Pattern_PatternList<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::Pattern_PatternList<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `right`.

This child has type `{assignment | augmented_assignment | expression | expression_list | pattern_list | yield}?`:

- [`Assignment`]
- [`AugmentedAssignment`]
- [`Expression`]
- [`ExpressionList`]
- [`PatternList`]
- [`Yield`]
*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
                'tree,
            >,
        >,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the optional field `type`.

This child has type `type?` ([`Type`])*/
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Assignment<'tree> {
    type WithLifetime<'a> = Assignment<'a>;
    const KIND: &'static str = "assignment";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "assignment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "assignment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `attribute`

This node has these fields:

- `attribute`: `identifier` ([`Identifier`])
- `object`: `primary_expression` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Attribute<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Attribute<'tree> {
    /**Get the field `attribute`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn attribute(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("attribute")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `object`.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn object(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("object")
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Attribute<'tree> {
    type WithLifetime<'a> = Attribute<'a>;
    const KIND: &'static str = "attribute";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `augmented_assignment`

This node has these fields:

- `left`: `{pattern | pattern_list}` ([`Pattern`] | [`PatternList`])
- `operator`: `{%= | &= | **= | *= | += | -= | //= | /= | <<= | >>= | @= | ^= | |=}` ([`symbols::ModEq`] | [`symbols::AndEq`] | [`symbols::MulMulEq`] | [`symbols::MulEq`] | [`symbols::AddEq`] | [`symbols::SubEq`] | [`symbols::DivDivEq`] | [`symbols::DivEq`] | [`symbols::LtLtEq`] | [`symbols::GtGtEq`] | [`symbols::AtEq`] | [`symbols::BitXorEq`] | [`symbols::OrEq`])
- `right`: `{assignment | augmented_assignment | expression | expression_list | pattern_list | yield}` ([`Assignment`] | [`AugmentedAssignment`] | [`Expression`] | [`ExpressionList`] | [`PatternList`] | [`Yield`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AugmentedAssignment<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AugmentedAssignment<'tree> {
    /**Get the field `left`.

This child has type `{pattern | pattern_list}`:

- [`Pattern`]
- [`PatternList`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::Pattern_PatternList<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::Pattern_PatternList<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{%= | &= | **= | *= | += | -= | //= | /= | <<= | >>= | @= | ^= | |=}`:

- [`symbols::ModEq`]
- [`symbols::AndEq`]
- [`symbols::MulMulEq`]
- [`symbols::MulEq`]
- [`symbols::AddEq`]
- [`symbols::SubEq`]
- [`symbols::DivDivEq`]
- [`symbols::DivEq`]
- [`symbols::LtLtEq`]
- [`symbols::GtGtEq`]
- [`symbols::AtEq`]
- [`symbols::BitXorEq`]
- [`symbols::OrEq`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivDivEq_DivEq_LtLtEq_GtGtEq_AtEq_BitXorEq_OrEq<
            'tree,
        >,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivDivEq_DivEq_LtLtEq_GtGtEq_AtEq_BitXorEq_OrEq<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `{assignment | augmented_assignment | expression | expression_list | pattern_list | yield}`:

- [`Assignment`]
- [`AugmentedAssignment`]
- [`Expression`]
- [`ExpressionList`]
- [`PatternList`]
- [`Yield`]
*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
            'tree,
        >,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for AugmentedAssignment<'tree> {
    type WithLifetime<'a> = AugmentedAssignment<'a>;
    const KIND: &'static str = "augmented_assignment";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "augmented_assignment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "augmented_assignment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `await`

This node has a named child of type `primary_expression` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Await<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Await<'tree> {
    /**Get the node's only not-extra named child.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn primary_expression(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for Await<'tree> {
    type Child = PrimaryExpression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Await<'tree> {
    type WithLifetime<'a> = Await<'a>;
    const KIND: &'static str = "await";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "await" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "await");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `binary_operator`

This node has these fields:

- `left`: `primary_expression` ([`PrimaryExpression`])
- `operator`: `{% | & | * | ** | + | - | / | // | << | >> | @ | ^ | |}` ([`symbols::Mod`] | [`symbols::And`] | [`symbols::Mul`] | [`symbols::MulMul`] | [`symbols::Add`] | [`symbols::Sub`] | [`symbols::Div`] | [`symbols::DivDiv`] | [`symbols::LtLt`] | [`symbols::GtGt`] | [`symbols::At`] | [`symbols::BitXor`] | [`symbols::Or`])
- `right`: `primary_expression` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BinaryOperator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BinaryOperator<'tree> {
    /**Get the field `left`.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{% | & | * | ** | + | - | / | // | << | >> | @ | ^ | |}`:

- [`symbols::Mod`]
- [`symbols::And`]
- [`symbols::Mul`]
- [`symbols::MulMul`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::Div`]
- [`symbols::DivDiv`]
- [`symbols::LtLt`]
- [`symbols::GtGt`]
- [`symbols::At`]
- [`symbols::BitXor`]
- [`symbols::Or`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Mod_And_Mul_MulMul_Add_Sub_Div_DivDiv_LtLt_GtGt_At_BitXor_Or<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::Mod_And_Mul_MulMul_Add_Sub_Div_DivDiv_LtLt_GtGt_At_BitXor_Or<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for BinaryOperator<'tree> {
    type WithLifetime<'a> = BinaryOperator<'a>;
    const KIND: &'static str = "binary_operator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "binary_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "binary_operator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `block`

This node has these fields:

- `alternative`: `case_clause*` ([`CaseClause`])

And additional named children of type `{_compound_statement | _simple_statement}*`:

- [`CompoundStatement`]
- [`SimpleStatement`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Block<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Block<'tree> {
    /**Get the children of field `alternative`.

These children have type `case_clause*` ([`CaseClause`])*/
    #[inline]
    pub fn alternatives<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, CaseClause<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("alternative", &mut c.0)
            .map(<CaseClause<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the node's non-field not-extra named children.

These children have type `{_compound_statement | _simple_statement}*`:

- [`CompoundStatement`]
- [`SimpleStatement`]
*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::CompoundStatement_SimpleStatement<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::CompoundStatement_SimpleStatement<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Block<'tree> {
    type WithLifetime<'a> = Block<'a>;
    const KIND: &'static str = "block";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "block");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `boolean_operator`

This node has these fields:

- `left`: `expression` ([`Expression`])
- `operator`: `{and | or}` ([`unnamed::And`] | [`unnamed::Or`])
- `right`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BooleanOperator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BooleanOperator<'tree> {
    /**Get the field `left`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn left(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{and | or}`:

- [`unnamed::And`]
- [`unnamed::Or`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::And_Or<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::And_Or<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn right(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("right")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for BooleanOperator<'tree> {
    type WithLifetime<'a> = BooleanOperator<'a>;
    const KIND: &'static str = "boolean_operator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "boolean_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "boolean_operator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `break_statement`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BreakStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BreakStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for BreakStatement<'tree> {
    type WithLifetime<'a> = BreakStatement<'a>;
    const KIND: &'static str = "break_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "break_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "break_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `call`

This node has these fields:

- `arguments`: `{argument_list | generator_expression}` ([`ArgumentList`] | [`GeneratorExpression`])
- `function`: `primary_expression` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Call<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Call<'tree> {
    /**Get the field `arguments`.

This child has type `{argument_list | generator_expression}`:

- [`ArgumentList`]
- [`GeneratorExpression`]
*/
    #[inline]
    pub fn arguments(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::ArgumentList_GeneratorExpression<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("arguments")
            .map(
                <anon_unions::ArgumentList_GeneratorExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `function`.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn function(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("function")
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Call<'tree> {
    type WithLifetime<'a> = Call<'a>;
    const KIND: &'static str = "call";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "call" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "call");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `case_clause`

This node has these fields:

- `consequence`: `block` ([`Block`])
- `guard`: `if_clause?` ([`IfClause`])

And additional named children of type `case_pattern+` ([`CasePattern`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CaseClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CaseClause<'tree> {
    /**Get the field `consequence`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn consequence(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("consequence")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `guard`.

This child has type `if_clause?` ([`IfClause`])*/
    #[inline]
    pub fn guard(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, IfClause<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("guard")
            .map(<IfClause<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the node's non-field not-extra named children.

These children have type `case_pattern+` ([`CasePattern`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn case_patterns<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, CasePattern<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(<CasePattern<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for CaseClause<'tree> {
    type WithLifetime<'a> = CaseClause<'a>;
    const KIND: &'static str = "case_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "case_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "case_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `case_pattern`

This node has an optional named child of type `{as_pattern | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | keyword_pattern | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}?`:

- [`AsPattern`]
- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Integer`]
- [`KeywordPattern`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CasePattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CasePattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasOptionalChild<'tree> for CasePattern<'tree> {
    type Child = anon_unions::AsPattern_ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_KeywordPattern_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for CasePattern<'tree> {
    type WithLifetime<'a> = CasePattern<'a>;
    const KIND: &'static str = "case_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "case_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "case_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `chevron`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Chevron<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Chevron<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for Chevron<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Chevron<'tree> {
    type WithLifetime<'a> = Chevron<'a>;
    const KIND: &'static str = "chevron";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "chevron" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "chevron");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `class_definition`

This node has these fields:

- `body`: `block` ([`Block`])
- `name`: `identifier` ([`Identifier`])
- `superclasses`: `argument_list?` ([`ArgumentList`])
- `type_parameters`: `type_parameter?` ([`TypeParameter`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ClassDefinition<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ClassDefinition<'tree> {
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `superclasses`.

This child has type `argument_list?` ([`ArgumentList`])*/
    #[inline]
    pub fn superclasses(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, ArgumentList<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("superclasses")
            .map(<ArgumentList<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `type_parameters`.

This child has type `type_parameter?` ([`TypeParameter`])*/
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, TypeParameter<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type_parameters")
            .map(<TypeParameter<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ClassDefinition<'tree> {
    type WithLifetime<'a> = ClassDefinition<'a>;
    const KIND: &'static str = "class_definition";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "class_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "class_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `class_pattern`

This node has named children of type `{case_pattern | dotted_name}+`:

- [`CasePattern`]
- [`DottedName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ClassPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ClassPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ClassPattern<'tree> {
    type Child = anon_unions::CasePattern_DottedName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ClassPattern<'tree> {
    type WithLifetime<'a> = ClassPattern<'a>;
    const KIND: &'static str = "class_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "class_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "class_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `comment`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Comment<'tree> {
    type WithLifetime<'a> = Comment<'a>;
    const KIND: &'static str = "comment";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `comparison_operator`

This node has these fields:

- `operators`: `{!= | < | <= | <> | == | > | >= | in | is | is not | not in}+` ([`symbols::NotEq`] | [`symbols::Lt`] | [`symbols::LtEq`] | [`symbols::LtGt`] | [`symbols::EqEq`] | [`symbols::Gt`] | [`symbols::GtEq`] | [`unnamed::In`] | [`unnamed::Is`] | [`symbols::IsSpacenot`] | [`symbols::NotSpacein`])

And additional named children of type `primary_expression+` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ComparisonOperator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ComparisonOperator<'tree> {
    /**Get the children of field `operators`.

These children have type `{!= | < | <= | <> | == | > | >= | in | is | is not | not in}+`:

- [`symbols::NotEq`]
- [`symbols::Lt`]
- [`symbols::LtEq`]
- [`symbols::LtGt`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
- [`unnamed::In`]
- [`unnamed::Is`]
- [`symbols::IsSpacenot`]
- [`symbols::NotSpacein`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn operatorss<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::NotEq_Lt_LtEq_LtGt_EqEq_Gt_GtEq_In_Is_IsSpacenot_NotSpacein<
                'tree,
            >,
        >,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("operators", &mut c.0)
            .map(
                <anon_unions::NotEq_Lt_LtEq_LtGt_EqEq_Gt_GtEq_In_Is_IsSpacenot_NotSpacein<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `primary_expression+` ([`PrimaryExpression`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn primary_expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ComparisonOperator<'tree> {
    type WithLifetime<'a> = ComparisonOperator<'a>;
    const KIND: &'static str = "comparison_operator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "comparison_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comparison_operator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `complex_pattern`

This node has named children of type `{float | integer}+`:

- [`Float`]
- [`Integer`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ComplexPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ComplexPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ComplexPattern<'tree> {
    type Child = anon_unions::Float_Integer<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ComplexPattern<'tree> {
    type WithLifetime<'a> = ComplexPattern<'a>;
    const KIND: &'static str = "complex_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "complex_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "complex_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `concatenated_string`

This node has named children of type `string+` ([`String`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConcatenatedString<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ConcatenatedString<'tree> {
    /**Get the node's not-extra named children.

These children have type `string+` ([`String`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn strings<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, String<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<String<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ConcatenatedString<'tree> {
    type Child = String<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ConcatenatedString<'tree> {
    type WithLifetime<'a> = ConcatenatedString<'a>;
    const KIND: &'static str = "concatenated_string";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "concatenated_string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "concatenated_string");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `conditional_expression`

This node has named children of type `expression+` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConditionalExpression<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ConditionalExpression<'tree> {
    /**Get the node's not-extra named children.

These children have type `expression+` ([`Expression`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ConditionalExpression<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ConditionalExpression<'tree> {
    type WithLifetime<'a> = ConditionalExpression<'a>;
    const KIND: &'static str = "conditional_expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "conditional_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "conditional_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `constrained_type`

This node has named children of type `type+` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConstrainedType<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ConstrainedType<'tree> {
    /**Get the node's not-extra named children.

These children have type `type+` ([`Type`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn types<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Type<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ConstrainedType<'tree> {
    type Child = Type<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ConstrainedType<'tree> {
    type WithLifetime<'a> = ConstrainedType<'a>;
    const KIND: &'static str = "constrained_type";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "constrained_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "constrained_type");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `continue_statement`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ContinueStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ContinueStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ContinueStatement<'tree> {
    type WithLifetime<'a> = ContinueStatement<'a>;
    const KIND: &'static str = "continue_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "continue_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "continue_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `decorated_definition`

This node has these fields:

- `definition`: `{class_definition | function_definition}` ([`ClassDefinition`] | [`FunctionDefinition`])

And additional named children of type `decorator+` ([`Decorator`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DecoratedDefinition<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DecoratedDefinition<'tree> {
    /**Get the field `definition`.

This child has type `{class_definition | function_definition}`:

- [`ClassDefinition`]
- [`FunctionDefinition`]
*/
    #[inline]
    pub fn definition(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::ClassDefinition_FunctionDefinition<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("definition")
            .map(
                <anon_unions::ClassDefinition_FunctionDefinition<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `decorator+` ([`Decorator`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn decorators<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Decorator<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(<Decorator<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DecoratedDefinition<'tree> {
    type WithLifetime<'a> = DecoratedDefinition<'a>;
    const KIND: &'static str = "decorated_definition";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "decorated_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "decorated_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `decorator`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Decorator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Decorator<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for Decorator<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Decorator<'tree> {
    type WithLifetime<'a> = Decorator<'a>;
    const KIND: &'static str = "decorator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "decorator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "decorator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `default_parameter`

This node has these fields:

- `name`: `{identifier | tuple_pattern}` ([`Identifier`] | [`TuplePattern`])
- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DefaultParameter<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DefaultParameter<'tree> {
    /**Get the field `name`.

This child has type `{identifier | tuple_pattern}`:

- [`Identifier`]
- [`TuplePattern`]
*/
    #[inline]
    pub fn name(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Identifier_TuplePattern<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(
                <anon_unions::Identifier_TuplePattern<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DefaultParameter<'tree> {
    type WithLifetime<'a> = DefaultParameter<'a>;
    const KIND: &'static str = "default_parameter";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "default_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "default_parameter");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `delete_statement`

This node has a named child of type `{expression | expression_list}`:

- [`Expression`]
- [`ExpressionList`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DeleteStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DeleteStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for DeleteStatement<'tree> {
    type Child = anon_unions::Expression_ExpressionList<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DeleteStatement<'tree> {
    type WithLifetime<'a> = DeleteStatement<'a>;
    const KIND: &'static str = "delete_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "delete_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "delete_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `dict_pattern`

This node has these fields:

- `key`: `{- | _ | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}*` ([`symbols::Sub`] | [`symbols::__`] | [`ClassPattern`] | [`ComplexPattern`] | [`ConcatenatedString`] | [`DictPattern`] | [`DottedName`] | [`False`] | [`Float`] | [`Integer`] | [`ListPattern`] | [`None`] | [`SplatPattern`] | [`String`] | [`True`] | [`TuplePattern`] | [`UnionPattern`])
- `value`: `case_pattern*` ([`CasePattern`])

And additional named children of type `splat_pattern*` ([`SplatPattern`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DictPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DictPattern<'tree> {
    /**Get the children of field `key`.

These children have type `{- | _ | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}*`:

- [`symbols::Sub`]
- [`symbols::__`]
- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Integer`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]
*/
    #[inline]
    pub fn keys<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::Sub____ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
                'tree,
            >,
        >,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("key", &mut c.0)
            .map(
                <anon_unions::Sub____ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the children of field `value`.

These children have type `case_pattern*` ([`CasePattern`])*/
    #[inline]
    pub fn values<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, CasePattern<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("value", &mut c.0)
            .map(<CasePattern<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the node's non-field not-extra named children.

These children have type `splat_pattern*` ([`SplatPattern`])*/
    #[inline]
    pub fn splat_patterns<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, SplatPattern<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(<SplatPattern<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DictPattern<'tree> {
    type WithLifetime<'a> = DictPattern<'a>;
    const KIND: &'static str = "dict_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "dict_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dict_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `dictionary`

This node has named children of type `{dictionary_splat | pair}*`:

- [`DictionarySplat`]
- [`Pair`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Dictionary<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Dictionary<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Dictionary<'tree> {
    type Child = anon_unions::DictionarySplat_Pair<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Dictionary<'tree> {
    type WithLifetime<'a> = Dictionary<'a>;
    const KIND: &'static str = "dictionary";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "dictionary" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dictionary");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `dictionary_comprehension`

This node has these fields:

- `body`: `pair` ([`Pair`])

And additional named children of type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DictionaryComprehension<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DictionaryComprehension<'tree> {
    /**Get the field `body`.

This child has type `pair` ([`Pair`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Pair<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Pair<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::ForInClause_IfClause<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::ForInClause_IfClause<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DictionaryComprehension<'tree> {
    type WithLifetime<'a> = DictionaryComprehension<'a>;
    const KIND: &'static str = "dictionary_comprehension";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "dictionary_comprehension" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dictionary_comprehension");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `dictionary_splat`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DictionarySplat<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DictionarySplat<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for DictionarySplat<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DictionarySplat<'tree> {
    type WithLifetime<'a> = DictionarySplat<'a>;
    const KIND: &'static str = "dictionary_splat";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "dictionary_splat" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dictionary_splat");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `dictionary_splat_pattern`

This node has a named child of type `{attribute | identifier | subscript}`:

- [`Attribute`]
- [`Identifier`]
- [`Subscript`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DictionarySplatPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DictionarySplatPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for DictionarySplatPattern<'tree> {
    type Child = anon_unions::Attribute_Identifier_Subscript<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DictionarySplatPattern<'tree> {
    type WithLifetime<'a> = DictionarySplatPattern<'a>;
    const KIND: &'static str = "dictionary_splat_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "dictionary_splat_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dictionary_splat_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `dotted_name`

This node has named children of type `identifier+` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DottedName<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DottedName<'tree> {
    /**Get the node's not-extra named children.

These children have type `identifier+` ([`Identifier`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn identifiers<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for DottedName<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for DottedName<'tree> {
    type WithLifetime<'a> = DottedName<'a>;
    const KIND: &'static str = "dotted_name";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "dotted_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dotted_name");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `elif_clause`

This node has these fields:

- `condition`: `expression` ([`Expression`])
- `consequence`: `block` ([`Block`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ElifClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ElifClause<'tree> {
    /**Get the field `condition`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn condition(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `consequence`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn consequence(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("consequence")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ElifClause<'tree> {
    type WithLifetime<'a> = ElifClause<'a>;
    const KIND: &'static str = "elif_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "elif_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "elif_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `ellipsis`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Ellipsis<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Ellipsis<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Ellipsis<'tree> {
    type WithLifetime<'a> = Ellipsis<'a>;
    const KIND: &'static str = "ellipsis";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "ellipsis" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "ellipsis");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `else_clause`

This node has these fields:

- `body`: `block` ([`Block`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ElseClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ElseClause<'tree> {
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ElseClause<'tree> {
    type WithLifetime<'a> = ElseClause<'a>;
    const KIND: &'static str = "else_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "else_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "else_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `escape_interpolation`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EscapeInterpolation<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> EscapeInterpolation<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for EscapeInterpolation<'tree> {
    type WithLifetime<'a> = EscapeInterpolation<'a>;
    const KIND: &'static str = "escape_interpolation";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "escape_interpolation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "escape_interpolation");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `escape_sequence`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for EscapeSequence<'tree> {
    type WithLifetime<'a> = EscapeSequence<'a>;
    const KIND: &'static str = "escape_sequence";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "escape_sequence");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `except_clause`

This node has these fields:

- `alias`: `expression?` ([`Expression`])
- `value`: `expression?` ([`Expression`])

And an additional named child of type `block` ([`Block`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExceptClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExceptClause<'tree> {
    /**Get the optional field `alias`.

This child has type `expression?` ([`Expression`])*/
    #[inline]
    pub fn alias(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("alias")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `value`.

This child has type `expression?` ([`Expression`])*/
    #[inline]
    pub fn value(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn block(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter_lib::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ExceptClause<'tree> {
    type WithLifetime<'a> = ExceptClause<'a>;
    const KIND: &'static str = "except_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "except_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "except_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `except_group_clause`

This node has named children of type `{block | expression}+`:

- [`Block`]
- [`Expression`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExceptGroupClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExceptGroupClause<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ExceptGroupClause<'tree> {
    type Child = anon_unions::Block_Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ExceptGroupClause<'tree> {
    type WithLifetime<'a> = ExceptGroupClause<'a>;
    const KIND: &'static str = "except_group_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "except_group_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "except_group_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `exec_statement`

This node has these fields:

- `code`: `{identifier | string}` ([`Identifier`] | [`String`])

And additional named children of type `expression*` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExecStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExecStatement<'tree> {
    /**Get the field `code`.

This child has type `{identifier | string}`:

- [`Identifier`]
- [`String`]
*/
    #[inline]
    pub fn code(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::Identifier_String<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("code")
            .map(
                <anon_unions::Identifier_String<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `expression*` ([`Expression`])*/
    #[inline]
    pub fn expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ExecStatement<'tree> {
    type WithLifetime<'a> = ExecStatement<'a>;
    const KIND: &'static str = "exec_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "exec_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "exec_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `expression`

This node type has subtypes:

- `as_pattern` ([`AsPattern`])
- `boolean_operator` ([`BooleanOperator`])
- `comparison_operator` ([`ComparisonOperator`])
- `conditional_expression` ([`ConditionalExpression`])
- `lambda` ([`Lambda`])
- `named_expression` ([`NamedExpression`])
- `not_operator` ([`NotOperator`])
- `primary_expression` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Expression<'tree> {
    AsPattern(AsPattern<'tree>),
    BooleanOperator(BooleanOperator<'tree>),
    ComparisonOperator(ComparisonOperator<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    Lambda(Lambda<'tree>),
    NamedExpression(NamedExpression<'tree>),
    NotOperator(NotOperator<'tree>),
    PrimaryExpression(PrimaryExpression<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> Expression<'tree> {
    ///Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`
    #[inline]
    pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::AsPattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`
    #[inline]
    pub fn as_boolean_operator(self) -> ::std::option::Option<BooleanOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BooleanOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`
    #[inline]
    pub fn as_comparison_operator(
        self,
    ) -> ::std::option::Option<ComparisonOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ComparisonOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`
    #[inline]
    pub fn as_conditional_expression(
        self,
    ) -> ::std::option::Option<ConditionalExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConditionalExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`
    #[inline]
    pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Lambda(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`
    #[inline]
    pub fn as_named_expression(self) -> ::std::option::Option<NamedExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::NamedExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`
    #[inline]
    pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::NotOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`
    #[inline]
    pub fn as_primary_expression(
        self,
    ) -> ::std::option::Option<PrimaryExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PrimaryExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
        self.as_primary_expression()?.as_attribute()
    }
    /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
        self.as_primary_expression()?.as_await()
    }
    /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
        self.as_primary_expression()?.as_binary_operator()
    }
    /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
        self.as_primary_expression()?.as_call()
    }
    /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_concatenated_string(
        self,
    ) -> ::std::option::Option<ConcatenatedString<'tree>> {
        self.as_primary_expression()?.as_concatenated_string()
    }
    /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
        self.as_primary_expression()?.as_dictionary()
    }
    /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_dictionary_comprehension(
        self,
    ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
        self.as_primary_expression()?.as_dictionary_comprehension()
    }
    /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
        self.as_primary_expression()?.as_ellipsis()
    }
    /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
        self.as_primary_expression()?.as_false()
    }
    /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
        self.as_primary_expression()?.as_float()
    }
    /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_generator_expression(
        self,
    ) -> ::std::option::Option<GeneratorExpression<'tree>> {
        self.as_primary_expression()?.as_generator_expression()
    }
    /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        self.as_primary_expression()?.as_identifier()
    }
    /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
        self.as_primary_expression()?.as_integer()
    }
    /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
        self.as_primary_expression()?.as_list()
    }
    /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_list_comprehension(
        self,
    ) -> ::std::option::Option<ListComprehension<'tree>> {
        self.as_primary_expression()?.as_list_comprehension()
    }
    /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
        self.as_primary_expression()?.as_list_splat()
    }
    /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
        self.as_primary_expression()?.as_none()
    }
    /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_parenthesized_expression(
        self,
    ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
        self.as_primary_expression()?.as_parenthesized_expression()
    }
    /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
        self.as_primary_expression()?.as_set()
    }
    /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_set_comprehension(self) -> ::std::option::Option<SetComprehension<'tree>> {
        self.as_primary_expression()?.as_set_comprehension()
    }
    /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
        self.as_primary_expression()?.as_string()
    }
    /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        self.as_primary_expression()?.as_subscript()
    }
    /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
        self.as_primary_expression()?.as_true()
    }
    /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
        self.as_primary_expression()?.as_tuple()
    }
    /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))*/
    #[inline]
    pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
        self.as_primary_expression()?.as_unary_operator()
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Expression<'tree> {
    type WithLifetime<'a> = Expression<'a>;
    const KIND: &'static str = "expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if let Ok(this) = <AsPattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::AsPattern(this));
        }
        if let Ok(this) = <BooleanOperator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::BooleanOperator(this));
        }
        if let Ok(this) = <ComparisonOperator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ComparisonOperator(this));
        }
        if let Ok(this) = <ConditionalExpression<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ConditionalExpression(this));
        }
        if let Ok(this) = <Lambda<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Lambda(this));
        }
        if let Ok(this) = <NamedExpression<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::NamedExpression(this));
        }
        if let Ok(this) = <NotOperator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::NotOperator(this));
        }
        if let Ok(this) = <PrimaryExpression<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::PrimaryExpression(this));
        }
        Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::AsPattern(x) => ::type_sitter_lib::Node::raw(x),
            Self::BooleanOperator(x) => ::type_sitter_lib::Node::raw(x),
            Self::ComparisonOperator(x) => ::type_sitter_lib::Node::raw(x),
            Self::ConditionalExpression(x) => ::type_sitter_lib::Node::raw(x),
            Self::Lambda(x) => ::type_sitter_lib::Node::raw(x),
            Self::NamedExpression(x) => ::type_sitter_lib::Node::raw(x),
            Self::NotOperator(x) => ::type_sitter_lib::Node::raw(x),
            Self::PrimaryExpression(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::AsPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::BooleanOperator(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ComparisonOperator(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ConditionalExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Lambda(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::NamedExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::NotOperator(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::PrimaryExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        match self {
            Self::AsPattern(x) => x.into_raw(),
            Self::BooleanOperator(x) => x.into_raw(),
            Self::ComparisonOperator(x) => x.into_raw(),
            Self::ConditionalExpression(x) => x.into_raw(),
            Self::Lambda(x) => x.into_raw(),
            Self::NamedExpression(x) => x.into_raw(),
            Self::NotOperator(x) => x.into_raw(),
            Self::PrimaryExpression(x) => x.into_raw(),
        }
    }
}
/**Typed node `expression_list`

This node has named children of type `expression+` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExpressionList<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExpressionList<'tree> {
    /**Get the node's not-extra named children.

These children have type `expression+` ([`Expression`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ExpressionList<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ExpressionList<'tree> {
    type WithLifetime<'a> = ExpressionList<'a>;
    const KIND: &'static str = "expression_list";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "expression_list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "expression_list");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `expression_statement`

This node has named children of type `{assignment | augmented_assignment | expression | yield}+`:

- [`Assignment`]
- [`AugmentedAssignment`]
- [`Expression`]
- [`Yield`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExpressionStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExpressionStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ExpressionStatement<'tree> {
    type Child = anon_unions::Assignment_AugmentedAssignment_Expression_Yield<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ExpressionStatement<'tree> {
    type WithLifetime<'a> = ExpressionStatement<'a>;
    const KIND: &'static str = "expression_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "expression_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "expression_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `false`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct False<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> False<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for False<'tree> {
    type WithLifetime<'a> = False<'a>;
    const KIND: &'static str = "false";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "false" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "false");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `finally_clause`

This node has a named child of type `block` ([`Block`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FinallyClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FinallyClause<'tree> {
    /**Get the node's only not-extra named child.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn block(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for FinallyClause<'tree> {
    type Child = Block<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for FinallyClause<'tree> {
    type WithLifetime<'a> = FinallyClause<'a>;
    const KIND: &'static str = "finally_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "finally_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "finally_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `float`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Float<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Float<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Float<'tree> {
    type WithLifetime<'a> = Float<'a>;
    const KIND: &'static str = "float";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "float" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "float");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `for_in_clause`

This node has these fields:

- `left`: `{pattern | pattern_list}` ([`Pattern`] | [`PatternList`])
- `right`: `{, | expression}+` ([`symbols::Comma`] | [`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ForInClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ForInClause<'tree> {
    /**Get the field `left`.

This child has type `{pattern | pattern_list}`:

- [`Pattern`]
- [`PatternList`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::Pattern_PatternList<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::Pattern_PatternList<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the children of field `right`.

These children have type `{, | expression}+`:

- [`symbols::Comma`]
- [`Expression`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn rights<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, anon_unions::Comma_Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("right", &mut c.0)
            .map(
                <anon_unions::Comma_Expression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ForInClause<'tree> {
    type WithLifetime<'a> = ForInClause<'a>;
    const KIND: &'static str = "for_in_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "for_in_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "for_in_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `for_statement`

This node has these fields:

- `alternative`: `else_clause?` ([`ElseClause`])
- `body`: `block` ([`Block`])
- `left`: `{pattern | pattern_list}` ([`Pattern`] | [`PatternList`])
- `right`: `{expression | expression_list}` ([`Expression`] | [`ExpressionList`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ForStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ForStatement<'tree> {
    /**Get the optional field `alternative`.

This child has type `else_clause?` ([`ElseClause`])*/
    #[inline]
    pub fn alternative(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, ElseClause<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("alternative")
            .map(<ElseClause<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `left`.

This child has type `{pattern | pattern_list}`:

- [`Pattern`]
- [`PatternList`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::Pattern_PatternList<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::Pattern_PatternList<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `{expression | expression_list}`:

- [`Expression`]
- [`ExpressionList`]
*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Expression_ExpressionList<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::Expression_ExpressionList<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ForStatement<'tree> {
    type WithLifetime<'a> = ForStatement<'a>;
    const KIND: &'static str = "for_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "for_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "for_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `format_expression`

This node has these fields:

- `expression`: `{expression | expression_list | pattern_list | yield}` ([`Expression`] | [`ExpressionList`] | [`PatternList`] | [`Yield`])
- `format_specifier`: `format_specifier?` ([`FormatSpecifier`])
- `type_conversion`: `type_conversion?` ([`TypeConversion`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FormatExpression<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FormatExpression<'tree> {
    /**Get the field `expression`.

This child has type `{expression | expression_list | pattern_list | yield}`:

- [`Expression`]
- [`ExpressionList`]
- [`PatternList`]
- [`Yield`]
*/
    #[inline]
    pub fn expression(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Expression_ExpressionList_PatternList_Yield<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("expression")
            .map(
                <anon_unions::Expression_ExpressionList_PatternList_Yield<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `format_specifier`.

This child has type `format_specifier?` ([`FormatSpecifier`])*/
    #[inline]
    pub fn format_specifier(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, FormatSpecifier<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("format_specifier")
            .map(
                <FormatSpecifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the optional field `type_conversion`.

This child has type `type_conversion?` ([`TypeConversion`])*/
    #[inline]
    pub fn type_conversion(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, TypeConversion<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type_conversion")
            .map(<TypeConversion<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for FormatExpression<'tree> {
    type WithLifetime<'a> = FormatExpression<'a>;
    const KIND: &'static str = "format_expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "format_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "format_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `format_specifier`

This node has named children of type `format_expression*` ([`FormatExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FormatSpecifier<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FormatSpecifier<'tree> {
    /**Get the node's not-extra named children.

These children have type `format_expression*` ([`FormatExpression`])*/
    #[inline]
    pub fn format_expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, FormatExpression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(
                <FormatExpression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for FormatSpecifier<'tree> {
    type Child = FormatExpression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for FormatSpecifier<'tree> {
    type WithLifetime<'a> = FormatSpecifier<'a>;
    const KIND: &'static str = "format_specifier";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "format_specifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "format_specifier");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `function_definition`

This node has these fields:

- `body`: `block` ([`Block`])
- `name`: `identifier` ([`Identifier`])
- `parameters`: `parameters` ([`Parameters`])
- `return_type`: `type?` ([`Type`])
- `type_parameters`: `type_parameter?` ([`TypeParameter`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FunctionDefinition<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FunctionDefinition<'tree> {
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `parameters`.

This child has type `parameters` ([`Parameters`])*/
    #[inline]
    pub fn parameters(&self) -> ::type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<Parameters<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `return_type`.

This child has type `type?` ([`Type`])*/
    #[inline]
    pub fn return_type(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("return_type")
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `type_parameters`.

This child has type `type_parameter?` ([`TypeParameter`])*/
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, TypeParameter<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type_parameters")
            .map(<TypeParameter<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for FunctionDefinition<'tree> {
    type WithLifetime<'a> = FunctionDefinition<'a>;
    const KIND: &'static str = "function_definition";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "function_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "function_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `future_import_statement`

This node has these fields:

- `name`: `{aliased_import | dotted_name}+` ([`AliasedImport`] | [`DottedName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FutureImportStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FutureImportStatement<'tree> {
    /**Get the children of field `name`.

These children have type `{aliased_import | dotted_name}+`:

- [`AliasedImport`]
- [`DottedName`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::AliasedImport_DottedName<'tree>,
        >,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("name", &mut c.0)
            .map(
                <anon_unions::AliasedImport_DottedName<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for FutureImportStatement<'tree> {
    type WithLifetime<'a> = FutureImportStatement<'a>;
    const KIND: &'static str = "future_import_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "future_import_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "future_import_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `generator_expression`

This node has these fields:

- `body`: `expression` ([`Expression`])

And additional named children of type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GeneratorExpression<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> GeneratorExpression<'tree> {
    /**Get the field `body`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::ForInClause_IfClause<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::ForInClause_IfClause<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for GeneratorExpression<'tree> {
    type WithLifetime<'a> = GeneratorExpression<'a>;
    const KIND: &'static str = "generator_expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "generator_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "generator_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `generic_type`

This node has named children of type `{identifier | type_parameter}+`:

- [`Identifier`]
- [`TypeParameter`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GenericType<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> GenericType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for GenericType<'tree> {
    type Child = anon_unions::Identifier_TypeParameter<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for GenericType<'tree> {
    type WithLifetime<'a> = GenericType<'a>;
    const KIND: &'static str = "generic_type";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "generic_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "generic_type");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `global_statement`

This node has named children of type `identifier+` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GlobalStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> GlobalStatement<'tree> {
    /**Get the node's not-extra named children.

These children have type `identifier+` ([`Identifier`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn identifiers<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for GlobalStatement<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for GlobalStatement<'tree> {
    type WithLifetime<'a> = GlobalStatement<'a>;
    const KIND: &'static str = "global_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "global_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "global_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `identifier`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Identifier<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Identifier<'tree> {
    type WithLifetime<'a> = Identifier<'a>;
    const KIND: &'static str = "identifier";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "identifier");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `if_clause`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IfClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> IfClause<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for IfClause<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for IfClause<'tree> {
    type WithLifetime<'a> = IfClause<'a>;
    const KIND: &'static str = "if_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "if_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "if_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `if_statement`

This node has these fields:

- `alternative`: `{elif_clause | else_clause}*` ([`ElifClause`] | [`ElseClause`])
- `condition`: `expression` ([`Expression`])
- `consequence`: `block` ([`Block`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IfStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> IfStatement<'tree> {
    /**Get the children of field `alternative`.

These children have type `{elif_clause | else_clause}*`:

- [`ElifClause`]
- [`ElseClause`]
*/
    #[inline]
    pub fn alternatives<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::ElifClause_ElseClause<'tree>,
        >,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("alternative", &mut c.0)
            .map(
                <anon_unions::ElifClause_ElseClause<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the field `condition`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn condition(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `consequence`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn consequence(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("consequence")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for IfStatement<'tree> {
    type WithLifetime<'a> = IfStatement<'a>;
    const KIND: &'static str = "if_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "if_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "if_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `import_from_statement`

This node has these fields:

- `module_name`: `{dotted_name | relative_import}` ([`DottedName`] | [`RelativeImport`])
- `name`: `{aliased_import | dotted_name}*` ([`AliasedImport`] | [`DottedName`])

And an optional additional named child of type `wildcard_import?` ([`WildcardImport`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ImportFromStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ImportFromStatement<'tree> {
    /**Get the field `module_name`.

This child has type `{dotted_name | relative_import}`:

- [`DottedName`]
- [`RelativeImport`]
*/
    #[inline]
    pub fn module_name(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::DottedName_RelativeImport<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("module_name")
            .map(
                <anon_unions::DottedName_RelativeImport<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the children of field `name`.

These children have type `{aliased_import | dotted_name}*`:

- [`AliasedImport`]
- [`DottedName`]
*/
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::AliasedImport_DottedName<'tree>,
        >,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("name", &mut c.0)
            .map(
                <anon_unions::AliasedImport_DottedName<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the node's only non-field not-extra named child, if it has one.

This child has type `wildcard_import?` ([`WildcardImport`])*/
    #[inline]
    pub fn wildcard_import(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, WildcardImport<'tree>>,
    > {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter_lib::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<WildcardImport<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ImportFromStatement<'tree> {
    type WithLifetime<'a> = ImportFromStatement<'a>;
    const KIND: &'static str = "import_from_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "import_from_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "import_from_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `import_prefix`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ImportPrefix<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ImportPrefix<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ImportPrefix<'tree> {
    type WithLifetime<'a> = ImportPrefix<'a>;
    const KIND: &'static str = "import_prefix";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "import_prefix" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "import_prefix");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `import_statement`

This node has these fields:

- `name`: `{aliased_import | dotted_name}+` ([`AliasedImport`] | [`DottedName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ImportStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ImportStatement<'tree> {
    /**Get the children of field `name`.

These children have type `{aliased_import | dotted_name}+`:

- [`AliasedImport`]
- [`DottedName`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::AliasedImport_DottedName<'tree>,
        >,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("name", &mut c.0)
            .map(
                <anon_unions::AliasedImport_DottedName<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ImportStatement<'tree> {
    type WithLifetime<'a> = ImportStatement<'a>;
    const KIND: &'static str = "import_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "import_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "import_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `integer`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Integer<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Integer<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Integer<'tree> {
    type WithLifetime<'a> = Integer<'a>;
    const KIND: &'static str = "integer";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "integer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "integer");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `interpolation`

This node has these fields:

- `expression`: `{expression | expression_list | pattern_list | yield}` ([`Expression`] | [`ExpressionList`] | [`PatternList`] | [`Yield`])
- `format_specifier`: `format_specifier?` ([`FormatSpecifier`])
- `type_conversion`: `type_conversion?` ([`TypeConversion`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Interpolation<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Interpolation<'tree> {
    /**Get the field `expression`.

This child has type `{expression | expression_list | pattern_list | yield}`:

- [`Expression`]
- [`ExpressionList`]
- [`PatternList`]
- [`Yield`]
*/
    #[inline]
    pub fn expression(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Expression_ExpressionList_PatternList_Yield<'tree>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("expression")
            .map(
                <anon_unions::Expression_ExpressionList_PatternList_Yield<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `format_specifier`.

This child has type `format_specifier?` ([`FormatSpecifier`])*/
    #[inline]
    pub fn format_specifier(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, FormatSpecifier<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("format_specifier")
            .map(
                <FormatSpecifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the optional field `type_conversion`.

This child has type `type_conversion?` ([`TypeConversion`])*/
    #[inline]
    pub fn type_conversion(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, TypeConversion<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type_conversion")
            .map(<TypeConversion<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Interpolation<'tree> {
    type WithLifetime<'a> = Interpolation<'a>;
    const KIND: &'static str = "interpolation";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "interpolation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "interpolation");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `keyword_argument`

This node has these fields:

- `name`: `identifier` ([`Identifier`])
- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct KeywordArgument<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> KeywordArgument<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for KeywordArgument<'tree> {
    type WithLifetime<'a> = KeywordArgument<'a>;
    const KIND: &'static str = "keyword_argument";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "keyword_argument" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "keyword_argument");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `keyword_pattern`

This node has named children of type `{class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | identifier | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}+`:

- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Identifier`]
- [`Integer`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct KeywordPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> KeywordPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for KeywordPattern<'tree> {
    type Child = anon_unions::ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Identifier_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for KeywordPattern<'tree> {
    type WithLifetime<'a> = KeywordPattern<'a>;
    const KIND: &'static str = "keyword_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "keyword_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "keyword_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `keyword_separator`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct KeywordSeparator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> KeywordSeparator<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for KeywordSeparator<'tree> {
    type WithLifetime<'a> = KeywordSeparator<'a>;
    const KIND: &'static str = "keyword_separator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "keyword_separator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "keyword_separator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `lambda`

This node has these fields:

- `body`: `expression` ([`Expression`])
- `parameters`: `lambda_parameters?` ([`LambdaParameters`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Lambda<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Lambda<'tree> {
    /**Get the field `body`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `parameters`.

This child has type `lambda_parameters?` ([`LambdaParameters`])*/
    #[inline]
    pub fn parameters(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<'tree, LambdaParameters<'tree>>,
    > {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("parameters")
            .map(
                <LambdaParameters<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Lambda<'tree> {
    type WithLifetime<'a> = Lambda<'a>;
    const KIND: &'static str = "lambda";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "lambda" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "lambda");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `lambda_parameters`

This node has named children of type `parameter+` ([`Parameter`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct LambdaParameters<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> LambdaParameters<'tree> {
    /**Get the node's not-extra named children.

These children have type `parameter+` ([`Parameter`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn parameters<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Parameter<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Parameter<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for LambdaParameters<'tree> {
    type Child = Parameter<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for LambdaParameters<'tree> {
    type WithLifetime<'a> = LambdaParameters<'a>;
    const KIND: &'static str = "lambda_parameters";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "lambda_parameters" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "lambda_parameters");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `line_continuation`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct LineContinuation<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> LineContinuation<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for LineContinuation<'tree> {
    type WithLifetime<'a> = LineContinuation<'a>;
    const KIND: &'static str = "line_continuation";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "line_continuation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "line_continuation");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `list`

This node has named children of type `{expression | list_splat | parenthesized_list_splat | yield}*`:

- [`Expression`]
- [`ListSplat`]
- [`ParenthesizedListSplat`]
- [`Yield`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct List<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> List<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for List<'tree> {
    type Child = anon_unions::Expression_ListSplat_ParenthesizedListSplat_Yield<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for List<'tree> {
    type WithLifetime<'a> = List<'a>;
    const KIND: &'static str = "list";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "list");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `list_comprehension`

This node has these fields:

- `body`: `expression` ([`Expression`])

And additional named children of type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ListComprehension<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ListComprehension<'tree> {
    /**Get the field `body`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::ForInClause_IfClause<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::ForInClause_IfClause<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ListComprehension<'tree> {
    type WithLifetime<'a> = ListComprehension<'a>;
    const KIND: &'static str = "list_comprehension";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "list_comprehension" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "list_comprehension");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `list_pattern`

This node has named children of type `{case_pattern | pattern}*`:

- [`CasePattern`]
- [`Pattern`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ListPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ListPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for ListPattern<'tree> {
    type Child = anon_unions::CasePattern_Pattern<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ListPattern<'tree> {
    type WithLifetime<'a> = ListPattern<'a>;
    const KIND: &'static str = "list_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "list_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "list_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `list_splat`

This node has a named child of type `{attribute | expression | identifier | subscript}`:

- [`Attribute`]
- [`Expression`]
- [`Identifier`]
- [`Subscript`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ListSplat<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ListSplat<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for ListSplat<'tree> {
    type Child = anon_unions::Attribute_Expression_Identifier_Subscript<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ListSplat<'tree> {
    type WithLifetime<'a> = ListSplat<'a>;
    const KIND: &'static str = "list_splat";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "list_splat" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "list_splat");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `list_splat_pattern`

This node has a named child of type `{attribute | identifier | subscript}`:

- [`Attribute`]
- [`Identifier`]
- [`Subscript`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ListSplatPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ListSplatPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for ListSplatPattern<'tree> {
    type Child = anon_unions::Attribute_Identifier_Subscript<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ListSplatPattern<'tree> {
    type WithLifetime<'a> = ListSplatPattern<'a>;
    const KIND: &'static str = "list_splat_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "list_splat_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "list_splat_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `match_statement`

This node has these fields:

- `body`: `block` ([`Block`])
- `subject`: `expression+` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MatchStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> MatchStatement<'tree> {
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the children of field `subject`.

These children have type `expression+` ([`Expression`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn subjects<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("subject", &mut c.0)
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for MatchStatement<'tree> {
    type WithLifetime<'a> = MatchStatement<'a>;
    const KIND: &'static str = "match_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "match_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "match_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `member_type`

This node has named children of type `{identifier | type}+`:

- [`Identifier`]
- [`Type`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MemberType<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> MemberType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for MemberType<'tree> {
    type Child = anon_unions::Identifier_Type<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for MemberType<'tree> {
    type WithLifetime<'a> = MemberType<'a>;
    const KIND: &'static str = "member_type";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "member_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "member_type");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `module`

This node has named children of type `{_compound_statement | _simple_statement}*`:

- [`CompoundStatement`]
- [`SimpleStatement`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Module<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Module<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Module<'tree> {
    type Child = anon_unions::CompoundStatement_SimpleStatement<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Module<'tree> {
    type WithLifetime<'a> = Module<'a>;
    const KIND: &'static str = "module";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "module" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "module");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `named_expression`

This node has these fields:

- `name`: `identifier` ([`Identifier`])
- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NamedExpression<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NamedExpression<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for NamedExpression<'tree> {
    type WithLifetime<'a> = NamedExpression<'a>;
    const KIND: &'static str = "named_expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "named_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "named_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `none`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct None<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> None<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for None<'tree> {
    type WithLifetime<'a> = None<'a>;
    const KIND: &'static str = "none";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "none" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "none");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `nonlocal_statement`

This node has named children of type `identifier+` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NonlocalStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NonlocalStatement<'tree> {
    /**Get the node's not-extra named children.

These children have type `identifier+` ([`Identifier`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn identifiers<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for NonlocalStatement<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for NonlocalStatement<'tree> {
    type WithLifetime<'a> = NonlocalStatement<'a>;
    const KIND: &'static str = "nonlocal_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "nonlocal_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "nonlocal_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `not_operator`

This node has these fields:

- `argument`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NotOperator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NotOperator<'tree> {
    /**Get the field `argument`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn argument(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("argument")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for NotOperator<'tree> {
    type WithLifetime<'a> = NotOperator<'a>;
    const KIND: &'static str = "not_operator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "not_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "not_operator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `pair`

This node has these fields:

- `key`: `expression` ([`Expression`])
- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Pair<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Pair<'tree> {
    /**Get the field `key`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn key(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("key")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Pair<'tree> {
    type WithLifetime<'a> = Pair<'a>;
    const KIND: &'static str = "pair";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "pair" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pair");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `parameter`

This node type has subtypes:

- `default_parameter` ([`DefaultParameter`])
- `dictionary_splat_pattern` ([`DictionarySplatPattern`])
- `identifier` ([`Identifier`])
- `keyword_separator` ([`KeywordSeparator`])
- `list_splat_pattern` ([`ListSplatPattern`])
- `positional_separator` ([`PositionalSeparator`])
- `tuple_pattern` ([`TuplePattern`])
- `typed_default_parameter` ([`TypedDefaultParameter`])
- `typed_parameter` ([`TypedParameter`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Parameter<'tree> {
    DefaultParameter(DefaultParameter<'tree>),
    DictionarySplatPattern(DictionarySplatPattern<'tree>),
    Identifier(Identifier<'tree>),
    KeywordSeparator(KeywordSeparator<'tree>),
    ListSplatPattern(ListSplatPattern<'tree>),
    PositionalSeparator(PositionalSeparator<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TypedDefaultParameter(TypedDefaultParameter<'tree>),
    TypedParameter(TypedParameter<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> Parameter<'tree> {
    ///Returns the node if it is of type `default_parameter` ([`DefaultParameter`]), otherwise returns `None`
    #[inline]
    pub fn as_default_parameter(self) -> ::std::option::Option<DefaultParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefaultParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `dictionary_splat_pattern` ([`DictionarySplatPattern`]), otherwise returns `None`
    #[inline]
    pub fn as_dictionary_splat_pattern(
        self,
    ) -> ::std::option::Option<DictionarySplatPattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DictionarySplatPattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Identifier(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `keyword_separator` ([`KeywordSeparator`]), otherwise returns `None`
    #[inline]
    pub fn as_keyword_separator(self) -> ::std::option::Option<KeywordSeparator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::KeywordSeparator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `list_splat_pattern` ([`ListSplatPattern`]), otherwise returns `None`
    #[inline]
    pub fn as_list_splat_pattern(
        self,
    ) -> ::std::option::Option<ListSplatPattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ListSplatPattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `positional_separator` ([`PositionalSeparator`]), otherwise returns `None`
    #[inline]
    pub fn as_positional_separator(
        self,
    ) -> ::std::option::Option<PositionalSeparator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PositionalSeparator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
    #[inline]
    pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TuplePattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `typed_default_parameter` ([`TypedDefaultParameter`]), otherwise returns `None`
    #[inline]
    pub fn as_typed_default_parameter(
        self,
    ) -> ::std::option::Option<TypedDefaultParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypedDefaultParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `typed_parameter` ([`TypedParameter`]), otherwise returns `None`
    #[inline]
    pub fn as_typed_parameter(self) -> ::std::option::Option<TypedParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypedParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Parameter<'tree> {
    type WithLifetime<'a> = Parameter<'a>;
    const KIND: &'static str = "parameter";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if let Ok(this) = <DefaultParameter<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::DefaultParameter(this));
        }
        if let Ok(this) = <DictionarySplatPattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::DictionarySplatPattern(this));
        }
        if let Ok(this) = <Identifier<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Identifier(this));
        }
        if let Ok(this) = <KeywordSeparator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::KeywordSeparator(this));
        }
        if let Ok(this) = <ListSplatPattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ListSplatPattern(this));
        }
        if let Ok(this) = <PositionalSeparator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::PositionalSeparator(this));
        }
        if let Ok(this) = <TuplePattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::TuplePattern(this));
        }
        if let Ok(this) = <TypedDefaultParameter<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::TypedDefaultParameter(this));
        }
        if let Ok(this) = <TypedParameter<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::TypedParameter(this));
        }
        Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::DefaultParameter(x) => ::type_sitter_lib::Node::raw(x),
            Self::DictionarySplatPattern(x) => ::type_sitter_lib::Node::raw(x),
            Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
            Self::KeywordSeparator(x) => ::type_sitter_lib::Node::raw(x),
            Self::ListSplatPattern(x) => ::type_sitter_lib::Node::raw(x),
            Self::PositionalSeparator(x) => ::type_sitter_lib::Node::raw(x),
            Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
            Self::TypedDefaultParameter(x) => ::type_sitter_lib::Node::raw(x),
            Self::TypedParameter(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::DefaultParameter(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::DictionarySplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::KeywordSeparator(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ListSplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::PositionalSeparator(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::TypedDefaultParameter(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::TypedParameter(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        match self {
            Self::DefaultParameter(x) => x.into_raw(),
            Self::DictionarySplatPattern(x) => x.into_raw(),
            Self::Identifier(x) => x.into_raw(),
            Self::KeywordSeparator(x) => x.into_raw(),
            Self::ListSplatPattern(x) => x.into_raw(),
            Self::PositionalSeparator(x) => x.into_raw(),
            Self::TuplePattern(x) => x.into_raw(),
            Self::TypedDefaultParameter(x) => x.into_raw(),
            Self::TypedParameter(x) => x.into_raw(),
        }
    }
}
/**Typed node `parameters`

This node has named children of type `parameter*` ([`Parameter`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Parameters<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Parameters<'tree> {
    /**Get the node's not-extra named children.

These children have type `parameter*` ([`Parameter`])*/
    #[inline]
    pub fn parameters<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Parameter<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Parameter<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Parameters<'tree> {
    type Child = Parameter<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Parameters<'tree> {
    type WithLifetime<'a> = Parameters<'a>;
    const KIND: &'static str = "parameters";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "parameters" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parameters");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `parenthesized_expression`

This node has a named child of type `{expression | list_splat | parenthesized_expression | yield}`:

- [`Expression`]
- [`ListSplat`]
- [`ParenthesizedExpression`]
- [`Yield`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedExpression<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ParenthesizedExpression<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for ParenthesizedExpression<'tree> {
    type Child = anon_unions::Expression_ListSplat_ParenthesizedExpression_Yield<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ParenthesizedExpression<'tree> {
    type WithLifetime<'a> = ParenthesizedExpression<'a>;
    const KIND: &'static str = "parenthesized_expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parenthesized_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `parenthesized_list_splat`

This node has a named child of type `{list_splat | parenthesized_expression}`:

- [`ListSplat`]
- [`ParenthesizedExpression`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedListSplat<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ParenthesizedListSplat<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for ParenthesizedListSplat<'tree> {
    type Child = anon_unions::ListSplat_ParenthesizedExpression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ParenthesizedListSplat<'tree> {
    type WithLifetime<'a> = ParenthesizedListSplat<'a>;
    const KIND: &'static str = "parenthesized_list_splat";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "parenthesized_list_splat" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parenthesized_list_splat");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `pass_statement`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PassStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PassStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for PassStatement<'tree> {
    type WithLifetime<'a> = PassStatement<'a>;
    const KIND: &'static str = "pass_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "pass_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pass_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `pattern`

This node type has subtypes:

- `attribute` ([`Attribute`])
- `identifier` ([`Identifier`])
- `list_pattern` ([`ListPattern`])
- `list_splat_pattern` ([`ListSplatPattern`])
- `subscript` ([`Subscript`])
- `tuple_pattern` ([`TuplePattern`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Pattern<'tree> {
    Attribute(Attribute<'tree>),
    Identifier(Identifier<'tree>),
    ListPattern(ListPattern<'tree>),
    ListSplatPattern(ListSplatPattern<'tree>),
    Subscript(Subscript<'tree>),
    TuplePattern(TuplePattern<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> Pattern<'tree> {
    ///Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`
    #[inline]
    pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Attribute(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Identifier(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`
    #[inline]
    pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ListPattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `list_splat_pattern` ([`ListSplatPattern`]), otherwise returns `None`
    #[inline]
    pub fn as_list_splat_pattern(
        self,
    ) -> ::std::option::Option<ListSplatPattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ListSplatPattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Subscript(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
    #[inline]
    pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TuplePattern(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Pattern<'tree> {
    type WithLifetime<'a> = Pattern<'a>;
    const KIND: &'static str = "pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if let Ok(this) = <Attribute<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Attribute(this));
        }
        if let Ok(this) = <Identifier<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Identifier(this));
        }
        if let Ok(this) = <ListPattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ListPattern(this));
        }
        if let Ok(this) = <ListSplatPattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ListSplatPattern(this));
        }
        if let Ok(this) = <Subscript<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Subscript(this));
        }
        if let Ok(this) = <TuplePattern<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::TuplePattern(this));
        }
        Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::Attribute(x) => ::type_sitter_lib::Node::raw(x),
            Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
            Self::ListPattern(x) => ::type_sitter_lib::Node::raw(x),
            Self::ListSplatPattern(x) => ::type_sitter_lib::Node::raw(x),
            Self::Subscript(x) => ::type_sitter_lib::Node::raw(x),
            Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::Attribute(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ListPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ListSplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Subscript(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        match self {
            Self::Attribute(x) => x.into_raw(),
            Self::Identifier(x) => x.into_raw(),
            Self::ListPattern(x) => x.into_raw(),
            Self::ListSplatPattern(x) => x.into_raw(),
            Self::Subscript(x) => x.into_raw(),
            Self::TuplePattern(x) => x.into_raw(),
        }
    }
}
/**Typed node `pattern_list`

This node has named children of type `pattern+` ([`Pattern`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PatternList<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PatternList<'tree> {
    /**Get the node's not-extra named children.

These children have type `pattern+` ([`Pattern`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn patterns<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Pattern<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Pattern<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for PatternList<'tree> {
    type Child = Pattern<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for PatternList<'tree> {
    type WithLifetime<'a> = PatternList<'a>;
    const KIND: &'static str = "pattern_list";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "pattern_list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pattern_list");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `positional_separator`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PositionalSeparator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PositionalSeparator<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for PositionalSeparator<'tree> {
    type WithLifetime<'a> = PositionalSeparator<'a>;
    const KIND: &'static str = "positional_separator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "positional_separator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "positional_separator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `primary_expression`

This node type has subtypes:

- `attribute` ([`Attribute`])
- `await` ([`Await`])
- `binary_operator` ([`BinaryOperator`])
- `call` ([`Call`])
- `concatenated_string` ([`ConcatenatedString`])
- `dictionary` ([`Dictionary`])
- `dictionary_comprehension` ([`DictionaryComprehension`])
- `ellipsis` ([`Ellipsis`])
- `false` ([`False`])
- `float` ([`Float`])
- `generator_expression` ([`GeneratorExpression`])
- `identifier` ([`Identifier`])
- `integer` ([`Integer`])
- `list` ([`List`])
- `list_comprehension` ([`ListComprehension`])
- `list_splat` ([`ListSplat`])
- `none` ([`None`])
- `parenthesized_expression` ([`ParenthesizedExpression`])
- `set` ([`Set`])
- `set_comprehension` ([`SetComprehension`])
- `string` ([`String`])
- `subscript` ([`Subscript`])
- `true` ([`True`])
- `tuple` ([`Tuple`])
- `unary_operator` ([`UnaryOperator`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum PrimaryExpression<'tree> {
    Attribute(Attribute<'tree>),
    Await(Await<'tree>),
    BinaryOperator(BinaryOperator<'tree>),
    Call(Call<'tree>),
    ConcatenatedString(ConcatenatedString<'tree>),
    Dictionary(Dictionary<'tree>),
    DictionaryComprehension(DictionaryComprehension<'tree>),
    Ellipsis(Ellipsis<'tree>),
    False(False<'tree>),
    Float(Float<'tree>),
    GeneratorExpression(GeneratorExpression<'tree>),
    Identifier(Identifier<'tree>),
    Integer(Integer<'tree>),
    List(List<'tree>),
    ListComprehension(ListComprehension<'tree>),
    ListSplat(ListSplat<'tree>),
    None(None<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    Set(Set<'tree>),
    SetComprehension(SetComprehension<'tree>),
    String(String<'tree>),
    Subscript(Subscript<'tree>),
    True(True<'tree>),
    Tuple(Tuple<'tree>),
    UnaryOperator(UnaryOperator<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> PrimaryExpression<'tree> {
    ///Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`
    #[inline]
    pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Attribute(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`
    #[inline]
    pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Await(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`
    #[inline]
    pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BinaryOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`
    #[inline]
    pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Call(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`
    #[inline]
    pub fn as_concatenated_string(
        self,
    ) -> ::std::option::Option<ConcatenatedString<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConcatenatedString(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`
    #[inline]
    pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Dictionary(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`
    #[inline]
    pub fn as_dictionary_comprehension(
        self,
    ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DictionaryComprehension(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`
    #[inline]
    pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Ellipsis(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `false` ([`False`]), otherwise returns `None`
    #[inline]
    pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::False(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
    #[inline]
    pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Float(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`
    #[inline]
    pub fn as_generator_expression(
        self,
    ) -> ::std::option::Option<GeneratorExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::GeneratorExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Identifier(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
    #[inline]
    pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Integer(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `list` ([`List`]), otherwise returns `None`
    #[inline]
    pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::List(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`
    #[inline]
    pub fn as_list_comprehension(
        self,
    ) -> ::std::option::Option<ListComprehension<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ListComprehension(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`
    #[inline]
    pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ListSplat(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `none` ([`None`]), otherwise returns `None`
    #[inline]
    pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::None(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
    #[inline]
    pub fn as_parenthesized_expression(
        self,
    ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ParenthesizedExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`
    #[inline]
    pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Set(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`
    #[inline]
    pub fn as_set_comprehension(self) -> ::std::option::Option<SetComprehension<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::SetComprehension(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
    #[inline]
    pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Subscript(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `true` ([`True`]), otherwise returns `None`
    #[inline]
    pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::True(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`
    #[inline]
    pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Tuple(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`
    #[inline]
    pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::UnaryOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for PrimaryExpression<'tree> {
    type WithLifetime<'a> = PrimaryExpression<'a>;
    const KIND: &'static str = "primary_expression";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if let Ok(this) = <Attribute<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Attribute(this));
        }
        if let Ok(this) = <Await<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Await(this));
        }
        if let Ok(this) = <BinaryOperator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::BinaryOperator(this));
        }
        if let Ok(this) = <Call<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Call(this));
        }
        if let Ok(this) = <ConcatenatedString<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ConcatenatedString(this));
        }
        if let Ok(this) = <Dictionary<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Dictionary(this));
        }
        if let Ok(this) = <DictionaryComprehension<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::DictionaryComprehension(this));
        }
        if let Ok(this) = <Ellipsis<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Ellipsis(this));
        }
        if let Ok(this) = <False<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::False(this));
        }
        if let Ok(this) = <Float<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Float(this));
        }
        if let Ok(this) = <GeneratorExpression<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::GeneratorExpression(this));
        }
        if let Ok(this) = <Identifier<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Identifier(this));
        }
        if let Ok(this) = <Integer<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Integer(this));
        }
        if let Ok(this) = <List<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::List(this));
        }
        if let Ok(this) = <ListComprehension<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ListComprehension(this));
        }
        if let Ok(this) = <ListSplat<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ListSplat(this));
        }
        if let Ok(this) = <None<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::None(this));
        }
        if let Ok(this) = <ParenthesizedExpression<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::ParenthesizedExpression(this));
        }
        if let Ok(this) = <Set<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Set(this));
        }
        if let Ok(this) = <SetComprehension<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::SetComprehension(this));
        }
        if let Ok(this) = <String<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::String(this));
        }
        if let Ok(this) = <Subscript<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Subscript(this));
        }
        if let Ok(this) = <True<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::True(this));
        }
        if let Ok(this) = <Tuple<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::Tuple(this));
        }
        if let Ok(this) = <UnaryOperator<
            'tree,
        > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
            return Ok(Self::UnaryOperator(this));
        }
        Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::Attribute(x) => ::type_sitter_lib::Node::raw(x),
            Self::Await(x) => ::type_sitter_lib::Node::raw(x),
            Self::BinaryOperator(x) => ::type_sitter_lib::Node::raw(x),
            Self::Call(x) => ::type_sitter_lib::Node::raw(x),
            Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw(x),
            Self::Dictionary(x) => ::type_sitter_lib::Node::raw(x),
            Self::DictionaryComprehension(x) => ::type_sitter_lib::Node::raw(x),
            Self::Ellipsis(x) => ::type_sitter_lib::Node::raw(x),
            Self::False(x) => ::type_sitter_lib::Node::raw(x),
            Self::Float(x) => ::type_sitter_lib::Node::raw(x),
            Self::GeneratorExpression(x) => ::type_sitter_lib::Node::raw(x),
            Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
            Self::Integer(x) => ::type_sitter_lib::Node::raw(x),
            Self::List(x) => ::type_sitter_lib::Node::raw(x),
            Self::ListComprehension(x) => ::type_sitter_lib::Node::raw(x),
            Self::ListSplat(x) => ::type_sitter_lib::Node::raw(x),
            Self::None(x) => ::type_sitter_lib::Node::raw(x),
            Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw(x),
            Self::Set(x) => ::type_sitter_lib::Node::raw(x),
            Self::SetComprehension(x) => ::type_sitter_lib::Node::raw(x),
            Self::String(x) => ::type_sitter_lib::Node::raw(x),
            Self::Subscript(x) => ::type_sitter_lib::Node::raw(x),
            Self::True(x) => ::type_sitter_lib::Node::raw(x),
            Self::Tuple(x) => ::type_sitter_lib::Node::raw(x),
            Self::UnaryOperator(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::Attribute(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Await(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::BinaryOperator(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Call(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Dictionary(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::DictionaryComprehension(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Ellipsis(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Float(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::GeneratorExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Integer(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::List(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ListComprehension(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ListSplat(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::None(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Set(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::SetComprehension(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Subscript(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Tuple(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::UnaryOperator(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        match self {
            Self::Attribute(x) => x.into_raw(),
            Self::Await(x) => x.into_raw(),
            Self::BinaryOperator(x) => x.into_raw(),
            Self::Call(x) => x.into_raw(),
            Self::ConcatenatedString(x) => x.into_raw(),
            Self::Dictionary(x) => x.into_raw(),
            Self::DictionaryComprehension(x) => x.into_raw(),
            Self::Ellipsis(x) => x.into_raw(),
            Self::False(x) => x.into_raw(),
            Self::Float(x) => x.into_raw(),
            Self::GeneratorExpression(x) => x.into_raw(),
            Self::Identifier(x) => x.into_raw(),
            Self::Integer(x) => x.into_raw(),
            Self::List(x) => x.into_raw(),
            Self::ListComprehension(x) => x.into_raw(),
            Self::ListSplat(x) => x.into_raw(),
            Self::None(x) => x.into_raw(),
            Self::ParenthesizedExpression(x) => x.into_raw(),
            Self::Set(x) => x.into_raw(),
            Self::SetComprehension(x) => x.into_raw(),
            Self::String(x) => x.into_raw(),
            Self::Subscript(x) => x.into_raw(),
            Self::True(x) => x.into_raw(),
            Self::Tuple(x) => x.into_raw(),
            Self::UnaryOperator(x) => x.into_raw(),
        }
    }
}
/**Typed node `print_statement`

This node has these fields:

- `argument`: `expression*` ([`Expression`])

And an optional additional named child of type `chevron?` ([`Chevron`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PrintStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PrintStatement<'tree> {
    /**Get the children of field `argument`.

These children have type `expression*` ([`Expression`])*/
    #[inline]
    pub fn arguments<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("argument", &mut c.0)
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child, if it has one.

This child has type `chevron?` ([`Chevron`])*/
    #[inline]
    pub fn chevron(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Chevron<'tree>>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter_lib::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Chevron<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for PrintStatement<'tree> {
    type WithLifetime<'a> = PrintStatement<'a>;
    const KIND: &'static str = "print_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "print_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "print_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `raise_statement`

This node has these fields:

- `cause`: `expression?` ([`Expression`])

And an optional additional named child of type `{expression | expression_list}?`:

- [`Expression`]
- [`ExpressionList`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RaiseStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RaiseStatement<'tree> {
    /**Get the optional field `cause`.

This child has type `expression?` ([`Expression`])*/
    #[inline]
    pub fn cause(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("cause")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child, if it has one.

This child has type `{expression | expression_list}?`:

- [`Expression`]
- [`ExpressionList`]
*/
    #[inline]
    pub fn other(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::Expression_ExpressionList<'tree>,
        >,
    > {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter_lib::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <anon_unions::Expression_ExpressionList<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for RaiseStatement<'tree> {
    type WithLifetime<'a> = RaiseStatement<'a>;
    const KIND: &'static str = "raise_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "raise_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "raise_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `relative_import`

This node has named children of type `{dotted_name | import_prefix}+`:

- [`DottedName`]
- [`ImportPrefix`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RelativeImport<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RelativeImport<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for RelativeImport<'tree> {
    type Child = anon_unions::DottedName_ImportPrefix<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for RelativeImport<'tree> {
    type WithLifetime<'a> = RelativeImport<'a>;
    const KIND: &'static str = "relative_import";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "relative_import" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "relative_import");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `return_statement`

This node has an optional named child of type `{expression | expression_list}?`:

- [`Expression`]
- [`ExpressionList`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ReturnStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ReturnStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasOptionalChild<'tree> for ReturnStatement<'tree> {
    type Child = anon_unions::Expression_ExpressionList<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for ReturnStatement<'tree> {
    type WithLifetime<'a> = ReturnStatement<'a>;
    const KIND: &'static str = "return_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "return_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "return_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `set`

This node has named children of type `{expression | list_splat | parenthesized_list_splat | yield}+`:

- [`Expression`]
- [`ListSplat`]
- [`ParenthesizedListSplat`]
- [`Yield`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Set<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Set<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Set<'tree> {
    type Child = anon_unions::Expression_ListSplat_ParenthesizedListSplat_Yield<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Set<'tree> {
    type WithLifetime<'a> = Set<'a>;
    const KIND: &'static str = "set";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "set" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "set");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `set_comprehension`

This node has these fields:

- `body`: `expression` ([`Expression`])

And additional named children of type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SetComprehension<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SetComprehension<'tree> {
    /**Get the field `body`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{for_in_clause | if_clause}+`:

- [`ForInClause`]
- [`IfClause`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::ForInClause_IfClause<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::ForInClause_IfClause<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for SetComprehension<'tree> {
    type WithLifetime<'a> = SetComprehension<'a>;
    const KIND: &'static str = "set_comprehension";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "set_comprehension" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "set_comprehension");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `slice`

This node has named children of type `expression*` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Slice<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Slice<'tree> {
    /**Get the node's not-extra named children.

These children have type `expression*` ([`Expression`])*/
    #[inline]
    pub fn expressions<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Expression<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Slice<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Slice<'tree> {
    type WithLifetime<'a> = Slice<'a>;
    const KIND: &'static str = "slice";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "slice" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "slice");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `splat_pattern`

This node has an optional named child of type `identifier?` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SplatPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SplatPattern<'tree> {
    /**Get the node's only not-extra named child, if it has one.

This child has type `identifier?` ([`Identifier`])*/
    #[inline]
    pub fn identifier(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, Identifier<'tree>>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasOptionalChild<'tree> for SplatPattern<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for SplatPattern<'tree> {
    type WithLifetime<'a> = SplatPattern<'a>;
    const KIND: &'static str = "splat_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "splat_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "splat_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `splat_type`

This node has a named child of type `identifier` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SplatType<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SplatType<'tree> {
    /**Get the node's only not-extra named child.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn identifier(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for SplatType<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for SplatType<'tree> {
    type WithLifetime<'a> = SplatType<'a>;
    const KIND: &'static str = "splat_type";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "splat_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "splat_type");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `string`

This node has named children of type `{interpolation | string_content | string_end | string_start}+`:

- [`Interpolation`]
- [`StringContent`]
- [`StringEnd`]
- [`StringStart`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> String<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for String<'tree> {
    type Child = anon_unions::Interpolation_StringContent_StringEnd_StringStart<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for String<'tree> {
    type WithLifetime<'a> = String<'a>;
    const KIND: &'static str = "string";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `string_content`

This node has named children of type `{escape_interpolation | escape_sequence}*`:

- [`EscapeInterpolation`]
- [`EscapeSequence`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StringContent<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StringContent<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for StringContent<'tree> {
    type Child = anon_unions::EscapeInterpolation_EscapeSequence<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for StringContent<'tree> {
    type WithLifetime<'a> = StringContent<'a>;
    const KIND: &'static str = "string_content";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "string_content" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string_content");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `string_end`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StringEnd<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StringEnd<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for StringEnd<'tree> {
    type WithLifetime<'a> = StringEnd<'a>;
    const KIND: &'static str = "string_end";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "string_end" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string_end");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `string_start`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StringStart<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StringStart<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for StringStart<'tree> {
    type WithLifetime<'a> = StringStart<'a>;
    const KIND: &'static str = "string_start";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "string_start" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string_start");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `subscript`

This node has these fields:

- `subscript`: `{expression | slice}+` ([`Expression`] | [`Slice`])
- `value`: `primary_expression` ([`PrimaryExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Subscript<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Subscript<'tree> {
    /**Get the children of field `subscript`.

These children have type `{expression | slice}+`:

- [`Expression`]
- [`Slice`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn subscripts<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, anon_unions::Expression_Slice<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .children_by_field_name("subscript", &mut c.0)
            .map(
                <anon_unions::Expression_Slice<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the field `value`.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn value(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Subscript<'tree> {
    type WithLifetime<'a> = Subscript<'a>;
    const KIND: &'static str = "subscript";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "subscript" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "subscript");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `true`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct True<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> True<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for True<'tree> {
    type WithLifetime<'a> = True<'a>;
    const KIND: &'static str = "true";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "true" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "true");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `try_statement`

This node has these fields:

- `body`: `block` ([`Block`])

And additional named children of type `{else_clause | except_clause | except_group_clause | finally_clause}+`:

- [`ElseClause`]
- [`ExceptClause`]
- [`ExceptGroupClause`]
- [`FinallyClause`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TryStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TryStatement<'tree> {
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{else_clause | except_clause | except_group_clause | finally_clause}+`:

- [`ElseClause`]
- [`ExceptClause`]
- [`ExceptGroupClause`]
- [`FinallyClause`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<
            'tree,
            anon_unions::ElseClause_ExceptClause_ExceptGroupClause_FinallyClause<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter_lib::Node::raw(self);
            ::type_sitter_lib::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::ElseClause_ExceptClause_ExceptGroupClause_FinallyClause<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TryStatement<'tree> {
    type WithLifetime<'a> = TryStatement<'a>;
    const KIND: &'static str = "try_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "try_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "try_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `tuple`

This node has named children of type `{expression | list_splat | parenthesized_list_splat | yield}*`:

- [`Expression`]
- [`ListSplat`]
- [`ParenthesizedListSplat`]
- [`Yield`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Tuple<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Tuple<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Tuple<'tree> {
    type Child = anon_unions::Expression_ListSplat_ParenthesizedListSplat_Yield<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Tuple<'tree> {
    type WithLifetime<'a> = Tuple<'a>;
    const KIND: &'static str = "tuple";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "tuple" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "tuple");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `tuple_pattern`

This node has named children of type `{case_pattern | pattern}*`:

- [`CasePattern`]
- [`Pattern`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TuplePattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TuplePattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for TuplePattern<'tree> {
    type Child = anon_unions::CasePattern_Pattern<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TuplePattern<'tree> {
    type WithLifetime<'a> = TuplePattern<'a>;
    const KIND: &'static str = "tuple_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "tuple_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "tuple_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `type`

This node has a named child of type `{constrained_type | expression | generic_type | member_type | splat_type | union_type}`:

- [`ConstrainedType`]
- [`Expression`]
- [`GenericType`]
- [`MemberType`]
- [`SplatType`]
- [`UnionType`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Type<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Type<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChild<'tree> for Type<'tree> {
    type Child = anon_unions::ConstrainedType_Expression_GenericType_MemberType_SplatType_UnionType<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Type<'tree> {
    type WithLifetime<'a> = Type<'a>;
    const KIND: &'static str = "type";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `type_alias_statement`

This node has these fields:

- `left`: `type` ([`Type`])
- `right`: `type` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeAliasStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeAliasStatement<'tree> {
    /**Get the field `left`.

This child has type `type` ([`Type`])*/
    #[inline]
    pub fn left(&self) -> ::type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("left")
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `type` ([`Type`])*/
    #[inline]
    pub fn right(&self) -> ::type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("right")
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TypeAliasStatement<'tree> {
    type WithLifetime<'a> = TypeAliasStatement<'a>;
    const KIND: &'static str = "type_alias_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "type_alias_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_alias_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `type_conversion`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeConversion<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeConversion<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TypeConversion<'tree> {
    type WithLifetime<'a> = TypeConversion<'a>;
    const KIND: &'static str = "type_conversion";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "type_conversion" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_conversion");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `type_parameter`

This node has named children of type `type+` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeParameter<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeParameter<'tree> {
    /**Get the node's not-extra named children.

These children have type `type+` ([`Type`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn types<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Type<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for TypeParameter<'tree> {
    type Child = Type<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TypeParameter<'tree> {
    type WithLifetime<'a> = TypeParameter<'a>;
    const KIND: &'static str = "type_parameter";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "type_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_parameter");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `typed_default_parameter`

This node has these fields:

- `name`: `identifier` ([`Identifier`])
- `type`: `type` ([`Type`])
- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypedDefaultParameter<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypedDefaultParameter<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `type`.

This child has type `type` ([`Type`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TypedDefaultParameter<'tree> {
    type WithLifetime<'a> = TypedDefaultParameter<'a>;
    const KIND: &'static str = "typed_default_parameter";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "typed_default_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "typed_default_parameter");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `typed_parameter`

This node has these fields:

- `type`: `type` ([`Type`])

And an additional named child of type `{dictionary_splat_pattern | identifier | list_splat_pattern}`:

- [`DictionarySplatPattern`]
- [`Identifier`]
- [`ListSplatPattern`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypedParameter<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypedParameter<'tree> {
    /**Get the field `type`.

This child has type `type` ([`Type`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `{dictionary_splat_pattern | identifier | list_splat_pattern}`:

- [`DictionarySplatPattern`]
- [`Identifier`]
- [`ListSplatPattern`]
*/
    #[inline]
    pub fn other(
        &self,
    ) -> ::type_sitter_lib::NodeResult<
        'tree,
        anon_unions::DictionarySplatPattern_Identifier_ListSplatPattern<'tree>,
    > {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter_lib::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <anon_unions::DictionarySplatPattern_Identifier_ListSplatPattern<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for TypedParameter<'tree> {
    type WithLifetime<'a> = TypedParameter<'a>;
    const KIND: &'static str = "typed_parameter";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "typed_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "typed_parameter");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `unary_operator`

This node has these fields:

- `argument`: `primary_expression` ([`PrimaryExpression`])
- `operator`: `{+ | - | ~}` ([`symbols::Add`] | [`symbols::Sub`] | [`symbols::BitNot`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UnaryOperator<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> UnaryOperator<'tree> {
    /**Get the field `argument`.

This child has type `primary_expression` ([`PrimaryExpression`])*/
    #[inline]
    pub fn argument(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("argument")
            .map(
                <PrimaryExpression<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{+ | - | ~}`:

- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::BitNot`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, anon_unions::Add_Sub_BitNot<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::Add_Sub_BitNot<
                    'tree,
                > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for UnaryOperator<'tree> {
    type WithLifetime<'a> = UnaryOperator<'a>;
    const KIND: &'static str = "unary_operator";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "unary_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "unary_operator");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `union_pattern`

This node has named children of type `{class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}*`:

- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Integer`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UnionPattern<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> UnionPattern<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for UnionPattern<'tree> {
    type Child = anon_unions::ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for UnionPattern<'tree> {
    type WithLifetime<'a> = UnionPattern<'a>;
    const KIND: &'static str = "union_pattern";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "union_pattern" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "union_pattern");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `union_type`

This node has named children of type `type+` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UnionType<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> UnionType<'tree> {
    /**Get the node's not-extra named children.

These children have type `type+` ([`Type`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn types<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Type<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Type<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for UnionType<'tree> {
    type Child = Type<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for UnionType<'tree> {
    type WithLifetime<'a> = UnionType<'a>;
    const KIND: &'static str = "union_type";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "union_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "union_type");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `while_statement`

This node has these fields:

- `alternative`: `else_clause?` ([`ElseClause`])
- `body`: `block` ([`Block`])
- `condition`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WhileStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> WhileStatement<'tree> {
    /**Get the optional field `alternative`.

This child has type `else_clause?` ([`ElseClause`])*/
    #[inline]
    pub fn alternative(
        &self,
    ) -> ::std::option::Option<::type_sitter_lib::NodeResult<'tree, ElseClause<'tree>>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("alternative")
            .map(<ElseClause<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `condition`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn condition(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for WhileStatement<'tree> {
    type WithLifetime<'a> = WhileStatement<'a>;
    const KIND: &'static str = "while_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "while_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "while_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `wildcard_import`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WildcardImport<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> WildcardImport<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for WildcardImport<'tree> {
    type WithLifetime<'a> = WildcardImport<'a>;
    const KIND: &'static str = "wildcard_import";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "wildcard_import" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "wildcard_import");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `with_clause`

This node has named children of type `with_item+` ([`WithItem`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WithClause<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> WithClause<'tree> {
    /**Get the node's not-extra named children.

These children have type `with_item+` ([`WithItem`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn with_items<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, WithItem<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<WithItem<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for WithClause<'tree> {
    type Child = WithItem<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for WithClause<'tree> {
    type WithLifetime<'a> = WithClause<'a>;
    const KIND: &'static str = "with_clause";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "with_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "with_clause");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `with_item`

This node has these fields:

- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WithItem<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> WithItem<'tree> {
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for WithItem<'tree> {
    type WithLifetime<'a> = WithItem<'a>;
    const KIND: &'static str = "with_item";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "with_item" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "with_item");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `with_statement`

This node has these fields:

- `body`: `block` ([`Block`])

And an additional named child of type `with_clause` ([`WithClause`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WithStatement<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> WithStatement<'tree> {
    /**Get the field `body`.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("body")
            .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `with_clause` ([`WithClause`])*/
    #[inline]
    pub fn with_clause(
        &self,
    ) -> ::type_sitter_lib::NodeResult<'tree, WithClause<'tree>> {
        (0..::type_sitter_lib::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter_lib::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter_lib::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<WithClause<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for WithStatement<'tree> {
    type WithLifetime<'a> = WithStatement<'a>;
    const KIND: &'static str = "with_statement";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "with_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "with_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
/**Typed node `yield`

This node has an optional named child of type `{expression | expression_list}?`:

- [`Expression`]
- [`ExpressionList`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Yield<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Yield<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasOptionalChild<'tree> for Yield<'tree> {
    type Child = anon_unions::Expression_ExpressionList<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Yield<'tree> {
    type WithLifetime<'a> = Yield<'a>;
    const KIND: &'static str = "yield";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "yield" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "yield");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
        self.0
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `__future__`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Future<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Future<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Future<'tree> {
        type WithLifetime<'a> = Future<'a>;
        const KIND: &'static str = "__future__";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "__future__" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "__future__");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `and`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for And<'tree> {
        type WithLifetime<'a> = And<'a>;
        const KIND: &'static str = "and";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "and" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "and");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `as`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct As<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> As<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for As<'tree> {
        type WithLifetime<'a> = As<'a>;
        const KIND: &'static str = "as";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "as" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "as");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `assert`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Assert<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Assert<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Assert<'tree> {
        type WithLifetime<'a> = Assert<'a>;
        const KIND: &'static str = "assert";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "assert" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "assert");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `async`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Async<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Async<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Async<'tree> {
        type WithLifetime<'a> = Async<'a>;
        const KIND: &'static str = "async";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "async" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "async");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `await`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Await<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Await<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Await<'tree> {
        type WithLifetime<'a> = Await<'a>;
        const KIND: &'static str = "await";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "await" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "await");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `break`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Break<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Break<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Break<'tree> {
        type WithLifetime<'a> = Break<'a>;
        const KIND: &'static str = "break";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "break" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "break");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `case`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Case<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Case<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Case<'tree> {
        type WithLifetime<'a> = Case<'a>;
        const KIND: &'static str = "case";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "case" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "case");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `class`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Class<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Class<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Class<'tree> {
        type WithLifetime<'a> = Class<'a>;
        const KIND: &'static str = "class";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "class" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "class");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `continue`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Continue<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Continue<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Continue<'tree> {
        type WithLifetime<'a> = Continue<'a>;
        const KIND: &'static str = "continue";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "continue" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "continue");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `def`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Def<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Def<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Def<'tree> {
        type WithLifetime<'a> = Def<'a>;
        const KIND: &'static str = "def";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "def" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "def");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `del`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Del<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Del<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Del<'tree> {
        type WithLifetime<'a> = Del<'a>;
        const KIND: &'static str = "del";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "del" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "del");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `elif`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Elif<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Elif<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Elif<'tree> {
        type WithLifetime<'a> = Elif<'a>;
        const KIND: &'static str = "elif";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "elif" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "elif");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `else`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Else<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Else<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Else<'tree> {
        type WithLifetime<'a> = Else<'a>;
        const KIND: &'static str = "else";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "else" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "else");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `except`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Except<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Except<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Except<'tree> {
        type WithLifetime<'a> = Except<'a>;
        const KIND: &'static str = "except";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "except" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "except");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `exec`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Exec<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Exec<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Exec<'tree> {
        type WithLifetime<'a> = Exec<'a>;
        const KIND: &'static str = "exec";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "exec" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "exec");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `finally`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Finally<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Finally<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Finally<'tree> {
        type WithLifetime<'a> = Finally<'a>;
        const KIND: &'static str = "finally";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "finally" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "finally");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `for`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct For<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> For<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for For<'tree> {
        type WithLifetime<'a> = For<'a>;
        const KIND: &'static str = "for";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "for" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "for");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `from`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct From<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> From<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for From<'tree> {
        type WithLifetime<'a> = From<'a>;
        const KIND: &'static str = "from";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "from" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "from");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `global`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Global<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Global<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Global<'tree> {
        type WithLifetime<'a> = Global<'a>;
        const KIND: &'static str = "global";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "global" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "global");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `if`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct If<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> If<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for If<'tree> {
        type WithLifetime<'a> = If<'a>;
        const KIND: &'static str = "if";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "if" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "if");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `import`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Import<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Import<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Import<'tree> {
        type WithLifetime<'a> = Import<'a>;
        const KIND: &'static str = "import";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "import" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "import");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `in`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct In<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> In<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for In<'tree> {
        type WithLifetime<'a> = In<'a>;
        const KIND: &'static str = "in";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "in" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "in");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `is`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Is<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Is<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Is<'tree> {
        type WithLifetime<'a> = Is<'a>;
        const KIND: &'static str = "is";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "is" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "is");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `lambda`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Lambda<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Lambda<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Lambda<'tree> {
        type WithLifetime<'a> = Lambda<'a>;
        const KIND: &'static str = "lambda";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "lambda" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "lambda");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `match`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Match<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Match<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Match<'tree> {
        type WithLifetime<'a> = Match<'a>;
        const KIND: &'static str = "match";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "match" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "match");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `nonlocal`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Nonlocal<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Nonlocal<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Nonlocal<'tree> {
        type WithLifetime<'a> = Nonlocal<'a>;
        const KIND: &'static str = "nonlocal";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "nonlocal" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "nonlocal");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `not`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Not<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Not<'tree> {
        type WithLifetime<'a> = Not<'a>;
        const KIND: &'static str = "not";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "not" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "not");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `or`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;
        const KIND: &'static str = "or";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "or" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "or");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `pass`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Pass<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Pass<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Pass<'tree> {
        type WithLifetime<'a> = Pass<'a>;
        const KIND: &'static str = "pass";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "pass" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "pass");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `print`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Print<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Print<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Print<'tree> {
        type WithLifetime<'a> = Print<'a>;
        const KIND: &'static str = "print";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "print" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "print");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `raise`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Raise<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Raise<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Raise<'tree> {
        type WithLifetime<'a> = Raise<'a>;
        const KIND: &'static str = "raise";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "raise" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "raise");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `return`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Return<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Return<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Return<'tree> {
        type WithLifetime<'a> = Return<'a>;
        const KIND: &'static str = "return";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "return" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "return");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `try`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Try<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Try<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Try<'tree> {
        type WithLifetime<'a> = Try<'a>;
        const KIND: &'static str = "try";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "try" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "try");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `type`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Type<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Type<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Type<'tree> {
        type WithLifetime<'a> = Type<'a>;
        const KIND: &'static str = "type";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "type" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "type");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `while`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct While<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> While<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for While<'tree> {
        type WithLifetime<'a> = While<'a>;
        const KIND: &'static str = "while";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "while" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "while");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `with`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct With<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> With<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for With<'tree> {
        type WithLifetime<'a> = With<'a>;
        const KIND: &'static str = "with";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "with" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "with");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `yield`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Yield<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Yield<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Yield<'tree> {
        type WithLifetime<'a> = Yield<'a>;
        const KIND: &'static str = "yield";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "yield" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "yield");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `!=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for NotEq<'tree> {
        type WithLifetime<'a> = NotEq<'a>;
        const KIND: &'static str = "!=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `%`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mod<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Mod<'tree> {
        type WithLifetime<'a> = Mod<'a>;
        const KIND: &'static str = "%";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "%" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "%");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `%=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ModEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ModEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for ModEq<'tree> {
        type WithLifetime<'a> = ModEq<'a>;
        const KIND: &'static str = "%=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "%=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "%=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `&`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for And<'tree> {
        type WithLifetime<'a> = And<'a>;
        const KIND: &'static str = "&";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "&" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `&=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AndEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AndEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for AndEq<'tree> {
        type WithLifetime<'a> = AndEq<'a>;
        const KIND: &'static str = "&=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "&=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `(`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LParen<'tree> {
        type WithLifetime<'a> = LParen<'a>;
        const KIND: &'static str = "(";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "(");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `)`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for RParen<'tree> {
        type WithLifetime<'a> = RParen<'a>;
        const KIND: &'static str = ")";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ")");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `*`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Mul<'tree> {
        type WithLifetime<'a> = Mul<'a>;
        const KIND: &'static str = "*";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `**`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulMul<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulMul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for MulMul<'tree> {
        type WithLifetime<'a> = MulMul<'a>;
        const KIND: &'static str = "**";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "**" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "**");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `**=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulMulEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulMulEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for MulMulEq<'tree> {
        type WithLifetime<'a> = MulMulEq<'a>;
        const KIND: &'static str = "**=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "**=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "**=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `*=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for MulEq<'tree> {
        type WithLifetime<'a> = MulEq<'a>;
        const KIND: &'static str = "*=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "*=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `+`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Add<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Add<'tree> {
        type WithLifetime<'a> = Add<'a>;
        const KIND: &'static str = "+";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `+=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AddEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AddEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for AddEq<'tree> {
        type WithLifetime<'a> = AddEq<'a>;
        const KIND: &'static str = "+=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "+=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `,`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Comma<'tree> {
        type WithLifetime<'a> = Comma<'a>;
        const KIND: &'static str = ",";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ",");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `-`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Sub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Sub<'tree> {
        type WithLifetime<'a> = Sub<'a>;
        const KIND: &'static str = "-";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `-=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for SubEq<'tree> {
        type WithLifetime<'a> = SubEq<'a>;
        const KIND: &'static str = "-=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "-=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `->`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubGt<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for SubGt<'tree> {
        type WithLifetime<'a> = SubGt<'a>;
        const KIND: &'static str = "->";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "->" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "->");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `.`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Dot<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Dot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Dot<'tree> {
        type WithLifetime<'a> = Dot<'a>;
        const KIND: &'static str = ".";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ".");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `/`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Div<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Div<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Div<'tree> {
        type WithLifetime<'a> = Div<'a>;
        const KIND: &'static str = "/";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `//`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DivDiv<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DivDiv<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DivDiv<'tree> {
        type WithLifetime<'a> = DivDiv<'a>;
        const KIND: &'static str = "//";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "//" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "//");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `//=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DivDivEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DivDivEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DivDivEq<'tree> {
        type WithLifetime<'a> = DivDivEq<'a>;
        const KIND: &'static str = "//=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "//=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "//=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `/=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DivEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DivEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DivEq<'tree> {
        type WithLifetime<'a> = DivEq<'a>;
        const KIND: &'static str = "/=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "/=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `:`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Colon<'tree> {
        type WithLifetime<'a> = Colon<'a>;
        const KIND: &'static str = ":";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `:=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ColonEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ColonEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for ColonEq<'tree> {
        type WithLifetime<'a> = ColonEq<'a>;
        const KIND: &'static str = ":=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ":=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `;`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Semicolon<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Semicolon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Semicolon<'tree> {
        type WithLifetime<'a> = Semicolon<'a>;
        const KIND: &'static str = ";";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ";" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ";");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Lt<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Lt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Lt<'tree> {
        type WithLifetime<'a> = Lt<'a>;
        const KIND: &'static str = "<";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<<`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtLt<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtLt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LtLt<'tree> {
        type WithLifetime<'a> = LtLt<'a>;
        const KIND: &'static str = "<<";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "<<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<<");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<<=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtLtEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtLtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LtLtEq<'tree> {
        type WithLifetime<'a> = LtLtEq<'a>;
        const KIND: &'static str = "<<=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "<<=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<<=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LtEq<'tree> {
        type WithLifetime<'a> = LtEq<'a>;
        const KIND: &'static str = "<=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtGt<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LtGt<'tree> {
        type WithLifetime<'a> = LtGt<'a>;
        const KIND: &'static str = "<>";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "<>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<>");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Eq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Eq<'tree> {
        type WithLifetime<'a> = Eq<'a>;
        const KIND: &'static str = "=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `==`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EqEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for EqEq<'tree> {
        type WithLifetime<'a> = EqEq<'a>;
        const KIND: &'static str = "==";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "==");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Gt<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Gt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Gt<'tree> {
        type WithLifetime<'a> = Gt<'a>;
        const KIND: &'static str = ">";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for GtEq<'tree> {
        type WithLifetime<'a> = GtEq<'a>;
        const KIND: &'static str = ">=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtGt<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for GtGt<'tree> {
        type WithLifetime<'a> = GtGt<'a>;
        const KIND: &'static str = ">>";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ">>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">>");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>>=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtGtEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtGtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for GtGtEq<'tree> {
        type WithLifetime<'a> = GtGtEq<'a>;
        const KIND: &'static str = ">>=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == ">>=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">>=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `@`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct At<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> At<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for At<'tree> {
        type WithLifetime<'a> = At<'a>;
        const KIND: &'static str = "@";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "@" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "@");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `@=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AtEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for AtEq<'tree> {
        type WithLifetime<'a> = AtEq<'a>;
        const KIND: &'static str = "@=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "@=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "@=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `[`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LBracket<'tree> {
        type WithLifetime<'a> = LBracket<'a>;
        const KIND: &'static str = "[";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "[");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `\`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Backslash<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Backslash<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Backslash<'tree> {
        type WithLifetime<'a> = Backslash<'a>;
        const KIND: &'static str = "\\";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "\\" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\\");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `]`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for RBracket<'tree> {
        type WithLifetime<'a> = RBracket<'a>;
        const KIND: &'static str = "]";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "]");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `^`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitXor<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitXor<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for BitXor<'tree> {
        type WithLifetime<'a> = BitXor<'a>;
        const KIND: &'static str = "^";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "^" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "^");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `^=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitXorEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitXorEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for BitXorEq<'tree> {
        type WithLifetime<'a> = BitXorEq<'a>;
        const KIND: &'static str = "^=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "^=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "^=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `_`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct __<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> __<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for __<'tree> {
        type WithLifetime<'a> = __<'a>;
        const KIND: &'static str = "_";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "_" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "_");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `except*`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ExceptMul<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ExceptMul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for ExceptMul<'tree> {
        type WithLifetime<'a> = ExceptMul<'a>;
        const KIND: &'static str = "except*";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "except*" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "except*");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `is not`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct IsSpacenot<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> IsSpacenot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for IsSpacenot<'tree> {
        type WithLifetime<'a> = IsSpacenot<'a>;
        const KIND: &'static str = "is not";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "is not" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "is not");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `not in`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotSpacein<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotSpacein<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for NotSpacein<'tree> {
        type WithLifetime<'a> = NotSpacein<'a>;
        const KIND: &'static str = "not in";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "not in" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "not in");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `{`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for LBrace<'tree> {
        type WithLifetime<'a> = LBrace<'a>;
        const KIND: &'static str = "{";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `|`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;
        const KIND: &'static str = "|";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `|=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct OrEq<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> OrEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for OrEq<'tree> {
        type WithLifetime<'a> = OrEq<'a>;
        const KIND: &'static str = "|=";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "|=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `}`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for RBrace<'tree> {
        type WithLifetime<'a> = RBrace<'a>;
        const KIND: &'static str = "}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
    /**Typed node `~`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitNot<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitNot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for BitNot<'tree> {
        type WithLifetime<'a> = BitNot<'a>;
        const KIND: &'static str = "~";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "~" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "~");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            self.0
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    /**One of `{+ | - | ~}`:
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::BitNot`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Add_Sub_BitNot<'tree> {
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        BitNot(symbols::BitNot<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Add_Sub_BitNot<'tree> {
        ///Returns the node if it is of type `+` ([`symbols::Add`]), otherwise returns `None`
        #[inline]
        pub fn as_add(self) -> ::std::option::Option<symbols::Add<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Add(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `~` ([`symbols::BitNot`]), otherwise returns `None`
        #[inline]
        pub fn as_bit_not(self) -> ::std::option::Option<symbols::BitNot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitNot(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Add_Sub_BitNot<'tree> {
        type WithLifetime<'a> = Add_Sub_BitNot<'a>;
        const KIND: &'static str = "{+ | - | ~}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <symbols::Add<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Add(this));
            }
            if let Ok(this) = <symbols::Sub<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Sub(this));
            }
            if let Ok(this) = <symbols::BitNot<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::BitNot(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Add(x) => ::type_sitter_lib::Node::raw(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw(x),
                Self::BitNot(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Add(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::BitNot(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Add(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::BitNot(x) => x.into_raw(),
            }
        }
    }
    /**One of `{aliased_import | dotted_name}`:
- [`AliasedImport`]
- [`DottedName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AliasedImport_DottedName<'tree> {
        AliasedImport(AliasedImport<'tree>),
        DottedName(DottedName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AliasedImport_DottedName<'tree> {
        ///Returns the node if it is of type `aliased_import` ([`AliasedImport`]), otherwise returns `None`
        #[inline]
        pub fn as_aliased_import(self) -> ::std::option::Option<AliasedImport<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AliasedImport(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for AliasedImport_DottedName<'tree> {
        type WithLifetime<'a> = AliasedImport_DottedName<'a>;
        const KIND: &'static str = "{aliased_import | dotted_name}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <AliasedImport<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AliasedImport(this));
            }
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::AliasedImport(x) => ::type_sitter_lib::Node::raw(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::AliasedImport(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::AliasedImport(x) => x.into_raw(),
                Self::DottedName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{and | or}`:
- [`unnamed::And`]
- [`unnamed::Or`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum And_Or<'tree> {
        And(unnamed::And<'tree>),
        Or(unnamed::Or<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And_Or<'tree> {
        ///Returns the node if it is of type `and` ([`unnamed::And`]), otherwise returns `None`
        #[inline]
        pub fn as_and_(self) -> ::std::option::Option<unnamed::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `or` ([`unnamed::Or`]), otherwise returns `None`
        #[inline]
        pub fn as_or(self) -> ::std::option::Option<unnamed::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for And_Or<'tree> {
        type WithLifetime<'a> = And_Or<'a>;
        const KIND: &'static str = "{and | or}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <unnamed::And<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::And(this));
            }
            if let Ok(this) = <unnamed::Or<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Or(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::And(x) => ::type_sitter_lib::Node::raw(x),
                Self::Or(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::And(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Or(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::And(x) => x.into_raw(),
                Self::Or(x) => x.into_raw(),
            }
        }
    }
    /**One of `{argument_list | generator_expression}`:
- [`ArgumentList`]
- [`GeneratorExpression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ArgumentList_GeneratorExpression<'tree> {
        ArgumentList(ArgumentList<'tree>),
        GeneratorExpression(GeneratorExpression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ArgumentList_GeneratorExpression<'tree> {
        ///Returns the node if it is of type `argument_list` ([`ArgumentList`]), otherwise returns `None`
        #[inline]
        pub fn as_argument_list(self) -> ::std::option::Option<ArgumentList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArgumentList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GeneratorExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ArgumentList_GeneratorExpression<'tree> {
        type WithLifetime<'a> = ArgumentList_GeneratorExpression<'a>;
        const KIND: &'static str = "{argument_list | generator_expression}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ArgumentList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ArgumentList(this));
            }
            if let Ok(this) = <GeneratorExpression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::GeneratorExpression(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ArgumentList(x) => ::type_sitter_lib::Node::raw(x),
                Self::GeneratorExpression(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ArgumentList(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GeneratorExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ArgumentList(x) => x.into_raw(),
                Self::GeneratorExpression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{as_pattern | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | keyword_pattern | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}`:
- [`AsPattern`]
- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Integer`]
- [`KeywordPattern`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AsPattern_ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_KeywordPattern_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        AsPattern(AsPattern<'tree>),
        ClassPattern(ClassPattern<'tree>),
        ComplexPattern(ComplexPattern<'tree>),
        ConcatenatedString(ConcatenatedString<'tree>),
        DictPattern(DictPattern<'tree>),
        DottedName(DottedName<'tree>),
        False(False<'tree>),
        Float(Float<'tree>),
        Integer(Integer<'tree>),
        KeywordPattern(KeywordPattern<'tree>),
        ListPattern(ListPattern<'tree>),
        None(None<'tree>),
        SplatPattern(SplatPattern<'tree>),
        String(String<'tree>),
        True(True<'tree>),
        TuplePattern(TuplePattern<'tree>),
        UnionPattern(UnionPattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > AsPattern_ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_KeywordPattern_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        ///Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AsPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `class_pattern` ([`ClassPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_class_pattern(self) -> ::std::option::Option<ClassPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `complex_pattern` ([`ComplexPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_complex_pattern(self) -> ::std::option::Option<ComplexPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ComplexPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConcatenatedString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dict_pattern` ([`DictPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_dict_pattern(self) -> ::std::option::Option<DictPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `false` ([`False`]), otherwise returns `None`
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::False(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Float(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword_pattern` ([`KeywordPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword_pattern(self) -> ::std::option::Option<KeywordPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::KeywordPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `none` ([`None`]), otherwise returns `None`
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::None(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `splat_pattern` ([`SplatPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_splat_pattern(self) -> ::std::option::Option<SplatPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SplatPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `true` ([`True`]), otherwise returns `None`
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::True(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TuplePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union_pattern` ([`UnionPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_union_pattern(self) -> ::std::option::Option<UnionPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for AsPattern_ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_KeywordPattern_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        type WithLifetime<'a> = AsPattern_ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_KeywordPattern_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
            'a,
        >;
        const KIND: &'static str = "{as_pattern | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | keyword_pattern | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <AsPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AsPattern(this));
            }
            if let Ok(this) = <ClassPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ClassPattern(this));
            }
            if let Ok(this) = <ComplexPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ComplexPattern(this));
            }
            if let Ok(this) = <ConcatenatedString<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ConcatenatedString(this));
            }
            if let Ok(this) = <DictPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictPattern(this));
            }
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            if let Ok(this) = <False<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::False(this));
            }
            if let Ok(this) = <Float<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Float(this));
            }
            if let Ok(this) = <Integer<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Integer(this));
            }
            if let Ok(this) = <KeywordPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::KeywordPattern(this));
            }
            if let Ok(this) = <ListPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListPattern(this));
            }
            if let Ok(this) = <None<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::None(this));
            }
            if let Ok(this) = <SplatPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SplatPattern(this));
            }
            if let Ok(this) = <String<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::String(this));
            }
            if let Ok(this) = <True<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::True(this));
            }
            if let Ok(this) = <TuplePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::TuplePattern(this));
            }
            if let Ok(this) = <UnionPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::UnionPattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::AsPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
                Self::False(x) => ::type_sitter_lib::Node::raw(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw(x),
                Self::KeywordPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::None(x) => ::type_sitter_lib::Node::raw(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::String(x) => ::type_sitter_lib::Node::raw(x),
                Self::True(x) => ::type_sitter_lib::Node::raw(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::AsPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::KeywordPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::None(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::AsPattern(x) => x.into_raw(),
                Self::ClassPattern(x) => x.into_raw(),
                Self::ComplexPattern(x) => x.into_raw(),
                Self::ConcatenatedString(x) => x.into_raw(),
                Self::DictPattern(x) => x.into_raw(),
                Self::DottedName(x) => x.into_raw(),
                Self::False(x) => x.into_raw(),
                Self::Float(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
                Self::KeywordPattern(x) => x.into_raw(),
                Self::ListPattern(x) => x.into_raw(),
                Self::None(x) => x.into_raw(),
                Self::SplatPattern(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
                Self::True(x) => x.into_raw(),
                Self::TuplePattern(x) => x.into_raw(),
                Self::UnionPattern(x) => x.into_raw(),
            }
        }
    }
    /**One of `{assignment | augmented_assignment | expression | expression_list | pattern_list | yield}`:
- [`Assignment`]
- [`AugmentedAssignment`]
- [`Expression`]
- [`ExpressionList`]
- [`PatternList`]
- [`Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
        'tree,
    > {
        Assignment(Assignment<'tree>),
        AugmentedAssignment(AugmentedAssignment<'tree>),
        Expression(Expression<'tree>),
        ExpressionList(ExpressionList<'tree>),
        PatternList(PatternList<'tree>),
        Yield(Yield<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<'tree> {
        ///Returns the node if it is of type `assignment` ([`Assignment`]), otherwise returns `None`
        #[inline]
        pub fn as_assignment(self) -> ::std::option::Option<Assignment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Assignment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `augmented_assignment` ([`AugmentedAssignment`]), otherwise returns `None`
        #[inline]
        pub fn as_augmented_assignment(
            self,
        ) -> ::std::option::Option<AugmentedAssignment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AugmentedAssignment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression_list` ([`ExpressionList`]), otherwise returns `None`
        #[inline]
        pub fn as_expression_list(self) -> ::std::option::Option<ExpressionList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExpressionList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `pattern_list` ([`PatternList`]), otherwise returns `None`
        #[inline]
        pub fn as_pattern_list(self) -> ::std::option::Option<PatternList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `yield` ([`Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield(self) -> ::std::option::Option<Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
        'tree,
    > {
        type WithLifetime<'a> = Assignment_AugmentedAssignment_Expression_ExpressionList_PatternList_Yield<
            'a,
        >;
        const KIND: &'static str = "{assignment | augmented_assignment | expression | expression_list | pattern_list | yield}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Assignment<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Assignment(this));
            }
            if let Ok(this) = <AugmentedAssignment<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AugmentedAssignment(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ExpressionList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ExpressionList(this));
            }
            if let Ok(this) = <PatternList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::PatternList(this));
            }
            if let Ok(this) = <Yield<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Yield(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Assignment(x) => ::type_sitter_lib::Node::raw(x),
                Self::AugmentedAssignment(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::ExpressionList(x) => ::type_sitter_lib::Node::raw(x),
                Self::PatternList(x) => ::type_sitter_lib::Node::raw(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Assignment(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AugmentedAssignment(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ExpressionList(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::PatternList(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Assignment(x) => x.into_raw(),
                Self::AugmentedAssignment(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::ExpressionList(x) => x.into_raw(),
                Self::PatternList(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{assignment | augmented_assignment | expression | yield}`:
- [`Assignment`]
- [`AugmentedAssignment`]
- [`Expression`]
- [`Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Assignment_AugmentedAssignment_Expression_Yield<'tree> {
        Assignment(Assignment<'tree>),
        AugmentedAssignment(AugmentedAssignment<'tree>),
        Expression(Expression<'tree>),
        Yield(Yield<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Assignment_AugmentedAssignment_Expression_Yield<'tree> {
        ///Returns the node if it is of type `assignment` ([`Assignment`]), otherwise returns `None`
        #[inline]
        pub fn as_assignment(self) -> ::std::option::Option<Assignment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Assignment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `augmented_assignment` ([`AugmentedAssignment`]), otherwise returns `None`
        #[inline]
        pub fn as_augmented_assignment(
            self,
        ) -> ::std::option::Option<AugmentedAssignment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AugmentedAssignment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `yield` ([`Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield(self) -> ::std::option::Option<Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Assignment_AugmentedAssignment_Expression_Yield<'tree> {
        type WithLifetime<'a> = Assignment_AugmentedAssignment_Expression_Yield<'a>;
        const KIND: &'static str = "{assignment | augmented_assignment | expression | yield}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Assignment<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Assignment(this));
            }
            if let Ok(this) = <AugmentedAssignment<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AugmentedAssignment(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Yield<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Yield(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Assignment(x) => ::type_sitter_lib::Node::raw(x),
                Self::AugmentedAssignment(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Assignment(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AugmentedAssignment(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Assignment(x) => x.into_raw(),
                Self::AugmentedAssignment(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute | expression | identifier | subscript}`:
- [`Attribute`]
- [`Expression`]
- [`Identifier`]
- [`Subscript`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute_Expression_Identifier_Subscript<'tree> {
        Attribute(Attribute<'tree>),
        Expression(Expression<'tree>),
        Identifier(Identifier<'tree>),
        Subscript(Subscript<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Attribute_Expression_Identifier_Subscript<'tree> {
        ///Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Attribute(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Subscript(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Attribute_Expression_Identifier_Subscript<'tree> {
        type WithLifetime<'a> = Attribute_Expression_Identifier_Subscript<'a>;
        const KIND: &'static str = "{attribute | expression | identifier | subscript}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Attribute<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Attribute(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Subscript<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Subscript(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::Subscript(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Subscript(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Attribute(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Subscript(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute | identifier | subscript}`:
- [`Attribute`]
- [`Identifier`]
- [`Subscript`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute_Identifier_Subscript<'tree> {
        Attribute(Attribute<'tree>),
        Identifier(Identifier<'tree>),
        Subscript(Subscript<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Attribute_Identifier_Subscript<'tree> {
        ///Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Attribute(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Subscript(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Attribute_Identifier_Subscript<'tree> {
        type WithLifetime<'a> = Attribute_Identifier_Subscript<'a>;
        const KIND: &'static str = "{attribute | identifier | subscript}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Attribute<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Attribute(this));
            }
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Subscript<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Subscript(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::Subscript(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Subscript(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Attribute(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Subscript(x) => x.into_raw(),
            }
        }
    }
    /**One of `{block | expression}`:
- [`Block`]
- [`Expression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Block_Expression<'tree> {
        Block(Block<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Block_Expression<'tree> {
        ///Returns the node if it is of type `block` ([`Block`]), otherwise returns `None`
        #[inline]
        pub fn as_block(self) -> ::std::option::Option<Block<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Block(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Block_Expression<'tree> {
        type WithLifetime<'a> = Block_Expression<'a>;
        const KIND: &'static str = "{block | expression}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Block<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Block(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Block(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Block(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Block(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{case_pattern | dotted_name}`:
- [`CasePattern`]
- [`DottedName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CasePattern_DottedName<'tree> {
        CasePattern(CasePattern<'tree>),
        DottedName(DottedName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CasePattern_DottedName<'tree> {
        ///Returns the node if it is of type `case_pattern` ([`CasePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_case_pattern(self) -> ::std::option::Option<CasePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CasePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for CasePattern_DottedName<'tree> {
        type WithLifetime<'a> = CasePattern_DottedName<'a>;
        const KIND: &'static str = "{case_pattern | dotted_name}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <CasePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::CasePattern(this));
            }
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => x.into_raw(),
                Self::DottedName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{case_pattern | expression | identifier}`:
- [`CasePattern`]
- [`Expression`]
- [`Identifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CasePattern_Expression_Identifier<'tree> {
        CasePattern(CasePattern<'tree>),
        Expression(Expression<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CasePattern_Expression_Identifier<'tree> {
        ///Returns the node if it is of type `case_pattern` ([`CasePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_case_pattern(self) -> ::std::option::Option<CasePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CasePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for CasePattern_Expression_Identifier<'tree> {
        type WithLifetime<'a> = CasePattern_Expression_Identifier<'a>;
        const KIND: &'static str = "{case_pattern | expression | identifier}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <CasePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::CasePattern(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{case_pattern | pattern}`:
- [`CasePattern`]
- [`Pattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CasePattern_Pattern<'tree> {
        CasePattern(CasePattern<'tree>),
        Pattern(Pattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CasePattern_Pattern<'tree> {
        ///Returns the node if it is of type `case_pattern` ([`CasePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_case_pattern(self) -> ::std::option::Option<CasePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CasePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `pattern` ([`Pattern`]), otherwise returns `None`
        #[inline]
        pub fn as_pattern(self) -> ::std::option::Option<Pattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_pattern()?.as_attribute()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_pattern()?.as_identifier()
        }
        /**Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
            self.as_pattern()?.as_list_pattern()
        }
        /**Returns the node if it is of type `list_splat_pattern` ([`ListSplatPattern`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_list_splat_pattern(
            self,
        ) -> ::std::option::Option<ListSplatPattern<'tree>> {
            self.as_pattern()?.as_list_splat_pattern()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_pattern()?.as_subscript()
        }
        /**Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            self.as_pattern()?.as_tuple_pattern()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for CasePattern_Pattern<'tree> {
        type WithLifetime<'a> = CasePattern_Pattern<'a>;
        const KIND: &'static str = "{case_pattern | pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <CasePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::CasePattern(this));
            }
            if let Ok(this) = <Pattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Pattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::Pattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Pattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::CasePattern(x) => x.into_raw(),
                Self::Pattern(x) => x.into_raw(),
            }
        }
    }
    /**One of `{class_definition | function_definition}`:
- [`ClassDefinition`]
- [`FunctionDefinition`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ClassDefinition_FunctionDefinition<'tree> {
        ClassDefinition(ClassDefinition<'tree>),
        FunctionDefinition(FunctionDefinition<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ClassDefinition_FunctionDefinition<'tree> {
        ///Returns the node if it is of type `class_definition` ([`ClassDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_class_definition(
            self,
        ) -> ::std::option::Option<ClassDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `function_definition` ([`FunctionDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_function_definition(
            self,
        ) -> ::std::option::Option<FunctionDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FunctionDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Get the field `body`.

This child has type `block` ([`Block`])*/
        #[inline]
        pub fn body(&self) -> ::type_sitter_lib::NodeResult<'tree, Block<'tree>> {
            ::type_sitter_lib::Node::raw(self)
                .child_by_field_name("body")
                .map(<Block<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
                .expect(
                    "required child not present, there should at least be a MISSING node in its place",
                )
        }
        /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
        #[inline]
        pub fn name(&self) -> ::type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
            ::type_sitter_lib::Node::raw(self)
                .child_by_field_name("name")
                .map(<Identifier<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
                .expect(
                    "required child not present, there should at least be a MISSING node in its place",
                )
        }
        /**Get the optional field `type_parameters`.

This child has type `type_parameter?` ([`TypeParameter`])*/
        #[inline]
        pub fn type_parameters(
            &self,
        ) -> ::std::option::Option<
            ::type_sitter_lib::NodeResult<'tree, TypeParameter<'tree>>,
        > {
            ::type_sitter_lib::Node::raw(self)
                .child_by_field_name("type_parameters")
                .map(
                    <TypeParameter<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::try_from_raw,
                )
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ClassDefinition_FunctionDefinition<'tree> {
        type WithLifetime<'a> = ClassDefinition_FunctionDefinition<'a>;
        const KIND: &'static str = "{class_definition | function_definition}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ClassDefinition<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ClassDefinition(this));
            }
            if let Ok(this) = <FunctionDefinition<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::FunctionDefinition(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ClassDefinition(x) => ::type_sitter_lib::Node::raw(x),
                Self::FunctionDefinition(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ClassDefinition(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::FunctionDefinition(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ClassDefinition(x) => x.into_raw(),
                Self::FunctionDefinition(x) => x.into_raw(),
            }
        }
    }
    /**One of `{class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | identifier | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}`:
- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Identifier`]
- [`Integer`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Identifier_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        ClassPattern(ClassPattern<'tree>),
        ComplexPattern(ComplexPattern<'tree>),
        ConcatenatedString(ConcatenatedString<'tree>),
        DictPattern(DictPattern<'tree>),
        DottedName(DottedName<'tree>),
        False(False<'tree>),
        Float(Float<'tree>),
        Identifier(Identifier<'tree>),
        Integer(Integer<'tree>),
        ListPattern(ListPattern<'tree>),
        None(None<'tree>),
        SplatPattern(SplatPattern<'tree>),
        String(String<'tree>),
        True(True<'tree>),
        TuplePattern(TuplePattern<'tree>),
        UnionPattern(UnionPattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Identifier_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        ///Returns the node if it is of type `class_pattern` ([`ClassPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_class_pattern(self) -> ::std::option::Option<ClassPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `complex_pattern` ([`ComplexPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_complex_pattern(self) -> ::std::option::Option<ComplexPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ComplexPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConcatenatedString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dict_pattern` ([`DictPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_dict_pattern(self) -> ::std::option::Option<DictPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `false` ([`False`]), otherwise returns `None`
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::False(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Float(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `none` ([`None`]), otherwise returns `None`
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::None(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `splat_pattern` ([`SplatPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_splat_pattern(self) -> ::std::option::Option<SplatPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SplatPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `true` ([`True`]), otherwise returns `None`
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::True(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TuplePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union_pattern` ([`UnionPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_union_pattern(self) -> ::std::option::Option<UnionPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Identifier_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        type WithLifetime<'a> = ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Identifier_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
            'a,
        >;
        const KIND: &'static str = "{class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | identifier | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ClassPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ClassPattern(this));
            }
            if let Ok(this) = <ComplexPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ComplexPattern(this));
            }
            if let Ok(this) = <ConcatenatedString<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ConcatenatedString(this));
            }
            if let Ok(this) = <DictPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictPattern(this));
            }
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            if let Ok(this) = <False<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::False(this));
            }
            if let Ok(this) = <Float<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Float(this));
            }
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Integer<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Integer(this));
            }
            if let Ok(this) = <ListPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListPattern(this));
            }
            if let Ok(this) = <None<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::None(this));
            }
            if let Ok(this) = <SplatPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SplatPattern(this));
            }
            if let Ok(this) = <String<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::String(this));
            }
            if let Ok(this) = <True<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::True(this));
            }
            if let Ok(this) = <TuplePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::TuplePattern(this));
            }
            if let Ok(this) = <UnionPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::UnionPattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
                Self::False(x) => ::type_sitter_lib::Node::raw(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::None(x) => ::type_sitter_lib::Node::raw(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::String(x) => ::type_sitter_lib::Node::raw(x),
                Self::True(x) => ::type_sitter_lib::Node::raw(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::None(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ClassPattern(x) => x.into_raw(),
                Self::ComplexPattern(x) => x.into_raw(),
                Self::ConcatenatedString(x) => x.into_raw(),
                Self::DictPattern(x) => x.into_raw(),
                Self::DottedName(x) => x.into_raw(),
                Self::False(x) => x.into_raw(),
                Self::Float(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
                Self::ListPattern(x) => x.into_raw(),
                Self::None(x) => x.into_raw(),
                Self::SplatPattern(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
                Self::True(x) => x.into_raw(),
                Self::TuplePattern(x) => x.into_raw(),
                Self::UnionPattern(x) => x.into_raw(),
            }
        }
    }
    /**One of `{class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}`:
- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Integer`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        ClassPattern(ClassPattern<'tree>),
        ComplexPattern(ComplexPattern<'tree>),
        ConcatenatedString(ConcatenatedString<'tree>),
        DictPattern(DictPattern<'tree>),
        DottedName(DottedName<'tree>),
        False(False<'tree>),
        Float(Float<'tree>),
        Integer(Integer<'tree>),
        ListPattern(ListPattern<'tree>),
        None(None<'tree>),
        SplatPattern(SplatPattern<'tree>),
        String(String<'tree>),
        True(True<'tree>),
        TuplePattern(TuplePattern<'tree>),
        UnionPattern(UnionPattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        ///Returns the node if it is of type `class_pattern` ([`ClassPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_class_pattern(self) -> ::std::option::Option<ClassPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `complex_pattern` ([`ComplexPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_complex_pattern(self) -> ::std::option::Option<ComplexPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ComplexPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConcatenatedString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dict_pattern` ([`DictPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_dict_pattern(self) -> ::std::option::Option<DictPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `false` ([`False`]), otherwise returns `None`
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::False(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Float(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `none` ([`None`]), otherwise returns `None`
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::None(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `splat_pattern` ([`SplatPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_splat_pattern(self) -> ::std::option::Option<SplatPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SplatPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `true` ([`True`]), otherwise returns `None`
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::True(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TuplePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union_pattern` ([`UnionPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_union_pattern(self) -> ::std::option::Option<UnionPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        type WithLifetime<'a> = ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
            'a,
        >;
        const KIND: &'static str = "{class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ClassPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ClassPattern(this));
            }
            if let Ok(this) = <ComplexPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ComplexPattern(this));
            }
            if let Ok(this) = <ConcatenatedString<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ConcatenatedString(this));
            }
            if let Ok(this) = <DictPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictPattern(this));
            }
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            if let Ok(this) = <False<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::False(this));
            }
            if let Ok(this) = <Float<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Float(this));
            }
            if let Ok(this) = <Integer<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Integer(this));
            }
            if let Ok(this) = <ListPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListPattern(this));
            }
            if let Ok(this) = <None<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::None(this));
            }
            if let Ok(this) = <SplatPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SplatPattern(this));
            }
            if let Ok(this) = <String<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::String(this));
            }
            if let Ok(this) = <True<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::True(this));
            }
            if let Ok(this) = <TuplePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::TuplePattern(this));
            }
            if let Ok(this) = <UnionPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::UnionPattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
                Self::False(x) => ::type_sitter_lib::Node::raw(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::None(x) => ::type_sitter_lib::Node::raw(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::String(x) => ::type_sitter_lib::Node::raw(x),
                Self::True(x) => ::type_sitter_lib::Node::raw(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::None(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ClassPattern(x) => x.into_raw(),
                Self::ComplexPattern(x) => x.into_raw(),
                Self::ConcatenatedString(x) => x.into_raw(),
                Self::DictPattern(x) => x.into_raw(),
                Self::DottedName(x) => x.into_raw(),
                Self::False(x) => x.into_raw(),
                Self::Float(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
                Self::ListPattern(x) => x.into_raw(),
                Self::None(x) => x.into_raw(),
                Self::SplatPattern(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
                Self::True(x) => x.into_raw(),
                Self::TuplePattern(x) => x.into_raw(),
                Self::UnionPattern(x) => x.into_raw(),
            }
        }
    }
    /**One of `{, | expression}`:
- [`symbols::Comma`]
- [`Expression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comma_Expression<'tree> {
        Comma(symbols::Comma<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comma_Expression<'tree> {
        ///Returns the node if it is of type `,` ([`symbols::Comma`]), otherwise returns `None`
        #[inline]
        pub fn as_comma(self) -> ::std::option::Option<symbols::Comma<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comma(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Comma_Expression<'tree> {
        type WithLifetime<'a> = Comma_Expression<'a>;
        const KIND: &'static str = "{, | expression}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <symbols::Comma<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Comma(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{_compound_statement | _simple_statement}`:
- [`CompoundStatement`]
- [`SimpleStatement`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CompoundStatement_SimpleStatement<'tree> {
        CompoundStatement(CompoundStatement<'tree>),
        SimpleStatement(SimpleStatement<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CompoundStatement_SimpleStatement<'tree> {
        ///Returns the node if it is of type `_compound_statement` ([`CompoundStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_compound_statement(
            self,
        ) -> ::std::option::Option<CompoundStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CompoundStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `_simple_statement` ([`SimpleStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_simple_statement(
            self,
        ) -> ::std::option::Option<SimpleStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SimpleStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `class_definition` ([`ClassDefinition`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_class_definition(
            self,
        ) -> ::std::option::Option<ClassDefinition<'tree>> {
            self.as_compound_statement()?.as_class_definition()
        }
        /**Returns the node if it is of type `decorated_definition` ([`DecoratedDefinition`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_decorated_definition(
            self,
        ) -> ::std::option::Option<DecoratedDefinition<'tree>> {
            self.as_compound_statement()?.as_decorated_definition()
        }
        /**Returns the node if it is of type `for_statement` ([`ForStatement`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_for_statement(self) -> ::std::option::Option<ForStatement<'tree>> {
            self.as_compound_statement()?.as_for_statement()
        }
        /**Returns the node if it is of type `function_definition` ([`FunctionDefinition`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_function_definition(
            self,
        ) -> ::std::option::Option<FunctionDefinition<'tree>> {
            self.as_compound_statement()?.as_function_definition()
        }
        /**Returns the node if it is of type `if_statement` ([`IfStatement`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_if_statement(self) -> ::std::option::Option<IfStatement<'tree>> {
            self.as_compound_statement()?.as_if_statement()
        }
        /**Returns the node if it is of type `match_statement` ([`MatchStatement`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_match_statement(self) -> ::std::option::Option<MatchStatement<'tree>> {
            self.as_compound_statement()?.as_match_statement()
        }
        /**Returns the node if it is of type `try_statement` ([`TryStatement`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_try_statement(self) -> ::std::option::Option<TryStatement<'tree>> {
            self.as_compound_statement()?.as_try_statement()
        }
        /**Returns the node if it is of type `while_statement` ([`WhileStatement`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_while_statement(self) -> ::std::option::Option<WhileStatement<'tree>> {
            self.as_compound_statement()?.as_while_statement()
        }
        /**Returns the node if it is of type `with_statement` ([`WithStatement`]), otherwise returns `None`.

Follows the following chain:
- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))*/
        #[inline]
        pub fn as_with_statement(self) -> ::std::option::Option<WithStatement<'tree>> {
            self.as_compound_statement()?.as_with_statement()
        }
        /**Returns the node if it is of type `assert_statement` ([`AssertStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_assert_statement(
            self,
        ) -> ::std::option::Option<AssertStatement<'tree>> {
            self.as_simple_statement()?.as_assert_statement()
        }
        /**Returns the node if it is of type `break_statement` ([`BreakStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_break_statement(self) -> ::std::option::Option<BreakStatement<'tree>> {
            self.as_simple_statement()?.as_break_statement()
        }
        /**Returns the node if it is of type `continue_statement` ([`ContinueStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_continue_statement(
            self,
        ) -> ::std::option::Option<ContinueStatement<'tree>> {
            self.as_simple_statement()?.as_continue_statement()
        }
        /**Returns the node if it is of type `delete_statement` ([`DeleteStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_delete_statement(
            self,
        ) -> ::std::option::Option<DeleteStatement<'tree>> {
            self.as_simple_statement()?.as_delete_statement()
        }
        /**Returns the node if it is of type `exec_statement` ([`ExecStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_exec_statement(self) -> ::std::option::Option<ExecStatement<'tree>> {
            self.as_simple_statement()?.as_exec_statement()
        }
        /**Returns the node if it is of type `expression_statement` ([`ExpressionStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_expression_statement(
            self,
        ) -> ::std::option::Option<ExpressionStatement<'tree>> {
            self.as_simple_statement()?.as_expression_statement()
        }
        /**Returns the node if it is of type `future_import_statement` ([`FutureImportStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_future_import_statement(
            self,
        ) -> ::std::option::Option<FutureImportStatement<'tree>> {
            self.as_simple_statement()?.as_future_import_statement()
        }
        /**Returns the node if it is of type `global_statement` ([`GlobalStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_global_statement(
            self,
        ) -> ::std::option::Option<GlobalStatement<'tree>> {
            self.as_simple_statement()?.as_global_statement()
        }
        /**Returns the node if it is of type `import_from_statement` ([`ImportFromStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_import_from_statement(
            self,
        ) -> ::std::option::Option<ImportFromStatement<'tree>> {
            self.as_simple_statement()?.as_import_from_statement()
        }
        /**Returns the node if it is of type `import_statement` ([`ImportStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_import_statement(
            self,
        ) -> ::std::option::Option<ImportStatement<'tree>> {
            self.as_simple_statement()?.as_import_statement()
        }
        /**Returns the node if it is of type `nonlocal_statement` ([`NonlocalStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_nonlocal_statement(
            self,
        ) -> ::std::option::Option<NonlocalStatement<'tree>> {
            self.as_simple_statement()?.as_nonlocal_statement()
        }
        /**Returns the node if it is of type `pass_statement` ([`PassStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_pass_statement(self) -> ::std::option::Option<PassStatement<'tree>> {
            self.as_simple_statement()?.as_pass_statement()
        }
        /**Returns the node if it is of type `print_statement` ([`PrintStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_print_statement(self) -> ::std::option::Option<PrintStatement<'tree>> {
            self.as_simple_statement()?.as_print_statement()
        }
        /**Returns the node if it is of type `raise_statement` ([`RaiseStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_raise_statement(self) -> ::std::option::Option<RaiseStatement<'tree>> {
            self.as_simple_statement()?.as_raise_statement()
        }
        /**Returns the node if it is of type `return_statement` ([`ReturnStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_return_statement(
            self,
        ) -> ::std::option::Option<ReturnStatement<'tree>> {
            self.as_simple_statement()?.as_return_statement()
        }
        /**Returns the node if it is of type `type_alias_statement` ([`TypeAliasStatement`]), otherwise returns `None`.

Follows the following chain:
- `_simple_statement` ([`SimpleStatement < 'tree >`], from [`as_simple_statement`](Self::as_simple_statement))*/
        #[inline]
        pub fn as_type_alias_statement(
            self,
        ) -> ::std::option::Option<TypeAliasStatement<'tree>> {
            self.as_simple_statement()?.as_type_alias_statement()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for CompoundStatement_SimpleStatement<'tree> {
        type WithLifetime<'a> = CompoundStatement_SimpleStatement<'a>;
        const KIND: &'static str = "{_compound_statement | _simple_statement}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <CompoundStatement<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::CompoundStatement(this));
            }
            if let Ok(this) = <SimpleStatement<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SimpleStatement(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::CompoundStatement(x) => ::type_sitter_lib::Node::raw(x),
                Self::SimpleStatement(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::CompoundStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SimpleStatement(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::CompoundStatement(x) => x.into_raw(),
                Self::SimpleStatement(x) => x.into_raw(),
            }
        }
    }
    /**One of `{constrained_type | expression | generic_type | member_type | splat_type | union_type}`:
- [`ConstrainedType`]
- [`Expression`]
- [`GenericType`]
- [`MemberType`]
- [`SplatType`]
- [`UnionType`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstrainedType_Expression_GenericType_MemberType_SplatType_UnionType<
        'tree,
    > {
        ConstrainedType(ConstrainedType<'tree>),
        Expression(Expression<'tree>),
        GenericType(GenericType<'tree>),
        MemberType(MemberType<'tree>),
        SplatType(SplatType<'tree>),
        UnionType(UnionType<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ConstrainedType_Expression_GenericType_MemberType_SplatType_UnionType<'tree> {
        ///Returns the node if it is of type `constrained_type` ([`ConstrainedType`]), otherwise returns `None`
        #[inline]
        pub fn as_constrained_type(
            self,
        ) -> ::std::option::Option<ConstrainedType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConstrainedType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `generic_type` ([`GenericType`]), otherwise returns `None`
        #[inline]
        pub fn as_generic_type(self) -> ::std::option::Option<GenericType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GenericType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_type` ([`MemberType`]), otherwise returns `None`
        #[inline]
        pub fn as_member_type(self) -> ::std::option::Option<MemberType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `splat_type` ([`SplatType`]), otherwise returns `None`
        #[inline]
        pub fn as_splat_type(self) -> ::std::option::Option<SplatType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SplatType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union_type` ([`UnionType`]), otherwise returns `None`
        #[inline]
        pub fn as_union_type(self) -> ::std::option::Option<UnionType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ConstrainedType_Expression_GenericType_MemberType_SplatType_UnionType<'tree> {
        type WithLifetime<'a> = ConstrainedType_Expression_GenericType_MemberType_SplatType_UnionType<
            'a,
        >;
        const KIND: &'static str = "{constrained_type | expression | generic_type | member_type | splat_type | union_type}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ConstrainedType<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ConstrainedType(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <GenericType<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::GenericType(this));
            }
            if let Ok(this) = <MemberType<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::MemberType(this));
            }
            if let Ok(this) = <SplatType<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SplatType(this));
            }
            if let Ok(this) = <UnionType<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::UnionType(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ConstrainedType(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::GenericType(x) => ::type_sitter_lib::Node::raw(x),
                Self::MemberType(x) => ::type_sitter_lib::Node::raw(x),
                Self::SplatType(x) => ::type_sitter_lib::Node::raw(x),
                Self::UnionType(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ConstrainedType(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GenericType(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MemberType(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SplatType(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::UnionType(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ConstrainedType(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::GenericType(x) => x.into_raw(),
                Self::MemberType(x) => x.into_raw(),
                Self::SplatType(x) => x.into_raw(),
                Self::UnionType(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dictionary_splat_pattern | identifier | list_splat_pattern}`:
- [`DictionarySplatPattern`]
- [`Identifier`]
- [`ListSplatPattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DictionarySplatPattern_Identifier_ListSplatPattern<'tree> {
        DictionarySplatPattern(DictionarySplatPattern<'tree>),
        Identifier(Identifier<'tree>),
        ListSplatPattern(ListSplatPattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DictionarySplatPattern_Identifier_ListSplatPattern<'tree> {
        ///Returns the node if it is of type `dictionary_splat_pattern` ([`DictionarySplatPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_dictionary_splat_pattern(
            self,
        ) -> ::std::option::Option<DictionarySplatPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictionarySplatPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_splat_pattern` ([`ListSplatPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_list_splat_pattern(
            self,
        ) -> ::std::option::Option<ListSplatPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListSplatPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for DictionarySplatPattern_Identifier_ListSplatPattern<'tree> {
        type WithLifetime<'a> = DictionarySplatPattern_Identifier_ListSplatPattern<'a>;
        const KIND: &'static str = "{dictionary_splat_pattern | identifier | list_splat_pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <DictionarySplatPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictionarySplatPattern(this));
            }
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <ListSplatPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListSplatPattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplatPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListSplatPattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListSplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplatPattern(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::ListSplatPattern(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dictionary_splat | expression | keyword_argument | list_splat | parenthesized_expression}`:
- [`DictionarySplat`]
- [`Expression`]
- [`KeywordArgument`]
- [`ListSplat`]
- [`ParenthesizedExpression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DictionarySplat_Expression_KeywordArgument_ListSplat_ParenthesizedExpression<
        'tree,
    > {
        DictionarySplat(DictionarySplat<'tree>),
        Expression(Expression<'tree>),
        KeywordArgument(KeywordArgument<'tree>),
        ListSplat(ListSplat<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > DictionarySplat_Expression_KeywordArgument_ListSplat_ParenthesizedExpression<
        'tree,
    > {
        ///Returns the node if it is of type `dictionary_splat` ([`DictionarySplat`]), otherwise returns `None`
        #[inline]
        pub fn as_dictionary_splat(
            self,
        ) -> ::std::option::Option<DictionarySplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictionarySplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword_argument` ([`KeywordArgument`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword_argument(
            self,
        ) -> ::std::option::Option<KeywordArgument<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::KeywordArgument(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListSplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for DictionarySplat_Expression_KeywordArgument_ListSplat_ParenthesizedExpression<
        'tree,
    > {
        type WithLifetime<'a> = DictionarySplat_Expression_KeywordArgument_ListSplat_ParenthesizedExpression<
            'a,
        >;
        const KIND: &'static str = "{dictionary_splat | expression | keyword_argument | list_splat | parenthesized_expression}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <DictionarySplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictionarySplat(this));
            }
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <KeywordArgument<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::KeywordArgument(this));
            }
            if let Ok(this) = <ListSplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListSplat(this));
            }
            if let Ok(this) = <ParenthesizedExpression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ParenthesizedExpression(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::KeywordArgument(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::KeywordArgument(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplat(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::KeywordArgument(x) => x.into_raw(),
                Self::ListSplat(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dictionary_splat | pair}`:
- [`DictionarySplat`]
- [`Pair`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DictionarySplat_Pair<'tree> {
        DictionarySplat(DictionarySplat<'tree>),
        Pair(Pair<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DictionarySplat_Pair<'tree> {
        ///Returns the node if it is of type `dictionary_splat` ([`DictionarySplat`]), otherwise returns `None`
        #[inline]
        pub fn as_dictionary_splat(
            self,
        ) -> ::std::option::Option<DictionarySplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictionarySplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `pair` ([`Pair`]), otherwise returns `None`
        #[inline]
        pub fn as_pair(self) -> ::std::option::Option<Pair<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pair(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DictionarySplat_Pair<'tree> {
        type WithLifetime<'a> = DictionarySplat_Pair<'a>;
        const KIND: &'static str = "{dictionary_splat | pair}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <DictionarySplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictionarySplat(this));
            }
            if let Ok(this) = <Pair<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Pair(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::Pair(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Pair(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::DictionarySplat(x) => x.into_raw(),
                Self::Pair(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dotted_name | import_prefix}`:
- [`DottedName`]
- [`ImportPrefix`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DottedName_ImportPrefix<'tree> {
        DottedName(DottedName<'tree>),
        ImportPrefix(ImportPrefix<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DottedName_ImportPrefix<'tree> {
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `import_prefix` ([`ImportPrefix`]), otherwise returns `None`
        #[inline]
        pub fn as_import_prefix(self) -> ::std::option::Option<ImportPrefix<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ImportPrefix(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DottedName_ImportPrefix<'tree> {
        type WithLifetime<'a> = DottedName_ImportPrefix<'a>;
        const KIND: &'static str = "{dotted_name | import_prefix}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            if let Ok(this) = <ImportPrefix<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ImportPrefix(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
                Self::ImportPrefix(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ImportPrefix(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::DottedName(x) => x.into_raw(),
                Self::ImportPrefix(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dotted_name | relative_import}`:
- [`DottedName`]
- [`RelativeImport`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DottedName_RelativeImport<'tree> {
        DottedName(DottedName<'tree>),
        RelativeImport(RelativeImport<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DottedName_RelativeImport<'tree> {
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `relative_import` ([`RelativeImport`]), otherwise returns `None`
        #[inline]
        pub fn as_relative_import(self) -> ::std::option::Option<RelativeImport<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RelativeImport(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DottedName_RelativeImport<'tree> {
        type WithLifetime<'a> = DottedName_RelativeImport<'a>;
        const KIND: &'static str = "{dotted_name | relative_import}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            if let Ok(this) = <RelativeImport<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::RelativeImport(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
                Self::RelativeImport(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::RelativeImport(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::DottedName(x) => x.into_raw(),
                Self::RelativeImport(x) => x.into_raw(),
            }
        }
    }
    /**One of `{elif_clause | else_clause}`:
- [`ElifClause`]
- [`ElseClause`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ElifClause_ElseClause<'tree> {
        ElifClause(ElifClause<'tree>),
        ElseClause(ElseClause<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ElifClause_ElseClause<'tree> {
        ///Returns the node if it is of type `elif_clause` ([`ElifClause`]), otherwise returns `None`
        #[inline]
        pub fn as_elif_clause(self) -> ::std::option::Option<ElifClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ElifClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `else_clause` ([`ElseClause`]), otherwise returns `None`
        #[inline]
        pub fn as_else_clause(self) -> ::std::option::Option<ElseClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ElseClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for ElifClause_ElseClause<'tree> {
        type WithLifetime<'a> = ElifClause_ElseClause<'a>;
        const KIND: &'static str = "{elif_clause | else_clause}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ElifClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ElifClause(this));
            }
            if let Ok(this) = <ElseClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ElseClause(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ElifClause(x) => ::type_sitter_lib::Node::raw(x),
                Self::ElseClause(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ElifClause(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ElseClause(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ElifClause(x) => x.into_raw(),
                Self::ElseClause(x) => x.into_raw(),
            }
        }
    }
    /**One of `{else_clause | except_clause | except_group_clause | finally_clause}`:
- [`ElseClause`]
- [`ExceptClause`]
- [`ExceptGroupClause`]
- [`FinallyClause`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ElseClause_ExceptClause_ExceptGroupClause_FinallyClause<'tree> {
        ElseClause(ElseClause<'tree>),
        ExceptClause(ExceptClause<'tree>),
        ExceptGroupClause(ExceptGroupClause<'tree>),
        FinallyClause(FinallyClause<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ElseClause_ExceptClause_ExceptGroupClause_FinallyClause<'tree> {
        ///Returns the node if it is of type `else_clause` ([`ElseClause`]), otherwise returns `None`
        #[inline]
        pub fn as_else_clause(self) -> ::std::option::Option<ElseClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ElseClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `except_clause` ([`ExceptClause`]), otherwise returns `None`
        #[inline]
        pub fn as_except_clause(self) -> ::std::option::Option<ExceptClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExceptClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `except_group_clause` ([`ExceptGroupClause`]), otherwise returns `None`
        #[inline]
        pub fn as_except_group_clause(
            self,
        ) -> ::std::option::Option<ExceptGroupClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExceptGroupClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `finally_clause` ([`FinallyClause`]), otherwise returns `None`
        #[inline]
        pub fn as_finally_clause(self) -> ::std::option::Option<FinallyClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FinallyClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ElseClause_ExceptClause_ExceptGroupClause_FinallyClause<'tree> {
        type WithLifetime<'a> = ElseClause_ExceptClause_ExceptGroupClause_FinallyClause<
            'a,
        >;
        const KIND: &'static str = "{else_clause | except_clause | except_group_clause | finally_clause}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ElseClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ElseClause(this));
            }
            if let Ok(this) = <ExceptClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ExceptClause(this));
            }
            if let Ok(this) = <ExceptGroupClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ExceptGroupClause(this));
            }
            if let Ok(this) = <FinallyClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::FinallyClause(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ElseClause(x) => ::type_sitter_lib::Node::raw(x),
                Self::ExceptClause(x) => ::type_sitter_lib::Node::raw(x),
                Self::ExceptGroupClause(x) => ::type_sitter_lib::Node::raw(x),
                Self::FinallyClause(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ElseClause(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ExceptClause(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ExceptGroupClause(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::FinallyClause(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ElseClause(x) => x.into_raw(),
                Self::ExceptClause(x) => x.into_raw(),
                Self::ExceptGroupClause(x) => x.into_raw(),
                Self::FinallyClause(x) => x.into_raw(),
            }
        }
    }
    /**One of `{escape_interpolation | escape_sequence}`:
- [`EscapeInterpolation`]
- [`EscapeSequence`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum EscapeInterpolation_EscapeSequence<'tree> {
        EscapeInterpolation(EscapeInterpolation<'tree>),
        EscapeSequence(EscapeSequence<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EscapeInterpolation_EscapeSequence<'tree> {
        ///Returns the node if it is of type `escape_interpolation` ([`EscapeInterpolation`]), otherwise returns `None`
        #[inline]
        pub fn as_escape_interpolation(
            self,
        ) -> ::std::option::Option<EscapeInterpolation<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EscapeInterpolation(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `escape_sequence` ([`EscapeSequence`]), otherwise returns `None`
        #[inline]
        pub fn as_escape_sequence(self) -> ::std::option::Option<EscapeSequence<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EscapeSequence(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for EscapeInterpolation_EscapeSequence<'tree> {
        type WithLifetime<'a> = EscapeInterpolation_EscapeSequence<'a>;
        const KIND: &'static str = "{escape_interpolation | escape_sequence}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <EscapeInterpolation<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::EscapeInterpolation(this));
            }
            if let Ok(this) = <EscapeSequence<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::EscapeSequence(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::EscapeInterpolation(x) => ::type_sitter_lib::Node::raw(x),
                Self::EscapeSequence(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::EscapeInterpolation(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::EscapeSequence(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::EscapeInterpolation(x) => x.into_raw(),
                Self::EscapeSequence(x) => x.into_raw(),
            }
        }
    }
    /**One of `{expression | expression_list}`:
- [`Expression`]
- [`ExpressionList`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_ExpressionList<'tree> {
        Expression(Expression<'tree>),
        ExpressionList(ExpressionList<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_ExpressionList<'tree> {
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression_list` ([`ExpressionList`]), otherwise returns `None`
        #[inline]
        pub fn as_expression_list(self) -> ::std::option::Option<ExpressionList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExpressionList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Expression_ExpressionList<'tree> {
        type WithLifetime<'a> = Expression_ExpressionList<'a>;
        const KIND: &'static str = "{expression | expression_list}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ExpressionList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ExpressionList(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::ExpressionList(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ExpressionList(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::ExpressionList(x) => x.into_raw(),
            }
        }
    }
    /**One of `{expression | expression_list | pattern_list | yield}`:
- [`Expression`]
- [`ExpressionList`]
- [`PatternList`]
- [`Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_ExpressionList_PatternList_Yield<'tree> {
        Expression(Expression<'tree>),
        ExpressionList(ExpressionList<'tree>),
        PatternList(PatternList<'tree>),
        Yield(Yield<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_ExpressionList_PatternList_Yield<'tree> {
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression_list` ([`ExpressionList`]), otherwise returns `None`
        #[inline]
        pub fn as_expression_list(self) -> ::std::option::Option<ExpressionList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExpressionList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `pattern_list` ([`PatternList`]), otherwise returns `None`
        #[inline]
        pub fn as_pattern_list(self) -> ::std::option::Option<PatternList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `yield` ([`Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield(self) -> ::std::option::Option<Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Expression_ExpressionList_PatternList_Yield<'tree> {
        type WithLifetime<'a> = Expression_ExpressionList_PatternList_Yield<'a>;
        const KIND: &'static str = "{expression | expression_list | pattern_list | yield}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ExpressionList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ExpressionList(this));
            }
            if let Ok(this) = <PatternList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::PatternList(this));
            }
            if let Ok(this) = <Yield<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Yield(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::ExpressionList(x) => ::type_sitter_lib::Node::raw(x),
                Self::PatternList(x) => ::type_sitter_lib::Node::raw(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ExpressionList(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::PatternList(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::ExpressionList(x) => x.into_raw(),
                Self::PatternList(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{expression | list_splat | parenthesized_expression | yield}`:
- [`Expression`]
- [`ListSplat`]
- [`ParenthesizedExpression`]
- [`Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_ListSplat_ParenthesizedExpression_Yield<'tree> {
        Expression(Expression<'tree>),
        ListSplat(ListSplat<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        Yield(Yield<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_ListSplat_ParenthesizedExpression_Yield<'tree> {
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListSplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `yield` ([`Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield(self) -> ::std::option::Option<Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Expression_ListSplat_ParenthesizedExpression_Yield<'tree> {
        type WithLifetime<'a> = Expression_ListSplat_ParenthesizedExpression_Yield<'a>;
        const KIND: &'static str = "{expression | list_splat | parenthesized_expression | yield}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ListSplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListSplat(this));
            }
            if let Ok(this) = <ParenthesizedExpression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ParenthesizedExpression(this));
            }
            if let Ok(this) = <Yield<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Yield(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::ListSplat(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{expression | list_splat | parenthesized_list_splat | yield}`:
- [`Expression`]
- [`ListSplat`]
- [`ParenthesizedListSplat`]
- [`Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_ListSplat_ParenthesizedListSplat_Yield<'tree> {
        Expression(Expression<'tree>),
        ListSplat(ListSplat<'tree>),
        ParenthesizedListSplat(ParenthesizedListSplat<'tree>),
        Yield(Yield<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_ListSplat_ParenthesizedListSplat_Yield<'tree> {
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListSplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_list_splat` ([`ParenthesizedListSplat`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_list_splat(
            self,
        ) -> ::std::option::Option<ParenthesizedListSplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedListSplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `yield` ([`Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield(self) -> ::std::option::Option<Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Expression_ListSplat_ParenthesizedListSplat_Yield<'tree> {
        type WithLifetime<'a> = Expression_ListSplat_ParenthesizedListSplat_Yield<'a>;
        const KIND: &'static str = "{expression | list_splat | parenthesized_list_splat | yield}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ListSplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListSplat(this));
            }
            if let Ok(this) = <ParenthesizedListSplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ParenthesizedListSplat(this));
            }
            if let Ok(this) = <Yield<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Yield(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::ParenthesizedListSplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ParenthesizedListSplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::ListSplat(x) => x.into_raw(),
                Self::ParenthesizedListSplat(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{expression | slice}`:
- [`Expression`]
- [`Slice`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Slice<'tree> {
        Expression(Expression<'tree>),
        Slice(Slice<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_Slice<'tree> {
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `slice` ([`Slice`]), otherwise returns `None`
        #[inline]
        pub fn as_slice(self) -> ::std::option::Option<Slice<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Slice(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `as_pattern` ([`AsPattern`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_as_pattern(self) -> ::std::option::Option<AsPattern<'tree>> {
            self.as_expression()?.as_as_pattern()
        }
        /**Returns the node if it is of type `boolean_operator` ([`BooleanOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_boolean_operator(
            self,
        ) -> ::std::option::Option<BooleanOperator<'tree>> {
            self.as_expression()?.as_boolean_operator()
        }
        /**Returns the node if it is of type `comparison_operator` ([`ComparisonOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_comparison_operator(
            self,
        ) -> ::std::option::Option<ComparisonOperator<'tree>> {
            self.as_expression()?.as_comparison_operator()
        }
        /**Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }
        /**Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            self.as_expression()?.as_lambda()
        }
        /**Returns the node if it is of type `named_expression` ([`NamedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_named_expression(
            self,
        ) -> ::std::option::Option<NamedExpression<'tree>> {
            self.as_expression()?.as_named_expression()
        }
        /**Returns the node if it is of type `not_operator` ([`NotOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_not_operator(self) -> ::std::option::Option<NotOperator<'tree>> {
            self.as_expression()?.as_not_operator()
        }
        /**Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))*/
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_attribute()
        }
        /**Returns the node if it is of type `await` ([`Await`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<Await<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_await()
        }
        /**Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_binary_operator()
        }
        /**Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }
        /**Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_concatenated_string()
        }
        /**Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary()
        }
        /**Returns the node if it is of type `dictionary_comprehension` ([`DictionaryComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_dictionary_comprehension(
            self,
        ) -> ::std::option::Option<DictionaryComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_dictionary_comprehension()
        }
        /**Returns the node if it is of type `ellipsis` ([`Ellipsis`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_ellipsis(self) -> ::std::option::Option<Ellipsis<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_ellipsis()
        }
        /**Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }
        /**Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }
        /**Returns the node if it is of type `generator_expression` ([`GeneratorExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_generator_expression(
            self,
        ) -> ::std::option::Option<GeneratorExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_generator_expression()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_identifier()
        }
        /**Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }
        /**Returns the node if it is of type `list` ([`List`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list(self) -> ::std::option::Option<List<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list()
        }
        /**Returns the node if it is of type `list_comprehension` ([`ListComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_comprehension(
            self,
        ) -> ::std::option::Option<ListComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_comprehension()
        }
        /**Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_list_splat()
        }
        /**Returns the node if it is of type `none` ([`None`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_none()
        }
        /**Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_parenthesized_expression()
        }
        /**Returns the node if it is of type `set` ([`Set`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set(self) -> ::std::option::Option<Set<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set()
        }
        /**Returns the node if it is of type `set_comprehension` ([`SetComprehension`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_set_comprehension(
            self,
        ) -> ::std::option::Option<SetComprehension<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_set_comprehension()
        }
        /**Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_subscript()
        }
        /**Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }
        /**Returns the node if it is of type `tuple` ([`Tuple`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_tuple(self) -> ::std::option::Option<Tuple<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_tuple()
        }
        /**Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.

Follows the following chain:
- `expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))
- `primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))*/
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Expression_Slice<'tree> {
        type WithLifetime<'a> = Expression_Slice<'a>;
        const KIND: &'static str = "{expression | slice}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Slice<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Slice(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw(x),
                Self::Slice(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Slice(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::Slice(x) => x.into_raw(),
            }
        }
    }
    /**One of `{float | integer}`:
- [`Float`]
- [`Integer`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Float_Integer<'tree> {
        Float(Float<'tree>),
        Integer(Integer<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Float_Integer<'tree> {
        ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Float(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Float_Integer<'tree> {
        type WithLifetime<'a> = Float_Integer<'a>;
        const KIND: &'static str = "{float | integer}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Float<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Float(this));
            }
            if let Ok(this) = <Integer<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Integer(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Float(x) => ::type_sitter_lib::Node::raw(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Float(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Float(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
            }
        }
    }
    /**One of `{for_in_clause | if_clause}`:
- [`ForInClause`]
- [`IfClause`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ForInClause_IfClause<'tree> {
        ForInClause(ForInClause<'tree>),
        IfClause(IfClause<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ForInClause_IfClause<'tree> {
        ///Returns the node if it is of type `for_in_clause` ([`ForInClause`]), otherwise returns `None`
        #[inline]
        pub fn as_for_in_clause(self) -> ::std::option::Option<ForInClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ForInClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `if_clause` ([`IfClause`]), otherwise returns `None`
        #[inline]
        pub fn as_if_clause(self) -> ::std::option::Option<IfClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::IfClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for ForInClause_IfClause<'tree> {
        type WithLifetime<'a> = ForInClause_IfClause<'a>;
        const KIND: &'static str = "{for_in_clause | if_clause}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ForInClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ForInClause(this));
            }
            if let Ok(this) = <IfClause<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::IfClause(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ForInClause(x) => ::type_sitter_lib::Node::raw(x),
                Self::IfClause(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ForInClause(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::IfClause(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ForInClause(x) => x.into_raw(),
                Self::IfClause(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | string}`:
- [`Identifier`]
- [`String`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_String<'tree> {
        Identifier(Identifier<'tree>),
        String(String<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Identifier_String<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Identifier_String<'tree> {
        type WithLifetime<'a> = Identifier_String<'a>;
        const KIND: &'static str = "{identifier | string}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <String<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::String(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::String(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | tuple_pattern}`:
- [`Identifier`]
- [`TuplePattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_TuplePattern<'tree> {
        Identifier(Identifier<'tree>),
        TuplePattern(TuplePattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Identifier_TuplePattern<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TuplePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Identifier_TuplePattern<'tree> {
        type WithLifetime<'a> = Identifier_TuplePattern<'a>;
        const KIND: &'static str = "{identifier | tuple_pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <TuplePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::TuplePattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TuplePattern(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | type}`:
- [`Identifier`]
- [`Type`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_Type<'tree> {
        Identifier(Identifier<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Identifier_Type<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `type` ([`Type`]), otherwise returns `None`
        #[inline]
        pub fn as_type(self) -> ::std::option::Option<Type<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Type(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Identifier_Type<'tree> {
        type WithLifetime<'a> = Identifier_Type<'a>;
        const KIND: &'static str = "{identifier | type}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Type<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Type(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::Type(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Type(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::Type(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | type_parameter}`:
- [`Identifier`]
- [`TypeParameter`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_TypeParameter<'tree> {
        Identifier(Identifier<'tree>),
        TypeParameter(TypeParameter<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Identifier_TypeParameter<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `type_parameter` ([`TypeParameter`]), otherwise returns `None`
        #[inline]
        pub fn as_type_parameter(self) -> ::std::option::Option<TypeParameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeParameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Identifier_TypeParameter<'tree> {
        type WithLifetime<'a> = Identifier_TypeParameter<'a>;
        const KIND: &'static str = "{identifier | type_parameter}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Identifier<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <TypeParameter<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::TypeParameter(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::TypeParameter(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TypeParameter(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TypeParameter(x) => x.into_raw(),
            }
        }
    }
    /**One of `{interpolation | string_content | string_end | string_start}`:
- [`Interpolation`]
- [`StringContent`]
- [`StringEnd`]
- [`StringStart`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Interpolation_StringContent_StringEnd_StringStart<'tree> {
        Interpolation(Interpolation<'tree>),
        StringContent(StringContent<'tree>),
        StringEnd(StringEnd<'tree>),
        StringStart(StringStart<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Interpolation_StringContent_StringEnd_StringStart<'tree> {
        ///Returns the node if it is of type `interpolation` ([`Interpolation`]), otherwise returns `None`
        #[inline]
        pub fn as_interpolation(self) -> ::std::option::Option<Interpolation<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Interpolation(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string_content` ([`StringContent`]), otherwise returns `None`
        #[inline]
        pub fn as_string_content(self) -> ::std::option::Option<StringContent<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringContent(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string_end` ([`StringEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_string_end(self) -> ::std::option::Option<StringEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string_start` ([`StringStart`]), otherwise returns `None`
        #[inline]
        pub fn as_string_start(self) -> ::std::option::Option<StringStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Interpolation_StringContent_StringEnd_StringStart<'tree> {
        type WithLifetime<'a> = Interpolation_StringContent_StringEnd_StringStart<'a>;
        const KIND: &'static str = "{interpolation | string_content | string_end | string_start}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Interpolation<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Interpolation(this));
            }
            if let Ok(this) = <StringContent<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::StringContent(this));
            }
            if let Ok(this) = <StringEnd<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::StringEnd(this));
            }
            if let Ok(this) = <StringStart<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::StringStart(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Interpolation(x) => ::type_sitter_lib::Node::raw(x),
                Self::StringContent(x) => ::type_sitter_lib::Node::raw(x),
                Self::StringEnd(x) => ::type_sitter_lib::Node::raw(x),
                Self::StringStart(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Interpolation(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::StringContent(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::StringEnd(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::StringStart(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Interpolation(x) => x.into_raw(),
                Self::StringContent(x) => x.into_raw(),
                Self::StringEnd(x) => x.into_raw(),
                Self::StringStart(x) => x.into_raw(),
            }
        }
    }
    /**One of `{list_splat | parenthesized_expression}`:
- [`ListSplat`]
- [`ParenthesizedExpression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ListSplat_ParenthesizedExpression<'tree> {
        ListSplat(ListSplat<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ListSplat_ParenthesizedExpression<'tree> {
        ///Returns the node if it is of type `list_splat` ([`ListSplat`]), otherwise returns `None`
        #[inline]
        pub fn as_list_splat(self) -> ::std::option::Option<ListSplat<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListSplat(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ListSplat_ParenthesizedExpression<'tree> {
        type WithLifetime<'a> = ListSplat_ParenthesizedExpression<'a>;
        const KIND: &'static str = "{list_splat | parenthesized_expression}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <ListSplat<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListSplat(this));
            }
            if let Ok(this) = <ParenthesizedExpression<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ParenthesizedExpression(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ListSplat(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ParenthesizedExpression(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ListSplat(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{%= | &= | **= | *= | += | -= | //= | /= | <<= | >>= | @= | ^= | |=}`:
- [`symbols::ModEq`]
- [`symbols::AndEq`]
- [`symbols::MulMulEq`]
- [`symbols::MulEq`]
- [`symbols::AddEq`]
- [`symbols::SubEq`]
- [`symbols::DivDivEq`]
- [`symbols::DivEq`]
- [`symbols::LtLtEq`]
- [`symbols::GtGtEq`]
- [`symbols::AtEq`]
- [`symbols::BitXorEq`]
- [`symbols::OrEq`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivDivEq_DivEq_LtLtEq_GtGtEq_AtEq_BitXorEq_OrEq<
        'tree,
    > {
        ModEq(symbols::ModEq<'tree>),
        AndEq(symbols::AndEq<'tree>),
        MulMulEq(symbols::MulMulEq<'tree>),
        MulEq(symbols::MulEq<'tree>),
        AddEq(symbols::AddEq<'tree>),
        SubEq(symbols::SubEq<'tree>),
        DivDivEq(symbols::DivDivEq<'tree>),
        DivEq(symbols::DivEq<'tree>),
        LtLtEq(symbols::LtLtEq<'tree>),
        GtGtEq(symbols::GtGtEq<'tree>),
        AtEq(symbols::AtEq<'tree>),
        BitXorEq(symbols::BitXorEq<'tree>),
        OrEq(symbols::OrEq<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivDivEq_DivEq_LtLtEq_GtGtEq_AtEq_BitXorEq_OrEq<
        'tree,
    > {
        ///Returns the node if it is of type `%=` ([`symbols::ModEq`]), otherwise returns `None`
        #[inline]
        pub fn as_mod_eq(self) -> ::std::option::Option<symbols::ModEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ModEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `&=` ([`symbols::AndEq`]), otherwise returns `None`
        #[inline]
        pub fn as_and_eq(self) -> ::std::option::Option<symbols::AndEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AndEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `**=` ([`symbols::MulMulEq`]), otherwise returns `None`
        #[inline]
        pub fn as_mul_mul_eq(self) -> ::std::option::Option<symbols::MulMulEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulMulEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `*=` ([`symbols::MulEq`]), otherwise returns `None`
        #[inline]
        pub fn as_mul_eq(self) -> ::std::option::Option<symbols::MulEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `+=` ([`symbols::AddEq`]), otherwise returns `None`
        #[inline]
        pub fn as_add_eq(self) -> ::std::option::Option<symbols::AddEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AddEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `-=` ([`symbols::SubEq`]), otherwise returns `None`
        #[inline]
        pub fn as_sub_eq(self) -> ::std::option::Option<symbols::SubEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `//=` ([`symbols::DivDivEq`]), otherwise returns `None`
        #[inline]
        pub fn as_div_div_eq(self) -> ::std::option::Option<symbols::DivDivEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DivDivEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `/=` ([`symbols::DivEq`]), otherwise returns `None`
        #[inline]
        pub fn as_div_eq(self) -> ::std::option::Option<symbols::DivEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DivEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<<=` ([`symbols::LtLtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_lt_eq(self) -> ::std::option::Option<symbols::LtLtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtLtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>>=` ([`symbols::GtGtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_gt_eq(self) -> ::std::option::Option<symbols::GtGtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtGtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `@=` ([`symbols::AtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_at_eq(self) -> ::std::option::Option<symbols::AtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `^=` ([`symbols::BitXorEq`]), otherwise returns `None`
        #[inline]
        pub fn as_bit_xor_eq(self) -> ::std::option::Option<symbols::BitXorEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitXorEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `|=` ([`symbols::OrEq`]), otherwise returns `None`
        #[inline]
        pub fn as_or_eq(self) -> ::std::option::Option<symbols::OrEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OrEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivDivEq_DivEq_LtLtEq_GtGtEq_AtEq_BitXorEq_OrEq<
        'tree,
    > {
        type WithLifetime<'a> = ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivDivEq_DivEq_LtLtEq_GtGtEq_AtEq_BitXorEq_OrEq<
            'a,
        >;
        const KIND: &'static str = "{%= | &= | **= | *= | += | -= | //= | /= | <<= | >>= | @= | ^= | |=}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <symbols::ModEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ModEq(this));
            }
            if let Ok(this) = <symbols::AndEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AndEq(this));
            }
            if let Ok(this) = <symbols::MulMulEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::MulMulEq(this));
            }
            if let Ok(this) = <symbols::MulEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::MulEq(this));
            }
            if let Ok(this) = <symbols::AddEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AddEq(this));
            }
            if let Ok(this) = <symbols::SubEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SubEq(this));
            }
            if let Ok(this) = <symbols::DivDivEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DivDivEq(this));
            }
            if let Ok(this) = <symbols::DivEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DivEq(this));
            }
            if let Ok(this) = <symbols::LtLtEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::LtLtEq(this));
            }
            if let Ok(this) = <symbols::GtGtEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::GtGtEq(this));
            }
            if let Ok(this) = <symbols::AtEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::AtEq(this));
            }
            if let Ok(this) = <symbols::BitXorEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::BitXorEq(this));
            }
            if let Ok(this) = <symbols::OrEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::OrEq(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::AndEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::MulMulEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::MulEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::AddEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::SubEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::DivDivEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::DivEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtLtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::GtGtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::AtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::BitXorEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::OrEq(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AndEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MulMulEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MulEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AddEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SubEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DivDivEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DivEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtLtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GtGtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::BitXorEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::OrEq(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => x.into_raw(),
                Self::AndEq(x) => x.into_raw(),
                Self::MulMulEq(x) => x.into_raw(),
                Self::MulEq(x) => x.into_raw(),
                Self::AddEq(x) => x.into_raw(),
                Self::SubEq(x) => x.into_raw(),
                Self::DivDivEq(x) => x.into_raw(),
                Self::DivEq(x) => x.into_raw(),
                Self::LtLtEq(x) => x.into_raw(),
                Self::GtGtEq(x) => x.into_raw(),
                Self::AtEq(x) => x.into_raw(),
                Self::BitXorEq(x) => x.into_raw(),
                Self::OrEq(x) => x.into_raw(),
            }
        }
    }
    /**One of `{% | & | * | ** | + | - | / | // | << | >> | @ | ^ | |}`:
- [`symbols::Mod`]
- [`symbols::And`]
- [`symbols::Mul`]
- [`symbols::MulMul`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::Div`]
- [`symbols::DivDiv`]
- [`symbols::LtLt`]
- [`symbols::GtGt`]
- [`symbols::At`]
- [`symbols::BitXor`]
- [`symbols::Or`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Mod_And_Mul_MulMul_Add_Sub_Div_DivDiv_LtLt_GtGt_At_BitXor_Or<'tree> {
        Mod(symbols::Mod<'tree>),
        And(symbols::And<'tree>),
        Mul(symbols::Mul<'tree>),
        MulMul(symbols::MulMul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        Div(symbols::Div<'tree>),
        DivDiv(symbols::DivDiv<'tree>),
        LtLt(symbols::LtLt<'tree>),
        GtGt(symbols::GtGt<'tree>),
        At(symbols::At<'tree>),
        BitXor(symbols::BitXor<'tree>),
        Or(symbols::Or<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mod_And_Mul_MulMul_Add_Sub_Div_DivDiv_LtLt_GtGt_At_BitXor_Or<'tree> {
        ///Returns the node if it is of type `%` ([`symbols::Mod`]), otherwise returns `None`
        #[inline]
        pub fn as_mod(self) -> ::std::option::Option<symbols::Mod<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mod(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `&` ([`symbols::And`]), otherwise returns `None`
        #[inline]
        pub fn as_and(self) -> ::std::option::Option<symbols::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `*` ([`symbols::Mul`]), otherwise returns `None`
        #[inline]
        pub fn as_mul(self) -> ::std::option::Option<symbols::Mul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `**` ([`symbols::MulMul`]), otherwise returns `None`
        #[inline]
        pub fn as_mul_mul(self) -> ::std::option::Option<symbols::MulMul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulMul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `+` ([`symbols::Add`]), otherwise returns `None`
        #[inline]
        pub fn as_add(self) -> ::std::option::Option<symbols::Add<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Add(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `/` ([`symbols::Div`]), otherwise returns `None`
        #[inline]
        pub fn as_div(self) -> ::std::option::Option<symbols::Div<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Div(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `//` ([`symbols::DivDiv`]), otherwise returns `None`
        #[inline]
        pub fn as_div_div(self) -> ::std::option::Option<symbols::DivDiv<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DivDiv(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<<` ([`symbols::LtLt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_lt(self) -> ::std::option::Option<symbols::LtLt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtLt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>>` ([`symbols::GtGt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_gt(self) -> ::std::option::Option<symbols::GtGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `@` ([`symbols::At`]), otherwise returns `None`
        #[inline]
        pub fn as_at(self) -> ::std::option::Option<symbols::At<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::At(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `^` ([`symbols::BitXor`]), otherwise returns `None`
        #[inline]
        pub fn as_bit_xor(self) -> ::std::option::Option<symbols::BitXor<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitXor(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `|` ([`symbols::Or`]), otherwise returns `None`
        #[inline]
        pub fn as_or_(self) -> ::std::option::Option<symbols::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Mod_And_Mul_MulMul_Add_Sub_Div_DivDiv_LtLt_GtGt_At_BitXor_Or<'tree> {
        type WithLifetime<'a> = Mod_And_Mul_MulMul_Add_Sub_Div_DivDiv_LtLt_GtGt_At_BitXor_Or<
            'a,
        >;
        const KIND: &'static str = "{% | & | * | ** | + | - | / | // | << | >> | @ | ^ | |}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <symbols::Mod<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Mod(this));
            }
            if let Ok(this) = <symbols::And<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::And(this));
            }
            if let Ok(this) = <symbols::Mul<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Mul(this));
            }
            if let Ok(this) = <symbols::MulMul<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::MulMul(this));
            }
            if let Ok(this) = <symbols::Add<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Add(this));
            }
            if let Ok(this) = <symbols::Sub<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Sub(this));
            }
            if let Ok(this) = <symbols::Div<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Div(this));
            }
            if let Ok(this) = <symbols::DivDiv<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DivDiv(this));
            }
            if let Ok(this) = <symbols::LtLt<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::LtLt(this));
            }
            if let Ok(this) = <symbols::GtGt<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::GtGt(this));
            }
            if let Ok(this) = <symbols::At<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::At(this));
            }
            if let Ok(this) = <symbols::BitXor<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::BitXor(this));
            }
            if let Ok(this) = <symbols::Or<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Or(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Mod(x) => ::type_sitter_lib::Node::raw(x),
                Self::And(x) => ::type_sitter_lib::Node::raw(x),
                Self::Mul(x) => ::type_sitter_lib::Node::raw(x),
                Self::MulMul(x) => ::type_sitter_lib::Node::raw(x),
                Self::Add(x) => ::type_sitter_lib::Node::raw(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw(x),
                Self::Div(x) => ::type_sitter_lib::Node::raw(x),
                Self::DivDiv(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtLt(x) => ::type_sitter_lib::Node::raw(x),
                Self::GtGt(x) => ::type_sitter_lib::Node::raw(x),
                Self::At(x) => ::type_sitter_lib::Node::raw(x),
                Self::BitXor(x) => ::type_sitter_lib::Node::raw(x),
                Self::Or(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Mod(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::And(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MulMul(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Div(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DivDiv(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtLt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GtGt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::At(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::BitXor(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Or(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Mod(x) => x.into_raw(),
                Self::And(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
                Self::MulMul(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::Div(x) => x.into_raw(),
                Self::DivDiv(x) => x.into_raw(),
                Self::LtLt(x) => x.into_raw(),
                Self::GtGt(x) => x.into_raw(),
                Self::At(x) => x.into_raw(),
                Self::BitXor(x) => x.into_raw(),
                Self::Or(x) => x.into_raw(),
            }
        }
    }
    /**One of `{!= | < | <= | <> | == | > | >= | in | is | is not | not in}`:
- [`symbols::NotEq`]
- [`symbols::Lt`]
- [`symbols::LtEq`]
- [`symbols::LtGt`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
- [`unnamed::In`]
- [`unnamed::Is`]
- [`symbols::IsSpacenot`]
- [`symbols::NotSpacein`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NotEq_Lt_LtEq_LtGt_EqEq_Gt_GtEq_In_Is_IsSpacenot_NotSpacein<'tree> {
        NotEq(symbols::NotEq<'tree>),
        Lt(symbols::Lt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        LtGt(symbols::LtGt<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
        In(unnamed::In<'tree>),
        Is(unnamed::Is<'tree>),
        IsSpacenot(symbols::IsSpacenot<'tree>),
        NotSpacein(symbols::NotSpacein<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq_Lt_LtEq_LtGt_EqEq_Gt_GtEq_In_Is_IsSpacenot_NotSpacein<'tree> {
        ///Returns the node if it is of type `!=` ([`symbols::NotEq`]), otherwise returns `None`
        #[inline]
        pub fn as_not_eq(self) -> ::std::option::Option<symbols::NotEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt(self) -> ::std::option::Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<=` ([`symbols::LtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_eq(self) -> ::std::option::Option<symbols::LtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<>` ([`symbols::LtGt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_gt(self) -> ::std::option::Option<symbols::LtGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `==` ([`symbols::EqEq`]), otherwise returns `None`
        #[inline]
        pub fn as_eq_eq(self) -> ::std::option::Option<symbols::EqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt(self) -> ::std::option::Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>=` ([`symbols::GtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_eq(self) -> ::std::option::Option<symbols::GtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `in` ([`unnamed::In`]), otherwise returns `None`
        #[inline]
        pub fn as_in(self) -> ::std::option::Option<unnamed::In<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::In(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `is` ([`unnamed::Is`]), otherwise returns `None`
        #[inline]
        pub fn as_is(self) -> ::std::option::Option<unnamed::Is<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Is(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `is not` ([`symbols::IsSpacenot`]), otherwise returns `None`
        #[inline]
        pub fn as_is_spacenot(
            self,
        ) -> ::std::option::Option<symbols::IsSpacenot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::IsSpacenot(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `not in` ([`symbols::NotSpacein`]), otherwise returns `None`
        #[inline]
        pub fn as_not_spacein(
            self,
        ) -> ::std::option::Option<symbols::NotSpacein<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotSpacein(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for NotEq_Lt_LtEq_LtGt_EqEq_Gt_GtEq_In_Is_IsSpacenot_NotSpacein<'tree> {
        type WithLifetime<'a> = NotEq_Lt_LtEq_LtGt_EqEq_Gt_GtEq_In_Is_IsSpacenot_NotSpacein<
            'a,
        >;
        const KIND: &'static str = "{!= | < | <= | <> | == | > | >= | in | is | is not | not in}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <symbols::NotEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::NotEq(this));
            }
            if let Ok(this) = <symbols::Lt<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Lt(this));
            }
            if let Ok(this) = <symbols::LtEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::LtEq(this));
            }
            if let Ok(this) = <symbols::LtGt<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::LtGt(this));
            }
            if let Ok(this) = <symbols::EqEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::EqEq(this));
            }
            if let Ok(this) = <symbols::Gt<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Gt(this));
            }
            if let Ok(this) = <symbols::GtEq<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::GtEq(this));
            }
            if let Ok(this) = <unnamed::In<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::In(this));
            }
            if let Ok(this) = <unnamed::Is<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Is(this));
            }
            if let Ok(this) = <symbols::IsSpacenot<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::IsSpacenot(this));
            }
            if let Ok(this) = <symbols::NotSpacein<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::NotSpacein(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Lt(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtGt(x) => ::type_sitter_lib::Node::raw(x),
                Self::EqEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Gt(x) => ::type_sitter_lib::Node::raw(x),
                Self::GtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::In(x) => ::type_sitter_lib::Node::raw(x),
                Self::Is(x) => ::type_sitter_lib::Node::raw(x),
                Self::IsSpacenot(x) => ::type_sitter_lib::Node::raw(x),
                Self::NotSpacein(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtGt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::In(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Is(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::IsSpacenot(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::NotSpacein(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::LtEq(x) => x.into_raw(),
                Self::LtGt(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::GtEq(x) => x.into_raw(),
                Self::In(x) => x.into_raw(),
                Self::Is(x) => x.into_raw(),
                Self::IsSpacenot(x) => x.into_raw(),
                Self::NotSpacein(x) => x.into_raw(),
            }
        }
    }
    /**One of `{pattern | pattern_list}`:
- [`Pattern`]
- [`PatternList`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern_PatternList<'tree> {
        Pattern(Pattern<'tree>),
        PatternList(PatternList<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Pattern_PatternList<'tree> {
        ///Returns the node if it is of type `pattern` ([`Pattern`]), otherwise returns `None`
        #[inline]
        pub fn as_pattern(self) -> ::std::option::Option<Pattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `pattern_list` ([`PatternList`]), otherwise returns `None`
        #[inline]
        pub fn as_pattern_list(self) -> ::std::option::Option<PatternList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_pattern()?.as_attribute()
        }
        /**Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_pattern()?.as_identifier()
        }
        /**Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
            self.as_pattern()?.as_list_pattern()
        }
        /**Returns the node if it is of type `list_splat_pattern` ([`ListSplatPattern`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_list_splat_pattern(
            self,
        ) -> ::std::option::Option<ListSplatPattern<'tree>> {
            self.as_pattern()?.as_list_splat_pattern()
        }
        /**Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_pattern()?.as_subscript()
        }
        /**Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`.

Follows the following chain:
- `pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))*/
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            self.as_pattern()?.as_tuple_pattern()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Pattern_PatternList<'tree> {
        type WithLifetime<'a> = Pattern_PatternList<'a>;
        const KIND: &'static str = "{pattern | pattern_list}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <Pattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) = <PatternList<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::PatternList(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::PatternList(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::PatternList(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Pattern(x) => x.into_raw(),
                Self::PatternList(x) => x.into_raw(),
            }
        }
    }
    /**One of `{- | _ | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}`:
- [`symbols::Sub`]
- [`symbols::__`]
- [`ClassPattern`]
- [`ComplexPattern`]
- [`ConcatenatedString`]
- [`DictPattern`]
- [`DottedName`]
- [`False`]
- [`Float`]
- [`Integer`]
- [`ListPattern`]
- [`None`]
- [`SplatPattern`]
- [`String`]
- [`True`]
- [`TuplePattern`]
- [`UnionPattern`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Sub____ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        Sub(symbols::Sub<'tree>),
        __(symbols::__<'tree>),
        ClassPattern(ClassPattern<'tree>),
        ComplexPattern(ComplexPattern<'tree>),
        ConcatenatedString(ConcatenatedString<'tree>),
        DictPattern(DictPattern<'tree>),
        DottedName(DottedName<'tree>),
        False(False<'tree>),
        Float(Float<'tree>),
        Integer(Integer<'tree>),
        ListPattern(ListPattern<'tree>),
        None(None<'tree>),
        SplatPattern(SplatPattern<'tree>),
        String(String<'tree>),
        True(True<'tree>),
        TuplePattern(TuplePattern<'tree>),
        UnionPattern(UnionPattern<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > Sub____ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        ///Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `_` ([`symbols::__`]), otherwise returns `None`
        #[inline]
        pub fn as___(self) -> ::std::option::Option<symbols::__<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::__(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `class_pattern` ([`ClassPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_class_pattern(self) -> ::std::option::Option<ClassPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `complex_pattern` ([`ComplexPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_complex_pattern(self) -> ::std::option::Option<ComplexPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ComplexPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `concatenated_string` ([`ConcatenatedString`]), otherwise returns `None`
        #[inline]
        pub fn as_concatenated_string(
            self,
        ) -> ::std::option::Option<ConcatenatedString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConcatenatedString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dict_pattern` ([`DictPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_dict_pattern(self) -> ::std::option::Option<DictPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DictPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dotted_name` ([`DottedName`]), otherwise returns `None`
        #[inline]
        pub fn as_dotted_name(self) -> ::std::option::Option<DottedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DottedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `false` ([`False`]), otherwise returns `None`
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::False(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Float(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `list_pattern` ([`ListPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_list_pattern(self) -> ::std::option::Option<ListPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `none` ([`None`]), otherwise returns `None`
        #[inline]
        pub fn as_none(self) -> ::std::option::Option<None<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::None(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `splat_pattern` ([`SplatPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_splat_pattern(self) -> ::std::option::Option<SplatPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SplatPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `true` ([`True`]), otherwise returns `None`
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::True(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `tuple_pattern` ([`TuplePattern`]), otherwise returns `None`
        #[inline]
        pub fn as_tuple_pattern(self) -> ::std::option::Option<TuplePattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TuplePattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union_pattern` ([`UnionPattern`]), otherwise returns `None`
        #[inline]
        pub fn as_union_pattern(self) -> ::std::option::Option<UnionPattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionPattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree>
    for Sub____ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
        'tree,
    > {
        type WithLifetime<'a> = Sub____ClassPattern_ComplexPattern_ConcatenatedString_DictPattern_DottedName_False_Float_Integer_ListPattern_None_SplatPattern_String_True_TuplePattern_UnionPattern<
            'a,
        >;
        const KIND: &'static str = "{- | _ | class_pattern | complex_pattern | concatenated_string | dict_pattern | dotted_name | false | float | integer | list_pattern | none | splat_pattern | string | true | tuple_pattern | union_pattern}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if let Ok(this) = <symbols::Sub<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Sub(this));
            }
            if let Ok(this) = <symbols::__<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::__(this));
            }
            if let Ok(this) = <ClassPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ClassPattern(this));
            }
            if let Ok(this) = <ComplexPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ComplexPattern(this));
            }
            if let Ok(this) = <ConcatenatedString<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ConcatenatedString(this));
            }
            if let Ok(this) = <DictPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DictPattern(this));
            }
            if let Ok(this) = <DottedName<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::DottedName(this));
            }
            if let Ok(this) = <False<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::False(this));
            }
            if let Ok(this) = <Float<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Float(this));
            }
            if let Ok(this) = <Integer<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Integer(this));
            }
            if let Ok(this) = <ListPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::ListPattern(this));
            }
            if let Ok(this) = <None<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::None(this));
            }
            if let Ok(this) = <SplatPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::SplatPattern(this));
            }
            if let Ok(this) = <String<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::String(this));
            }
            if let Ok(this) = <True<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::True(this));
            }
            if let Ok(this) = <TuplePattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::TuplePattern(this));
            }
            if let Ok(this) = <UnionPattern<
                'tree,
            > as ::type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::UnionPattern(this));
            }
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::Sub(x) => ::type_sitter_lib::Node::raw(x),
                Self::__(x) => ::type_sitter_lib::Node::raw(x),
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw(x),
                Self::False(x) => ::type_sitter_lib::Node::raw(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::None(x) => ::type_sitter_lib::Node::raw(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::String(x) => ::type_sitter_lib::Node::raw(x),
                Self::True(x) => ::type_sitter_lib::Node::raw(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Sub(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::__(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ClassPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ComplexPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ConcatenatedString(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DictPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DottedName(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Float(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Integer(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ListPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::None(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SplatPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TuplePattern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::UnionPattern(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Sub(x) => x.into_raw(),
                Self::__(x) => x.into_raw(),
                Self::ClassPattern(x) => x.into_raw(),
                Self::ComplexPattern(x) => x.into_raw(),
                Self::ConcatenatedString(x) => x.into_raw(),
                Self::DictPattern(x) => x.into_raw(),
                Self::DottedName(x) => x.into_raw(),
                Self::False(x) => x.into_raw(),
                Self::Float(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
                Self::ListPattern(x) => x.into_raw(),
                Self::None(x) => x.into_raw(),
                Self::SplatPattern(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
                Self::True(x) => x.into_raw(),
                Self::TuplePattern(x) => x.into_raw(),
                Self::UnionPattern(x) => x.into_raw(),
            }
        }
    }
}
