#[doc = concat!("Typed node `", "_value", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub enum Value<'tree> {
    Array(Array<'tree>),
    False(False<'tree>),
    Null(Null<'tree>),
    Number(Number<'tree>),
    Object(Object<'tree>),
    String(String<'tree>),
    True(True<'tree>),
}
impl<'tree> TryFrom<TSNode<'tree>> for Value<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "array" => Ok(Self::Array(Array(node))),
            "false" => Ok(Self::False(False(node))),
            "null" => Ok(Self::Null(Null(node))),
            "number" => Ok(Self::Number(Number(node))),
            "object" => Ok(Self::Object(Object(node))),
            "string" => Ok(Self::String(String(node))),
            "true" => Ok(Self::True(True(node))),
            _ => {
                Err(tree_sitter_lib::IncorrectKind {
                    node,
                    kind: "_value",
                })
            }
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Value<'tree> {
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
#[automatically_derived]
pub struct Array<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Array<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "array",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Array<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Array<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Value<'tree>>> {
        self.0.children(&mut c).map(<Value<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Value<'tree>>> {
        self.0.child(i).map(<Value<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "document", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Document<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Document<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "document" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "document",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Document<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Document<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Value<'tree>>> {
        self.0.children(&mut c).map(<Value<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Value<'tree>>> {
        self.0.child(i).map(<Value<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "object", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Object<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Object<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "object" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "object",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Object<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Object<'tree> {
    ///Get the node's children
    pub fn children(
        &self,
        c: &mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, Pair<'tree>>> {
        self.0.children(&mut c).map(<Pair<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's child #i
    pub fn child(
        &self,
        i: usize,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, Pair<'tree>>> {
        self.0.child(i).map(<Pair<'tree> as TryFrom<_>>::try_from)
    }
}
#[doc = concat!("Typed node `", "pair", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Pair<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Pair<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pair" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "pair",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Pair<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Pair<'tree> {
    #[doc = concat!("Get the field `", "key", "`")]
    pub fn key(
        &self,
    ) -> tree_sitter_lib::NodeResult<
        'tree,
        tree_sitter_lib::either_n::Either2<Number<'tree>, String<'tree>>,
    > {
        self.0
            .child_by_field_name("key")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    pub fn value(&self) -> tree_sitter_lib::NodeResult<'tree, Value<'tree>> {
        self.0
            .child_by_field_name("value")
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[doc = concat!("Typed node `", "string", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct String<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for String<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "string",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for String<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> String<'tree> {
    ///Get the node's child
    pub fn child(
        &self,
    ) -> Option<tree_sitter_lib::NodeResult<'tree, StringContent<'tree>>> {
        self.0.child(0)
    }
}
#[doc = concat!("Typed node `", "string_content", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct StringContent<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for StringContent<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string_content" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "string_content",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for StringContent<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> StringContent<'tree> {
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
#[doc = concat!("Typed node `", "comment", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Comment<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Comment<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "comment",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Comment<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Comment<'tree> {}
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
#[doc = concat!("Typed node `", "false", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct False<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for False<'tree> {
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
impl<'tree> tree_sitter_lib::TypedNode<'tree> for False<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> False<'tree> {}
#[doc = concat!("Typed node `", "null", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Null<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Null<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "null" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "null",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Null<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Null<'tree> {}
#[doc = concat!("Typed node `", "number", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct Number<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for Number<'tree> {
    type Error = tree_sitter_lib::IncorrectKind<'tree>;
    fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "number" {
            Ok(Self(node))
        } else {
            Err(tree_sitter_lib::IncorrectKind {
                node,
                kind: "number",
            })
        }
    }
}
impl<'tree> tree_sitter_lib::TypedNode<'tree> for Number<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> Number<'tree> {}
#[doc = concat!("Typed node `", "true", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[automatically_derived]
pub struct True<'tree>(tree_sitter::Node<'tree>);
impl<'tree> TryFrom<TSNode<'tree>> for True<'tree> {
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
impl<'tree> tree_sitter_lib::TypedNode<'tree> for True<'tree> {
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
}
impl<'tree> True<'tree> {}
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
