/**Typed node `_value`

This node type is a supertype of:
- `array` ([Array])
- `false` ([False])
- `null` ([Null])
- `number` ([Number])
- `object` ([Object])
- `string` ([String])
- `true` ([True])
*/
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
    ///Returns the node if it is of kind `array` ([Array]), otherwise returns None
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array(self) -> Option<Array<'tree>> {
        match self {
            Self::Array(x) => Some(x),
            _ => None,
        }
    }
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
    ///Returns the node if it is of kind `number` ([Number]), otherwise returns None
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn number(self) -> Option<Number<'tree>> {
        match self {
            Self::Number(x) => Some(x),
            _ => None,
        }
    }
    ///Returns the node if it is of kind `object` ([Object]), otherwise returns None
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn object(self) -> Option<Object<'tree>> {
        match self {
            Self::Object(x) => Some(x),
            _ => None,
        }
    }
    ///Returns the node if it is of kind `string` ([String]), otherwise returns None
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string(self) -> Option<String<'tree>> {
        match self {
            Self::String(x) => Some(x),
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Value<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
        match node.kind() {
            "array" => {
                Ok(unsafe {
                    Self::Array(
                        <Array<
                            'tree,
                        > as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
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
            "number" => {
                Ok(unsafe {
                    Self::Number(
                        <Number<
                            'tree,
                        > as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "object" => {
                Ok(unsafe {
                    Self::Object(
                        <Object<
                            'tree,
                        > as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "string" => {
                Ok(unsafe {
                    Self::String(
                        <String<
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
impl<'tree> type_sitter_lib::TypedNode<'tree> for Value<'tree> {
    const KIND: &'static str = "_value";
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
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
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        match self {
            Self::Array(x) => x.node_mut(),
            Self::False(x) => x.node_mut(),
            Self::Null(x) => x.node_mut(),
            Self::Number(x) => x.node_mut(),
            Self::Object(x) => x.node_mut(),
            Self::String(x) => x.node_mut(),
            Self::True(x) => x.node_mut(),
        }
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        match self {
            Self::Array(x) => x.into_node(),
            Self::False(x) => x.into_node(),
            Self::Null(x) => x.into_node(),
            Self::Number(x) => x.into_node(),
            Self::Object(x) => x.into_node(),
            Self::String(x) => x.into_node(),
            Self::True(x) => x.into_node(),
        }
    }
}
/**Typed node `array`

This node has children: `_value*` ([Value])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Array<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Array<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter_lib::tree_sitter_wrapper::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Value<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<
                'tree,
                Value<'tree>,
            > as TryFrom<_>>::try_from(n))
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Array<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `document`

This node has children: `_value*` ([Value])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Document<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Document<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter_lib::tree_sitter_wrapper::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Value<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<
                'tree,
                Value<'tree>,
            > as TryFrom<_>>::try_from(n))
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for Document<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `object`

This node has children: `pair*` ([Pair])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Object<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Object<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter_lib::tree_sitter_wrapper::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pair<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<
                'tree,
                Pair<'tree>,
            > as TryFrom<_>>::try_from(n))
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for Object<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `pair`

This node has these fields:
- `key`: `{number | string}` ([anon_unions::Number_String])
- `value`: `_value` ([Value])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Pair<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Pair<'tree> {
    ///Get the field `key` which has kind `{number | string}` ([anon_unions::Number_String])
    #[allow(dead_code)]
    #[inline]
    pub fn key(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Number_String<'tree>> {
        self.0
            .child_by_field_name("key")
            .map(<anon_unions::Number_String<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the field `value` which has kind `_value` ([Value])
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Pair<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `string`

This node has an (optional) child: `string_content?` ([StringContent])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for String<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `string_content`

This node has children: `escape_sequence*` ([EscapeSequence])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StringContent<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> StringContent<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter_lib::tree_sitter_wrapper::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<
                'tree,
                EscapeSequence<'tree>,
            > as TryFrom<_>>::try_from(n))
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
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for StringContent<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `comment`

This node has no children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for Comment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `escape_sequence`

This node has no children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for EscapeSequence<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `false`

This node has no children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct False<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> False<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for False<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `null`

This node has no children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Null<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Null<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Null<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `number`

This node has no children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Number<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> Number<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
for Number<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
/**Typed node `true`

This node has no children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct True<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
#[automatically_derived]
impl<'tree> True<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for True<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Result<Self, Self::Error> {
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
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(
        node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
    ) -> Self {
        Self(node)
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `"`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DoubleQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for DoubleQuote<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
    /**Typed node `,`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for Comma<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
    /**Typed node `:`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for Colon<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
    /**Typed node `[`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for LBracket<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
    /**Typed node `]`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for RBracket<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
    /**Typed node `{`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for LBrace<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
    /**Typed node `}`

This node has no children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(type_sitter_lib::tree_sitter_wrapper::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for RBrace<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
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
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Self {
            Self(node)
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    /**one of `{number | string}`:
- [Number]
- [String]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Number_String<'tree> {
        Number(Number<'tree>),
        String(String<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Number_String<'tree> {
        ///Returns the node if it is of kind `number` ([Number]), otherwise returns None
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn number(self) -> Option<Number<'tree>> {
            match self {
                Self::Number(x) => Some(x),
                _ => None,
            }
        }
        ///Returns the node if it is of kind `string` ([String]), otherwise returns None
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn string(self) -> Option<String<'tree>> {
            match self {
                Self::String(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
    for Number_String<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "number" => {
                    Ok(unsafe {
                        Self::Number(
                            <Number<
                                'tree,
                            > as type_sitter_lib::TypedNode<
                                'tree,
                            >>::from_node_unchecked(node),
                        )
                    })
                }
                "string" => {
                    Ok(unsafe {
                        Self::String(
                            <String<
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
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Number_String<'tree> {
        const KIND: &'static str = "{number | string}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Number(x) => x.node(),
                Self::String(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(
            &mut self,
        ) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Number(x) => x.node_mut(),
                Self::String(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Number(x) => x.into_node(),
                Self::String(x) => x.into_node(),
            }
        }
    }
}
