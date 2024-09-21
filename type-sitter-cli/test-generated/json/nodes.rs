#[doc = "Typed node `_value`\n\nThis node type has subtypes:\n\n- `array` ([`Array`])\n- `false` ([`False`])\n- `null` ([`Null`])\n- `number` ([`Number`])\n- `object` ([`Object`])\n- `string` ([`String`])\n- `true` ([`True`])\n"]
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
    #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_array(self) -> Option<Array<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Array(x) = self {
            Some(x)
        } else {
            None
        }
    }
    #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_false(self) -> Option<False<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::False(x) = self {
            Some(x)
        } else {
            None
        }
    }
    #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_null(self) -> Option<Null<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Null(x) = self {
            Some(x)
        } else {
            None
        }
    }
    #[doc = "Returns the node if it is of type `number` ([`Number`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_number(self) -> Option<Number<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Number(x) = self {
            Some(x)
        } else {
            None
        }
    }
    #[doc = "Returns the node if it is of type `object` ([`Object`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_object(self) -> Option<Object<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Object(x) = self {
            Some(x)
        } else {
            None
        }
    }
    #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_string(self) -> Option<String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(x) = self {
            Some(x)
        } else {
            None
        }
    }
    #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`"]
    #[inline]
    #[allow(non_snake_case)]
    pub fn as_true(self) -> Option<True<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::True(x) = self {
            Some(x)
        } else {
            None
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Value<'tree> {
    type WithLifetime<'a> = Value<'a>;
    const KIND: &'static str = "_value";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        match node.kind() {
            "array" => Ok(unsafe {
                Self::Array(<Array<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "false" => Ok(unsafe {
                Self::False(<False<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "null" => Ok(unsafe {
                Self::Null(<Null<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "number" => Ok(unsafe {
                Self::Number(<Number<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "object" => Ok(unsafe {
                Self::Object(<Object<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "string" => Ok(unsafe {
                Self::String(<String<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "true" => Ok(unsafe {
                Self::True(<True<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            _ => Err(type_sitter::IncorrectKind::new::<Self>(node)),
        }
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => type_sitter::Node::raw(x),
            Self::False(x) => type_sitter::Node::raw(x),
            Self::Null(x) => type_sitter::Node::raw(x),
            Self::Number(x) => type_sitter::Node::raw(x),
            Self::Object(x) => type_sitter::Node::raw(x),
            Self::String(x) => type_sitter::Node::raw(x),
            Self::True(x) => type_sitter::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => type_sitter::Node::raw_mut(x),
            Self::False(x) => type_sitter::Node::raw_mut(x),
            Self::Null(x) => type_sitter::Node::raw_mut(x),
            Self::Number(x) => type_sitter::Node::raw_mut(x),
            Self::Object(x) => type_sitter::Node::raw_mut(x),
            Self::String(x) => type_sitter::Node::raw_mut(x),
            Self::True(x) => type_sitter::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => x.into_raw(),
            Self::False(x) => x.into_raw(),
            Self::Null(x) => x.into_raw(),
            Self::Number(x) => x.into_raw(),
            Self::Object(x) => x.into_raw(),
            Self::String(x) => x.into_raw(),
            Self::True(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `array`\n\nThis node has named children of type `_value*` ([`Value`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Array<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Array<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `_value*` ([`Value`])"]
    #[allow(non_snake_case)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = type_sitter::NodeResult<'tree, Value<'tree>>> + 'a {
        self.0
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Value<'tree> as type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Array<'tree> {
    type WithLifetime<'a> = Array<'a>;
    const KIND: &'static str = "array";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "array" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `document`\n\nThis node has named children of type `_value*` ([`Value`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Document<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Document<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `_value*` ([`Value`])"]
    #[allow(non_snake_case)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = type_sitter::NodeResult<'tree, Value<'tree>>> + 'a {
        self.0
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Value<'tree> as type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Document<'tree> {
    type WithLifetime<'a> = Document<'a>;
    const KIND: &'static str = "document";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "document" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "document");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `object`\n\nThis node has named children of type `pair*` ([`Pair`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Object<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Object<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `pair*` ([`Pair`])"]
    #[allow(non_snake_case)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<Item = type_sitter::NodeResult<'tree, Pair<'tree>>> + 'a {
        self.0
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Pair<'tree> as type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Object<'tree> {
    type WithLifetime<'a> = Object<'a>;
    const KIND: &'static str = "object";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "object" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "object");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pair`\n\nThis node has these fields:\n\n- `key`: `string` ([`String`])\n- `value`: `_value` ([`Value`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Pair<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Pair<'tree> {
    #[doc = "Get the field `key`.\n\nThis child has type `string` ([`String`])"]
    #[allow(non_snake_case)]
    #[inline]
    pub fn key(&self) -> type_sitter::NodeResult<'tree, String<'tree>> {
        self.0
            .child_by_field_name("key")
            .map(<String<'tree> as type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    #[doc = "Get the field `value`.\n\nThis child has type `_value` ([`Value`])"]
    #[allow(non_snake_case)]
    #[inline]
    pub fn value(&self) -> type_sitter::NodeResult<'tree, Value<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Value<'tree> as type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Pair<'tree> {
    type WithLifetime<'a> = Pair<'a>;
    const KIND: &'static str = "pair";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "pair" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pair");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `string`\n\nThis node has named children of type `{escape_sequence | string_content}*`:\n\n- [`EscapeSequence`]\n- [`StringContent`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> String<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{escape_sequence | string_content}*`:\n\n- [`EscapeSequence`]\n- [`StringContent`]\n"]
    #[allow(non_snake_case)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut type_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter::NodeResult<'tree, anon_unions::EscapeSequence_StringContent<'tree>>,
    > + 'a {
        self . 0 . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: EscapeSequence_StringContent < 'tree > as type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for String<'tree> {
    type WithLifetime<'a> = String<'a>;
    const KIND: &'static str = "string";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `comment`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Comment<'tree> {
    type WithLifetime<'a> = Comment<'a>;
    const KIND: &'static str = "comment";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `escape_sequence`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for EscapeSequence<'tree> {
    type WithLifetime<'a> = EscapeSequence<'a>;
    const KIND: &'static str = "escape_sequence";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "escape_sequence");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `false`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct False<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> False<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for False<'tree> {
    type WithLifetime<'a> = False<'a>;
    const KIND: &'static str = "false";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "false" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "false");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `null`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Null<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Null<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Null<'tree> {
    type WithLifetime<'a> = Null<'a>;
    const KIND: &'static str = "null";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "null" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "null");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `number`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Number<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> Number<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for Number<'tree> {
    type WithLifetime<'a> = Number<'a>;
    const KIND: &'static str = "number";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "number" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "number");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `string_content`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StringContent<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> StringContent<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for StringContent<'tree> {
    type WithLifetime<'a> = StringContent<'a>;
    const KIND: &'static str = "string_content";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "string_content" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string_content");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `true`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct True<'tree>(type_sitter::raw::Node<'tree>);
#[automatically_derived]
impl<'tree> True<'tree> {}
#[automatically_derived]
impl<'tree> type_sitter::Node<'tree> for True<'tree> {
    type WithLifetime<'a> = True<'a>;
    const KIND: &'static str = "true";
    #[inline]
    fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
        if node.kind() == "true" {
            Ok(Self(node))
        } else {
            Err(type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "true");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> type_sitter::raw::Node<'tree> {
        self.0
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `\"`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DoubleQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for DoubleQuote<'tree> {
        type WithLifetime<'a> = DoubleQuote<'a>;
        const KIND: &'static str = "\"";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == "\"" {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\"");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `,`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for Comma<'tree> {
        type WithLifetime<'a> = Comma<'a>;
        const KIND: &'static str = ",";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ",");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `:`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for Colon<'tree> {
        type WithLifetime<'a> = Colon<'a>;
        const KIND: &'static str = ":";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `[`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for LBracket<'tree> {
        type WithLifetime<'a> = LBracket<'a>;
        const KIND: &'static str = "[";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "[");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `]`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for RBracket<'tree> {
        type WithLifetime<'a> = RBracket<'a>;
        const KIND: &'static str = "]";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "]");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `{`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for LBrace<'tree> {
        type WithLifetime<'a> = LBrace<'a>;
        const KIND: &'static str = "{";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `}`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for RBrace<'tree> {
        type WithLifetime<'a> = RBrace<'a>;
        const KIND: &'static str = "}";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "One of `{escape_sequence | string_content}`:\n- [`EscapeSequence`]\n- [`StringContent`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum EscapeSequence_StringContent<'tree> {
        EscapeSequence(EscapeSequence<'tree>),
        StringContent(StringContent<'tree>),
    }
    #[automatically_derived]
    impl<'tree> EscapeSequence_StringContent<'tree> {
        #[doc = "Returns the node if it is of type `escape_sequence` ([`EscapeSequence`]), otherwise returns `None`"]
        #[inline]
        #[allow(non_snake_case)]
        pub fn as_escape_sequence(self) -> Option<EscapeSequence<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EscapeSequence(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `string_content` ([`StringContent`]), otherwise returns `None`"]
        #[inline]
        #[allow(non_snake_case)]
        pub fn as_string_content(self) -> Option<StringContent<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringContent(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter::Node<'tree> for EscapeSequence_StringContent<'tree> {
        type WithLifetime<'a> = EscapeSequence_StringContent<'a>;
        const KIND: &'static str = "{escape_sequence | string_content}";
        #[inline]
        fn try_from_raw(node: type_sitter::raw::Node<'tree>) -> type_sitter::NodeResult<Self> {
            match node.kind() {
                "escape_sequence" => Ok(unsafe {
                    Self::EscapeSequence(
                        <EscapeSequence<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "string_content" => Ok(unsafe {
                    Self::StringContent(
                        <StringContent<'tree> as type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                _ => Err(type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &type_sitter::raw::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => type_sitter::Node::raw(x),
                Self::StringContent(x) => type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut type_sitter::raw::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => type_sitter::Node::raw_mut(x),
                Self::StringContent(x) => type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> type_sitter::raw::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => x.into_raw(),
                Self::StringContent(x) => x.into_raw(),
            }
        }
    }
}
