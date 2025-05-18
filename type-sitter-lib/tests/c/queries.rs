/**Typed version of the query:

```sexp
(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

"break" @keyword
"case" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"for" @keyword
"if" @keyword
"inline" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"volatile" @keyword
"while" @keyword

"#define" @keyword
"#elif" @keyword
"#else" @keyword
"#endif" @keyword
"#if" @keyword
"#ifdef" @keyword
"#ifndef" @keyword
"#include" @keyword
(preproc_directive) @keyword

"--" @operator
"-" @operator
"-=" @operator
"->" @operator
"=" @operator
"!=" @operator
"*" @operator
"&" @operator
"&&" @operator
"+" @operator
"++" @operator
"+=" @operator
"<" @operator
"==" @operator
">" @operator
"||" @operator

"." @delimiter
";" @delimiter

(string_literal) @string
(system_lib_string) @string

(null) @constant
(number_literal) @number
(char_literal) @number

(field_identifier) @property
(statement_identifier) @label
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(comment) @comment

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
/**Matches returned by a query cursor running the query [`Highlights`]:

```sexp
(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

"break" @keyword
"case" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"for" @keyword
"if" @keyword
"inline" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"volatile" @keyword
"while" @keyword

"#define" @keyword
"#elif" @keyword
"#else" @keyword
"#endif" @keyword
"#if" @keyword
"#ifdef" @keyword
"#ifndef" @keyword
"#include" @keyword
(preproc_directive) @keyword

"--" @operator
"-" @operator
"-=" @operator
"->" @operator
"=" @operator
"!=" @operator
"*" @operator
"&" @operator
"&&" @operator
"+" @operator
"++" @operator
"+=" @operator
"<" @operator
"==" @operator
">" @operator
"||" @operator

"." @delimiter
";" @delimiter

(string_literal) @string
(system_lib_string) @string

(null) @constant
(number_literal) @number
(char_literal) @number

(field_identifier) @property
(statement_identifier) @label
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(comment) @comment

```*/
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'query, 'tree> = ::type_sitter_lib::QueryMatches<
    'query,
    'tree,
    Highlights,
