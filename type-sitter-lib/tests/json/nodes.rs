/**Typed node `_value`

This node type has subtypes:

- `array` ([`Array`])
- `false` ([`False`])
- `null` ([`Null`])
- `number` ([`Number`])
- `object` ([`Object`])
- `string` ([`String`])
- `true` ([`True`])
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
#[allow(unused)]
impl<'tree> Value<'tree> {
    ///Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`
    #[inline]
    pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Array(x) = self {
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
    ///Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`
    #[inline]
    pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Null(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `number` ([`Number`]), otherwise returns `None`
    #[inline]
    pub fn as_number(self) -> ::std::option::Option<Number<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Number(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
    ///Returns the node if it is of type `object` ([`Object`]), otherwise returns `None`
    #[inline]
    pub fn as_object(self) -> ::std::option::Option<Object<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Object(x) = self {
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
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Value<'tree> {
    type WithLifetime<'a> = Value<'a>;
    const KIND: &'static str = "_value";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        match node.kind() {
            "array" => {
                Ok(unsafe {
                    Self::Array(
                        <Array<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            "false" => {
                Ok(unsafe {
                    Self::False(
                        <False<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            "null" => {
                Ok(unsafe {
                    Self::Null(
                        <Null<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            "number" => {
                Ok(unsafe {
                    Self::Number(
                        <Number<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            "object" => {
                Ok(unsafe {
                    Self::Object(
                        <Object<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            "string" => {
                Ok(unsafe {
                    Self::String(
                        <String<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            "true" => {
                Ok(unsafe {
                    Self::True(
                        <True<
                            'tree,
                        > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                    )
                })
            }
            _ => Err(::type_sitter_lib::IncorrectKind::new::<Self>(node)),
        }
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::Node<'tree> {
        match self {
            Self::Array(x) => ::type_sitter_lib::Node::raw(x),
            Self::False(x) => ::type_sitter_lib::Node::raw(x),
            Self::Null(x) => ::type_sitter_lib::Node::raw(x),
            Self::Number(x) => ::type_sitter_lib::Node::raw(x),
            Self::Object(x) => ::type_sitter_lib::Node::raw(x),
            Self::String(x) => ::type_sitter_lib::Node::raw(x),
            Self::True(x) => ::type_sitter_lib::Node::raw(x),
        }
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
        match self {
            Self::Array(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Null(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Number(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::Object(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::String(x) => ::type_sitter_lib::Node::raw_mut(x),
            Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
        }
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::Node<'tree> {
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
/**Typed node `array`

This node has named children of type `_value*` ([`Value`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Array<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Array<'tree> {
    /**Get the node's not-extra named children.

These children have type `_value*` ([`Value`])*/
    #[inline]
    pub fn values<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Value<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Value<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Array<'tree> {
    type Child = Value<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Array<'tree> {
    type WithLifetime<'a> = Array<'a>;
    const KIND: &'static str = "array";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "array" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array");
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
/**Typed node `document`

This node has named children of type `_value*` ([`Value`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Document<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Document<'tree> {
    /**Get the node's not-extra named children.

These children have type `_value*` ([`Value`])*/
    #[inline]
    pub fn values<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Value<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Value<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Document<'tree> {
    type Child = Value<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Document<'tree> {
    type WithLifetime<'a> = Document<'a>;
    const KIND: &'static str = "document";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "document" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "document");
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
/**Typed node `null`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Null<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Null<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Null<'tree> {
    type WithLifetime<'a> = Null<'a>;
    const KIND: &'static str = "null";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "null" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "null");
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
/**Typed node `number`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Number<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Number<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Number<'tree> {
    type WithLifetime<'a> = Number<'a>;
    const KIND: &'static str = "number";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "number" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "number");
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
/**Typed node `object`

This node has named children of type `pair*` ([`Pair`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Object<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Object<'tree> {
    /**Get the node's not-extra named children.

These children have type `pair*` ([`Pair`])*/
    #[inline]
    pub fn pairs<'a>(
        &self,
        c: &'a mut ::type_sitter_lib::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter_lib::NodeResult<'tree, Pair<'tree>>,
    > + 'a {
        ::type_sitter_lib::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Pair<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::HasChildren<'tree> for Object<'tree> {
    type Child = Pair<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter_lib::Node<'tree> for Object<'tree> {
    type WithLifetime<'a> = Object<'a>;
    const KIND: &'static str = "object";
    #[inline]
    fn try_from_raw(
        node: ::yak_sitter::Node<'tree>,
    ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
        if node.kind() == "object" {
            Ok(Self(node))
        } else {
            Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "object");
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

- `key`: `string` ([`String`])
- `value`: `_value` ([`Value`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Pair<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Pair<'tree> {
    /**Get the field `key`.

This child has type `string` ([`String`])*/
    #[inline]
    pub fn key(&self) -> ::type_sitter_lib::NodeResult<'tree, String<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("key")
            .map(<String<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `_value` ([`Value`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter_lib::NodeResult<'tree, Value<'tree>> {
        ::type_sitter_lib::Node::raw(self)
            .child_by_field_name("value")
            .map(<Value<'tree> as ::type_sitter_lib::Node<'tree>>::try_from_raw)
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
/**Typed node `string`

This node has named children of type `{escape_sequence | string_content}*`:

- [`EscapeSequence`]
- [`StringContent`]

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
    type Child = anon_unions::EscapeSequence_StringContent<'tree>;
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

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StringContent<'tree>(::yak_sitter::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StringContent<'tree> {}
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
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `"`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(::yak_sitter::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DoubleQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DoubleQuote<'tree> {
        type WithLifetime<'a> = DoubleQuote<'a>;
        const KIND: &'static str = "\"";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            if node.kind() == "\"" {
                Ok(Self(node))
            } else {
                Err(::type_sitter_lib::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::yak_sitter::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\"");
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
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    /**One of `{escape_sequence | string_content}`:
- [`EscapeSequence`]
- [`StringContent`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum EscapeSequence_StringContent<'tree> {
        EscapeSequence(EscapeSequence<'tree>),
        StringContent(StringContent<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EscapeSequence_StringContent<'tree> {
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
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for EscapeSequence_StringContent<'tree> {
        type WithLifetime<'a> = EscapeSequence_StringContent<'a>;
        const KIND: &'static str = "{escape_sequence | string_content}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "escape_sequence" => {
                    Ok(unsafe {
                        Self::EscapeSequence(
                            <EscapeSequence<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "string_content" => {
                    Ok(unsafe {
                        Self::StringContent(
                            <StringContent<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::yak_sitter::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => ::type_sitter_lib::Node::raw(x),
                Self::StringContent(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::StringContent(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => x.into_raw(),
                Self::StringContent(x) => x.into_raw(),
            }
        }
    }
}
