#[doc = "Typed version of the query:\n\n```sexp\n(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
#[doc = "Matches returned by a query cursor running the query [`Highlights`]:\n\n```sexp\n(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'query, 'tree, Text, I> =
    ::type_sitter::QueryMatches<'query, 'tree, Highlights, Text, I>;
#[doc = "Captures returned by a query cursor running the query [`Highlights`]:\n\n```sexp\n(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'query, 'tree, Text, I> =
    ::type_sitter::QueryCaptures<'query, 'tree, Highlights, Text, I>;
#[doc = "A match returned by the query [`Highlights`]:\n\n```sexp\n(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n\n```"]
#[repr(transparent)]
pub struct HighlightsMatch<'query, 'tree: 'query>(::type_sitter::raw::QueryMatch<'query, 'tree>);
#[doc = "A capture returned by the query [`Highlights`]:\n\n```sexp\n(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n\n```"]
#[derive(Clone, Debug)]
pub enum HighlightsCapture<'tree> {
    #[doc = "A `string.special.key` ([`type_sitter_lib::UntypedNamedNode`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(_) @string.special.key"]
    #[doc = "```"]
    StringSpecialKey(::type_sitter::UntypedNamedNode<'tree>),
    #[doc = "A `string` ([`super::nodes::String`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(string) @string"]
    #[doc = "```"]
    String(super::nodes::String<'tree>),
    #[doc = "A `number` ([`super::nodes::Number`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(number) @number"]
    #[doc = "```"]
    Number(super::nodes::Number<'tree>),
    #[doc = "A `constant.builtin` ([`anon_unions::ConstantBuiltin`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "[\n  (null)\n  (true)\n  (false)\n] @constant.builtin"]
    #[doc = "```"]
    ConstantBuiltin(anon_unions::ConstantBuiltin<'tree>),
    #[doc = "A `escape` ([`super::nodes::EscapeSequence`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    Escape(super::nodes::EscapeSequence<'tree>),
    #[doc = "A `comment` ([`super::nodes::Comment`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(comment) @comment"]
    #[doc = "```"]
    Comment(super::nodes::Comment<'tree>),
}
#[automatically_derived]
impl ::type_sitter::Query for Highlights {
    type Match<'query, 'tree: 'query> = HighlightsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = HighlightsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n"
    }
    fn raw(&self) -> &'static ::type_sitter::raw::Query {
        #[allow(non_upper_case_globals)]
        static __Highlights__: std::sync::OnceLock<::type_sitter::raw::Query> =
            std::sync::OnceLock::new();
        __Highlights__ . get_or_init (|| { # [allow (unused_mut)] let mut query = :: type_sitter :: raw :: Query :: new (& tree_sitter_json :: LANGUAGE . into () , "(pair\n  key: (_) @string.special.key)\n\n(string) @string\n\n(number) @number\n\n[\n  (null)\n  (true)\n  (false)\n] @constant.builtin\n\n(escape_sequence) @escape\n\n(comment) @comment\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_json' correct, and did you use the same tree-sitter / tree_sitter_json version?") ; query })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: ::type_sitter::raw::QueryMatch<'query, 'tree>,
    ) -> HighlightsMatch<'query, 'tree> {
        HighlightsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_match_ref<'m, 'query, 'tree>(
        &self,
        r#match: &'m ::type_sitter::raw::QueryMatch<'query, 'tree>,
    ) -> &'m HighlightsMatch<'query, 'tree> {
        &*(r#match as *const ::type_sitter::raw::QueryMatch<'query, 'tree>
            as *const HighlightsMatch<'query, 'tree>)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: ::type_sitter::raw::QueryCapture<'tree>,
    ) -> HighlightsCapture<'tree> {
        match capture . index as usize { 0usize => HighlightsCapture :: StringSpecialKey (< :: type_sitter :: UntypedNamedNode < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 1usize => HighlightsCapture :: String (< super :: nodes :: String < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 2usize => HighlightsCapture :: Number (< super :: nodes :: Number < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 3usize => HighlightsCapture :: ConstantBuiltin (< anon_unions :: ConstantBuiltin < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 4usize => HighlightsCapture :: Escape (< super :: nodes :: EscapeSequence < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 5usize => HighlightsCapture :: Comment (< super :: nodes :: Comment < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'query, 'tree: 'query> HighlightsMatch<'query, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `string.special.key` ([`type_sitter_lib::UntypedNamedNode`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(_) @string.special.key"]
    #[doc = "```"]
    #[inline]
    pub fn string_special_key(
        &self,
    ) -> ::std::option::Option<::type_sitter::UntypedNamedNode<'tree>> {
        { [0u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < :: type_sitter :: UntypedNamedNode < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `string` ([`super::nodes::String`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(string) @string"]
    #[doc = "```"]
    #[inline]
    pub fn string(&self) -> ::std::option::Option<super::nodes::String<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::String<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        n,
                    )
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `number` ([`super::nodes::Number`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(number) @number"]
    #[doc = "```"]
    #[inline]
    pub fn number(&self) -> ::std::option::Option<super::nodes::Number<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Number<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        n,
                    )
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `constant.builtin` ([`anon_unions::ConstantBuiltin`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "[\n  (null)\n  (true)\n  (false)\n] @constant.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn constant_builtin(&self) -> ::std::option::Option<anon_unions::ConstantBuiltin<'tree>> {
        { [3u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: ConstantBuiltin < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `escape` ([`super::nodes::EscapeSequence`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    #[inline]
    pub fn escape(&self) -> ::std::option::Option<super::nodes::EscapeSequence<'tree>> {
        { [4u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: EscapeSequence < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `comment` ([`super::nodes::Comment`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(comment) @comment"]
    #[doc = "```"]
    #[inline]
    pub fn comment(&self) -> ::std::option::Option<super::nodes::Comment<'tree>> {
        { [5u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Comment < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for HighlightsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple(stringify!(HighlightsMatch))
            .field(&self.0)
            .finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter::QueryMatch<'query, 'tree>
    for HighlightsMatch<'query, 'tree>
{
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Highlights
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::QueryMatch<'query, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::QueryMatch<'query, 'tree> {
        self.0
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> HighlightsCapture<'tree> {
    #[doc = "Try to interpret this capture as a `string.special.key` ([`type_sitter_lib::UntypedNamedNode`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(_) @string.special.key"]
    #[doc = "```"]
    #[inline]
    pub fn as_string_special_key(
        &self,
    ) -> ::std::option::Option<&::type_sitter::UntypedNamedNode<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::StringSpecialKey(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `string` ([`super::nodes::String`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(string) @string"]
    #[doc = "```"]
    #[inline]
    pub fn as_string(&self) -> ::std::option::Option<&super::nodes::String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `number` ([`super::nodes::Number`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(number) @number"]
    #[doc = "```"]
    #[inline]
    pub fn as_number(&self) -> ::std::option::Option<&super::nodes::Number<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Number(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `constant.builtin` ([`anon_unions::ConstantBuiltin`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "[\n  (null)\n  (true)\n  (false)\n] @constant.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn as_constant_builtin(
        &self,
    ) -> ::std::option::Option<&anon_unions::ConstantBuiltin<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConstantBuiltin(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `escape` ([`super::nodes::EscapeSequence`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    #[inline]
    pub fn as_escape(&self) -> ::std::option::Option<&super::nodes::EscapeSequence<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Escape(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `comment` ([`super::nodes::Comment`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(comment) @comment"]
    #[doc = "```"]
    #[inline]
    pub fn as_comment(&self) -> ::std::option::Option<&super::nodes::Comment<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Comment(node) = self {
            Some(node)
        } else {
            None
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter::QueryCapture<'query, 'tree>
    for HighlightsCapture<'tree>
{
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Highlights
    }
    #[inline]
    fn raw(&self) -> ::type_sitter::raw::QueryCapture<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::StringSpecialKey(node) => ::type_sitter::raw::QueryCapture {
                index: 0usize as u32,
                node: *node.raw(),
            },
            Self::String(node) => ::type_sitter::raw::QueryCapture {
                index: 1usize as u32,
                node: *node.raw(),
            },
            Self::Number(node) => ::type_sitter::raw::QueryCapture {
                index: 2usize as u32,
                node: *node.raw(),
            },
            Self::ConstantBuiltin(node) => ::type_sitter::raw::QueryCapture {
                index: 3usize as u32,
                node: *node.raw(),
            },
            Self::Escape(node) => ::type_sitter::raw::QueryCapture {
                index: 4usize as u32,
                node: *node.raw(),
            },
            Self::Comment(node) => ::type_sitter::raw::QueryCapture {
                index: 5usize as u32,
                node: *node.raw(),
            },
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node(&self) -> &::type_sitter::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::StringSpecialKey(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::String(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Number(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::ConstantBuiltin(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Escape(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Comment(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut ::type_sitter::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::StringSpecialKey(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::String(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Number(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::ConstantBuiltin(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Escape(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Comment(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn name(&self) -> &'query str {
        match self {
            Self::StringSpecialKey { .. } => "string.special.key",
            Self::String { .. } => "string",
            Self::Number { .. } => "number",
            Self::ConstantBuiltin { .. } => "constant.builtin",
            Self::Escape { .. } => "escape",
            Self::Comment { .. } => "comment",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::StringSpecialKey { .. } => 0usize,
            Self::String { .. } => 1usize,
            Self::Number { .. } => 2usize,
            Self::ConstantBuiltin { .. } => 3usize,
            Self::Escape { .. } => 4usize,
            Self::Comment { .. } => 5usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::super::nodes::*;
    #[doc = "One of `{false | null | true}`:\n- [`False`]\n- [`Null`]\n- [`True`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstantBuiltin<'tree> {
        False(False<'tree>),
        Null(Null<'tree>),
        True(True<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ConstantBuiltin<'tree> {
        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::False(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Null(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`"]
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
    impl<'tree> ::type_sitter::Node<'tree> for ConstantBuiltin<'tree> {
        type WithLifetime<'a> = ConstantBuiltin<'a>;
        const KIND: &'static str = "{false | null | true}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <False<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::False(this));
            }
            if let Ok(this) = <Null<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Null(this));
            }
            if let Ok(this) = <True<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::True(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::False(x) => ::type_sitter::Node::raw(x),
                Self::Null(x) => ::type_sitter::Node::raw(x),
                Self::True(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::False(x) => ::type_sitter::Node::raw_mut(x),
                Self::Null(x) => ::type_sitter::Node::raw_mut(x),
                Self::True(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::False(x) => x.into_raw(),
                Self::Null(x) => x.into_raw(),
                Self::True(x) => x.into_raw(),
            }
        }
    }
}
