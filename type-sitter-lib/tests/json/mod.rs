#[doc = concat!("Typed node `", "_value", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Value<'tree> {
    Array(Array<'tree>),
    False(False<'tree>),
    Null(Null<'tree>),
    Number(Number<'tree>),
    Object(Object<'tree>),
    String(String<'tree>),
    True(True<'tree>),
}
#[automatically_derived]
impl<'tree> Value<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "array", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array(self) -> Option<Array<'tree>> {
        match self {
            Self::Array(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "false", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn r#false(self) -> Option<False<'tree>> {
        match self {
            Self::False(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "null", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn null(self) -> Option<Null<'tree>> {
        match self {
            Self::Null(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "number", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn number(self) -> Option<Number<'tree>> {
        match self {
            Self::Number(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "object", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn object(self) -> Option<Object<'tree>> {
        match self {
            Self::Object(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "string", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string(self) -> Option<String<'tree>> {
        match self {
            Self::String(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "true", "`, otherwise returns None"
    )]
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
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Value<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "array" => {
                Ok(unsafe {
                    Self::Array(
                        <Array as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "false" => {
                Ok(unsafe {
                    Self::False(
                        <False as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "null" => {
                Ok(unsafe {
                    Self::Null(
                        <Null as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "number" => {
                Ok(unsafe {
                    Self::Number(
                        <Number as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "object" => {
                Ok(unsafe {
                    Self::Object(
                        <Object as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "string" => {
                Ok(unsafe {
                    Self::String(
                        <String as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "true" => {
                Ok(unsafe {
                    Self::True(
                        <True as type_sitter_lib::TypedNode<
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
impl<'tree> type_sitter_lib::TypedNode<'tree> for Value<'tree> {
    const KIND: &'static str = "_value";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::Array(x) => x.node(),
            Self::False(x) => x.node(),
            Self::Null(x) => x.node(),
            Self::Number(x) => x.node(),
            Self::Object(x) => x.node(),
            Self::String(x) => x.node(),
            Self::True(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "array", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Array<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Array<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Value<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(<type_sitter_lib::ExtraOr<'tree, Value<'tree>> as TryFrom<_>>::try_from)
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Value<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Value<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Array<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Array<'tree> {
    const KIND: &'static str = "array";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "document", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Document<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Document<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Value<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(<type_sitter_lib::ExtraOr<'tree, Value<'tree>> as TryFrom<_>>::try_from)
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Value<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Value<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Document<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "document" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Document<'tree> {
    const KIND: &'static str = "document";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "object", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Object<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Object<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pair<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(<type_sitter_lib::ExtraOr<'tree, Pair<'tree>> as TryFrom<_>>::try_from)
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Pair<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Pair<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Object<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "object" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Object<'tree> {
    const KIND: &'static str = "object";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "pair", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Pair<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Pair<'tree> {
    #[doc = concat!("Get the field `", "key", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn key(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Number<'tree>, String<'tree>>,
    > {
        self.0
            .child_by_field_name("key")
            .map(
                <type_sitter_lib::Either2<
                    Number<'tree>,
                    String<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Value<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Value<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pair<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pair" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Pair<'tree> {
    const KIND: &'static str = "pair";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "string", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> String<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, StringContent<'tree>>> {
        self.0.named_child(0).map(<StringContent<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for String<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for String<'tree> {
    const KIND: &'static str = "string";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "string_content", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StringContent<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StringContent<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    EscapeSequence<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    EscapeSequence<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StringContent<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string_content" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StringContent<'tree> {
    const KIND: &'static str = "string_content";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "\"", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ",", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ":", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "[", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "]", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "comment", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Comment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Comment<'tree> {
    const KIND: &'static str = "comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "escape_sequence", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "false", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "null", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Null<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Null<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Null<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "null" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Null<'tree> {
    const KIND: &'static str = "null";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "number", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Number<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Number<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Number<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "number" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Number<'tree> {
    const KIND: &'static str = "number";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "true", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "{", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "}", "`")]
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
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
