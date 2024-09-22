/**Typed version of the query:

```sexp
; Identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

(line_comment) @comment
(block_comment) @comment

(line_comment (doc_comment)) @comment.documentation
(block_comment (doc_comment)) @comment.documentation

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
/**Matches returned by a query cursor running the query [`Highlights`]:

```sexp
; Identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

(line_comment) @comment
(block_comment) @comment

(line_comment (doc_comment)) @comment.documentation
(block_comment (doc_comment)) @comment.documentation

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'query, 'tree> = type_sitter_lib::QueryMatches<
    'query,
    'tree,
    Highlights,
>;
/**Captures returned by a query cursor running the query [`Highlights`]:

```sexp
; Identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

(line_comment) @comment
(block_comment) @comment

(line_comment (doc_comment)) @comment.documentation
(block_comment (doc_comment)) @comment.documentation

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'query, 'tree> = type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Highlights,
>;
/**A match returned by the query [`Highlights`]:

```sexp
; Identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

(line_comment) @comment
(block_comment) @comment

(line_comment (doc_comment)) @comment.documentation
(block_comment (doc_comment)) @comment.documentation

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
#[repr(transparent)]
pub struct HighlightsMatch<'query, 'tree: 'query>(yak_sitter::QueryMatch<'query, 'tree>);
/**A capture returned by the query [`Highlights`]:

```sexp
; Identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

(line_comment) @comment
(block_comment) @comment

(line_comment (doc_comment)) @comment.documentation
(block_comment (doc_comment)) @comment.documentation

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
(crate) @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"*" @operator
"&" @operator
"'" @operator

```*/
pub enum HighlightsCapture<'query, 'tree: 'query> {
    ///A `type` ([`anon_unions::Type`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///```
    Type {
        node: anon_unions::Type<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `type.builtin` ([`super::nodes::PrimitiveType`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(primitive_type) @type.builtin
    ///```
    TypeBuiltin {
        node: super::nodes::PrimitiveType<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `property` ([`super::nodes::FieldIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @property
    ///```
    Property {
        node: super::nodes::FieldIdentifier<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `constant` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///```
    Constant {
        node: super::nodes::Identifier<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `constructor` ([`anon_unions::Constructor`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constructor
    ///(type_identifier) @constructor
    ///```
    Constructor {
        node: anon_unions::Constructor<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///```
    Function {
        node: super::nodes::Identifier<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `function.method` ([`anon_unions::FunctionMethod`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @function.method
    ///(field_identifier) @function.method
    ///```
    FunctionMethod {
        node: super::nodes::FieldIdentifier<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `function.macro` ([`anon_unions::FunctionMacro`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.macro
    ///"!" @function.macro
    ///```
    FunctionMacro {
        node: anon_unions::FunctionMacro<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `comment` ([`anon_unions::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(line_comment) @comment
    ///(block_comment) @comment
    ///```
    Comment {
        node: anon_unions::Comment<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `comment.documentation` ([`anon_unions::CommentDocumentation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(line_comment (doc_comment)) @comment.documentation
    ///(block_comment (doc_comment)) @comment.documentation
    ///```
    CommentDocumentation {
        node: anon_unions::CommentDocumentation<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `punctuation.bracket` ([`anon_unions::PunctuationBracket`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"(" @punctuation.bracket
    ///")" @punctuation.bracket
    ///"[" @punctuation.bracket
    ///"]" @punctuation.bracket
    ///"{" @punctuation.bracket
    ///"}" @punctuation.bracket
    ///"<" @punctuation.bracket
    ///">" @punctuation.bracket
    ///"<" @punctuation.bracket
    ///">" @punctuation.bracket
    ///```
    PunctuationBracket {
        node: anon_unions::PunctuationBracket<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `punctuation.delimiter` ([`anon_unions::PunctuationDelimiter`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"::" @punctuation.delimiter
    ///":" @punctuation.delimiter
    ///"." @punctuation.delimiter
    ///"," @punctuation.delimiter
    ///";" @punctuation.delimiter
    ///```
    PunctuationDelimiter {
        node: anon_unions::PunctuationDelimiter<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `variable.parameter` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @variable.parameter
    ///```
    VariableParameter {
        node: super::nodes::Identifier<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `label` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @label
    ///```
    Label {
        node: super::nodes::Identifier<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"as" @keyword
    ///"async" @keyword
    ///"await" @keyword
    ///"break" @keyword
    ///"const" @keyword
    ///"continue" @keyword
    ///"default" @keyword
    ///"dyn" @keyword
    ///"else" @keyword
    ///"enum" @keyword
    ///"extern" @keyword
    ///"fn" @keyword
    ///"for" @keyword
    ///"if" @keyword
    ///"impl" @keyword
    ///"in" @keyword
    ///"let" @keyword
    ///"loop" @keyword
    ///"macro_rules!" @keyword
    ///"match" @keyword
    ///"mod" @keyword
    ///"move" @keyword
    ///"pub" @keyword
    ///"ref" @keyword
    ///"return" @keyword
    ///"static" @keyword
    ///"struct" @keyword
    ///"trait" @keyword
    ///"type" @keyword
    ///"union" @keyword
    ///"unsafe" @keyword
    ///"use" @keyword
    ///"where" @keyword
    ///"while" @keyword
    ///"yield" @keyword
    ///(crate) @keyword
    ///(mutable_specifier) @keyword
    ///(super) @keyword
    ///(self) @keyword
    ///(self) @keyword
    ///(self) @keyword
    ///```
    Keyword {
        node: anon_unions::Keyword<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `variable.builtin` ([`super::nodes::Self_`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(self) @variable.builtin
    ///```
    VariableBuiltin {
        node: super::nodes::Self_<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `string` ([`anon_unions::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(char_literal) @string
    ///(string_literal) @string
    ///(raw_string_literal) @string
    ///```
    String {
        node: anon_unions::String<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `constant.builtin` ([`anon_unions::ConstantBuiltin`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(boolean_literal) @constant.builtin
    ///(integer_literal) @constant.builtin
    ///(float_literal) @constant.builtin
    ///```
    ConstantBuiltin {
        node: anon_unions::ConstantBuiltin<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `escape` ([`super::nodes::EscapeSequence`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    Escape {
        node: super::nodes::EscapeSequence<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `attribute` ([`anon_unions::Attribute`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(attribute_item) @attribute
    ///(inner_attribute_item) @attribute
    ///```
    Attribute {
        node: anon_unions::Attribute<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
    ///A `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"*" @operator
    ///"&" @operator
    ///"'" @operator
    ///```
    Operator {
        node: anon_unions::Operator<'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::Query for Highlights {
    type Match<'query, 'tree: 'query> = HighlightsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = HighlightsCapture<'query, 'tree>;
    fn as_str(&self) -> &'static str {
        "; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n"
    }
    fn raw(&self) -> &'static yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Highlights__: std::sync::OnceLock<yak_sitter::Query> = std::sync::OnceLock::new();
        __Highlights__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = yak_sitter::Query::new(
                        &tree_sitter_rust::LANGUAGE.into(),
                        "; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?",
                    );
                query
            })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: yak_sitter::QueryMatch<'query, 'tree>,
    ) -> HighlightsMatch<'query, 'tree> {
        HighlightsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: yak_sitter::QueryCapture<'query, 'tree>,
        r#match: Option<HighlightsMatch<'query, 'tree>>,
    ) -> HighlightsCapture<'query, 'tree> {
        match capture.index as usize {
            0usize => {
                HighlightsCapture::Type {
                    node: <anon_unions::Type<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            1usize => {
                HighlightsCapture::TypeBuiltin {
                    node: <super::nodes::PrimitiveType<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            2usize => {
                HighlightsCapture::Property {
                    node: <super::nodes::FieldIdentifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            3usize => {
                HighlightsCapture::Constant {
                    node: <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            4usize => {
                HighlightsCapture::Constructor {
                    node: <anon_unions::Constructor<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            5usize => {
                HighlightsCapture::Function {
                    node: <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            6usize => {
                HighlightsCapture::FunctionMethod {
                    node: <super::nodes::FieldIdentifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            7usize => {
                HighlightsCapture::FunctionMacro {
                    node: <anon_unions::FunctionMacro<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            8usize => {
                HighlightsCapture::Comment {
                    node: <anon_unions::Comment<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            9usize => {
                HighlightsCapture::CommentDocumentation {
                    node: <anon_unions::CommentDocumentation<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            10usize => {
                HighlightsCapture::PunctuationBracket {
                    node: <anon_unions::PunctuationBracket<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            11usize => {
                HighlightsCapture::PunctuationDelimiter {
                    node: <anon_unions::PunctuationDelimiter<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            12usize => {
                HighlightsCapture::VariableParameter {
                    node: <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            13usize => {
                HighlightsCapture::Label {
                    node: <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            14usize => {
                HighlightsCapture::Keyword {
                    node: <anon_unions::Keyword<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            15usize => {
                HighlightsCapture::VariableBuiltin {
                    node: <super::nodes::Self_<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            16usize => {
                HighlightsCapture::String {
                    node: <anon_unions::String<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            17usize => {
                HighlightsCapture::ConstantBuiltin {
                    node: <anon_unions::ConstantBuiltin<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            18usize => {
                HighlightsCapture::Escape {
                    node: <super::nodes::EscapeSequence<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            19usize => {
                HighlightsCapture::Attribute {
                    node: <anon_unions::Attribute<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            20usize => {
                HighlightsCapture::Operator {
                    node: <anon_unions::Operator<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> HighlightsMatch<'query, 'tree> {
    ///Returns an iterator over the nodes captured by `type` ([`anon_unions::Type`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///```
    #[inline]
    pub fn r#type(&self) -> Option<anon_unions::Type<'tree>> {
        {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Type<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `type.builtin` ([`super::nodes::PrimitiveType`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(primitive_type) @type.builtin
    ///```
    #[inline]
    pub fn type_builtin(&self) -> Option<super::nodes::PrimitiveType<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::PrimitiveType<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
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
    pub fn property(&self) -> Option<super::nodes::FieldIdentifier<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::FieldIdentifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `constant` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///```
    #[inline]
    pub fn constant(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            [3u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `constructor` ([`anon_unions::Constructor`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constructor
    ///(type_identifier) @constructor
    ///```
    #[inline]
    pub fn constructor(&self) -> Option<anon_unions::Constructor<'tree>> {
        {
            [4u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Constructor<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///```
    #[inline]
    pub fn function(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            [5u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function.method` ([`anon_unions::FunctionMethod`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @function.method
    ///(field_identifier) @function.method
    ///```
    #[inline]
    pub fn function_method(&self) -> Option<super::nodes::FieldIdentifier<'tree>> {
        {
            [6u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::FieldIdentifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `function.macro` ([`anon_unions::FunctionMacro`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.macro
    ///"!" @function.macro
    ///```
    #[inline]
    pub fn function_macro(&self) -> Option<anon_unions::FunctionMacro<'tree>> {
        {
            [7u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::FunctionMacro<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `comment` ([`anon_unions::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(line_comment) @comment
    ///(block_comment) @comment
    ///```
    #[inline]
    pub fn comment(&self) -> Option<anon_unions::Comment<'tree>> {
        {
            [8u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Comment<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `comment.documentation` ([`anon_unions::CommentDocumentation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(line_comment (doc_comment)) @comment.documentation
    ///(block_comment (doc_comment)) @comment.documentation
    ///```
    #[inline]
    pub fn comment_documentation(
        &self,
    ) -> Option<anon_unions::CommentDocumentation<'tree>> {
        {
            [9u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::CommentDocumentation<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `punctuation.bracket` ([`anon_unions::PunctuationBracket`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"(" @punctuation.bracket
    ///")" @punctuation.bracket
    ///"[" @punctuation.bracket
    ///"]" @punctuation.bracket
    ///"{" @punctuation.bracket
    ///"}" @punctuation.bracket
    ///"<" @punctuation.bracket
    ///">" @punctuation.bracket
    ///"<" @punctuation.bracket
    ///">" @punctuation.bracket
    ///```
    #[inline]
    pub fn punctuation_bracket(&self) -> Option<anon_unions::PunctuationBracket<'tree>> {
        {
            [10u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::PunctuationBracket<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `punctuation.delimiter` ([`anon_unions::PunctuationDelimiter`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"::" @punctuation.delimiter
    ///":" @punctuation.delimiter
    ///"." @punctuation.delimiter
    ///"," @punctuation.delimiter
    ///";" @punctuation.delimiter
    ///```
    #[inline]
    pub fn punctuation_delimiter(
        &self,
    ) -> Option<anon_unions::PunctuationDelimiter<'tree>> {
        {
            [11u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::PunctuationDelimiter<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `variable.parameter` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @variable.parameter
    ///```
    #[inline]
    pub fn variable_parameter(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            [12u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `label` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @label
    ///```
    #[inline]
    pub fn label(&self) -> Option<super::nodes::Identifier<'tree>> {
        {
            [13u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"as" @keyword
    ///"async" @keyword
    ///"await" @keyword
    ///"break" @keyword
    ///"const" @keyword
    ///"continue" @keyword
    ///"default" @keyword
    ///"dyn" @keyword
    ///"else" @keyword
    ///"enum" @keyword
    ///"extern" @keyword
    ///"fn" @keyword
    ///"for" @keyword
    ///"if" @keyword
    ///"impl" @keyword
    ///"in" @keyword
    ///"let" @keyword
    ///"loop" @keyword
    ///"macro_rules!" @keyword
    ///"match" @keyword
    ///"mod" @keyword
    ///"move" @keyword
    ///"pub" @keyword
    ///"ref" @keyword
    ///"return" @keyword
    ///"static" @keyword
    ///"struct" @keyword
    ///"trait" @keyword
    ///"type" @keyword
    ///"union" @keyword
    ///"unsafe" @keyword
    ///"use" @keyword
    ///"where" @keyword
    ///"while" @keyword
    ///"yield" @keyword
    ///(crate) @keyword
    ///(mutable_specifier) @keyword
    ///(super) @keyword
    ///(self) @keyword
    ///(self) @keyword
    ///(self) @keyword
    ///```
    #[inline]
    pub fn keyword(&self) -> Option<anon_unions::Keyword<'tree>> {
        {
            [14u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Keyword<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `variable.builtin` ([`super::nodes::Self_`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(self) @variable.builtin
    ///```
    #[inline]
    pub fn variable_builtin(&self) -> Option<super::nodes::Self_<'tree>> {
        {
            [15u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Self_<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `string` ([`anon_unions::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(char_literal) @string
    ///(string_literal) @string
    ///(raw_string_literal) @string
    ///```
    #[inline]
    pub fn string(&self) -> impl Iterator<Item = anon_unions::String<'tree>> + '_ {
        {
            [16u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::String<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
    }
    ///Returns an iterator over the nodes captured by `constant.builtin` ([`anon_unions::ConstantBuiltin`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(boolean_literal) @constant.builtin
    ///(integer_literal) @constant.builtin
    ///(float_literal) @constant.builtin
    ///```
    #[inline]
    pub fn constant_builtin(&self) -> Option<anon_unions::ConstantBuiltin<'tree>> {
        {
            [17u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::ConstantBuiltin<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `escape` ([`super::nodes::EscapeSequence`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    #[inline]
    pub fn escape(&self) -> Option<super::nodes::EscapeSequence<'tree>> {
        {
            [18u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::EscapeSequence<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `attribute` ([`anon_unions::Attribute`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(attribute_item) @attribute
    ///(inner_attribute_item) @attribute
    ///```
    #[inline]
    pub fn attribute(&self) -> Option<anon_unions::Attribute<'tree>> {
        {
            [19u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Attribute<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"*" @operator
    ///"&" @operator
    ///"'" @operator
    ///```
    #[inline]
    pub fn operator(&self) -> Option<anon_unions::Operator<'tree>> {
        {
            [20u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Operator<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for HighlightsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(HighlightsMatch)).field(&self.0).finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> type_sitter_lib::QueryMatch<'query, 'tree>
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
    fn raw(&self) -> &yak_sitter::QueryMatch<'query, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> yak_sitter::QueryMatch<'query, 'tree> {
        self.0
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> HighlightsCapture<'query, 'tree> {
    ///Try to interpret this capture as a `type` ([`anon_unions::Type`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///(identifier) @type
    ///```
    #[inline]
    pub fn as_type(&self) -> Option<&anon_unions::Type<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Type { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `type.builtin` ([`super::nodes::PrimitiveType`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(primitive_type) @type.builtin
    ///```
    #[inline]
    pub fn as_type_builtin(&self) -> Option<&super::nodes::PrimitiveType<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypeBuiltin { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `property` ([`super::nodes::FieldIdentifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @property
    ///```
    #[inline]
    pub fn as_property(&self) -> Option<&super::nodes::FieldIdentifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Property { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `constant` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///```
    #[inline]
    pub fn as_constant(&self) -> Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constant { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `constructor` ([`anon_unions::Constructor`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constructor
    ///(type_identifier) @constructor
    ///```
    #[inline]
    pub fn as_constructor(&self) -> Option<&anon_unions::Constructor<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constructor { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///```
    #[inline]
    pub fn as_function(&self) -> Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Function { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function.method` ([`anon_unions::FunctionMethod`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(field_identifier) @function.method
    ///(field_identifier) @function.method
    ///```
    #[inline]
    pub fn as_function_method(&self) -> Option<&super::nodes::FieldIdentifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionMethod { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function.macro` ([`anon_unions::FunctionMacro`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.macro
    ///"!" @function.macro
    ///```
    #[inline]
    pub fn as_function_macro(&self) -> Option<&anon_unions::FunctionMacro<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionMacro { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `comment` ([`anon_unions::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(line_comment) @comment
    ///(block_comment) @comment
    ///```
    #[inline]
    pub fn as_comment(&self) -> Option<&anon_unions::Comment<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Comment { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `comment.documentation` ([`anon_unions::CommentDocumentation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(line_comment (doc_comment)) @comment.documentation
    ///(block_comment (doc_comment)) @comment.documentation
    ///```
    #[inline]
    pub fn as_comment_documentation(
        &self,
    ) -> Option<&anon_unions::CommentDocumentation<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::CommentDocumentation { node, .. } = self {
            Some(node)
        } else {
            None
        }
    }
    ///Try to interpret this capture as a `punctuation.bracket` ([`anon_unions::PunctuationBracket`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"(" @punctuation.bracket
    ///")" @punctuation.bracket
    ///"[" @punctuation.bracket
    ///"]" @punctuation.bracket
    ///"{" @punctuation.bracket
    ///"}" @punctuation.bracket
    ///"<" @punctuation.bracket
    ///">" @punctuation.bracket
    ///"<" @punctuation.bracket
    ///">" @punctuation.bracket
    ///```
    #[inline]
    pub fn as_punctuation_bracket(
        &self,
    ) -> Option<&anon_unions::PunctuationBracket<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PunctuationBracket { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `punctuation.delimiter` ([`anon_unions::PunctuationDelimiter`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"::" @punctuation.delimiter
    ///":" @punctuation.delimiter
    ///"." @punctuation.delimiter
    ///"," @punctuation.delimiter
    ///";" @punctuation.delimiter
    ///```
    #[inline]
    pub fn as_punctuation_delimiter(
        &self,
    ) -> Option<&anon_unions::PunctuationDelimiter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PunctuationDelimiter { node, .. } = self {
            Some(node)
        } else {
            None
        }
    }
    ///Try to interpret this capture as a `variable.parameter` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @variable.parameter
    ///```
    #[inline]
    pub fn as_variable_parameter(&self) -> Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::VariableParameter { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `label` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @label
    ///```
    #[inline]
    pub fn as_label(&self) -> Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Label { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"as" @keyword
    ///"async" @keyword
    ///"await" @keyword
    ///"break" @keyword
    ///"const" @keyword
    ///"continue" @keyword
    ///"default" @keyword
    ///"dyn" @keyword
    ///"else" @keyword
    ///"enum" @keyword
    ///"extern" @keyword
    ///"fn" @keyword
    ///"for" @keyword
    ///"if" @keyword
    ///"impl" @keyword
    ///"in" @keyword
    ///"let" @keyword
    ///"loop" @keyword
    ///"macro_rules!" @keyword
    ///"match" @keyword
    ///"mod" @keyword
    ///"move" @keyword
    ///"pub" @keyword
    ///"ref" @keyword
    ///"return" @keyword
    ///"static" @keyword
    ///"struct" @keyword
    ///"trait" @keyword
    ///"type" @keyword
    ///"union" @keyword
    ///"unsafe" @keyword
    ///"use" @keyword
    ///"where" @keyword
    ///"while" @keyword
    ///"yield" @keyword
    ///(crate) @keyword
    ///(mutable_specifier) @keyword
    ///(super) @keyword
    ///(self) @keyword
    ///(self) @keyword
    ///(self) @keyword
    ///```
    #[inline]
    pub fn as_keyword(&self) -> Option<&anon_unions::Keyword<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Keyword { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `variable.builtin` ([`super::nodes::Self_`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(self) @variable.builtin
    ///```
    #[inline]
    pub fn as_variable_builtin(&self) -> Option<&super::nodes::Self_<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::VariableBuiltin { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `string` ([`anon_unions::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(char_literal) @string
    ///(string_literal) @string
    ///(raw_string_literal) @string
    ///```
    #[inline]
    pub fn as_string(&self) -> Option<&anon_unions::String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `constant.builtin` ([`anon_unions::ConstantBuiltin`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(boolean_literal) @constant.builtin
    ///(integer_literal) @constant.builtin
    ///(float_literal) @constant.builtin
    ///```
    #[inline]
    pub fn as_constant_builtin(&self) -> Option<&anon_unions::ConstantBuiltin<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConstantBuiltin { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `escape` ([`super::nodes::EscapeSequence`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    #[inline]
    pub fn as_escape(&self) -> Option<&super::nodes::EscapeSequence<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Escape { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `attribute` ([`anon_unions::Attribute`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(attribute_item) @attribute
    ///(inner_attribute_item) @attribute
    ///```
    #[inline]
    pub fn as_attribute(&self) -> Option<&anon_unions::Attribute<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Attribute { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"*" @operator
    ///"&" @operator
    ///"'" @operator
    ///```
    #[inline]
    pub fn as_operator(&self) -> Option<&anon_unions::Operator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Operator { node, .. } = self { Some(node) } else { None }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for HighlightsCapture<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Type)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::TypeBuiltin { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(TypeBuiltin)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Property { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(Property)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Constant { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(Constant)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Constructor { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(Constructor)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Function { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(Function)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::FunctionMethod { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(FunctionMethod)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::FunctionMacro { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(FunctionMacro)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Comment { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Comment)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::CommentDocumentation { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(CommentDocumentation)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::PunctuationBracket { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(PunctuationBracket)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::PunctuationDelimiter { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(PunctuationDelimiter)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::VariableParameter { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(VariableParameter)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Label { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Label)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Keyword { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Keyword)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::VariableBuiltin { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(VariableBuiltin)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::String { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(String)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::ConstantBuiltin { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::",
                            stringify!(ConstantBuiltin)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Escape { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(HighlightsCapture), "::", stringify!(Escape)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Attribute { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(Attribute)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::Operator { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(HighlightsCapture), "::", stringify!(Operator)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> Clone for HighlightsCapture<'query, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::Type { node, .. } => {
                Self::Type {
                    node: *node,
                    r#match: None,
                }
            }
            Self::TypeBuiltin { node, .. } => {
                Self::TypeBuiltin {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Property { node, .. } => {
                Self::Property {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Constant { node, .. } => {
                Self::Constant {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Constructor { node, .. } => {
                Self::Constructor {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Function { node, .. } => {
                Self::Function {
                    node: *node,
                    r#match: None,
                }
            }
            Self::FunctionMethod { node, .. } => {
                Self::FunctionMethod {
                    node: *node,
                    r#match: None,
                }
            }
            Self::FunctionMacro { node, .. } => {
                Self::FunctionMacro {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Comment { node, .. } => {
                Self::Comment {
                    node: *node,
                    r#match: None,
                }
            }
            Self::CommentDocumentation { node, .. } => {
                Self::CommentDocumentation {
                    node: *node,
                    r#match: None,
                }
            }
            Self::PunctuationBracket { node, .. } => {
                Self::PunctuationBracket {
                    node: *node,
                    r#match: None,
                }
            }
            Self::PunctuationDelimiter { node, .. } => {
                Self::PunctuationDelimiter {
                    node: *node,
                    r#match: None,
                }
            }
            Self::VariableParameter { node, .. } => {
                Self::VariableParameter {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Label { node, .. } => {
                Self::Label {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Keyword { node, .. } => {
                Self::Keyword {
                    node: *node,
                    r#match: None,
                }
            }
            Self::VariableBuiltin { node, .. } => {
                Self::VariableBuiltin {
                    node: *node,
                    r#match: None,
                }
            }
            Self::String { node, .. } => {
                Self::String {
                    node: *node,
                    r#match: None,
                }
            }
            Self::ConstantBuiltin { node, .. } => {
                Self::ConstantBuiltin {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Escape { node, .. } => {
                Self::Escape {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Attribute { node, .. } => {
                Self::Attribute {
                    node: *node,
                    r#match: None,
                }
            }
            Self::Operator { node, .. } => {
                Self::Operator {
                    node: *node,
                    r#match: None,
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> type_sitter_lib::QueryCapture<'query, 'tree>
for HighlightsCapture<'query, 'tree> {
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Highlights
    }
    #[inline]
    fn r#match(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::Query>::Match<'query, 'tree>> {
        match self {
            Self::Type { r#match, .. } => r#match.as_ref(),
            Self::TypeBuiltin { r#match, .. } => r#match.as_ref(),
            Self::Property { r#match, .. } => r#match.as_ref(),
            Self::Constant { r#match, .. } => r#match.as_ref(),
            Self::Constructor { r#match, .. } => r#match.as_ref(),
            Self::Function { r#match, .. } => r#match.as_ref(),
            Self::FunctionMethod { r#match, .. } => r#match.as_ref(),
            Self::FunctionMacro { r#match, .. } => r#match.as_ref(),
            Self::Comment { r#match, .. } => r#match.as_ref(),
            Self::CommentDocumentation { r#match, .. } => r#match.as_ref(),
            Self::PunctuationBracket { r#match, .. } => r#match.as_ref(),
            Self::PunctuationDelimiter { r#match, .. } => r#match.as_ref(),
            Self::VariableParameter { r#match, .. } => r#match.as_ref(),
            Self::Label { r#match, .. } => r#match.as_ref(),
            Self::Keyword { r#match, .. } => r#match.as_ref(),
            Self::VariableBuiltin { r#match, .. } => r#match.as_ref(),
            Self::String { r#match, .. } => r#match.as_ref(),
            Self::ConstantBuiltin { r#match, .. } => r#match.as_ref(),
            Self::Escape { r#match, .. } => r#match.as_ref(),
            Self::Attribute { r#match, .. } => r#match.as_ref(),
            Self::Operator { r#match, .. } => r#match.as_ref(),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::Query>::Match<'query, 'tree>> {
        match self {
            Self::Type { r#match, .. } => r#match,
            Self::TypeBuiltin { r#match, .. } => r#match,
            Self::Property { r#match, .. } => r#match,
            Self::Constant { r#match, .. } => r#match,
            Self::Constructor { r#match, .. } => r#match,
            Self::Function { r#match, .. } => r#match,
            Self::FunctionMethod { r#match, .. } => r#match,
            Self::FunctionMacro { r#match, .. } => r#match,
            Self::Comment { r#match, .. } => r#match,
            Self::CommentDocumentation { r#match, .. } => r#match,
            Self::PunctuationBracket { r#match, .. } => r#match,
            Self::PunctuationDelimiter { r#match, .. } => r#match,
            Self::VariableParameter { r#match, .. } => r#match,
            Self::Label { r#match, .. } => r#match,
            Self::Keyword { r#match, .. } => r#match,
            Self::VariableBuiltin { r#match, .. } => r#match,
            Self::String { r#match, .. } => r#match,
            Self::ConstantBuiltin { r#match, .. } => r#match,
            Self::Escape { r#match, .. } => r#match,
            Self::Attribute { r#match, .. } => r#match,
            Self::Operator { r#match, .. } => r#match,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn raw(&self) -> yak_sitter::QueryCapture<'query, 'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::Type { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 0usize,
                    name: "type",
                }
            }
            Self::TypeBuiltin { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 1usize,
                    name: "type.builtin",
                }
            }
            Self::Property { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 2usize,
                    name: "property",
                }
            }
            Self::Constant { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 3usize,
                    name: "constant",
                }
            }
            Self::Constructor { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 4usize,
                    name: "constructor",
                }
            }
            Self::Function { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 5usize,
                    name: "function",
                }
            }
            Self::FunctionMethod { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 6usize,
                    name: "function.method",
                }
            }
            Self::FunctionMacro { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 7usize,
                    name: "function.macro",
                }
            }
            Self::Comment { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 8usize,
                    name: "comment",
                }
            }
            Self::CommentDocumentation { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 9usize,
                    name: "comment.documentation",
                }
            }
            Self::PunctuationBracket { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 10usize,
                    name: "punctuation.bracket",
                }
            }
            Self::PunctuationDelimiter { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 11usize,
                    name: "punctuation.delimiter",
                }
            }
            Self::VariableParameter { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 12usize,
                    name: "variable.parameter",
                }
            }
            Self::Label { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 13usize,
                    name: "label",
                }
            }
            Self::Keyword { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 14usize,
                    name: "keyword",
                }
            }
            Self::VariableBuiltin { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 15usize,
                    name: "variable.builtin",
                }
            }
            Self::String { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 16usize,
                    name: "string",
                }
            }
            Self::ConstantBuiltin { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 17usize,
                    name: "constant.builtin",
                }
            }
            Self::Escape { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 18usize,
                    name: "escape",
                }
            }
            Self::Attribute { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 19usize,
                    name: "attribute",
                }
            }
            Self::Operator { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 20usize,
                    name: "operator",
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::Type { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::TypeBuiltin { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Property { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Constant { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Constructor { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Function { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::FunctionMethod { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::FunctionMacro { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Comment { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::CommentDocumentation { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::PunctuationBracket { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::PunctuationDelimiter { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::VariableParameter { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Label { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Keyword { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::VariableBuiltin { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::String { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::ConstantBuiltin { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Escape { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Attribute { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Operator { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::Type { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::TypeBuiltin { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Property { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Constant { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Constructor { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Function { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::FunctionMethod { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::FunctionMacro { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Comment { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::CommentDocumentation { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::PunctuationBracket { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::PunctuationDelimiter { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::VariableParameter { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Label { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Keyword { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::VariableBuiltin { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::String { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::ConstantBuiltin { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Escape { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Attribute { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Operator { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn name(&self) -> &'query str {
        match self {
            Self::Type { .. } => "type",
            Self::TypeBuiltin { .. } => "type.builtin",
            Self::Property { .. } => "property",
            Self::Constant { .. } => "constant",
            Self::Constructor { .. } => "constructor",
            Self::Function { .. } => "function",
            Self::FunctionMethod { .. } => "function.method",
            Self::FunctionMacro { .. } => "function.macro",
            Self::Comment { .. } => "comment",
            Self::CommentDocumentation { .. } => "comment.documentation",
            Self::PunctuationBracket { .. } => "punctuation.bracket",
            Self::PunctuationDelimiter { .. } => "punctuation.delimiter",
            Self::VariableParameter { .. } => "variable.parameter",
            Self::Label { .. } => "label",
            Self::Keyword { .. } => "keyword",
            Self::VariableBuiltin { .. } => "variable.builtin",
            Self::String { .. } => "string",
            Self::ConstantBuiltin { .. } => "constant.builtin",
            Self::Escape { .. } => "escape",
            Self::Attribute { .. } => "attribute",
            Self::Operator { .. } => "operator",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Type { .. } => 0usize,
            Self::TypeBuiltin { .. } => 1usize,
            Self::Property { .. } => 2usize,
            Self::Constant { .. } => 3usize,
            Self::Constructor { .. } => 4usize,
            Self::Function { .. } => 5usize,
            Self::FunctionMethod { .. } => 6usize,
            Self::FunctionMacro { .. } => 7usize,
            Self::Comment { .. } => 8usize,
            Self::CommentDocumentation { .. } => 9usize,
            Self::PunctuationBracket { .. } => 10usize,
            Self::PunctuationDelimiter { .. } => 11usize,
            Self::VariableParameter { .. } => 12usize,
            Self::Label { .. } => 13usize,
            Self::Keyword { .. } => 14usize,
            Self::VariableBuiltin { .. } => 15usize,
            Self::String { .. } => 16usize,
            Self::ConstantBuiltin { .. } => 17usize,
            Self::Escape { .. } => 18usize,
            Self::Attribute { .. } => 19usize,
            Self::Operator { .. } => 20usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
/**Typed version of the query:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Injections;
/**Matches returned by a query cursor running the query [`Injections`]:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[allow(unused, non_camel_case_types)]
pub type InjectionsMatches<'query, 'tree> = type_sitter_lib::QueryMatches<
    'query,
    'tree,
    Injections,
>;
/**Captures returned by a query cursor running the query [`Injections`]:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[allow(unused, non_camel_case_types)]
pub type InjectionsCaptures<'query, 'tree> = type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Injections,
>;
/**A match returned by the query [`Injections`]:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
#[repr(transparent)]
pub struct InjectionsMatch<'query, 'tree: 'query>(yak_sitter::QueryMatch<'query, 'tree>);
/**A capture returned by the query [`Injections`]:

```sexp
((macro_invocation
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

((macro_rule
  (token_tree) @injection.content)
 (#set! injection.language "rust")
 (#set! injection.include-children))

```*/
pub enum InjectionsCapture<'query, 'tree: 'query> {
    ///A `injection.content` ([`anon_unions::InjectionContent`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(token_tree) @injection.content
    ///(token_tree) @injection.content
    ///```
    InjectionContent {
        node: super::nodes::TokenTree<'tree>,
        r#match: Option<InjectionsMatch<'query, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::Query for Injections {
    type Match<'query, 'tree: 'query> = InjectionsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = InjectionsCapture<'query, 'tree>;
    fn as_str(&self) -> &'static str {
        "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n"
    }
    fn raw(&self) -> &'static yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Injections__: std::sync::OnceLock<yak_sitter::Query> = std::sync::OnceLock::new();
        __Injections__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = yak_sitter::Query::new(
                        &tree_sitter_rust::LANGUAGE.into(),
                        "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?",
                    );
                query
            })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: yak_sitter::QueryMatch<'query, 'tree>,
    ) -> InjectionsMatch<'query, 'tree> {
        InjectionsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: yak_sitter::QueryCapture<'query, 'tree>,
        r#match: Option<InjectionsMatch<'query, 'tree>>,
    ) -> InjectionsCapture<'query, 'tree> {
        match capture.index as usize {
            0usize => {
                InjectionsCapture::InjectionContent {
                    node: <super::nodes::TokenTree<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> InjectionsMatch<'query, 'tree> {
    ///Returns an iterator over the nodes captured by `injection.content` ([`anon_unions::InjectionContent`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(token_tree) @injection.content
    ///(token_tree) @injection.content
    ///```
    #[inline]
    pub fn injection_content(&self) -> super::nodes::TokenTree<'tree> {
        let result = {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::TokenTree<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
            .expect("one quantifier returned nothing");
        debug_assert!(
            { [0u32].into_iter().flat_map(| i | self.0.nodes_for_capture_index(i)).map(|
            n | unsafe { < super::nodes::TokenTree < 'tree > as type_sitter_lib::Node <
            'tree >> ::from_raw_unchecked(n) }) } .next().is_none(),
            "one quantifier returned more than one item"
        );
        result
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for InjectionsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(InjectionsMatch)).field(&self.0).finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> type_sitter_lib::QueryMatch<'query, 'tree>
for InjectionsMatch<'query, 'tree> {
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Injections
    }
    #[inline]
    fn tree(&self) -> &'tree yak_sitter::Tree {
        self.0.tree()
    }
    #[inline]
    fn raw(&self) -> &yak_sitter::QueryMatch<'query, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> yak_sitter::QueryMatch<'query, 'tree> {
        self.0
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> InjectionsCapture<'query, 'tree> {
    ///Try to interpret this capture as a `injection.content` ([`anon_unions::InjectionContent`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(token_tree) @injection.content
    ///(token_tree) @injection.content
    ///```
    #[inline]
    pub fn as_injection_content(&self) -> Option<&super::nodes::TokenTree<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::InjectionContent { node, .. } = self { Some(node) } else { None }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for InjectionsCapture<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InjectionContent { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(InjectionsCapture), "::",
                            stringify!(InjectionContent)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> Clone for InjectionsCapture<'query, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::InjectionContent { node, .. } => {
                Self::InjectionContent {
                    node: *node,
                    r#match: None,
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> type_sitter_lib::QueryCapture<'query, 'tree>
for InjectionsCapture<'query, 'tree> {
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Injections
    }
    #[inline]
    fn r#match(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::Query>::Match<'query, 'tree>> {
        match self {
            Self::InjectionContent { r#match, .. } => r#match.as_ref(),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::Query>::Match<'query, 'tree>> {
        match self {
            Self::InjectionContent { r#match, .. } => r#match,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn raw(&self) -> yak_sitter::QueryCapture<'query, 'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::InjectionContent { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 0usize,
                    name: "injection.content",
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::InjectionContent { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::InjectionContent { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn name(&self) -> &'query str {
        match self {
            Self::InjectionContent { .. } => "injection.content",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::InjectionContent { .. } => 0usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
/**Typed version of the query:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
/**Matches returned by a query cursor running the query [`Tags`]:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[allow(unused, non_camel_case_types)]
pub type TagsMatches<'query, 'tree> = type_sitter_lib::QueryMatches<'query, 'tree, Tags>;
/**Captures returned by a query cursor running the query [`Tags`]:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[allow(unused, non_camel_case_types)]
pub type TagsCaptures<'query, 'tree> = type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Tags,
>;
/**A match returned by the query [`Tags`]:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
#[repr(transparent)]
pub struct TagsMatch<'query, 'tree: 'query>(yak_sitter::QueryMatch<'query, 'tree>);
/**A capture returned by the query [`Tags`]:

```sexp
; ADT definitions

(struct_item
    name: (type_identifier) @name) @definition.class

(enum_item
    name: (type_identifier) @name) @definition.class

(union_item
    name: (type_identifier) @name) @definition.class

; type aliases

(type_item
    name: (type_identifier) @name) @definition.class

; method definitions

(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method

; function definitions

(function_item
    name: (identifier) @name) @definition.function

; trait definitions
(trait_item
    name: (type_identifier) @name) @definition.interface

; module definitions
(mod_item
    name: (identifier) @name) @definition.module

; macro definitions

(macro_definition
    name: (identifier) @name) @definition.macro

; references

(call_expression
    function: (identifier) @name) @reference.call

(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call

(macro_invocation
    macro: (identifier) @name) @reference.call

; implementations

(impl_item
    trait: (type_identifier) @name) @reference.implementation

(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation

```*/
pub enum TagsCapture<'query, 'tree: 'query> {
    ///A `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(field_identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///```
    Name { node: anon_unions::Name<'tree>, r#match: Option<TagsMatch<'query, 'tree>> },
    ///A `definition.class` ([`anon_unions::DefinitionClass`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.class*/
    /**(enum_item
    name: (type_identifier) @name) @definition.class*/
    /**(union_item
    name: (type_identifier) @name) @definition.class*/
    /**(type_item
    name: (type_identifier) @name) @definition.class*/
    ///```
    DefinitionClass {
        node: anon_unions::DefinitionClass<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `definition.method` ([`super::nodes::DeclarationList`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method*/
    ///```
    DefinitionMethod {
        node: super::nodes::DeclarationList<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `definition.function` ([`super::nodes::FunctionItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(function_item
    name: (identifier) @name) @definition.function*/
    ///```
    DefinitionFunction {
        node: super::nodes::FunctionItem<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `definition.interface` ([`super::nodes::TraitItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(trait_item
    name: (type_identifier) @name) @definition.interface*/
    ///```
    DefinitionInterface {
        node: super::nodes::TraitItem<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `definition.module` ([`super::nodes::ModItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(mod_item
    name: (identifier) @name) @definition.module*/
    ///```
    DefinitionModule {
        node: super::nodes::ModItem<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `definition.macro` ([`super::nodes::MacroDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(macro_definition
    name: (identifier) @name) @definition.macro*/
    ///```
    DefinitionMacro {
        node: super::nodes::MacroDefinition<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `reference.call` ([`anon_unions::ReferenceCall`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(call_expression
    function: (identifier) @name) @reference.call*/
    /**(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call*/
    /**(macro_invocation
    macro: (identifier) @name) @reference.call*/
    ///```
    ReferenceCall {
        node: anon_unions::ReferenceCall<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
    ///A `reference.implementation` ([`anon_unions::ReferenceImplementation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(impl_item
    trait: (type_identifier) @name) @reference.implementation*/
    /**(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation*/
    ///```
    ReferenceImplementation {
        node: super::nodes::ImplItem<'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::Query for Tags {
    type Match<'query, 'tree: 'query> = TagsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = TagsCapture<'query, 'tree>;
    fn as_str(&self) -> &'static str {
        "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n"
    }
    fn raw(&self) -> &'static yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Tags__: std::sync::OnceLock<yak_sitter::Query> = std::sync::OnceLock::new();
        __Tags__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = yak_sitter::Query::new(
                        &tree_sitter_rust::LANGUAGE.into(),
                        "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?",
                    );
                query
            })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: yak_sitter::QueryMatch<'query, 'tree>,
    ) -> TagsMatch<'query, 'tree> {
        TagsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: yak_sitter::QueryCapture<'query, 'tree>,
        r#match: Option<TagsMatch<'query, 'tree>>,
    ) -> TagsCapture<'query, 'tree> {
        match capture.index as usize {
            0usize => {
                TagsCapture::Name {
                    node: <anon_unions::Name<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            1usize => {
                TagsCapture::DefinitionClass {
                    node: <anon_unions::DefinitionClass<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            2usize => {
                TagsCapture::DefinitionMethod {
                    node: <super::nodes::DeclarationList<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            3usize => {
                TagsCapture::DefinitionFunction {
                    node: <super::nodes::FunctionItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            4usize => {
                TagsCapture::DefinitionInterface {
                    node: <super::nodes::TraitItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            5usize => {
                TagsCapture::DefinitionModule {
                    node: <super::nodes::ModItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            6usize => {
                TagsCapture::DefinitionMacro {
                    node: <super::nodes::MacroDefinition<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            7usize => {
                TagsCapture::ReferenceCall {
                    node: <anon_unions::ReferenceCall<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            8usize => {
                TagsCapture::ReferenceImplementation {
                    node: <super::nodes::ImplItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node),
                    r#match,
                }
            }
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> TagsMatch<'query, 'tree> {
    ///Returns an iterator over the nodes captured by `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(field_identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///```
    #[inline]
    pub fn name(&self) -> Option<anon_unions::Name<'tree>> {
        {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Name<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.class` ([`anon_unions::DefinitionClass`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.class*/
    /**(enum_item
    name: (type_identifier) @name) @definition.class*/
    /**(union_item
    name: (type_identifier) @name) @definition.class*/
    /**(type_item
    name: (type_identifier) @name) @definition.class*/
    ///```
    #[inline]
    pub fn definition_class(&self) -> Option<anon_unions::DefinitionClass<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::DefinitionClass<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.method` ([`super::nodes::DeclarationList`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method*/
    ///```
    #[inline]
    pub fn definition_method(&self) -> Option<super::nodes::DeclarationList<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::DeclarationList<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.function` ([`super::nodes::FunctionItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(function_item
    name: (identifier) @name) @definition.function*/
    ///```
    #[inline]
    pub fn definition_function(&self) -> Option<super::nodes::FunctionItem<'tree>> {
        {
            [3u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::FunctionItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.interface` ([`super::nodes::TraitItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(trait_item
    name: (type_identifier) @name) @definition.interface*/
    ///```
    #[inline]
    pub fn definition_interface(&self) -> Option<super::nodes::TraitItem<'tree>> {
        {
            [4u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::TraitItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.module` ([`super::nodes::ModItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(mod_item
    name: (identifier) @name) @definition.module*/
    ///```
    #[inline]
    pub fn definition_module(&self) -> Option<super::nodes::ModItem<'tree>> {
        {
            [5u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::ModItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.macro` ([`super::nodes::MacroDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(macro_definition
    name: (identifier) @name) @definition.macro*/
    ///```
    #[inline]
    pub fn definition_macro(&self) -> Option<super::nodes::MacroDefinition<'tree>> {
        {
            [6u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::MacroDefinition<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `reference.call` ([`anon_unions::ReferenceCall`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(call_expression
    function: (identifier) @name) @reference.call*/
    /**(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call*/
    /**(macro_invocation
    macro: (identifier) @name) @reference.call*/
    ///```
    #[inline]
    pub fn reference_call(&self) -> Option<anon_unions::ReferenceCall<'tree>> {
        {
            [7u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::ReferenceCall<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `reference.implementation` ([`anon_unions::ReferenceImplementation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(impl_item
    trait: (type_identifier) @name) @reference.implementation*/
    /**(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation*/
    ///```
    #[inline]
    pub fn reference_implementation(&self) -> Option<super::nodes::ImplItem<'tree>> {
        {
            [8u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::ImplItem<
                        'tree,
                    > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for TagsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(TagsMatch)).field(&self.0).finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> type_sitter_lib::QueryMatch<'query, 'tree>
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
    fn raw(&self) -> &yak_sitter::QueryMatch<'query, 'tree> {
        &self.0
    }
    #[inline]
    fn into_raw(self) -> yak_sitter::QueryMatch<'query, 'tree> {
        self.0
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> TagsCapture<'query, 'tree> {
    ///Try to interpret this capture as a `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(field_identifier) @name
    ///(identifier) @name
    ///(type_identifier) @name
    ///(type_identifier) @name
    ///```
    #[inline]
    pub fn as_name(&self) -> Option<&anon_unions::Name<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Name { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.class` ([`anon_unions::DefinitionClass`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(struct_item
    name: (type_identifier) @name) @definition.class*/
    /**(enum_item
    name: (type_identifier) @name) @definition.class*/
    /**(union_item
    name: (type_identifier) @name) @definition.class*/
    /**(type_item
    name: (type_identifier) @name) @definition.class*/
    ///```
    #[inline]
    pub fn as_definition_class(&self) -> Option<&anon_unions::DefinitionClass<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionClass { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.method` ([`super::nodes::DeclarationList`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(declaration_list
    (function_item
        name: (identifier) @name)) @definition.method*/
    ///```
    #[inline]
    pub fn as_definition_method(&self) -> Option<&super::nodes::DeclarationList<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionMethod { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.function` ([`super::nodes::FunctionItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(function_item
    name: (identifier) @name) @definition.function*/
    ///```
    #[inline]
    pub fn as_definition_function(&self) -> Option<&super::nodes::FunctionItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionFunction { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.interface` ([`super::nodes::TraitItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(trait_item
    name: (type_identifier) @name) @definition.interface*/
    ///```
    #[inline]
    pub fn as_definition_interface(&self) -> Option<&super::nodes::TraitItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionInterface { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.module` ([`super::nodes::ModItem`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(mod_item
    name: (identifier) @name) @definition.module*/
    ///```
    #[inline]
    pub fn as_definition_module(&self) -> Option<&super::nodes::ModItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionModule { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.macro` ([`super::nodes::MacroDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(macro_definition
    name: (identifier) @name) @definition.macro*/
    ///```
    #[inline]
    pub fn as_definition_macro(&self) -> Option<&super::nodes::MacroDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionMacro { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `reference.call` ([`anon_unions::ReferenceCall`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(call_expression
    function: (identifier) @name) @reference.call*/
    /**(call_expression
    function: (field_expression
        field: (field_identifier) @name)) @reference.call*/
    /**(macro_invocation
    macro: (identifier) @name) @reference.call*/
    ///```
    #[inline]
    pub fn as_reference_call(&self) -> Option<&anon_unions::ReferenceCall<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ReferenceCall { node, .. } = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `reference.implementation` ([`anon_unions::ReferenceImplementation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(impl_item
    trait: (type_identifier) @name) @reference.implementation*/
    /**(impl_item
    type: (type_identifier) @name
    !trait) @reference.implementation*/
    ///```
    #[inline]
    pub fn as_reference_implementation(&self) -> Option<&super::nodes::ImplItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ReferenceImplementation { node, .. } = self {
            Some(node)
        } else {
            None
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for TagsCapture<'query, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name { node, .. } => {
                f.debug_struct(concat!(stringify!(TagsCapture), "::", stringify!(Name)))
                    .field("node", node)
                    .finish()
            }
            Self::DefinitionClass { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::", stringify!(DefinitionClass)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::DefinitionMethod { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::", stringify!(DefinitionMethod)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::DefinitionFunction { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::", stringify!(DefinitionFunction)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::DefinitionInterface { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::",
                            stringify!(DefinitionInterface)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::DefinitionModule { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::", stringify!(DefinitionModule)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::DefinitionMacro { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::", stringify!(DefinitionMacro)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::ReferenceCall { node, .. } => {
                f.debug_struct(
                        concat!(stringify!(TagsCapture), "::", stringify!(ReferenceCall)),
                    )
                    .field("node", node)
                    .finish()
            }
            Self::ReferenceImplementation { node, .. } => {
                f.debug_struct(
                        concat!(
                            stringify!(TagsCapture), "::",
                            stringify!(ReferenceImplementation)
                        ),
                    )
                    .field("node", node)
                    .finish()
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> Clone for TagsCapture<'query, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::Name { node, .. } => {
                Self::Name {
                    node: *node,
                    r#match: None,
                }
            }
            Self::DefinitionClass { node, .. } => {
                Self::DefinitionClass {
                    node: *node,
                    r#match: None,
                }
            }
            Self::DefinitionMethod { node, .. } => {
                Self::DefinitionMethod {
                    node: *node,
                    r#match: None,
                }
            }
            Self::DefinitionFunction { node, .. } => {
                Self::DefinitionFunction {
                    node: *node,
                    r#match: None,
                }
            }
            Self::DefinitionInterface { node, .. } => {
                Self::DefinitionInterface {
                    node: *node,
                    r#match: None,
                }
            }
            Self::DefinitionModule { node, .. } => {
                Self::DefinitionModule {
                    node: *node,
                    r#match: None,
                }
            }
            Self::DefinitionMacro { node, .. } => {
                Self::DefinitionMacro {
                    node: *node,
                    r#match: None,
                }
            }
            Self::ReferenceCall { node, .. } => {
                Self::ReferenceCall {
                    node: *node,
                    r#match: None,
                }
            }
            Self::ReferenceImplementation { node, .. } => {
                Self::ReferenceImplementation {
                    node: *node,
                    r#match: None,
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> type_sitter_lib::QueryCapture<'query, 'tree>
for TagsCapture<'query, 'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Tags
    }
    #[inline]
    fn r#match(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::Query>::Match<'query, 'tree>> {
        match self {
            Self::Name { r#match, .. } => r#match.as_ref(),
            Self::DefinitionClass { r#match, .. } => r#match.as_ref(),
            Self::DefinitionMethod { r#match, .. } => r#match.as_ref(),
            Self::DefinitionFunction { r#match, .. } => r#match.as_ref(),
            Self::DefinitionInterface { r#match, .. } => r#match.as_ref(),
            Self::DefinitionModule { r#match, .. } => r#match.as_ref(),
            Self::DefinitionMacro { r#match, .. } => r#match.as_ref(),
            Self::ReferenceCall { r#match, .. } => r#match.as_ref(),
            Self::ReferenceImplementation { r#match, .. } => r#match.as_ref(),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::Query>::Match<'query, 'tree>> {
        match self {
            Self::Name { r#match, .. } => r#match,
            Self::DefinitionClass { r#match, .. } => r#match,
            Self::DefinitionMethod { r#match, .. } => r#match,
            Self::DefinitionFunction { r#match, .. } => r#match,
            Self::DefinitionInterface { r#match, .. } => r#match,
            Self::DefinitionModule { r#match, .. } => r#match,
            Self::DefinitionMacro { r#match, .. } => r#match,
            Self::ReferenceCall { r#match, .. } => r#match,
            Self::ReferenceImplementation { r#match, .. } => r#match,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn raw(&self) -> yak_sitter::QueryCapture<'query, 'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::Name { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 0usize,
                    name: "name",
                }
            }
            Self::DefinitionClass { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 1usize,
                    name: "definition.class",
                }
            }
            Self::DefinitionMethod { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 2usize,
                    name: "definition.method",
                }
            }
            Self::DefinitionFunction { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 3usize,
                    name: "definition.function",
                }
            }
            Self::DefinitionInterface { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 4usize,
                    name: "definition.interface",
                }
            }
            Self::DefinitionModule { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 5usize,
                    name: "definition.module",
                }
            }
            Self::DefinitionMacro { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 6usize,
                    name: "definition.macro",
                }
            }
            Self::ReferenceCall { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 7usize,
                    name: "reference.call",
                }
            }
            Self::ReferenceImplementation { node, .. } => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 8usize,
                    name: "reference.implementation",
                }
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::Name { node, .. } => type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::DefinitionClass { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionMethod { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionFunction { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionInterface { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionModule { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionMacro { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::ReferenceCall { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::ReferenceImplementation { node, .. } => {
                type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use type_sitter_lib::Node;
        match self {
            Self::Name { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionClass { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionMethod { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionFunction { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionInterface { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionModule { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionMacro { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::ReferenceCall { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::ReferenceImplementation { node, .. } => {
                type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
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
            Self::DefinitionMethod { .. } => "definition.method",
            Self::DefinitionFunction { .. } => "definition.function",
            Self::DefinitionInterface { .. } => "definition.interface",
            Self::DefinitionModule { .. } => "definition.module",
            Self::DefinitionMacro { .. } => "definition.macro",
            Self::ReferenceCall { .. } => "reference.call",
            Self::ReferenceImplementation { .. } => "reference.implementation",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Name { .. } => 0usize,
            Self::DefinitionClass { .. } => 1usize,
            Self::DefinitionMethod { .. } => 2usize,
            Self::DefinitionFunction { .. } => 3usize,
            Self::DefinitionInterface { .. } => 4usize,
            Self::DefinitionModule { .. } => 5usize,
            Self::DefinitionMacro { .. } => 6usize,
            Self::ReferenceCall { .. } => 7usize,
            Self::ReferenceImplementation { .. } => 8usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::super::nodes::*;
    /**One of `{identifier | type_identifier}`:
- [`Identifier`]
- [`TypeIdentifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Type<'tree> {
        type WithLifetime<'a> = Type<'a>;
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(
                            <TypeIdentifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => type_sitter_lib::Node::raw(x),
                Self::TypeIdentifier(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => type_sitter_lib::Node::raw_mut(x),
                Self::TypeIdentifier(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | type_identifier}`:
- [`Identifier`]
- [`TypeIdentifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Constructor<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Constructor<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Constructor<'tree> {
        type WithLifetime<'a> = Constructor<'a>;
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(
                            <TypeIdentifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => type_sitter_lib::Node::raw(x),
                Self::TypeIdentifier(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => type_sitter_lib::Node::raw_mut(x),
                Self::TypeIdentifier(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{! | identifier}`:
- [`symbols::Not`]
- [`Identifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FunctionMacro<'tree> {
        Not(symbols::Not<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FunctionMacro<'tree> {
        ///Returns the node if it is of type `!` ([`symbols::Not`]), otherwise returns `None`
        #[inline]
        pub fn as_not(self) -> Option<symbols::Not<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Not(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for FunctionMacro<'tree> {
        type WithLifetime<'a> = FunctionMacro<'a>;
        const KIND: &'static str = "{! | identifier}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "!" => {
                    Ok(unsafe {
                        Self::Not(
                            <symbols::Not<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Not(x) => type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Not(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Not(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{block_comment | line_comment}`:
- [`BlockComment`]
- [`LineComment`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comment<'tree> {
        BlockComment(BlockComment<'tree>),
        LineComment(LineComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Comment<'tree> {
        ///Returns the node if it is of type `block_comment` ([`BlockComment`]), otherwise returns `None`
        #[inline]
        pub fn as_block_comment(self) -> Option<BlockComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BlockComment(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `line_comment` ([`LineComment`]), otherwise returns `None`
        #[inline]
        pub fn as_line_comment(self) -> Option<LineComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LineComment(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Comment<'tree> {
        type WithLifetime<'a> = Comment<'a>;
        const KIND: &'static str = "{block_comment | line_comment}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "block_comment" => {
                    Ok(unsafe {
                        Self::BlockComment(
                            <BlockComment<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "line_comment" => {
                    Ok(unsafe {
                        Self::LineComment(
                            <LineComment<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::BlockComment(x) => type_sitter_lib::Node::raw(x),
                Self::LineComment(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::BlockComment(x) => type_sitter_lib::Node::raw_mut(x),
                Self::LineComment(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.into_raw(),
                Self::LineComment(x) => x.into_raw(),
            }
        }
    }
    /**One of `{block_comment | line_comment}`:
- [`BlockComment`]
- [`LineComment`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CommentDocumentation<'tree> {
        BlockComment(BlockComment<'tree>),
        LineComment(LineComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree> CommentDocumentation<'tree> {
        ///Returns the node if it is of type `block_comment` ([`BlockComment`]), otherwise returns `None`
        #[inline]
        pub fn as_block_comment(self) -> Option<BlockComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BlockComment(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `line_comment` ([`LineComment`]), otherwise returns `None`
        #[inline]
        pub fn as_line_comment(self) -> Option<LineComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LineComment(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for CommentDocumentation<'tree> {
        type WithLifetime<'a> = CommentDocumentation<'a>;
        const KIND: &'static str = "{block_comment | line_comment}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "block_comment" => {
                    Ok(unsafe {
                        Self::BlockComment(
                            <BlockComment<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "line_comment" => {
                    Ok(unsafe {
                        Self::LineComment(
                            <LineComment<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::BlockComment(x) => type_sitter_lib::Node::raw(x),
                Self::LineComment(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::BlockComment(x) => type_sitter_lib::Node::raw_mut(x),
                Self::LineComment(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.into_raw(),
                Self::LineComment(x) => x.into_raw(),
            }
        }
    }
    /**One of `{( | ) | < | > | [ | ] | { | }}`:
- [`symbols::LParen`]
- [`symbols::RParen`]
- [`symbols::Lt`]
- [`symbols::Gt`]
- [`symbols::LBracket`]
- [`symbols::RBracket`]
- [`symbols::LBrace`]
- [`symbols::RBrace`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PunctuationBracket<'tree> {
        LParen(symbols::LParen<'tree>),
        RParen(symbols::RParen<'tree>),
        Lt(symbols::Lt<'tree>),
        Gt(symbols::Gt<'tree>),
        LBracket(symbols::LBracket<'tree>),
        RBracket(symbols::RBracket<'tree>),
        LBrace(symbols::LBrace<'tree>),
        RBrace(symbols::RBrace<'tree>),
    }
    #[automatically_derived]
    impl<'tree> PunctuationBracket<'tree> {
        ///Returns the node if it is of type `(` ([`symbols::LParen`]), otherwise returns `None`
        #[inline]
        pub fn as_l_paren(self) -> Option<symbols::LParen<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LParen(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `)` ([`symbols::RParen`]), otherwise returns `None`
        #[inline]
        pub fn as_r_paren(self) -> Option<symbols::RParen<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RParen(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt(self) -> Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt(self) -> Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `[` ([`symbols::LBracket`]), otherwise returns `None`
        #[inline]
        pub fn as_l_bracket(self) -> Option<symbols::LBracket<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LBracket(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `]` ([`symbols::RBracket`]), otherwise returns `None`
        #[inline]
        pub fn as_r_bracket(self) -> Option<symbols::RBracket<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RBracket(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `{` ([`symbols::LBrace`]), otherwise returns `None`
        #[inline]
        pub fn as_l_brace(self) -> Option<symbols::LBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LBrace(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `}` ([`symbols::RBrace`]), otherwise returns `None`
        #[inline]
        pub fn as_r_brace(self) -> Option<symbols::RBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RBrace(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for PunctuationBracket<'tree> {
        type WithLifetime<'a> = PunctuationBracket<'a>;
        const KIND: &'static str = "{( | ) | < | > | [ | ] | { | }}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "(" => {
                    Ok(unsafe {
                        Self::LParen(
                            <symbols::LParen<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ")" => {
                    Ok(unsafe {
                        Self::RParen(
                            <symbols::RParen<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<" => {
                    Ok(unsafe {
                        Self::Lt(
                            <symbols::Lt<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">" => {
                    Ok(unsafe {
                        Self::Gt(
                            <symbols::Gt<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "[" => {
                    Ok(unsafe {
                        Self::LBracket(
                            <symbols::LBracket<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "]" => {
                    Ok(unsafe {
                        Self::RBracket(
                            <symbols::RBracket<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "{" => {
                    Ok(unsafe {
                        Self::LBrace(
                            <symbols::LBrace<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "}" => {
                    Ok(unsafe {
                        Self::RBrace(
                            <symbols::RBrace<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::LParen(x) => type_sitter_lib::Node::raw(x),
                Self::RParen(x) => type_sitter_lib::Node::raw(x),
                Self::Lt(x) => type_sitter_lib::Node::raw(x),
                Self::Gt(x) => type_sitter_lib::Node::raw(x),
                Self::LBracket(x) => type_sitter_lib::Node::raw(x),
                Self::RBracket(x) => type_sitter_lib::Node::raw(x),
                Self::LBrace(x) => type_sitter_lib::Node::raw(x),
                Self::RBrace(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::LParen(x) => type_sitter_lib::Node::raw_mut(x),
                Self::RParen(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Lt(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Gt(x) => type_sitter_lib::Node::raw_mut(x),
                Self::LBracket(x) => type_sitter_lib::Node::raw_mut(x),
                Self::RBracket(x) => type_sitter_lib::Node::raw_mut(x),
                Self::LBrace(x) => type_sitter_lib::Node::raw_mut(x),
                Self::RBrace(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::LParen(x) => x.into_raw(),
                Self::RParen(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::LBracket(x) => x.into_raw(),
                Self::RBracket(x) => x.into_raw(),
                Self::LBrace(x) => x.into_raw(),
                Self::RBrace(x) => x.into_raw(),
            }
        }
    }
    /**One of `{, | . | : | :: | ;}`:
- [`symbols::Comma`]
- [`symbols::Dot`]
- [`symbols::Colon`]
- [`symbols::ColonColon`]
- [`symbols::Semicolon`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PunctuationDelimiter<'tree> {
        Comma(symbols::Comma<'tree>),
        Dot(symbols::Dot<'tree>),
        Colon(symbols::Colon<'tree>),
        ColonColon(symbols::ColonColon<'tree>),
        Semicolon(symbols::Semicolon<'tree>),
    }
    #[automatically_derived]
    impl<'tree> PunctuationDelimiter<'tree> {
        ///Returns the node if it is of type `,` ([`symbols::Comma`]), otherwise returns `None`
        #[inline]
        pub fn as_comma(self) -> Option<symbols::Comma<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comma(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `.` ([`symbols::Dot`]), otherwise returns `None`
        #[inline]
        pub fn as_dot(self) -> Option<symbols::Dot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Dot(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `:` ([`symbols::Colon`]), otherwise returns `None`
        #[inline]
        pub fn as_colon(self) -> Option<symbols::Colon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Colon(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `::` ([`symbols::ColonColon`]), otherwise returns `None`
        #[inline]
        pub fn as_colon_colon(self) -> Option<symbols::ColonColon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ColonColon(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `;` ([`symbols::Semicolon`]), otherwise returns `None`
        #[inline]
        pub fn as_semicolon(self) -> Option<symbols::Semicolon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Semicolon(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for PunctuationDelimiter<'tree> {
        type WithLifetime<'a> = PunctuationDelimiter<'a>;
        const KIND: &'static str = "{, | . | : | :: | ;}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "," => {
                    Ok(unsafe {
                        Self::Comma(
                            <symbols::Comma<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "." => {
                    Ok(unsafe {
                        Self::Dot(
                            <symbols::Dot<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ":" => {
                    Ok(unsafe {
                        Self::Colon(
                            <symbols::Colon<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "::" => {
                    Ok(unsafe {
                        Self::ColonColon(
                            <symbols::ColonColon<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ";" => {
                    Ok(unsafe {
                        Self::Semicolon(
                            <symbols::Semicolon<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => type_sitter_lib::Node::raw(x),
                Self::Dot(x) => type_sitter_lib::Node::raw(x),
                Self::Colon(x) => type_sitter_lib::Node::raw(x),
                Self::ColonColon(x) => type_sitter_lib::Node::raw(x),
                Self::Semicolon(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Dot(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Colon(x) => type_sitter_lib::Node::raw_mut(x),
                Self::ColonColon(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Semicolon(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.into_raw(),
                Self::Dot(x) => x.into_raw(),
                Self::Colon(x) => x.into_raw(),
                Self::ColonColon(x) => x.into_raw(),
                Self::Semicolon(x) => x.into_raw(),
            }
        }
    }
    /**One of `{as | async | await | break | const | continue | crate | default | dyn | else | enum | extern | fn | for | if | impl | in | let | loop | macro_rules! | match | mod | move | mutable_specifier | pub | ref | return | self | static | struct | super | trait | type | union | unsafe | use | where | while | yield}`:
- [`unnamed::As`]
- [`unnamed::Async`]
- [`unnamed::Await`]
- [`unnamed::Break`]
- [`unnamed::Const`]
- [`unnamed::Continue`]
- [`Crate`]
- [`unnamed::Default`]
- [`unnamed::Dyn`]
- [`unnamed::Else`]
- [`unnamed::Enum`]
- [`unnamed::Extern`]
- [`unnamed::Fn`]
- [`unnamed::For`]
- [`unnamed::If`]
- [`unnamed::Impl`]
- [`unnamed::In`]
- [`unnamed::Let`]
- [`unnamed::Loop`]
- [`symbols::MacroRulesNot`]
- [`unnamed::Match`]
- [`unnamed::Mod`]
- [`unnamed::Move`]
- [`MutableSpecifier`]
- [`unnamed::Pub`]
- [`unnamed::Ref`]
- [`unnamed::Return`]
- [`Self_`]
- [`unnamed::Static`]
- [`unnamed::Struct`]
- [`Super`]
- [`unnamed::Trait`]
- [`unnamed::Type`]
- [`unnamed::Union`]
- [`unnamed::Unsafe`]
- [`unnamed::Use`]
- [`unnamed::Where`]
- [`unnamed::While`]
- [`unnamed::Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Keyword<'tree> {
        As(unnamed::As<'tree>),
        Async(unnamed::Async<'tree>),
        Await(unnamed::Await<'tree>),
        Break(unnamed::Break<'tree>),
        Const(unnamed::Const<'tree>),
        Continue(unnamed::Continue<'tree>),
        Crate(Crate<'tree>),
        Default(unnamed::Default<'tree>),
        Dyn(unnamed::Dyn<'tree>),
        Else(unnamed::Else<'tree>),
        Enum(unnamed::Enum<'tree>),
        Extern(unnamed::Extern<'tree>),
        Fn(unnamed::Fn<'tree>),
        For(unnamed::For<'tree>),
        If(unnamed::If<'tree>),
        Impl(unnamed::Impl<'tree>),
        In(unnamed::In<'tree>),
        Let(unnamed::Let<'tree>),
        Loop(unnamed::Loop<'tree>),
        MacroRulesNot(symbols::MacroRulesNot<'tree>),
        Match(unnamed::Match<'tree>),
        Mod(unnamed::Mod<'tree>),
        Move(unnamed::Move<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
        Pub(unnamed::Pub<'tree>),
        Ref(unnamed::Ref<'tree>),
        Return(unnamed::Return<'tree>),
        Self_(Self_<'tree>),
        Static(unnamed::Static<'tree>),
        Struct(unnamed::Struct<'tree>),
        Super(Super<'tree>),
        Trait(unnamed::Trait<'tree>),
        Type(unnamed::Type<'tree>),
        Union(unnamed::Union<'tree>),
        Unsafe(unnamed::Unsafe<'tree>),
        Use(unnamed::Use<'tree>),
        Where(unnamed::Where<'tree>),
        While(unnamed::While<'tree>),
        Yield(unnamed::Yield<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Keyword<'tree> {
        ///Returns the node if it is of type `as` ([`unnamed::As`]), otherwise returns `None`
        #[inline]
        pub fn as_as(self) -> Option<unnamed::As<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::As(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `async` ([`unnamed::Async`]), otherwise returns `None`
        #[inline]
        pub fn as_async(self) -> Option<unnamed::Async<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Async(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `await` ([`unnamed::Await`]), otherwise returns `None`
        #[inline]
        pub fn as_await(self) -> Option<unnamed::Await<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Await(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `break` ([`unnamed::Break`]), otherwise returns `None`
        #[inline]
        pub fn as_break(self) -> Option<unnamed::Break<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Break(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `const` ([`unnamed::Const`]), otherwise returns `None`
        #[inline]
        pub fn as_const(self) -> Option<unnamed::Const<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Const(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `continue` ([`unnamed::Continue`]), otherwise returns `None`
        #[inline]
        pub fn as_continue(self) -> Option<unnamed::Continue<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Continue(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `crate` ([`Crate`]), otherwise returns `None`
        #[inline]
        pub fn as_crate(self) -> Option<Crate<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Crate(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `default` ([`unnamed::Default`]), otherwise returns `None`
        #[inline]
        pub fn as_default(self) -> Option<unnamed::Default<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Default(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `dyn` ([`unnamed::Dyn`]), otherwise returns `None`
        #[inline]
        pub fn as_dyn(self) -> Option<unnamed::Dyn<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Dyn(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `else` ([`unnamed::Else`]), otherwise returns `None`
        #[inline]
        pub fn as_else(self) -> Option<unnamed::Else<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Else(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `enum` ([`unnamed::Enum`]), otherwise returns `None`
        #[inline]
        pub fn as_enum(self) -> Option<unnamed::Enum<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Enum(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `extern` ([`unnamed::Extern`]), otherwise returns `None`
        #[inline]
        pub fn as_extern(self) -> Option<unnamed::Extern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Extern(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `fn` ([`unnamed::Fn`]), otherwise returns `None`
        #[inline]
        pub fn as_fn(self) -> Option<unnamed::Fn<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fn(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `for` ([`unnamed::For`]), otherwise returns `None`
        #[inline]
        pub fn as_for(self) -> Option<unnamed::For<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::For(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `if` ([`unnamed::If`]), otherwise returns `None`
        #[inline]
        pub fn as_if(self) -> Option<unnamed::If<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::If(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `impl` ([`unnamed::Impl`]), otherwise returns `None`
        #[inline]
        pub fn as_impl(self) -> Option<unnamed::Impl<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Impl(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `in` ([`unnamed::In`]), otherwise returns `None`
        #[inline]
        pub fn as_in(self) -> Option<unnamed::In<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::In(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `let` ([`unnamed::Let`]), otherwise returns `None`
        #[inline]
        pub fn as_let(self) -> Option<unnamed::Let<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Let(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `loop` ([`unnamed::Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> Option<unnamed::Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `macro_rules!` ([`symbols::MacroRulesNot`]), otherwise returns `None`
        #[inline]
        pub fn as_macro_rules_not(self) -> Option<symbols::MacroRulesNot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MacroRulesNot(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `match` ([`unnamed::Match`]), otherwise returns `None`
        #[inline]
        pub fn as_match(self) -> Option<unnamed::Match<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Match(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `mod` ([`unnamed::Mod`]), otherwise returns `None`
        #[inline]
        pub fn as_mod_(self) -> Option<unnamed::Mod<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mod(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `move` ([`unnamed::Move`]), otherwise returns `None`
        #[inline]
        pub fn as_move(self) -> Option<unnamed::Move<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Move(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `mutable_specifier` ([`MutableSpecifier`]), otherwise returns `None`
        #[inline]
        pub fn as_mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MutableSpecifier(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `pub` ([`unnamed::Pub`]), otherwise returns `None`
        #[inline]
        pub fn as_pub(self) -> Option<unnamed::Pub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pub(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `ref` ([`unnamed::Ref`]), otherwise returns `None`
        #[inline]
        pub fn as_ref(self) -> Option<unnamed::Ref<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Ref(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `return` ([`unnamed::Return`]), otherwise returns `None`
        #[inline]
        pub fn as_return(self) -> Option<unnamed::Return<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Return(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `self` ([`Self_`]), otherwise returns `None`
        #[inline]
        pub fn as_self(self) -> Option<Self_<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Self_(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `static` ([`unnamed::Static`]), otherwise returns `None`
        #[inline]
        pub fn as_static(self) -> Option<unnamed::Static<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Static(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `struct` ([`unnamed::Struct`]), otherwise returns `None`
        #[inline]
        pub fn as_struct(self) -> Option<unnamed::Struct<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Struct(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `super` ([`Super`]), otherwise returns `None`
        #[inline]
        pub fn as_super(self) -> Option<Super<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Super(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `trait` ([`unnamed::Trait`]), otherwise returns `None`
        #[inline]
        pub fn as_trait(self) -> Option<unnamed::Trait<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Trait(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `type` ([`unnamed::Type`]), otherwise returns `None`
        #[inline]
        pub fn as_type_(self) -> Option<unnamed::Type<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Type(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `union` ([`unnamed::Union`]), otherwise returns `None`
        #[inline]
        pub fn as_union(self) -> Option<unnamed::Union<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Union(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `unsafe` ([`unnamed::Unsafe`]), otherwise returns `None`
        #[inline]
        pub fn as_unsafe(self) -> Option<unnamed::Unsafe<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Unsafe(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `use` ([`unnamed::Use`]), otherwise returns `None`
        #[inline]
        pub fn as_use(self) -> Option<unnamed::Use<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Use(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `where` ([`unnamed::Where`]), otherwise returns `None`
        #[inline]
        pub fn as_where(self) -> Option<unnamed::Where<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Where(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `while` ([`unnamed::While`]), otherwise returns `None`
        #[inline]
        pub fn as_while(self) -> Option<unnamed::While<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::While(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `yield` ([`unnamed::Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield(self) -> Option<unnamed::Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Keyword<'tree> {
        type WithLifetime<'a> = Keyword<'a>;
        const KIND: &'static str = "{as | async | await | break | const | continue | crate | default | dyn | else | enum | extern | fn | for | if | impl | in | let | loop | macro_rules! | match | mod | move | mutable_specifier | pub | ref | return | self | static | struct | super | trait | type | union | unsafe | use | where | while | yield}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "as" => {
                    Ok(unsafe {
                        Self::As(
                            <unnamed::As<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "async" => {
                    Ok(unsafe {
                        Self::Async(
                            <unnamed::Async<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "await" => {
                    Ok(unsafe {
                        Self::Await(
                            <unnamed::Await<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "break" => {
                    Ok(unsafe {
                        Self::Break(
                            <unnamed::Break<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "const" => {
                    Ok(unsafe {
                        Self::Const(
                            <unnamed::Const<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "continue" => {
                    Ok(unsafe {
                        Self::Continue(
                            <unnamed::Continue<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "crate" => {
                    Ok(unsafe {
                        Self::Crate(
                            <Crate<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "default" => {
                    Ok(unsafe {
                        Self::Default(
                            <unnamed::Default<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "dyn" => {
                    Ok(unsafe {
                        Self::Dyn(
                            <unnamed::Dyn<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "else" => {
                    Ok(unsafe {
                        Self::Else(
                            <unnamed::Else<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "enum" => {
                    Ok(unsafe {
                        Self::Enum(
                            <unnamed::Enum<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "extern" => {
                    Ok(unsafe {
                        Self::Extern(
                            <unnamed::Extern<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fn" => {
                    Ok(unsafe {
                        Self::Fn(
                            <unnamed::Fn<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "for" => {
                    Ok(unsafe {
                        Self::For(
                            <unnamed::For<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "if" => {
                    Ok(unsafe {
                        Self::If(
                            <unnamed::If<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "impl" => {
                    Ok(unsafe {
                        Self::Impl(
                            <unnamed::Impl<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "in" => {
                    Ok(unsafe {
                        Self::In(
                            <unnamed::In<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "let" => {
                    Ok(unsafe {
                        Self::Let(
                            <unnamed::Let<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <unnamed::Loop<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "macro_rules!" => {
                    Ok(unsafe {
                        Self::MacroRulesNot(
                            <symbols::MacroRulesNot<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "match" => {
                    Ok(unsafe {
                        Self::Match(
                            <unnamed::Match<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "mod" => {
                    Ok(unsafe {
                        Self::Mod(
                            <unnamed::Mod<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "move" => {
                    Ok(unsafe {
                        Self::Move(
                            <unnamed::Move<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "mutable_specifier" => {
                    Ok(unsafe {
                        Self::MutableSpecifier(
                            <MutableSpecifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "pub" => {
                    Ok(unsafe {
                        Self::Pub(
                            <unnamed::Pub<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "ref" => {
                    Ok(unsafe {
                        Self::Ref(
                            <unnamed::Ref<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "return" => {
                    Ok(unsafe {
                        Self::Return(
                            <unnamed::Return<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "self" => {
                    Ok(unsafe {
                        Self::Self_(
                            <Self_<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "static" => {
                    Ok(unsafe {
                        Self::Static(
                            <unnamed::Static<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "struct" => {
                    Ok(unsafe {
                        Self::Struct(
                            <unnamed::Struct<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "super" => {
                    Ok(unsafe {
                        Self::Super(
                            <Super<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "trait" => {
                    Ok(unsafe {
                        Self::Trait(
                            <unnamed::Trait<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type" => {
                    Ok(unsafe {
                        Self::Type(
                            <unnamed::Type<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "union" => {
                    Ok(unsafe {
                        Self::Union(
                            <unnamed::Union<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "unsafe" => {
                    Ok(unsafe {
                        Self::Unsafe(
                            <unnamed::Unsafe<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "use" => {
                    Ok(unsafe {
                        Self::Use(
                            <unnamed::Use<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "where" => {
                    Ok(unsafe {
                        Self::Where(
                            <unnamed::Where<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "while" => {
                    Ok(unsafe {
                        Self::While(
                            <unnamed::While<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "yield" => {
                    Ok(unsafe {
                        Self::Yield(
                            <unnamed::Yield<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::As(x) => type_sitter_lib::Node::raw(x),
                Self::Async(x) => type_sitter_lib::Node::raw(x),
                Self::Await(x) => type_sitter_lib::Node::raw(x),
                Self::Break(x) => type_sitter_lib::Node::raw(x),
                Self::Const(x) => type_sitter_lib::Node::raw(x),
                Self::Continue(x) => type_sitter_lib::Node::raw(x),
                Self::Crate(x) => type_sitter_lib::Node::raw(x),
                Self::Default(x) => type_sitter_lib::Node::raw(x),
                Self::Dyn(x) => type_sitter_lib::Node::raw(x),
                Self::Else(x) => type_sitter_lib::Node::raw(x),
                Self::Enum(x) => type_sitter_lib::Node::raw(x),
                Self::Extern(x) => type_sitter_lib::Node::raw(x),
                Self::Fn(x) => type_sitter_lib::Node::raw(x),
                Self::For(x) => type_sitter_lib::Node::raw(x),
                Self::If(x) => type_sitter_lib::Node::raw(x),
                Self::Impl(x) => type_sitter_lib::Node::raw(x),
                Self::In(x) => type_sitter_lib::Node::raw(x),
                Self::Let(x) => type_sitter_lib::Node::raw(x),
                Self::Loop(x) => type_sitter_lib::Node::raw(x),
                Self::MacroRulesNot(x) => type_sitter_lib::Node::raw(x),
                Self::Match(x) => type_sitter_lib::Node::raw(x),
                Self::Mod(x) => type_sitter_lib::Node::raw(x),
                Self::Move(x) => type_sitter_lib::Node::raw(x),
                Self::MutableSpecifier(x) => type_sitter_lib::Node::raw(x),
                Self::Pub(x) => type_sitter_lib::Node::raw(x),
                Self::Ref(x) => type_sitter_lib::Node::raw(x),
                Self::Return(x) => type_sitter_lib::Node::raw(x),
                Self::Self_(x) => type_sitter_lib::Node::raw(x),
                Self::Static(x) => type_sitter_lib::Node::raw(x),
                Self::Struct(x) => type_sitter_lib::Node::raw(x),
                Self::Super(x) => type_sitter_lib::Node::raw(x),
                Self::Trait(x) => type_sitter_lib::Node::raw(x),
                Self::Type(x) => type_sitter_lib::Node::raw(x),
                Self::Union(x) => type_sitter_lib::Node::raw(x),
                Self::Unsafe(x) => type_sitter_lib::Node::raw(x),
                Self::Use(x) => type_sitter_lib::Node::raw(x),
                Self::Where(x) => type_sitter_lib::Node::raw(x),
                Self::While(x) => type_sitter_lib::Node::raw(x),
                Self::Yield(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::As(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Async(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Await(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Break(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Const(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Continue(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Crate(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Default(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Dyn(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Else(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Enum(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Extern(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Fn(x) => type_sitter_lib::Node::raw_mut(x),
                Self::For(x) => type_sitter_lib::Node::raw_mut(x),
                Self::If(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Impl(x) => type_sitter_lib::Node::raw_mut(x),
                Self::In(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Let(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Loop(x) => type_sitter_lib::Node::raw_mut(x),
                Self::MacroRulesNot(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Match(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Mod(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Move(x) => type_sitter_lib::Node::raw_mut(x),
                Self::MutableSpecifier(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Pub(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Ref(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Return(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Self_(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Static(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Struct(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Super(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Trait(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Type(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Union(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Unsafe(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Use(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Where(x) => type_sitter_lib::Node::raw_mut(x),
                Self::While(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::As(x) => x.into_raw(),
                Self::Async(x) => x.into_raw(),
                Self::Await(x) => x.into_raw(),
                Self::Break(x) => x.into_raw(),
                Self::Const(x) => x.into_raw(),
                Self::Continue(x) => x.into_raw(),
                Self::Crate(x) => x.into_raw(),
                Self::Default(x) => x.into_raw(),
                Self::Dyn(x) => x.into_raw(),
                Self::Else(x) => x.into_raw(),
                Self::Enum(x) => x.into_raw(),
                Self::Extern(x) => x.into_raw(),
                Self::Fn(x) => x.into_raw(),
                Self::For(x) => x.into_raw(),
                Self::If(x) => x.into_raw(),
                Self::Impl(x) => x.into_raw(),
                Self::In(x) => x.into_raw(),
                Self::Let(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::MacroRulesNot(x) => x.into_raw(),
                Self::Match(x) => x.into_raw(),
                Self::Mod(x) => x.into_raw(),
                Self::Move(x) => x.into_raw(),
                Self::MutableSpecifier(x) => x.into_raw(),
                Self::Pub(x) => x.into_raw(),
                Self::Ref(x) => x.into_raw(),
                Self::Return(x) => x.into_raw(),
                Self::Self_(x) => x.into_raw(),
                Self::Static(x) => x.into_raw(),
                Self::Struct(x) => x.into_raw(),
                Self::Super(x) => x.into_raw(),
                Self::Trait(x) => x.into_raw(),
                Self::Type(x) => x.into_raw(),
                Self::Union(x) => x.into_raw(),
                Self::Unsafe(x) => x.into_raw(),
                Self::Use(x) => x.into_raw(),
                Self::Where(x) => x.into_raw(),
                Self::While(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{char_literal | raw_string_literal | string_literal}`:
- [`CharLiteral`]
- [`RawStringLiteral`]
- [`StringLiteral`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum String<'tree> {
        CharLiteral(CharLiteral<'tree>),
        RawStringLiteral(RawStringLiteral<'tree>),
        StringLiteral(StringLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> String<'tree> {
        ///Returns the node if it is of type `char_literal` ([`CharLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_char_literal(self) -> Option<CharLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CharLiteral(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `raw_string_literal` ([`RawStringLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RawStringLiteral(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `string_literal` ([`StringLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_string_literal(self) -> Option<StringLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringLiteral(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for String<'tree> {
        type WithLifetime<'a> = String<'a>;
        const KIND: &'static str = "{char_literal | raw_string_literal | string_literal}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "char_literal" => {
                    Ok(unsafe {
                        Self::CharLiteral(
                            <CharLiteral<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "raw_string_literal" => {
                    Ok(unsafe {
                        Self::RawStringLiteral(
                            <RawStringLiteral<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "string_literal" => {
                    Ok(unsafe {
                        Self::StringLiteral(
                            <StringLiteral<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::CharLiteral(x) => type_sitter_lib::Node::raw(x),
                Self::RawStringLiteral(x) => type_sitter_lib::Node::raw(x),
                Self::StringLiteral(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::CharLiteral(x) => type_sitter_lib::Node::raw_mut(x),
                Self::RawStringLiteral(x) => type_sitter_lib::Node::raw_mut(x),
                Self::StringLiteral(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::CharLiteral(x) => x.into_raw(),
                Self::RawStringLiteral(x) => x.into_raw(),
                Self::StringLiteral(x) => x.into_raw(),
            }
        }
    }
    /**One of `{boolean_literal | float_literal | integer_literal}`:
- [`BooleanLiteral`]
- [`FloatLiteral`]
- [`IntegerLiteral`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstantBuiltin<'tree> {
        BooleanLiteral(BooleanLiteral<'tree>),
        FloatLiteral(FloatLiteral<'tree>),
        IntegerLiteral(IntegerLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ConstantBuiltin<'tree> {
        ///Returns the node if it is of type `boolean_literal` ([`BooleanLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_boolean_literal(self) -> Option<BooleanLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BooleanLiteral(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `float_literal` ([`FloatLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_float_literal(self) -> Option<FloatLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FloatLiteral(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `integer_literal` ([`IntegerLiteral`]), otherwise returns `None`
        #[inline]
        pub fn as_integer_literal(self) -> Option<IntegerLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::IntegerLiteral(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for ConstantBuiltin<'tree> {
        type WithLifetime<'a> = ConstantBuiltin<'a>;
        const KIND: &'static str = "{boolean_literal | float_literal | integer_literal}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "boolean_literal" => {
                    Ok(unsafe {
                        Self::BooleanLiteral(
                            <BooleanLiteral<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "float_literal" => {
                    Ok(unsafe {
                        Self::FloatLiteral(
                            <FloatLiteral<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "integer_literal" => {
                    Ok(unsafe {
                        Self::IntegerLiteral(
                            <IntegerLiteral<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => type_sitter_lib::Node::raw(x),
                Self::FloatLiteral(x) => type_sitter_lib::Node::raw(x),
                Self::IntegerLiteral(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => type_sitter_lib::Node::raw_mut(x),
                Self::FloatLiteral(x) => type_sitter_lib::Node::raw_mut(x),
                Self::IntegerLiteral(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => x.into_raw(),
                Self::FloatLiteral(x) => x.into_raw(),
                Self::IntegerLiteral(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute_item | inner_attribute_item}`:
- [`AttributeItem`]
- [`InnerAttributeItem`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute<'tree> {
        AttributeItem(AttributeItem<'tree>),
        InnerAttributeItem(InnerAttributeItem<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Attribute<'tree> {
        ///Returns the node if it is of type `attribute_item` ([`AttributeItem`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute_item(self) -> Option<AttributeItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeItem(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `inner_attribute_item` ([`InnerAttributeItem`]), otherwise returns `None`
        #[inline]
        pub fn as_inner_attribute_item(self) -> Option<InnerAttributeItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::InnerAttributeItem(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Attribute<'tree> {
        type WithLifetime<'a> = Attribute<'a>;
        const KIND: &'static str = "{attribute_item | inner_attribute_item}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "attribute_item" => {
                    Ok(unsafe {
                        Self::AttributeItem(
                            <AttributeItem<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "inner_attribute_item" => {
                    Ok(unsafe {
                        Self::InnerAttributeItem(
                            <InnerAttributeItem<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => type_sitter_lib::Node::raw(x),
                Self::InnerAttributeItem(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => type_sitter_lib::Node::raw_mut(x),
                Self::InnerAttributeItem(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.into_raw(),
                Self::InnerAttributeItem(x) => x.into_raw(),
            }
        }
    }
    /**One of `{& | ' | *}`:
- [`symbols::And`]
- [`symbols::Quote`]
- [`symbols::Mul`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Operator<'tree> {
        And(symbols::And<'tree>),
        Quote(symbols::Quote<'tree>),
        Mul(symbols::Mul<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Operator<'tree> {
        ///Returns the node if it is of type `&` ([`symbols::And`]), otherwise returns `None`
        #[inline]
        pub fn as_and(self) -> Option<symbols::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `'` ([`symbols::Quote`]), otherwise returns `None`
        #[inline]
        pub fn as_quote(self) -> Option<symbols::Quote<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Quote(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `*` ([`symbols::Mul`]), otherwise returns `None`
        #[inline]
        pub fn as_mul(self) -> Option<symbols::Mul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mul(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Operator<'tree> {
        type WithLifetime<'a> = Operator<'a>;
        const KIND: &'static str = "{& | ' | *}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "&" => {
                    Ok(unsafe {
                        Self::And(
                            <symbols::And<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "'" => {
                    Ok(unsafe {
                        Self::Quote(
                            <symbols::Quote<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "*" => {
                    Ok(unsafe {
                        Self::Mul(
                            <symbols::Mul<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::And(x) => type_sitter_lib::Node::raw(x),
                Self::Quote(x) => type_sitter_lib::Node::raw(x),
                Self::Mul(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::And(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Quote(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Mul(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::And(x) => x.into_raw(),
                Self::Quote(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
            }
        }
    }
    /**One of `{field_identifier | identifier | type_identifier}`:
- [`FieldIdentifier`]
- [`Identifier`]
- [`TypeIdentifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Name<'tree> {
        FieldIdentifier(FieldIdentifier<'tree>),
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Name<'tree> {
        ///Returns the node if it is of type `field_identifier` ([`FieldIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FieldIdentifier(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for Name<'tree> {
        type WithLifetime<'a> = Name<'a>;
        const KIND: &'static str = "{field_identifier | identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "field_identifier" => {
                    Ok(unsafe {
                        Self::FieldIdentifier(
                            <FieldIdentifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(
                            <TypeIdentifier<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => type_sitter_lib::Node::raw(x),
                Self::TypeIdentifier(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => type_sitter_lib::Node::raw_mut(x),
                Self::TypeIdentifier(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{enum_item | struct_item | type_item | union_item}`:
- [`EnumItem`]
- [`StructItem`]
- [`TypeItem`]
- [`UnionItem`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefinitionClass<'tree> {
        EnumItem(EnumItem<'tree>),
        StructItem(StructItem<'tree>),
        TypeItem(TypeItem<'tree>),
        UnionItem(UnionItem<'tree>),
    }
    #[automatically_derived]
    impl<'tree> DefinitionClass<'tree> {
        ///Returns the node if it is of type `enum_item` ([`EnumItem`]), otherwise returns `None`
        #[inline]
        pub fn as_enum_item(self) -> Option<EnumItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EnumItem(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `struct_item` ([`StructItem`]), otherwise returns `None`
        #[inline]
        pub fn as_struct_item(self) -> Option<StructItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StructItem(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `type_item` ([`TypeItem`]), otherwise returns `None`
        #[inline]
        pub fn as_type_item(self) -> Option<TypeItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeItem(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `union_item` ([`UnionItem`]), otherwise returns `None`
        #[inline]
        pub fn as_union_item(self) -> Option<UnionItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionItem(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for DefinitionClass<'tree> {
        type WithLifetime<'a> = DefinitionClass<'a>;
        const KIND: &'static str = "{enum_item | struct_item | type_item | union_item}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "enum_item" => {
                    Ok(unsafe {
                        Self::EnumItem(
                            <EnumItem<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "struct_item" => {
                    Ok(unsafe {
                        Self::StructItem(
                            <StructItem<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_item" => {
                    Ok(unsafe {
                        Self::TypeItem(
                            <TypeItem<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "union_item" => {
                    Ok(unsafe {
                        Self::UnionItem(
                            <UnionItem<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::EnumItem(x) => type_sitter_lib::Node::raw(x),
                Self::StructItem(x) => type_sitter_lib::Node::raw(x),
                Self::TypeItem(x) => type_sitter_lib::Node::raw(x),
                Self::UnionItem(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::EnumItem(x) => type_sitter_lib::Node::raw_mut(x),
                Self::StructItem(x) => type_sitter_lib::Node::raw_mut(x),
                Self::TypeItem(x) => type_sitter_lib::Node::raw_mut(x),
                Self::UnionItem(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::EnumItem(x) => x.into_raw(),
                Self::StructItem(x) => x.into_raw(),
                Self::TypeItem(x) => x.into_raw(),
                Self::UnionItem(x) => x.into_raw(),
            }
        }
    }
    /**One of `{call_expression | macro_invocation}`:
- [`CallExpression`]
- [`MacroInvocation`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ReferenceCall<'tree> {
        CallExpression(CallExpression<'tree>),
        MacroInvocation(MacroInvocation<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ReferenceCall<'tree> {
        ///Returns the node if it is of type `call_expression` ([`CallExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_call_expression(self) -> Option<CallExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CallExpression(x) = self { Some(x) } else { None }
        }
        ///Returns the node if it is of type `macro_invocation` ([`MacroInvocation`]), otherwise returns `None`
        #[inline]
        pub fn as_macro_invocation(self) -> Option<MacroInvocation<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MacroInvocation(x) = self { Some(x) } else { None }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::Node<'tree> for ReferenceCall<'tree> {
        type WithLifetime<'a> = ReferenceCall<'a>;
        const KIND: &'static str = "{call_expression | macro_invocation}";
        #[inline]
        fn try_from_raw(
            node: yak_sitter::Node<'tree>,
        ) -> type_sitter_lib::NodeResult<Self> {
            match node.kind() {
                "call_expression" => {
                    Ok(unsafe {
                        Self::CallExpression(
                            <CallExpression<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "macro_invocation" => {
                    Ok(unsafe {
                        Self::MacroInvocation(
                            <MacroInvocation<
                                'tree,
                            > as type_sitter_lib::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::CallExpression(x) => type_sitter_lib::Node::raw(x),
                Self::MacroInvocation(x) => type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::CallExpression(x) => type_sitter_lib::Node::raw_mut(x),
                Self::MacroInvocation(x) => type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::CallExpression(x) => x.into_raw(),
                Self::MacroInvocation(x) => x.into_raw(),
            }
        }
    }
}