>;
/**Captures returned by a query cursor running the query [`Highlights`]:

```sexp
(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

"break" @keyword
"case" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"for" @keyword
"if" @keyword
"inline" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"volatile" @keyword
"while" @keyword

"#define" @keyword
"#elif" @keyword
"#else" @keyword
"#endif" @keyword
"#if" @keyword
"#ifdef" @keyword
"#ifndef" @keyword
"#include" @keyword
(preproc_directive) @keyword

"--" @operator
"-" @operator
"-=" @operator
"->" @operator
"=" @operator
"!=" @operator
"*" @operator
"&" @operator
"&&" @operator
"+" @operator
"++" @operator
"+=" @operator
"<" @operator
"==" @operator
">" @operator
"||" @operator

"." @delimiter
";" @delimiter

(string_literal) @string
(system_lib_string) @string

(null) @constant
(number_literal) @number
(char_literal) @number

(field_identifier) @property
(statement_identifier) @label
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(comment) @comment

```*/
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'query, 'tree> = ::type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Highlights,
>;
/**A match returned by the query [`Highlights`]:

```sexp
(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

"break" @keyword
"case" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"for" @keyword
"if" @keyword
"inline" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"volatile" @keyword
"while" @keyword

"#define" @keyword
"#elif" @keyword
"#else" @keyword
"#endif" @keyword
"#if" @keyword
"#ifdef" @keyword
"#ifndef" @keyword
"#include" @keyword
(preproc_directive) @keyword

"--" @operator
"-" @operator
"-=" @operator
"->" @operator
"=" @operator
"!=" @operator
"*" @operator
"&" @operator
"&&" @operator
"+" @operator
"++" @operator
"+=" @operator
"<" @operator
"==" @operator
">" @operator
"||" @operator

"." @delimiter
";" @delimiter

(string_literal) @string
(system_lib_string) @string

(null) @constant
(number_literal) @number
(char_literal) @number

(field_identifier) @property
(statement_identifier) @label
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(comment) @comment

```*/
#[repr(transparent)]
pub struct HighlightsMatch<'query, 'tree: 'query>(
    ::yak_sitter::QueryMatch<'query, 'tree>,
);
/**A capture returned by the query [`Highlights`]:

```sexp
(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

"break" @keyword
"case" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"for" @keyword
"if" @keyword
"inline" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"volatile" @keyword
"while" @keyword

"#define" @keyword
"#elif" @keyword
"#else" @keyword
"#endif" @keyword
"#if" @keyword
"#ifdef" @keyword
"#ifndef" @keyword
"#include" @keyword
(preproc_directive) @keyword

"--" @operator
"-" @operator
"-=" @operator
"->" @operator
"=" @operator
"!=" @operator
"*" @operator
"&" @operator
"&&" @operator
"+" @operator
"++" @operator
"+=" @operator
"<" @operator
"==" @operator
">" @operator
"||" @operator

"." @delimiter
";" @delimiter

(string_literal) @string
(system_lib_string) @string

(null) @constant
(number_literal) @number
(char_literal) @number

(field_identifier) @property
(statement_identifier) @label
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(comment) @comment

```*/
#[derive(Clone, Debug)]
pub enum HighlightsCapture<'tree> {
    ///A `variable` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @variable
    ///```
    Variable(super::nodes::Identifier<'tree>),
    ///A `constant` ([`anon_unions::Constant`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///(null) @constant
    ///```
    Constant(anon_unions::Constant<'tree>),
    ///A `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"break" @keyword
    ///"case" @keyword
    ///"const" @keyword
    ///"continue" @keyword
    ///"default" @keyword
    ///"do" @keyword
    ///"else" @keyword
    ///"enum" @keyword
    ///"extern" @keyword
    ///"for" @keyword
    ///"if" @keyword
    ///"inline" @keyword
    ///"return" @keyword
    ///"sizeof" @keyword
    ///"static" @keyword
    ///"struct" @keyword
    ///"switch" @keyword
    ///"typedef" @keyword
    ///"union" @keyword
    ///"volatile" @keyword
    ///"while" @keyword
    ///"#define" @keyword
    ///"#elif" @keyword
    ///"#else" @keyword
    ///"#endif" @keyword
    ///"#if" @keyword
    ///"#ifdef" @keyword
    ///"#ifndef" @keyword
    ///"#include" @keyword
    ///(preproc_directive) @keyword
    ///```
    Keyword(anon_unions::Keyword<'tree>),
    ///A `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"--" @operator
    ///"-" @operator
    ///"-=" @operator
    ///"->" @operator
    ///"=" @operator
    ///"!=" @operator
    ///"*" @operator
    ///"&" @operator
    ///"&&" @operator
    ///"+" @operator
    ///"++" @operator
    ///"+=" @operator
    ///"<" @operator
    ///"==" @operator
    ///">" @operator
    ///"||" @operator
    ///```
    Operator(anon_unions::Operator<'tree>),
    ///A `delimiter` ([`anon_unions::Delimiter`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"." @delimiter
    ///";" @delimiter
    ///```
    Delimiter(anon_unions::Delimiter<'tree>),
    ///A `string` ([`anon_unions::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string_literal) @string
    ///(system_lib_string) @string
    ///```
    String(anon_unions::String<'tree>),
    ///A `number` ([`anon_unions::Number`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(number_literal) @number
    ///(char_literal) @number
    ///```
    Number(anon_unions::Number<'tree>),
    ///A `property` ([`super::nodes::FieldIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @property
    ///```
    Property(super::nodes::FieldIdentifier<'tree>),
    ///A `label` ([`super::nodes::StatementIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(statement_identifier) @label
    ///```
    Label(super::nodes::StatementIdentifier<'tree>),
    ///A `type` ([`anon_unions::Type`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @type
    ///(primitive_type) @type
    ///(sized_type_specifier) @type
    ///```
    Type(anon_unions::Type<'tree>),
    ///A `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function
    ///(field_identifier) @function
    ///(identifier) @function
    ///```
    Function(anon_unions::Function<'tree>),
    ///A `function.special` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.special
    ///```
    FunctionSpecial(super::nodes::Identifier<'tree>),
    ///A `comment` ([`super::nodes::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    Comment(super::nodes::Comment<'tree>),
}
#[automatically_derived]
impl ::type_sitter_lib::Query for Highlights {
    type Match<'query, 'tree: 'query> = HighlightsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = HighlightsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "(identifier) @variable\n\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]*$\"))\n\n\"break\" @keyword\n\"case\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"do\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"inline\" @keyword\n\"return\" @keyword\n\"sizeof\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"switch\" @keyword\n\"typedef\" @keyword\n\"union\" @keyword\n\"volatile\" @keyword\n\"while\" @keyword\n\n\"#define\" @keyword\n\"#elif\" @keyword\n\"#else\" @keyword\n\"#endif\" @keyword\n\"#if\" @keyword\n\"#ifdef\" @keyword\n\"#ifndef\" @keyword\n\"#include\" @keyword\n(preproc_directive) @keyword\n\n\"--\" @operator\n\"-\" @operator\n\"-=\" @operator\n\"->\" @operator\n\"=\" @operator\n\"!=\" @operator\n\"*\" @operator\n\"&\" @operator\n\"&&\" @operator\n\"+\" @operator\n\"++\" @operator\n\"+=\" @operator\n\"<\" @operator\n\"==\" @operator\n\">\" @operator\n\"||\" @operator\n\n\".\" @delimiter\n\";\" @delimiter\n\n(string_literal) @string\n(system_lib_string) @string\n\n(null) @constant\n(number_literal) @number\n(char_literal) @number\n\n(field_identifier) @property\n(statement_identifier) @label\n(type_identifier) @type\n(primitive_type) @type\n(sized_type_specifier) @type\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function))\n(function_declarator\n  declarator: (identifier) @function)\n(preproc_function_def\n  name: (identifier) @function.special)\n\n(comment) @comment\n"
    }
    fn raw(&self) -> &'static ::yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Highlights__: std::sync::OnceLock<::yak_sitter::Query> = std::sync::OnceLock::new();
        __Highlights__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = ::yak_sitter::Query::new(
                        &tree_sitter_c::LANGUAGE.into(),
                        "(identifier) @variable\n\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]*$\"))\n\n\"break\" @keyword\n\"case\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"do\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"inline\" @keyword\n\"return\" @keyword\n\"sizeof\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"switch\" @keyword\n\"typedef\" @keyword\n\"union\" @keyword\n\"volatile\" @keyword\n\"while\" @keyword\n\n\"#define\" @keyword\n\"#elif\" @keyword\n\"#else\" @keyword\n\"#endif\" @keyword\n\"#if\" @keyword\n\"#ifdef\" @keyword\n\"#ifndef\" @keyword\n\"#include\" @keyword\n(preproc_directive) @keyword\n\n\"--\" @operator\n\"-\" @operator\n\"-=\" @operator\n\"->\" @operator\n\"=\" @operator\n\"!=\" @operator\n\"*\" @operator\n\"&\" @operator\n\"&&\" @operator\n\"+\" @operator\n\"++\" @operator\n\"+=\" @operator\n\"<\" @operator\n\"==\" @operator\n\">\" @operator\n\"||\" @operator\n\n\".\" @delimiter\n\";\" @delimiter\n\n(string_literal) @string\n(system_lib_string) @string\n\n(null) @constant\n(number_literal) @number\n(char_literal) @number\n\n(field_identifier) @property\n(statement_identifier) @label\n(type_identifier) @type\n(primitive_type) @type\n(sized_type_specifier) @type\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function))\n(function_declarator\n  declarator: (identifier) @function)\n(preproc_function_def\n  name: (identifier) @function.special)\n\n(comment) @comment\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_c' correct, and did you use the same tree-sitter / tree_sitter_c version?",
                    );
                query
            })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: ::yak_sitter::QueryMatch<'query, 'tree>,
    ) -> HighlightsMatch<'query, 'tree> {
        HighlightsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_match_ref<'m, 'query, 'tree>(
        &self,
        r#match: &'m ::yak_sitter::QueryMatch<'query, 'tree>,
    ) -> &'m HighlightsMatch<'query, 'tree> {
        &*(r#match as *const ::yak_sitter::QueryMatch<'query, 'tree>
            as *const HighlightsMatch<'query, 'tree>)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: ::yak_sitter::QueryCapture<'query, 'tree>,
    ) -> HighlightsCapture<'tree> {
        match capture.index as usize {
            0usize => {
                HighlightsCapture::Variable(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            1usize => {
                HighlightsCapture::Constant(
                    <anon_unions::Constant<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            2usize => {
                HighlightsCapture::Keyword(
                    <anon_unions::Keyword<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            3usize => {
                HighlightsCapture::Operator(
                    <anon_unions::Operator<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            4usize => {
                HighlightsCapture::Delimiter(
                    <anon_unions::Delimiter<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            5usize => {
                HighlightsCapture::String(
                    <anon_unions::String<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            6usize => {
                HighlightsCapture::Number(
                    <anon_unions::Number<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            7usize => {
                HighlightsCapture::Property(
                    <super::nodes::FieldIdentifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            8usize => {
                HighlightsCapture::Label(
                    <super::nodes::StatementIdentifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            9usize => {
                HighlightsCapture::Type(
                    <anon_unions::Type<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            10usize => {
                HighlightsCapture::Function(
                    <anon_unions::Function<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            11usize => {
                HighlightsCapture::FunctionSpecial(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            12usize => {
                HighlightsCapture::Comment(
                    <super::nodes::Comment<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'query, 'tree: 'query> HighlightsMatch<'query, 'tree> {
    ///Returns an iterator over the nodes captured by `variable` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @variable
    ///```
    #[inline]
    pub fn variable(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `constant` ([`anon_unions::Constant`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///(null) @constant
    ///```
    #[inline]
    pub fn constant(&self) -> ::std::option::Option<anon_unions::Constant<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Constant<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"break" @keyword
    ///"case" @keyword
    ///"const" @keyword
    ///"continue" @keyword
    ///"default" @keyword
    ///"do" @keyword
    ///"else" @keyword
    ///"enum" @keyword
    ///"extern" @keyword
    ///"for" @keyword
    ///"if" @keyword
    ///"inline" @keyword
    ///"return" @keyword
    ///"sizeof" @keyword
    ///"static" @keyword
    ///"struct" @keyword
    ///"switch" @keyword
    ///"typedef" @keyword
    ///"union" @keyword
    ///"volatile" @keyword
    ///"while" @keyword
    ///"#define" @keyword
    ///"#elif" @keyword
    ///"#else" @keyword
    ///"#endif" @keyword
    ///"#if" @keyword
    ///"#ifdef" @keyword
    ///"#ifndef" @keyword
    ///"#include" @keyword
    ///(preproc_directive) @keyword
    ///```
    #[inline]
    pub fn keyword(&self) -> ::std::option::Option<anon_unions::Keyword<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Keyword<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"--" @operator
    ///"-" @operator
    ///"-=" @operator
    ///"->" @operator
    ///"=" @operator
    ///"!=" @operator
    ///"*" @operator
    ///"&" @operator
    ///"&&" @operator
    ///"+" @operator
    ///"++" @operator
    ///"+=" @operator
    ///"<" @operator
    ///"==" @operator
    ///">" @operator
    ///"||" @operator
    ///```
    #[inline]
    pub fn operator(&self) -> ::std::option::Option<anon_unions::Operator<'tree>> {
        {
            [3u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Operator<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `delimiter` ([`anon_unions::Delimiter`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"." @delimiter
    ///";" @delimiter
    ///```
    #[inline]
    pub fn delimiter(&self) -> ::std::option::Option<anon_unions::Delimiter<'tree>> {
        {
            [4u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Delimiter<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `string` ([`anon_unions::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string_literal) @string
    ///(system_lib_string) @string
    ///```
    #[inline]
    pub fn string(&self) -> ::std::option::Option<anon_unions::String<'tree>> {
        {
            [5u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::String<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `number` ([`anon_unions::Number`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(number_literal) @number
    ///(char_literal) @number
    ///```
    #[inline]
    pub fn number(&self) -> ::std::option::Option<anon_unions::Number<'tree>> {
        {
            [6u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Number<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `property` ([`super::nodes::FieldIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @property
    ///```
    #[inline]
    pub fn property(
        &self,
    ) -> ::std::option::Option<super::nodes::FieldIdentifier<'tree>> {
        {
            [7u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::FieldIdentifier<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `label` ([`super::nodes::StatementIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(statement_identifier) @label
    ///```
    #[inline]
    pub fn label(
        &self,
    ) -> ::std::option::Option<super::nodes::StatementIdentifier<'tree>> {
        {
            [8u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::StatementIdentifier<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `type` ([`anon_unions::Type`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @type
    ///(primitive_type) @type
    ///(sized_type_specifier) @type
    ///```
    #[inline]
    pub fn r#type(&self) -> ::std::option::Option<anon_unions::Type<'tree>> {
        {
            [9u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Type<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function
    ///(field_identifier) @function
    ///(identifier) @function
    ///```
    #[inline]
    pub fn function(&self) -> ::std::option::Option<anon_unions::Function<'tree>> {
        {
            [10u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Function<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function.special` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.special
    ///```
    #[inline]
    pub fn function_special(
        &self,
    ) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [11u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `comment` ([`super::nodes::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    #[inline]
    pub fn comment(&self) -> ::std::option::Option<super::nodes::Comment<'tree>> {
        {
            [12u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Comment<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for HighlightsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple(stringify!(HighlightsMatch)).field(&self.0).finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter_lib::QueryMatch<'query, 'tree>
for HighlightsMatch<'query, 'tree> {
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Highlights
    }
    #[inline]
    fn tree(&self) -> &'tree yak_sitter::Tree {
        self.0.tree()
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::QueryMatch<'query, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::QueryMatch<'query, 'tree> {
        self.0
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> HighlightsCapture<'tree> {
    ///Try to interpret this capture as a `variable` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @variable
    ///```
    #[inline]
    pub fn as_variable(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Variable(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `constant` ([`anon_unions::Constant`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///(null) @constant
    ///```
    #[inline]
    pub fn as_constant(&self) -> ::std::option::Option<&anon_unions::Constant<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constant(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"break" @keyword
    ///"case" @keyword
    ///"const" @keyword
    ///"continue" @keyword
    ///"default" @keyword
    ///"do" @keyword
    ///"else" @keyword
    ///"enum" @keyword
    ///"extern" @keyword
    ///"for" @keyword
    ///"if" @keyword
    ///"inline" @keyword
    ///"return" @keyword
    ///"sizeof" @keyword
    ///"static" @keyword
    ///"struct" @keyword
    ///"switch" @keyword
    ///"typedef" @keyword
    ///"union" @keyword
    ///"volatile" @keyword
    ///"while" @keyword
    ///"#define" @keyword
    ///"#elif" @keyword
    ///"#else" @keyword
    ///"#endif" @keyword
    ///"#if" @keyword
    ///"#ifdef" @keyword
    ///"#ifndef" @keyword
    ///"#include" @keyword
    ///(preproc_directive) @keyword
    ///```
    #[inline]
    pub fn as_keyword(&self) -> ::std::option::Option<&anon_unions::Keyword<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Keyword(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"--" @operator
    ///"-" @operator
    ///"-=" @operator
    ///"->" @operator
    ///"=" @operator
    ///"!=" @operator
    ///"*" @operator
    ///"&" @operator
    ///"&&" @operator
    ///"+" @operator
    ///"++" @operator
    ///"+=" @operator
    ///"<" @operator
    ///"==" @operator
    ///">" @operator
    ///"||" @operator
    ///```
    #[inline]
    pub fn as_operator(&self) -> ::std::option::Option<&anon_unions::Operator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Operator(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `delimiter` ([`anon_unions::Delimiter`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"." @delimiter
    ///";" @delimiter
    ///```
    #[inline]
    pub fn as_delimiter(&self) -> ::std::option::Option<&anon_unions::Delimiter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Delimiter(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `string` ([`anon_unions::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string_literal) @string
    ///(system_lib_string) @string
    ///```
    #[inline]
    pub fn as_string(&self) -> ::std::option::Option<&anon_unions::String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `number` ([`anon_unions::Number`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(number_literal) @number
    ///(char_literal) @number
    ///```
    #[inline]
    pub fn as_number(&self) -> ::std::option::Option<&anon_unions::Number<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Number(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `property` ([`super::nodes::FieldIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @property
    ///```
    #[inline]
    pub fn as_property(
        &self,
    ) -> ::std::option::Option<&super::nodes::FieldIdentifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Property(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `label` ([`super::nodes::StatementIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(statement_identifier) @label
    ///```
    #[inline]
    pub fn as_label(
        &self,
    ) -> ::std::option::Option<&super::nodes::StatementIdentifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Label(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `type` ([`anon_unions::Type`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @type
    ///(primitive_type) @type
    ///(sized_type_specifier) @type
    ///```
    #[inline]
    pub fn as_type(&self) -> ::std::option::Option<&anon_unions::Type<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Type(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function
    ///(field_identifier) @function
    ///(identifier) @function
    ///```
    #[inline]
    pub fn as_function(&self) -> ::std::option::Option<&anon_unions::Function<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Function(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function.special` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.special
    ///```
    #[inline]
    pub fn as_function_special(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionSpecial(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `comment` ([`super::nodes::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    #[inline]
    pub fn as_comment(&self) -> ::std::option::Option<&super::nodes::Comment<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Comment(node) = self { Some(node) } else { None }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter_lib::QueryCapture<'query, 'tree>
for HighlightsCapture<'tree> {
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Highlights
    }
    #[inline]
    fn raw(&self) -> ::yak_sitter::QueryCapture<'query, 'tree> {
        #[allow(unused_imports)]
        use ::type_sitter_lib::Node;
        match self {
            Self::Variable(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 0usize,
                    name: "variable",
                }
            }
            Self::Constant(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 1usize,
                    name: "constant",
                }
            }
            Self::Keyword(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 2usize,
                    name: "keyword",
                }
            }
            Self::Operator(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 3usize,
                    name: "operator",
                }
            }
            Self::Delimiter(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 4usize,
                    name: "delimiter",
                }
            }
            Self::String(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 5usize,
                    name: "string",
                }
            }
            Self::Number(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 6usize,
                    name: "number",
                }
            }
            Self::Property(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 7usize,
                    name: "property",
                }
            }
            Self::Label(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 8usize,
                    name: "label",
                }
            }
            Self::Type(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 9usize,
                    name: "type",
                }
            }
            Self::Function(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 10usize,
                    name: "function",
                }
            }
            Self::FunctionSpecial(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 11usize,
                    name: "function.special",
                }
            }
            Self::Comment(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 12usize,
                    name: "comment",
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node(&self) -> &::type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter_lib::Node;
        match self {
            Self::Variable(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Constant(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Keyword(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Operator(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Delimiter(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::String(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Number(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Property(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Label(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Type(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Function(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::FunctionSpecial(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Comment(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut ::type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter_lib::Node;
        match self {
            Self::Variable(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Constant(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Keyword(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Operator(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Delimiter(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::String(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Number(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Property(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Label(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Type(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Function(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::FunctionSpecial(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Comment(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn name(&self) -> &'query str {
        match self {
            Self::Variable { .. } => "variable",
            Self::Constant { .. } => "constant",
            Self::Keyword { .. } => "keyword",
            Self::Operator { .. } => "operator",
            Self::Delimiter { .. } => "delimiter",
            Self::String { .. } => "string",
            Self::Number { .. } => "number",
            Self::Property { .. } => "property",
            Self::Label { .. } => "label",
            Self::Type { .. } => "type",
            Self::Function { .. } => "function",
            Self::FunctionSpecial { .. } => "function.special",
            Self::Comment { .. } => "comment",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Variable { .. } => 0usize,
            Self::Constant { .. } => 1usize,
            Self::Keyword { .. } => 2usize,
            Self::Operator { .. } => 3usize,
            Self::Delimiter { .. } => 4usize,
            Self::String { .. } => 5usize,
            Self::Number { .. } => 6usize,
            Self::Property { .. } => 7usize,
            Self::Label { .. } => 8usize,
            Self::Type { .. } => 9usize,
            Self::Function { .. } => 10usize,
            Self::FunctionSpecial { .. } => 11usize,
            Self::Comment { .. } => 12usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
/**Typed version of the query:

```sexp
(struct_specifier name: (type_identifier) @name body:(_)) @definition.class

(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class

(function_declarator declarator: (identifier) @name) @definition.function

(type_definition declarator: (type_identifier) @name) @definition.type

(enum_specifier name: (type_identifier) @name) @definition.type

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
/**Matches returned by a query cursor running the query [`Tags`]:

```sexp
(struct_specifier name: (type_identifier) @name body:(_)) @definition.class

(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class

(function_declarator declarator: (identifier) @name) @definition.function

(type_definition declarator: (type_identifier) @name) @definition.type

(enum_specifier name: (type_identifier) @name) @definition.type

```*/
#[allow(unused, non_camel_case_types)]
pub type TagsMatches<'query, 'tree> = ::type_sitter_lib::QueryMatches<
    'query,
    'tree,
    Tags,
>;
/**Captures returned by a query cursor running the query [`Tags`]:

```sexp
(struct_specifier name: (type_identifier) @name body:(_)) @definition.class

(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class

(function_declarator declarator: (identifier) @name) @definition.function

(type_definition declarator: (type_identifier) @name) @definition.type

(enum_specifier name: (type_identifier) @name) @definition.type

```*/
#[allow(unused, non_camel_case_types)]
pub type TagsCaptures<'query, 'tree> = ::type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Tags,
>;
/**A match returned by the query [`Tags`]:

```sexp
(struct_specifier name: (type_identifier) @name body:(_)) @definition.class

(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class

(function_declarator declarator: (identifier) @name) @definition.function

(type_definition declarator: (type_identifier) @name) @definition.type

(enum_specifier name: (type_identifier) @name) @definition.type

```*/
#[repr(transparent)]
pub struct TagsMatch<'query, 'tree: 'query>(::yak_sitter::QueryMatch<'query, 'tree>);
/**A capture returned by the query [`Tags`]:

```sexp
(struct_specifier name: (type_identifier) @name body:(_)) @definition.class

(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class

(function_declarator declarator: (identifier) @name) @definition.function

(type_definition declarator: (type_identifier) @name) @definition.type

(enum_specifier name: (type_identifier) @name) @definition.type

```*/
#[derive(Clone, Debug)]
pub enum TagsCapture<'tree> {
    ///A `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///```
    Name(anon_unions::Name<'tree>),
    ///A `definition.class` ([`anon_unions::DefinitionClass`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(struct_specifier name: (type_identifier) @name body:(_)) @definition.class
    ///(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class
    ///```
    DefinitionClass(anon_unions::DefinitionClass<'tree>),
    ///A `definition.function` ([`super::nodes::FunctionDeclarator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(function_declarator declarator: (identifier) @name) @definition.function
    ///```
    DefinitionFunction(super::nodes::FunctionDeclarator<'tree>),
    ///A `definition.type` ([`anon_unions::DefinitionType`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_definition declarator: (type_identifier) @name) @definition.type
    ///(enum_specifier name: (type_identifier) @name) @definition.type
    ///```
    DefinitionType(anon_unions::DefinitionType<'tree>),
}
#[automatically_derived]
impl ::type_sitter_lib::Query for Tags {
    type Match<'query, 'tree: 'query> = TagsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = TagsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "(struct_specifier name: (type_identifier) @name body:(_)) @definition.class\n\n(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class\n\n(function_declarator declarator: (identifier) @name) @definition.function\n\n(type_definition declarator: (type_identifier) @name) @definition.type\n\n(enum_specifier name: (type_identifier) @name) @definition.type\n"
    }
    fn raw(&self) -> &'static ::yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Tags__: std::sync::OnceLock<::yak_sitter::Query> = std::sync::OnceLock::new();
        __Tags__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = ::yak_sitter::Query::new(
                        &tree_sitter_c::LANGUAGE.into(),
                        "(struct_specifier name: (type_identifier) @name body:(_)) @definition.class\n\n(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class\n\n(function_declarator declarator: (identifier) @name) @definition.function\n\n(type_definition declarator: (type_identifier) @name) @definition.type\n\n(enum_specifier name: (type_identifier) @name) @definition.type\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_c' correct, and did you use the same tree-sitter / tree_sitter_c version?",
                    );
                query
            })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: ::yak_sitter::QueryMatch<'query, 'tree>,
    ) -> TagsMatch<'query, 'tree> {
        TagsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_match_ref<'m, 'query, 'tree>(
        &self,
        r#match: &'m ::yak_sitter::QueryMatch<'query, 'tree>,
    ) -> &'m TagsMatch<'query, 'tree> {
        &*(r#match as *const ::yak_sitter::QueryMatch<'query, 'tree>
            as *const TagsMatch<'query, 'tree>)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: ::yak_sitter::QueryCapture<'query, 'tree>,
    ) -> TagsCapture<'tree> {
        match capture.index as usize {
            0usize => {
                TagsCapture::Name(
                    <anon_unions::Name<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            1usize => {
                TagsCapture::DefinitionClass(
                    <anon_unions::DefinitionClass<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            2usize => {
                TagsCapture::DefinitionFunction(
                    <super::nodes::FunctionDeclarator<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            3usize => {
                TagsCapture::DefinitionType(
                    <anon_unions::DefinitionType<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'query, 'tree: 'query> TagsMatch<'query, 'tree> {
    ///Returns an iterator over the nodes captured by `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///```
    #[inline]
    pub fn name(&self) -> anon_unions::Name<'tree> {
        let mut iterator = {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Name<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        };
        let result = iterator.next().expect("one quantifier returned nothing");
        ::std::debug_assert!(
            iterator.next().is_none(), "one quantifier returned more than one item"
        );
        result
    }
    ///Returns an iterator over the nodes captured by `definition.class` ([`anon_unions::DefinitionClass`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(struct_specifier name: (type_identifier) @name body:(_)) @definition.class
    ///(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class
    ///```
    #[inline]
    pub fn definition_class(
        &self,
    ) -> ::std::option::Option<anon_unions::DefinitionClass<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::DefinitionClass<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.function` ([`super::nodes::FunctionDeclarator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(function_declarator declarator: (identifier) @name) @definition.function
    ///```
    #[inline]
    pub fn definition_function(
        &self,
    ) -> ::std::option::Option<super::nodes::FunctionDeclarator<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::FunctionDeclarator<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.type` ([`anon_unions::DefinitionType`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_definition declarator: (type_identifier) @name) @definition.type
    ///(enum_specifier name: (type_identifier) @name) @definition.type
    ///```
    #[inline]
    pub fn definition_type(
        &self,
    ) -> ::std::option::Option<anon_unions::DefinitionType<'tree>> {
        {
            [3u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::DefinitionType<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for TagsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple(stringify!(TagsMatch)).field(&self.0).finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter_lib::QueryMatch<'query, 'tree>
for TagsMatch<'query, 'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Tags
    }
    #[inline]
    fn tree(&self) -> &'tree yak_sitter::Tree {
        self.0.tree()
    }
    #[inline]
    fn raw(&self) -> &::yak_sitter::QueryMatch<'query, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> ::yak_sitter::QueryMatch<'query, 'tree> {
        self.0
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> TagsCapture<'tree> {
    ///Try to interpret this capture as a `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///```
    #[inline]
    pub fn as_name(&self) -> ::std::option::Option<&anon_unions::Name<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Name(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.class` ([`anon_unions::DefinitionClass`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(struct_specifier name: (type_identifier) @name body:(_)) @definition.class
    ///(declaration type: (union_specifier name: (type_identifier) @name)) @definition.class
    ///```
    #[inline]
    pub fn as_definition_class(
        &self,
    ) -> ::std::option::Option<&anon_unions::DefinitionClass<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionClass(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.function` ([`super::nodes::FunctionDeclarator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(function_declarator declarator: (identifier) @name) @definition.function
    ///```
    #[inline]
    pub fn as_definition_function(
        &self,
    ) -> ::std::option::Option<&super::nodes::FunctionDeclarator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionFunction(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.type` ([`anon_unions::DefinitionType`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_definition declarator: (type_identifier) @name) @definition.type
    ///(enum_specifier name: (type_identifier) @name) @definition.type
    ///```
    #[inline]
    pub fn as_definition_type(
        &self,
    ) -> ::std::option::Option<&anon_unions::DefinitionType<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionType(node) = self { Some(node) } else { None }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter_lib::QueryCapture<'query, 'tree>
for TagsCapture<'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Tags
    }
    #[inline]
    fn raw(&self) -> ::yak_sitter::QueryCapture<'query, 'tree> {
        #[allow(unused_imports)]
        use ::type_sitter_lib::Node;
        match self {
            Self::Name(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 0usize,
                    name: "name",
                }
            }
            Self::DefinitionClass(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 1usize,
                    name: "definition.class",
                }
            }
            Self::DefinitionFunction(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 2usize,
                    name: "definition.function",
                }
            }
            Self::DefinitionType(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 3usize,
                    name: "definition.type",
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node(&self) -> &::type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter_lib::Node;
        match self {
            Self::Name(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::DefinitionClass(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionFunction(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionType(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut ::type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter_lib::Node;
        match self {
            Self::Name(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionClass(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionFunction(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionType(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn name(&self) -> &'query str {
        match self {
            Self::Name { .. } => "name",
            Self::DefinitionClass { .. } => "definition.class",
            Self::DefinitionFunction { .. } => "definition.function",
            Self::DefinitionType { .. } => "definition.type",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Name { .. } => 0usize,
            Self::DefinitionClass { .. } => 1usize,
            Self::DefinitionFunction { .. } => 2usize,
            Self::DefinitionType { .. } => 3usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::super::nodes::*;
    /**One of `{identifier | null}`:
- [`Identifier`]
- [`Null`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Constant<'tree> {
        Identifier(Identifier<'tree>),
        Null(Null<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Constant<'tree> {
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
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Constant<'tree> {
        type WithLifetime<'a> = Constant<'a>;
        const KIND: &'static str = "{identifier | null}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "null" => {
                    Ok(unsafe {
                        Self::Null(
                            <Null<
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
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::Null(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Null(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::Null(x) => x.into_raw(),
            }
        }
    }
    /**One of `{declaration | struct_specifier}`:
- [`Declaration`]
- [`StructSpecifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefinitionClass<'tree> {
        Declaration(Declaration<'tree>),
        StructSpecifier(StructSpecifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DefinitionClass<'tree> {
        ///Returns the node if it is of type `declaration` ([`Declaration`]), otherwise returns `None`
        #[inline]
        pub fn as_declaration(self) -> ::std::option::Option<Declaration<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Declaration(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `struct_specifier` ([`StructSpecifier`]), otherwise returns `None`
        #[inline]
        pub fn as_struct_specifier(
            self,
        ) -> ::std::option::Option<StructSpecifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StructSpecifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DefinitionClass<'tree> {
        type WithLifetime<'a> = DefinitionClass<'a>;
        const KIND: &'static str = "{declaration | struct_specifier}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "declaration" => {
                    Ok(unsafe {
                        Self::Declaration(
                            <Declaration<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "struct_specifier" => {
                    Ok(unsafe {
                        Self::StructSpecifier(
                            <StructSpecifier<
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
                Self::Declaration(x) => ::type_sitter_lib::Node::raw(x),
                Self::StructSpecifier(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Declaration(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::StructSpecifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Declaration(x) => x.into_raw(),
                Self::StructSpecifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{enum_specifier | type_definition}`:
- [`EnumSpecifier`]
- [`TypeDefinition`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefinitionType<'tree> {
        EnumSpecifier(EnumSpecifier<'tree>),
        TypeDefinition(TypeDefinition<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DefinitionType<'tree> {
        ///Returns the node if it is of type `enum_specifier` ([`EnumSpecifier`]), otherwise returns `None`
        #[inline]
        pub fn as_enum_specifier(self) -> ::std::option::Option<EnumSpecifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EnumSpecifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `type_definition` ([`TypeDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_type_definition(self) -> ::std::option::Option<TypeDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for DefinitionType<'tree> {
        type WithLifetime<'a> = DefinitionType<'a>;
        const KIND: &'static str = "{enum_specifier | type_definition}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "enum_specifier" => {
                    Ok(unsafe {
                        Self::EnumSpecifier(
                            <EnumSpecifier<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_definition" => {
                    Ok(unsafe {
                        Self::TypeDefinition(
                            <TypeDefinition<
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
                Self::EnumSpecifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::TypeDefinition(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::EnumSpecifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TypeDefinition(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::EnumSpecifier(x) => x.into_raw(),
                Self::TypeDefinition(x) => x.into_raw(),
            }
        }
    }
    /**One of `{. | ;}`:
- [`symbols::Dot`]
- [`symbols::Semicolon`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Delimiter<'tree> {
        Dot(symbols::Dot<'tree>),
        Semicolon(symbols::Semicolon<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Delimiter<'tree> {
        ///Returns the node if it is of type `.` ([`symbols::Dot`]), otherwise returns `None`
        #[inline]
        pub fn as_dot(self) -> ::std::option::Option<symbols::Dot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Dot(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `;` ([`symbols::Semicolon`]), otherwise returns `None`
        #[inline]
        pub fn as_semicolon(self) -> ::std::option::Option<symbols::Semicolon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Semicolon(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Delimiter<'tree> {
        type WithLifetime<'a> = Delimiter<'a>;
        const KIND: &'static str = "{. | ;}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "." => {
                    Ok(unsafe {
                        Self::Dot(
                            <symbols::Dot<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                ";" => {
                    Ok(unsafe {
                        Self::Semicolon(
                            <symbols::Semicolon<
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
                Self::Dot(x) => ::type_sitter_lib::Node::raw(x),
                Self::Semicolon(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Dot(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Semicolon(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Dot(x) => x.into_raw(),
                Self::Semicolon(x) => x.into_raw(),
            }
        }
    }
    /**One of `{field_identifier | identifier}`:
- [`FieldIdentifier`]
- [`Identifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Function<'tree> {
        FieldIdentifier(FieldIdentifier<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Function<'tree> {
        ///Returns the node if it is of type `field_identifier` ([`FieldIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_field_identifier(
            self,
        ) -> ::std::option::Option<FieldIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FieldIdentifier(x) = self {
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
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Function<'tree> {
        type WithLifetime<'a> = Function<'a>;
        const KIND: &'static str = "{field_identifier | identifier}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "field_identifier" => {
                    Ok(unsafe {
                        Self::FieldIdentifier(
                            <FieldIdentifier<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
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
                Self::FieldIdentifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{#define | #elif | #else | #endif | #if | #ifdef | #ifndef | #include | break | case | const | continue | default | do | else | enum | extern | for | if | inline | preproc_directive | return | sizeof | static | struct | switch | typedef | union | volatile | while}`:
- [`symbols::Hashdefine`]
- [`symbols::Hashelif`]
- [`symbols::Hashelse`]
- [`symbols::Hashendif`]
- [`symbols::Hashif`]
- [`symbols::Hashifdef`]
- [`symbols::Hashifndef`]
- [`symbols::Hashinclude`]
- [`unnamed::Break`]
- [`unnamed::Case`]
- [`unnamed::Const`]
- [`unnamed::Continue`]
- [`unnamed::Default`]
- [`unnamed::Do`]
- [`unnamed::Else`]
- [`unnamed::Enum`]
- [`unnamed::Extern`]
- [`unnamed::For`]
- [`unnamed::If`]
- [`unnamed::Inline__`]
- [`PreprocDirective`]
- [`unnamed::Return`]
- [`unnamed::Sizeof`]
- [`unnamed::Static`]
- [`unnamed::Struct`]
- [`unnamed::Switch`]
- [`unnamed::Typedef`]
- [`unnamed::Union`]
- [`unnamed::Volatile_`]
- [`unnamed::While`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Keyword<'tree> {
        Hashdefine(symbols::Hashdefine<'tree>),
        Hashelif(symbols::Hashelif<'tree>),
        Hashelse(symbols::Hashelse<'tree>),
        Hashendif(symbols::Hashendif<'tree>),
        Hashif(symbols::Hashif<'tree>),
        Hashifdef(symbols::Hashifdef<'tree>),
        Hashifndef(symbols::Hashifndef<'tree>),
        Hashinclude(symbols::Hashinclude<'tree>),
        Break(unnamed::Break<'tree>),
        Case(unnamed::Case<'tree>),
        Const(unnamed::Const<'tree>),
        Continue(unnamed::Continue<'tree>),
        Default(unnamed::Default<'tree>),
        Do(unnamed::Do<'tree>),
        Else(unnamed::Else<'tree>),
        Enum(unnamed::Enum<'tree>),
        Extern(unnamed::Extern<'tree>),
        For(unnamed::For<'tree>),
        If(unnamed::If<'tree>),
        Inline__(unnamed::Inline__<'tree>),
        PreprocDirective(PreprocDirective<'tree>),
        Return(unnamed::Return<'tree>),
        Sizeof(unnamed::Sizeof<'tree>),
        Static(unnamed::Static<'tree>),
        Struct(unnamed::Struct<'tree>),
        Switch(unnamed::Switch<'tree>),
        Typedef(unnamed::Typedef<'tree>),
        Union(unnamed::Union<'tree>),
        Volatile_(unnamed::Volatile_<'tree>),
        While(unnamed::While<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Keyword<'tree> {
        ///Returns the node if it is of type `#define` ([`symbols::Hashdefine`]), otherwise returns `None`
        #[inline]
        pub fn as_hashdefine(self) -> ::std::option::Option<symbols::Hashdefine<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashdefine(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#elif` ([`symbols::Hashelif`]), otherwise returns `None`
        #[inline]
        pub fn as_hashelif(self) -> ::std::option::Option<symbols::Hashelif<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashelif(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#else` ([`symbols::Hashelse`]), otherwise returns `None`
        #[inline]
        pub fn as_hashelse(self) -> ::std::option::Option<symbols::Hashelse<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashelse(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#endif` ([`symbols::Hashendif`]), otherwise returns `None`
        #[inline]
        pub fn as_hashendif(self) -> ::std::option::Option<symbols::Hashendif<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashendif(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#if` ([`symbols::Hashif`]), otherwise returns `None`
        #[inline]
        pub fn as_hashif(self) -> ::std::option::Option<symbols::Hashif<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashif(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#ifdef` ([`symbols::Hashifdef`]), otherwise returns `None`
        #[inline]
        pub fn as_hashifdef(self) -> ::std::option::Option<symbols::Hashifdef<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashifdef(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#ifndef` ([`symbols::Hashifndef`]), otherwise returns `None`
        #[inline]
        pub fn as_hashifndef(self) -> ::std::option::Option<symbols::Hashifndef<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashifndef(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `#include` ([`symbols::Hashinclude`]), otherwise returns `None`
        #[inline]
        pub fn as_hashinclude(
            self,
        ) -> ::std::option::Option<symbols::Hashinclude<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Hashinclude(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `break` ([`unnamed::Break`]), otherwise returns `None`
        #[inline]
        pub fn as_break(self) -> ::std::option::Option<unnamed::Break<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Break(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `case` ([`unnamed::Case`]), otherwise returns `None`
        #[inline]
        pub fn as_case(self) -> ::std::option::Option<unnamed::Case<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Case(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `const` ([`unnamed::Const`]), otherwise returns `None`
        #[inline]
        pub fn as_const(self) -> ::std::option::Option<unnamed::Const<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Const(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `continue` ([`unnamed::Continue`]), otherwise returns `None`
        #[inline]
        pub fn as_continue(self) -> ::std::option::Option<unnamed::Continue<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Continue(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `default` ([`unnamed::Default`]), otherwise returns `None`
        #[inline]
        pub fn as_default(self) -> ::std::option::Option<unnamed::Default<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Default(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `do` ([`unnamed::Do`]), otherwise returns `None`
        #[inline]
        pub fn as_do(self) -> ::std::option::Option<unnamed::Do<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Do(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `else` ([`unnamed::Else`]), otherwise returns `None`
        #[inline]
        pub fn as_else(self) -> ::std::option::Option<unnamed::Else<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Else(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `enum` ([`unnamed::Enum`]), otherwise returns `None`
        #[inline]
        pub fn as_enum(self) -> ::std::option::Option<unnamed::Enum<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Enum(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `extern` ([`unnamed::Extern`]), otherwise returns `None`
        #[inline]
        pub fn as_extern(self) -> ::std::option::Option<unnamed::Extern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Extern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `for` ([`unnamed::For`]), otherwise returns `None`
        #[inline]
        pub fn as_for(self) -> ::std::option::Option<unnamed::For<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::For(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `if` ([`unnamed::If`]), otherwise returns `None`
        #[inline]
        pub fn as_if(self) -> ::std::option::Option<unnamed::If<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::If(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `inline` ([`unnamed::Inline__`]), otherwise returns `None`
        #[inline]
        pub fn as_inline__(self) -> ::std::option::Option<unnamed::Inline__<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Inline__(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `preproc_directive` ([`PreprocDirective`]), otherwise returns `None`
        #[inline]
        pub fn as_preproc_directive(
            self,
        ) -> ::std::option::Option<PreprocDirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PreprocDirective(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `return` ([`unnamed::Return`]), otherwise returns `None`
        #[inline]
        pub fn as_return(self) -> ::std::option::Option<unnamed::Return<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Return(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `sizeof` ([`unnamed::Sizeof`]), otherwise returns `None`
        #[inline]
        pub fn as_sizeof(self) -> ::std::option::Option<unnamed::Sizeof<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sizeof(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `static` ([`unnamed::Static`]), otherwise returns `None`
        #[inline]
        pub fn as_static(self) -> ::std::option::Option<unnamed::Static<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Static(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `struct` ([`unnamed::Struct`]), otherwise returns `None`
        #[inline]
        pub fn as_struct(self) -> ::std::option::Option<unnamed::Struct<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Struct(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`unnamed::Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<unnamed::Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `typedef` ([`unnamed::Typedef`]), otherwise returns `None`
        #[inline]
        pub fn as_typedef(self) -> ::std::option::Option<unnamed::Typedef<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Typedef(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union` ([`unnamed::Union`]), otherwise returns `None`
        #[inline]
        pub fn as_union(self) -> ::std::option::Option<unnamed::Union<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Union(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `volatile` ([`unnamed::Volatile_`]), otherwise returns `None`
        #[inline]
        pub fn as_volatile_(self) -> ::std::option::Option<unnamed::Volatile_<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Volatile_(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `while` ([`unnamed::While`]), otherwise returns `None`
        #[inline]
        pub fn as_while(self) -> ::std::option::Option<unnamed::While<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::While(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Keyword<'tree> {
        type WithLifetime<'a> = Keyword<'a>;
        const KIND: &'static str = "{#define | #elif | #else | #endif | #if | #ifdef | #ifndef | #include | break | case | const | continue | default | do | else | enum | extern | for | if | inline | preproc_directive | return | sizeof | static | struct | switch | typedef | union | volatile | while}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "#define" => {
                    Ok(unsafe {
                        Self::Hashdefine(
                            <symbols::Hashdefine<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#elif" => {
                    Ok(unsafe {
                        Self::Hashelif(
                            <symbols::Hashelif<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#else" => {
                    Ok(unsafe {
                        Self::Hashelse(
                            <symbols::Hashelse<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#endif" => {
                    Ok(unsafe {
                        Self::Hashendif(
                            <symbols::Hashendif<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#if" => {
                    Ok(unsafe {
                        Self::Hashif(
                            <symbols::Hashif<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#ifdef" => {
                    Ok(unsafe {
                        Self::Hashifdef(
                            <symbols::Hashifdef<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#ifndef" => {
                    Ok(unsafe {
                        Self::Hashifndef(
                            <symbols::Hashifndef<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "#include" => {
                    Ok(unsafe {
                        Self::Hashinclude(
                            <symbols::Hashinclude<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "break" => {
                    Ok(unsafe {
                        Self::Break(
                            <unnamed::Break<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "case" => {
                    Ok(unsafe {
                        Self::Case(
                            <unnamed::Case<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "const" => {
                    Ok(unsafe {
                        Self::Const(
                            <unnamed::Const<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "continue" => {
                    Ok(unsafe {
                        Self::Continue(
                            <unnamed::Continue<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "default" => {
                    Ok(unsafe {
                        Self::Default(
                            <unnamed::Default<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "do" => {
                    Ok(unsafe {
                        Self::Do(
                            <unnamed::Do<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "else" => {
                    Ok(unsafe {
                        Self::Else(
                            <unnamed::Else<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "enum" => {
                    Ok(unsafe {
                        Self::Enum(
                            <unnamed::Enum<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "extern" => {
                    Ok(unsafe {
                        Self::Extern(
                            <unnamed::Extern<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "for" => {
                    Ok(unsafe {
                        Self::For(
                            <unnamed::For<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "if" => {
                    Ok(unsafe {
                        Self::If(
                            <unnamed::If<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "inline" => {
                    Ok(unsafe {
                        Self::Inline__(
                            <unnamed::Inline__<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "preproc_directive" => {
                    Ok(unsafe {
                        Self::PreprocDirective(
                            <PreprocDirective<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "return" => {
                    Ok(unsafe {
                        Self::Return(
                            <unnamed::Return<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "sizeof" => {
                    Ok(unsafe {
                        Self::Sizeof(
                            <unnamed::Sizeof<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "static" => {
                    Ok(unsafe {
                        Self::Static(
                            <unnamed::Static<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "struct" => {
                    Ok(unsafe {
                        Self::Struct(
                            <unnamed::Struct<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <unnamed::Switch<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "typedef" => {
                    Ok(unsafe {
                        Self::Typedef(
                            <unnamed::Typedef<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "union" => {
                    Ok(unsafe {
                        Self::Union(
                            <unnamed::Union<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "volatile" => {
                    Ok(unsafe {
                        Self::Volatile_(
                            <unnamed::Volatile_<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "while" => {
                    Ok(unsafe {
                        Self::While(
                            <unnamed::While<
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
                Self::Hashdefine(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashelif(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashelse(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashendif(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashif(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashifdef(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashifndef(x) => ::type_sitter_lib::Node::raw(x),
                Self::Hashinclude(x) => ::type_sitter_lib::Node::raw(x),
                Self::Break(x) => ::type_sitter_lib::Node::raw(x),
                Self::Case(x) => ::type_sitter_lib::Node::raw(x),
                Self::Const(x) => ::type_sitter_lib::Node::raw(x),
                Self::Continue(x) => ::type_sitter_lib::Node::raw(x),
                Self::Default(x) => ::type_sitter_lib::Node::raw(x),
                Self::Do(x) => ::type_sitter_lib::Node::raw(x),
                Self::Else(x) => ::type_sitter_lib::Node::raw(x),
                Self::Enum(x) => ::type_sitter_lib::Node::raw(x),
                Self::Extern(x) => ::type_sitter_lib::Node::raw(x),
                Self::For(x) => ::type_sitter_lib::Node::raw(x),
                Self::If(x) => ::type_sitter_lib::Node::raw(x),
                Self::Inline__(x) => ::type_sitter_lib::Node::raw(x),
                Self::PreprocDirective(x) => ::type_sitter_lib::Node::raw(x),
                Self::Return(x) => ::type_sitter_lib::Node::raw(x),
                Self::Sizeof(x) => ::type_sitter_lib::Node::raw(x),
                Self::Static(x) => ::type_sitter_lib::Node::raw(x),
                Self::Struct(x) => ::type_sitter_lib::Node::raw(x),
                Self::Switch(x) => ::type_sitter_lib::Node::raw(x),
                Self::Typedef(x) => ::type_sitter_lib::Node::raw(x),
                Self::Union(x) => ::type_sitter_lib::Node::raw(x),
                Self::Volatile_(x) => ::type_sitter_lib::Node::raw(x),
                Self::While(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Hashdefine(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashelif(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashelse(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashendif(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashif(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashifdef(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashifndef(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Hashinclude(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Break(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Case(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Const(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Continue(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Default(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Do(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Else(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Enum(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Extern(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::For(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::If(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Inline__(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::PreprocDirective(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Return(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Sizeof(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Static(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Struct(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Typedef(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Union(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Volatile_(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::While(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Hashdefine(x) => x.into_raw(),
                Self::Hashelif(x) => x.into_raw(),
                Self::Hashelse(x) => x.into_raw(),
                Self::Hashendif(x) => x.into_raw(),
                Self::Hashif(x) => x.into_raw(),
                Self::Hashifdef(x) => x.into_raw(),
                Self::Hashifndef(x) => x.into_raw(),
                Self::Hashinclude(x) => x.into_raw(),
                Self::Break(x) => x.into_raw(),
                Self::Case(x) => x.into_raw(),
                Self::Const(x) => x.into_raw(),
                Self::Continue(x) => x.into_raw(),
                Self::Default(x) => x.into_raw(),
                Self::Do(x) => x.into_raw(),
                Self::Else(x) => x.into_raw(),
                Self::Enum(x) => x.into_raw(),
                Self::Extern(x) => x.into_raw(),
                Self::For(x) => x.into_raw(),
                Self::If(x) => x.into_raw(),
                Self::Inline__(x) => x.into_raw(),
                Self::PreprocDirective(x) => x.into_raw(),
                Self::Return(x) => x.into_raw(),
                Self::Sizeof(x) => x.into_raw(),
                Self::Static(x) => x.into_raw(),
                Self::Struct(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Typedef(x) => x.into_raw(),
                Self::Union(x) => x.into_raw(),
                Self::Volatile_(x) => x.into_raw(),
                Self::While(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | type_identifier}`:
- [`Identifier`]
- [`TypeIdentifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Name<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Name<'tree> {
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
        ///Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_type_identifier(self) -> ::std::option::Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Name<'tree> {
        type WithLifetime<'a> = Name<'a>;
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(
                            <TypeIdentifier<
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
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::TypeIdentifier(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TypeIdentifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{char_literal | number_literal}`:
- [`CharLiteral`]
- [`NumberLiteral`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Number<'tree> {
        CharLiteral(CharLiteral<'tree>),
        NumberLiteral(NumberLiteral<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Number<'tree> {
        ///Returns the node if it is of type `char_literal` ([`CharLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_char_literal(self) -> ::std::option::Option<CharLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CharLiteral(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `number_literal` ([`NumberLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_number_literal(self) -> ::std::option::Option<NumberLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NumberLiteral(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Number<'tree> {
        type WithLifetime<'a> = Number<'a>;
        const KIND: &'static str = "{char_literal | number_literal}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "char_literal" => {
                    Ok(unsafe {
                        Self::CharLiteral(
                            <CharLiteral<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "number_literal" => {
                    Ok(unsafe {
                        Self::NumberLiteral(
                            <NumberLiteral<
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
                Self::CharLiteral(x) => ::type_sitter_lib::Node::raw(x),
                Self::NumberLiteral(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::CharLiteral(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::NumberLiteral(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::CharLiteral(x) => x.into_raw(),
                Self::NumberLiteral(x) => x.into_raw(),
            }
        }
    }
    /**One of `{!= | & | && | * | + | ++ | += | - | -- | -= | -> | < | = | == | > | ||}`:
- [`symbols::NotEq`]
- [`symbols::And`]
- [`symbols::AndAnd`]
- [`symbols::Mul`]
- [`symbols::Add`]
- [`symbols::AddAdd`]
- [`symbols::AddEq`]
- [`symbols::Sub`]
- [`symbols::SubSub`]
- [`symbols::SubEq`]
- [`symbols::SubGt`]
- [`symbols::Lt`]
- [`symbols::Eq`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::OrOr`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Operator<'tree> {
        NotEq(symbols::NotEq<'tree>),
        And(symbols::And<'tree>),
        AndAnd(symbols::AndAnd<'tree>),
        Mul(symbols::Mul<'tree>),
        Add(symbols::Add<'tree>),
        AddAdd(symbols::AddAdd<'tree>),
        AddEq(symbols::AddEq<'tree>),
        Sub(symbols::Sub<'tree>),
        SubSub(symbols::SubSub<'tree>),
        SubEq(symbols::SubEq<'tree>),
        SubGt(symbols::SubGt<'tree>),
        Lt(symbols::Lt<'tree>),
        Eq(symbols::Eq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        OrOr(symbols::OrOr<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Operator<'tree> {
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
        ///Returns the node if it is of type `&&` ([`symbols::AndAnd`]), otherwise returns `None`
        #[inline]
        pub fn as_and_and(self) -> ::std::option::Option<symbols::AndAnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AndAnd(x) = self {
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
        ///Returns the node if it is of type `++` ([`symbols::AddAdd`]), otherwise returns `None`
        #[inline]
        pub fn as_add_add(self) -> ::std::option::Option<symbols::AddAdd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AddAdd(x) = self {
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
        ///Returns the node if it is of type `--` ([`symbols::SubSub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub_sub(self) -> ::std::option::Option<symbols::SubSub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubSub(x) = self {
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
        ///Returns the node if it is of type `->` ([`symbols::SubGt`]), otherwise returns `None`
        #[inline]
        pub fn as_sub_gt(self) -> ::std::option::Option<symbols::SubGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubGt(x) = self {
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
        ///Returns the node if it is of type `=` ([`symbols::Eq`]), otherwise returns `None`
        #[inline]
        pub fn as_eq(self) -> ::std::option::Option<symbols::Eq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Eq(x) = self {
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
        ///Returns the node if it is of type `||` ([`symbols::OrOr`]), otherwise returns `None`
        #[inline]
        pub fn as_or_or(self) -> ::std::option::Option<symbols::OrOr<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OrOr(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Operator<'tree> {
        type WithLifetime<'a> = Operator<'a>;
        const KIND: &'static str = "{!= | & | && | * | + | ++ | += | - | -- | -= | -> | < | = | == | > | ||}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "!=" => {
                    Ok(unsafe {
                        Self::NotEq(
                            <symbols::NotEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "&" => {
                    Ok(unsafe {
                        Self::And(
                            <symbols::And<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "&&" => {
                    Ok(unsafe {
                        Self::AndAnd(
                            <symbols::AndAnd<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "*" => {
                    Ok(unsafe {
                        Self::Mul(
                            <symbols::Mul<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "+" => {
                    Ok(unsafe {
                        Self::Add(
                            <symbols::Add<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "++" => {
                    Ok(unsafe {
                        Self::AddAdd(
                            <symbols::AddAdd<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "+=" => {
                    Ok(unsafe {
                        Self::AddEq(
                            <symbols::AddEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "-" => {
                    Ok(unsafe {
                        Self::Sub(
                            <symbols::Sub<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "--" => {
                    Ok(unsafe {
                        Self::SubSub(
                            <symbols::SubSub<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "-=" => {
                    Ok(unsafe {
                        Self::SubEq(
                            <symbols::SubEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "->" => {
                    Ok(unsafe {
                        Self::SubGt(
                            <symbols::SubGt<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "<" => {
                    Ok(unsafe {
                        Self::Lt(
                            <symbols::Lt<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "=" => {
                    Ok(unsafe {
                        Self::Eq(
                            <symbols::Eq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "==" => {
                    Ok(unsafe {
                        Self::EqEq(
                            <symbols::EqEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                ">" => {
                    Ok(unsafe {
                        Self::Gt(
                            <symbols::Gt<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "||" => {
                    Ok(unsafe {
                        Self::OrOr(
                            <symbols::OrOr<
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
                Self::NotEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::And(x) => ::type_sitter_lib::Node::raw(x),
                Self::AndAnd(x) => ::type_sitter_lib::Node::raw(x),
                Self::Mul(x) => ::type_sitter_lib::Node::raw(x),
                Self::Add(x) => ::type_sitter_lib::Node::raw(x),
                Self::AddAdd(x) => ::type_sitter_lib::Node::raw(x),
                Self::AddEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw(x),
                Self::SubSub(x) => ::type_sitter_lib::Node::raw(x),
                Self::SubEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::SubGt(x) => ::type_sitter_lib::Node::raw(x),
                Self::Lt(x) => ::type_sitter_lib::Node::raw(x),
                Self::Eq(x) => ::type_sitter_lib::Node::raw(x),
                Self::EqEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Gt(x) => ::type_sitter_lib::Node::raw(x),
                Self::OrOr(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::And(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AndAnd(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AddAdd(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AddEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SubSub(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SubEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SubGt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Eq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::OrOr(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::And(x) => x.into_raw(),
                Self::AndAnd(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::AddAdd(x) => x.into_raw(),
                Self::AddEq(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::SubSub(x) => x.into_raw(),
                Self::SubEq(x) => x.into_raw(),
                Self::SubGt(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::Eq(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::OrOr(x) => x.into_raw(),
            }
        }
    }
    /**One of `{string_literal | system_lib_string}`:
- [`StringLiteral`]
- [`SystemLibString`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum String<'tree> {
        StringLiteral(StringLiteral<'tree>),
        SystemLibString(SystemLibString<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> String<'tree> {
        ///Returns the node if it is of type `string_literal` ([`StringLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_string_literal(self) -> ::std::option::Option<StringLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringLiteral(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `system_lib_string` ([`SystemLibString`]), otherwise returns `None`
        #[inline]
        pub fn as_system_lib_string(
            self,
        ) -> ::std::option::Option<SystemLibString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SystemLibString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for String<'tree> {
        type WithLifetime<'a> = String<'a>;
        const KIND: &'static str = "{string_literal | system_lib_string}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "string_literal" => {
                    Ok(unsafe {
                        Self::StringLiteral(
                            <StringLiteral<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "system_lib_string" => {
                    Ok(unsafe {
                        Self::SystemLibString(
                            <SystemLibString<
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
                Self::StringLiteral(x) => ::type_sitter_lib::Node::raw(x),
                Self::SystemLibString(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::StringLiteral(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SystemLibString(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::StringLiteral(x) => x.into_raw(),
                Self::SystemLibString(x) => x.into_raw(),
            }
        }
    }
    /**One of `{primitive_type | sized_type_specifier | type_identifier}`:
- [`PrimitiveType`]
- [`SizedTypeSpecifier`]
- [`TypeIdentifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type<'tree> {
        PrimitiveType(PrimitiveType<'tree>),
        SizedTypeSpecifier(SizedTypeSpecifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Type<'tree> {
        ///Returns the node if it is of type `primitive_type` ([`PrimitiveType`]), otherwise returns `None`
        #[inline]
        pub fn as_primitive_type(self) -> ::std::option::Option<PrimitiveType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimitiveType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `sized_type_specifier` ([`SizedTypeSpecifier`]), otherwise returns `None`
        #[inline]
        pub fn as_sized_type_specifier(
            self,
        ) -> ::std::option::Option<SizedTypeSpecifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SizedTypeSpecifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_type_identifier(self) -> ::std::option::Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Type<'tree> {
        type WithLifetime<'a> = Type<'a>;
        const KIND: &'static str = "{primitive_type | sized_type_specifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "primitive_type" => {
                    Ok(unsafe {
                        Self::PrimitiveType(
                            <PrimitiveType<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "sized_type_specifier" => {
                    Ok(unsafe {
                        Self::SizedTypeSpecifier(
                            <SizedTypeSpecifier<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(
                            <TypeIdentifier<
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
                Self::PrimitiveType(x) => ::type_sitter_lib::Node::raw(x),
                Self::SizedTypeSpecifier(x) => ::type_sitter_lib::Node::raw(x),
                Self::TypeIdentifier(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::PrimitiveType(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SizedTypeSpecifier(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::TypeIdentifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::PrimitiveType(x) => x.into_raw(),
                Self::SizedTypeSpecifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
}
