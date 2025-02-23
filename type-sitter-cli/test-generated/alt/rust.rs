#[doc = "Typed version of the query:\n\n```sexp\n; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
#[doc = "Matches returned by a query cursor running the query [`Highlights`]:\n\n```sexp\n; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'query, 'tree, Text, I> =
    ::type_sitter::QueryMatches<'query, 'tree, Highlights, Text, I>;
#[doc = "Captures returned by a query cursor running the query [`Highlights`]:\n\n```sexp\n; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'query, 'tree, Text, I> =
    ::type_sitter::QueryCaptures<'query, 'tree, Highlights, Text, I>;
#[doc = "A match returned by the query [`Highlights`]:\n\n```sexp\n; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[repr(transparent)]
pub struct HighlightsMatch<'query, 'tree: 'query>(::type_sitter::raw::QueryMatch<'query, 'tree>);
#[doc = "A capture returned by the query [`Highlights`]:\n\n```sexp\n; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[derive(Clone, Debug)]
pub enum HighlightsCapture<'tree> {
    #[doc = "A `type` ([`anon_unions::Type`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "```"]
    Type(anon_unions::Type<'tree>),
    #[doc = "A `type.builtin` ([`super::nodes::PrimitiveType`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(primitive_type) @type.builtin"]
    #[doc = "```"]
    TypeBuiltin(super::nodes::PrimitiveType<'tree>),
    #[doc = "A `property` ([`super::nodes::FieldIdentifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @property"]
    #[doc = "```"]
    Property(super::nodes::FieldIdentifier<'tree>),
    #[doc = "A `constant` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    Constant(super::nodes::Identifier<'tree>),
    #[doc = "A `constructor` ([`anon_unions::Constructor`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "(type_identifier) @constructor"]
    #[doc = "```"]
    Constructor(anon_unions::Constructor<'tree>),
    #[doc = "A `function` ([`anon_unions::Function`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "```"]
    Function(super::nodes::Identifier<'tree>),
    #[doc = "A `function.method` ([`anon_unions::FunctionMethod`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "```"]
    FunctionMethod(super::nodes::FieldIdentifier<'tree>),
    #[doc = "A `function.macro` ([`anon_unions::FunctionMacro`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function.macro"]
    #[doc = "\"!\" @function.macro"]
    #[doc = "```"]
    FunctionMacro(anon_unions::FunctionMacro<'tree>),
    #[doc = "A `comment` ([`anon_unions::Comment`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment) @comment"]
    #[doc = "(block_comment) @comment"]
    #[doc = "```"]
    Comment(anon_unions::Comment<'tree>),
    #[doc = "A `comment.documentation` ([`anon_unions::CommentDocumentation`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment (doc_comment)) @comment.documentation"]
    #[doc = "(block_comment (doc_comment)) @comment.documentation"]
    #[doc = "```"]
    CommentDocumentation(anon_unions::CommentDocumentation<'tree>),
    #[doc = "A `punctuation.bracket` ([`anon_unions::PunctuationBracket`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"(\" @punctuation.bracket"]
    #[doc = "\")\" @punctuation.bracket"]
    #[doc = "\"[\" @punctuation.bracket"]
    #[doc = "\"]\" @punctuation.bracket"]
    #[doc = "\"{\" @punctuation.bracket"]
    #[doc = "\"}\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "```"]
    PunctuationBracket(anon_unions::PunctuationBracket<'tree>),
    #[doc = "A `punctuation.delimiter` ([`anon_unions::PunctuationDelimiter`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"::\" @punctuation.delimiter"]
    #[doc = "\":\" @punctuation.delimiter"]
    #[doc = "\".\" @punctuation.delimiter"]
    #[doc = "\",\" @punctuation.delimiter"]
    #[doc = "\";\" @punctuation.delimiter"]
    #[doc = "```"]
    PunctuationDelimiter(anon_unions::PunctuationDelimiter<'tree>),
    #[doc = "A `variable.parameter` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @variable.parameter"]
    #[doc = "```"]
    VariableParameter(super::nodes::Identifier<'tree>),
    #[doc = "A `label` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @label"]
    #[doc = "```"]
    Label(super::nodes::Identifier<'tree>),
    #[doc = "A `keyword` ([`anon_unions::Keyword`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"as\" @keyword"]
    #[doc = "\"async\" @keyword"]
    #[doc = "\"await\" @keyword"]
    #[doc = "\"break\" @keyword"]
    #[doc = "\"const\" @keyword"]
    #[doc = "\"continue\" @keyword"]
    #[doc = "\"default\" @keyword"]
    #[doc = "\"dyn\" @keyword"]
    #[doc = "\"else\" @keyword"]
    #[doc = "\"enum\" @keyword"]
    #[doc = "\"extern\" @keyword"]
    #[doc = "\"fn\" @keyword"]
    #[doc = "\"for\" @keyword"]
    #[doc = "\"gen\" @keyword"]
    #[doc = "\"if\" @keyword"]
    #[doc = "\"impl\" @keyword"]
    #[doc = "\"in\" @keyword"]
    #[doc = "\"let\" @keyword"]
    #[doc = "\"loop\" @keyword"]
    #[doc = "\"macro_rules!\" @keyword"]
    #[doc = "\"match\" @keyword"]
    #[doc = "\"mod\" @keyword"]
    #[doc = "\"move\" @keyword"]
    #[doc = "\"pub\" @keyword"]
    #[doc = "\"raw\" @keyword"]
    #[doc = "\"ref\" @keyword"]
    #[doc = "\"return\" @keyword"]
    #[doc = "\"static\" @keyword"]
    #[doc = "\"struct\" @keyword"]
    #[doc = "\"trait\" @keyword"]
    #[doc = "\"type\" @keyword"]
    #[doc = "\"union\" @keyword"]
    #[doc = "\"unsafe\" @keyword"]
    #[doc = "\"use\" @keyword"]
    #[doc = "\"where\" @keyword"]
    #[doc = "\"while\" @keyword"]
    #[doc = "\"yield\" @keyword"]
    #[doc = "(crate) @keyword"]
    #[doc = "(mutable_specifier) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(super) @keyword"]
    #[doc = "```"]
    Keyword(anon_unions::Keyword<'tree>),
    #[doc = "A `variable.builtin` ([`super::nodes::Self_`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(self) @variable.builtin"]
    #[doc = "```"]
    VariableBuiltin(super::nodes::Self_<'tree>),
    #[doc = "A `string` ([`anon_unions::String`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(char_literal) @string"]
    #[doc = "(string_literal) @string"]
    #[doc = "(raw_string_literal) @string"]
    #[doc = "```"]
    String(anon_unions::String<'tree>),
    #[doc = "A `constant.builtin` ([`anon_unions::ConstantBuiltin`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(boolean_literal) @constant.builtin"]
    #[doc = "(integer_literal) @constant.builtin"]
    #[doc = "(float_literal) @constant.builtin"]
    #[doc = "```"]
    ConstantBuiltin(anon_unions::ConstantBuiltin<'tree>),
    #[doc = "A `escape` ([`super::nodes::EscapeSequence`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    Escape(super::nodes::EscapeSequence<'tree>),
    #[doc = "A `attribute` ([`anon_unions::Attribute`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(attribute_item) @attribute"]
    #[doc = "(inner_attribute_item) @attribute"]
    #[doc = "```"]
    Attribute(anon_unions::Attribute<'tree>),
    #[doc = "A `operator` ([`anon_unions::Operator`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"*\" @operator"]
    #[doc = "\"&\" @operator"]
    #[doc = "\"'\" @operator"]
    #[doc = "```"]
    Operator(anon_unions::Operator<'tree>),
}
#[automatically_derived]
impl ::type_sitter::Query for Highlights {
    type Match<'query, 'tree: 'query> = HighlightsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = HighlightsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n"
    }
    fn raw(&self) -> &'static ::type_sitter::raw::Query {
        #[allow(non_upper_case_globals)]
        static __Highlights__: std::sync::OnceLock<::type_sitter::raw::Query> =
            std::sync::OnceLock::new();
        __Highlights__ . get_or_init (|| { # [allow (unused_mut)] let mut query = :: type_sitter :: raw :: Query :: new (& tree_sitter_rust :: LANGUAGE . into () , "; Identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n(line_comment) @comment\n(block_comment) @comment\n\n(line_comment (doc_comment)) @comment.documentation\n(block_comment (doc_comment)) @comment.documentation\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"gen\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"raw\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n\"yield\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ; query })
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
        match capture . index as usize { 0usize => HighlightsCapture :: Type (< anon_unions :: Type < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 1usize => HighlightsCapture :: TypeBuiltin (< super :: nodes :: PrimitiveType < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 2usize => HighlightsCapture :: Property (< super :: nodes :: FieldIdentifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 3usize => HighlightsCapture :: Constant (< super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 4usize => HighlightsCapture :: Constructor (< anon_unions :: Constructor < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 5usize => HighlightsCapture :: Function (< super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 6usize => HighlightsCapture :: FunctionMethod (< super :: nodes :: FieldIdentifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 7usize => HighlightsCapture :: FunctionMacro (< anon_unions :: FunctionMacro < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 8usize => HighlightsCapture :: Comment (< anon_unions :: Comment < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 9usize => HighlightsCapture :: CommentDocumentation (< anon_unions :: CommentDocumentation < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 10usize => HighlightsCapture :: PunctuationBracket (< anon_unions :: PunctuationBracket < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 11usize => HighlightsCapture :: PunctuationDelimiter (< anon_unions :: PunctuationDelimiter < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 12usize => HighlightsCapture :: VariableParameter (< super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 13usize => HighlightsCapture :: Label (< super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 14usize => HighlightsCapture :: Keyword (< anon_unions :: Keyword < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 15usize => HighlightsCapture :: VariableBuiltin (< super :: nodes :: Self_ < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 16usize => HighlightsCapture :: String (< anon_unions :: String < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 17usize => HighlightsCapture :: ConstantBuiltin (< anon_unions :: ConstantBuiltin < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 18usize => HighlightsCapture :: Escape (< super :: nodes :: EscapeSequence < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 19usize => HighlightsCapture :: Attribute (< anon_unions :: Attribute < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 20usize => HighlightsCapture :: Operator (< anon_unions :: Operator < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'query, 'tree: 'query> HighlightsMatch<'query, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `type` ([`anon_unions::Type`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "```"]
    #[inline]
    pub fn r#type(&self) -> ::std::option::Option<anon_unions::Type<'tree>> {
        {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Type<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(n)
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `type.builtin` ([`super::nodes::PrimitiveType`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(primitive_type) @type.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn type_builtin(&self) -> ::std::option::Option<super::nodes::PrimitiveType<'tree>> {
        { [1u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: PrimitiveType < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `property` ([`super::nodes::FieldIdentifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @property"]
    #[doc = "```"]
    #[inline]
    pub fn property(&self) -> ::std::option::Option<super::nodes::FieldIdentifier<'tree>> {
        { [2u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FieldIdentifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `constant` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    #[inline]
    pub fn constant(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        { [3u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `constructor` ([`anon_unions::Constructor`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "(type_identifier) @constructor"]
    #[doc = "```"]
    #[inline]
    pub fn constructor(&self) -> ::std::option::Option<anon_unions::Constructor<'tree>> {
        { [4u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Constructor < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `function` ([`anon_unions::Function`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "```"]
    #[inline]
    pub fn function(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        { [5u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `function.method` ([`anon_unions::FunctionMethod`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "```"]
    #[inline]
    pub fn function_method(&self) -> ::std::option::Option<super::nodes::FieldIdentifier<'tree>> {
        { [6u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FieldIdentifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `function.macro` ([`anon_unions::FunctionMacro`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function.macro"]
    #[doc = "\"!\" @function.macro"]
    #[doc = "```"]
    #[inline]
    pub fn function_macro(&self) -> ::std::option::Option<anon_unions::FunctionMacro<'tree>> {
        { [7u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: FunctionMacro < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `comment` ([`anon_unions::Comment`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment) @comment"]
    #[doc = "(block_comment) @comment"]
    #[doc = "```"]
    #[inline]
    pub fn comment(&self) -> ::std::option::Option<anon_unions::Comment<'tree>> {
        {
            [8u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Comment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        n,
                    )
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `comment.documentation` ([`anon_unions::CommentDocumentation`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment (doc_comment)) @comment.documentation"]
    #[doc = "(block_comment (doc_comment)) @comment.documentation"]
    #[doc = "```"]
    #[inline]
    pub fn comment_documentation(
        &self,
    ) -> ::std::option::Option<anon_unions::CommentDocumentation<'tree>> {
        { [9u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: CommentDocumentation < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `punctuation.bracket` ([`anon_unions::PunctuationBracket`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"(\" @punctuation.bracket"]
    #[doc = "\")\" @punctuation.bracket"]
    #[doc = "\"[\" @punctuation.bracket"]
    #[doc = "\"]\" @punctuation.bracket"]
    #[doc = "\"{\" @punctuation.bracket"]
    #[doc = "\"}\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "```"]
    #[inline]
    pub fn punctuation_bracket(
        &self,
    ) -> ::std::option::Option<anon_unions::PunctuationBracket<'tree>> {
        { [10u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: PunctuationBracket < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `punctuation.delimiter` ([`anon_unions::PunctuationDelimiter`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"::\" @punctuation.delimiter"]
    #[doc = "\":\" @punctuation.delimiter"]
    #[doc = "\".\" @punctuation.delimiter"]
    #[doc = "\",\" @punctuation.delimiter"]
    #[doc = "\";\" @punctuation.delimiter"]
    #[doc = "```"]
    #[inline]
    pub fn punctuation_delimiter(
        &self,
    ) -> ::std::option::Option<anon_unions::PunctuationDelimiter<'tree>> {
        { [11u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: PunctuationDelimiter < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `variable.parameter` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @variable.parameter"]
    #[doc = "```"]
    #[inline]
    pub fn variable_parameter(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        { [12u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `label` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @label"]
    #[doc = "```"]
    #[inline]
    pub fn label(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        { [13u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `keyword` ([`anon_unions::Keyword`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"as\" @keyword"]
    #[doc = "\"async\" @keyword"]
    #[doc = "\"await\" @keyword"]
    #[doc = "\"break\" @keyword"]
    #[doc = "\"const\" @keyword"]
    #[doc = "\"continue\" @keyword"]
    #[doc = "\"default\" @keyword"]
    #[doc = "\"dyn\" @keyword"]
    #[doc = "\"else\" @keyword"]
    #[doc = "\"enum\" @keyword"]
    #[doc = "\"extern\" @keyword"]
    #[doc = "\"fn\" @keyword"]
    #[doc = "\"for\" @keyword"]
    #[doc = "\"gen\" @keyword"]
    #[doc = "\"if\" @keyword"]
    #[doc = "\"impl\" @keyword"]
    #[doc = "\"in\" @keyword"]
    #[doc = "\"let\" @keyword"]
    #[doc = "\"loop\" @keyword"]
    #[doc = "\"macro_rules!\" @keyword"]
    #[doc = "\"match\" @keyword"]
    #[doc = "\"mod\" @keyword"]
    #[doc = "\"move\" @keyword"]
    #[doc = "\"pub\" @keyword"]
    #[doc = "\"raw\" @keyword"]
    #[doc = "\"ref\" @keyword"]
    #[doc = "\"return\" @keyword"]
    #[doc = "\"static\" @keyword"]
    #[doc = "\"struct\" @keyword"]
    #[doc = "\"trait\" @keyword"]
    #[doc = "\"type\" @keyword"]
    #[doc = "\"union\" @keyword"]
    #[doc = "\"unsafe\" @keyword"]
    #[doc = "\"use\" @keyword"]
    #[doc = "\"where\" @keyword"]
    #[doc = "\"while\" @keyword"]
    #[doc = "\"yield\" @keyword"]
    #[doc = "(crate) @keyword"]
    #[doc = "(mutable_specifier) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(super) @keyword"]
    #[doc = "```"]
    #[inline]
    pub fn keyword(&self) -> ::std::option::Option<anon_unions::Keyword<'tree>> {
        {
            [14u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Keyword<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        n,
                    )
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `variable.builtin` ([`super::nodes::Self_`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(self) @variable.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn variable_builtin(&self) -> ::std::option::Option<super::nodes::Self_<'tree>> {
        {
            [15u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Self_<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        n,
                    )
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `string` ([`anon_unions::String`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(char_literal) @string"]
    #[doc = "(string_literal) @string"]
    #[doc = "(raw_string_literal) @string"]
    #[doc = "```"]
    #[inline]
    pub fn string(&self) -> impl ::std::iter::Iterator<Item = anon_unions::String<'tree>> + '_ {
        {
            [16u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::String<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        n,
                    )
                })
        }
    }
    #[doc = "Returns an iterator over the nodes captured by `constant.builtin` ([`anon_unions::ConstantBuiltin`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(boolean_literal) @constant.builtin"]
    #[doc = "(integer_literal) @constant.builtin"]
    #[doc = "(float_literal) @constant.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn constant_builtin(&self) -> ::std::option::Option<anon_unions::ConstantBuiltin<'tree>> {
        { [17u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: ConstantBuiltin < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `escape` ([`super::nodes::EscapeSequence`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    #[inline]
    pub fn escape(&self) -> ::std::option::Option<super::nodes::EscapeSequence<'tree>> {
        { [18u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: EscapeSequence < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `attribute` ([`anon_unions::Attribute`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(attribute_item) @attribute"]
    #[doc = "(inner_attribute_item) @attribute"]
    #[doc = "```"]
    #[inline]
    pub fn attribute(&self) -> ::std::option::Option<anon_unions::Attribute<'tree>> {
        { [19u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Attribute < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `operator` ([`anon_unions::Operator`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"*\" @operator"]
    #[doc = "\"&\" @operator"]
    #[doc = "\"'\" @operator"]
    #[doc = "```"]
    #[inline]
    pub fn operator(&self) -> ::std::option::Option<anon_unions::Operator<'tree>> {
        { [20u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Operator < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
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
    #[doc = "Try to interpret this capture as a `type` ([`anon_unions::Type`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "```"]
    #[inline]
    pub fn as_type(&self) -> ::std::option::Option<&anon_unions::Type<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Type(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `type.builtin` ([`super::nodes::PrimitiveType`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(primitive_type) @type.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn as_type_builtin(&self) -> ::std::option::Option<&super::nodes::PrimitiveType<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypeBuiltin(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `property` ([`super::nodes::FieldIdentifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @property"]
    #[doc = "```"]
    #[inline]
    pub fn as_property(&self) -> ::std::option::Option<&super::nodes::FieldIdentifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Property(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `constant` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    #[inline]
    pub fn as_constant(&self) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constant(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `constructor` ([`anon_unions::Constructor`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "(type_identifier) @constructor"]
    #[doc = "```"]
    #[inline]
    pub fn as_constructor(&self) -> ::std::option::Option<&anon_unions::Constructor<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constructor(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `function` ([`anon_unions::Function`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "```"]
    #[inline]
    pub fn as_function(&self) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Function(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `function.method` ([`anon_unions::FunctionMethod`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "```"]
    #[inline]
    pub fn as_function_method(
        &self,
    ) -> ::std::option::Option<&super::nodes::FieldIdentifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionMethod(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `function.macro` ([`anon_unions::FunctionMacro`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function.macro"]
    #[doc = "\"!\" @function.macro"]
    #[doc = "```"]
    #[inline]
    pub fn as_function_macro(&self) -> ::std::option::Option<&anon_unions::FunctionMacro<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionMacro(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `comment` ([`anon_unions::Comment`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment) @comment"]
    #[doc = "(block_comment) @comment"]
    #[doc = "```"]
    #[inline]
    pub fn as_comment(&self) -> ::std::option::Option<&anon_unions::Comment<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Comment(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `comment.documentation` ([`anon_unions::CommentDocumentation`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment (doc_comment)) @comment.documentation"]
    #[doc = "(block_comment (doc_comment)) @comment.documentation"]
    #[doc = "```"]
    #[inline]
    pub fn as_comment_documentation(
        &self,
    ) -> ::std::option::Option<&anon_unions::CommentDocumentation<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::CommentDocumentation(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `punctuation.bracket` ([`anon_unions::PunctuationBracket`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"(\" @punctuation.bracket"]
    #[doc = "\")\" @punctuation.bracket"]
    #[doc = "\"[\" @punctuation.bracket"]
    #[doc = "\"]\" @punctuation.bracket"]
    #[doc = "\"{\" @punctuation.bracket"]
    #[doc = "\"}\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "```"]
    #[inline]
    pub fn as_punctuation_bracket(
        &self,
    ) -> ::std::option::Option<&anon_unions::PunctuationBracket<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PunctuationBracket(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `punctuation.delimiter` ([`anon_unions::PunctuationDelimiter`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"::\" @punctuation.delimiter"]
    #[doc = "\":\" @punctuation.delimiter"]
    #[doc = "\".\" @punctuation.delimiter"]
    #[doc = "\",\" @punctuation.delimiter"]
    #[doc = "\";\" @punctuation.delimiter"]
    #[doc = "```"]
    #[inline]
    pub fn as_punctuation_delimiter(
        &self,
    ) -> ::std::option::Option<&anon_unions::PunctuationDelimiter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PunctuationDelimiter(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `variable.parameter` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @variable.parameter"]
    #[doc = "```"]
    #[inline]
    pub fn as_variable_parameter(&self) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::VariableParameter(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `label` ([`super::nodes::Identifier`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @label"]
    #[doc = "```"]
    #[inline]
    pub fn as_label(&self) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Label(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `keyword` ([`anon_unions::Keyword`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"as\" @keyword"]
    #[doc = "\"async\" @keyword"]
    #[doc = "\"await\" @keyword"]
    #[doc = "\"break\" @keyword"]
    #[doc = "\"const\" @keyword"]
    #[doc = "\"continue\" @keyword"]
    #[doc = "\"default\" @keyword"]
    #[doc = "\"dyn\" @keyword"]
    #[doc = "\"else\" @keyword"]
    #[doc = "\"enum\" @keyword"]
    #[doc = "\"extern\" @keyword"]
    #[doc = "\"fn\" @keyword"]
    #[doc = "\"for\" @keyword"]
    #[doc = "\"gen\" @keyword"]
    #[doc = "\"if\" @keyword"]
    #[doc = "\"impl\" @keyword"]
    #[doc = "\"in\" @keyword"]
    #[doc = "\"let\" @keyword"]
    #[doc = "\"loop\" @keyword"]
    #[doc = "\"macro_rules!\" @keyword"]
    #[doc = "\"match\" @keyword"]
    #[doc = "\"mod\" @keyword"]
    #[doc = "\"move\" @keyword"]
    #[doc = "\"pub\" @keyword"]
    #[doc = "\"raw\" @keyword"]
    #[doc = "\"ref\" @keyword"]
    #[doc = "\"return\" @keyword"]
    #[doc = "\"static\" @keyword"]
    #[doc = "\"struct\" @keyword"]
    #[doc = "\"trait\" @keyword"]
    #[doc = "\"type\" @keyword"]
    #[doc = "\"union\" @keyword"]
    #[doc = "\"unsafe\" @keyword"]
    #[doc = "\"use\" @keyword"]
    #[doc = "\"where\" @keyword"]
    #[doc = "\"while\" @keyword"]
    #[doc = "\"yield\" @keyword"]
    #[doc = "(crate) @keyword"]
    #[doc = "(mutable_specifier) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(super) @keyword"]
    #[doc = "```"]
    #[inline]
    pub fn as_keyword(&self) -> ::std::option::Option<&anon_unions::Keyword<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Keyword(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `variable.builtin` ([`super::nodes::Self_`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(self) @variable.builtin"]
    #[doc = "```"]
    #[inline]
    pub fn as_variable_builtin(&self) -> ::std::option::Option<&super::nodes::Self_<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::VariableBuiltin(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `string` ([`anon_unions::String`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(char_literal) @string"]
    #[doc = "(string_literal) @string"]
    #[doc = "(raw_string_literal) @string"]
    #[doc = "```"]
    #[inline]
    pub fn as_string(&self) -> ::std::option::Option<&anon_unions::String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `constant.builtin` ([`anon_unions::ConstantBuiltin`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(boolean_literal) @constant.builtin"]
    #[doc = "(integer_literal) @constant.builtin"]
    #[doc = "(float_literal) @constant.builtin"]
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
    #[doc = "Try to interpret this capture as a `attribute` ([`anon_unions::Attribute`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(attribute_item) @attribute"]
    #[doc = "(inner_attribute_item) @attribute"]
    #[doc = "```"]
    #[inline]
    pub fn as_attribute(&self) -> ::std::option::Option<&anon_unions::Attribute<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Attribute(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `operator` ([`anon_unions::Operator`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"*\" @operator"]
    #[doc = "\"&\" @operator"]
    #[doc = "\"'\" @operator"]
    #[doc = "```"]
    #[inline]
    pub fn as_operator(&self) -> ::std::option::Option<&anon_unions::Operator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Operator(node) = self {
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
            Self::Type(node) => ::type_sitter::raw::QueryCapture {
                index: 0usize as u32,
                node: *node.raw(),
            },
            Self::TypeBuiltin(node) => ::type_sitter::raw::QueryCapture {
                index: 1usize as u32,
                node: *node.raw(),
            },
            Self::Property(node) => ::type_sitter::raw::QueryCapture {
                index: 2usize as u32,
                node: *node.raw(),
            },
            Self::Constant(node) => ::type_sitter::raw::QueryCapture {
                index: 3usize as u32,
                node: *node.raw(),
            },
            Self::Constructor(node) => ::type_sitter::raw::QueryCapture {
                index: 4usize as u32,
                node: *node.raw(),
            },
            Self::Function(node) => ::type_sitter::raw::QueryCapture {
                index: 5usize as u32,
                node: *node.raw(),
            },
            Self::FunctionMethod(node) => ::type_sitter::raw::QueryCapture {
                index: 6usize as u32,
                node: *node.raw(),
            },
            Self::FunctionMacro(node) => ::type_sitter::raw::QueryCapture {
                index: 7usize as u32,
                node: *node.raw(),
            },
            Self::Comment(node) => ::type_sitter::raw::QueryCapture {
                index: 8usize as u32,
                node: *node.raw(),
            },
            Self::CommentDocumentation(node) => ::type_sitter::raw::QueryCapture {
                index: 9usize as u32,
                node: *node.raw(),
            },
            Self::PunctuationBracket(node) => ::type_sitter::raw::QueryCapture {
                index: 10usize as u32,
                node: *node.raw(),
            },
            Self::PunctuationDelimiter(node) => ::type_sitter::raw::QueryCapture {
                index: 11usize as u32,
                node: *node.raw(),
            },
            Self::VariableParameter(node) => ::type_sitter::raw::QueryCapture {
                index: 12usize as u32,
                node: *node.raw(),
            },
            Self::Label(node) => ::type_sitter::raw::QueryCapture {
                index: 13usize as u32,
                node: *node.raw(),
            },
            Self::Keyword(node) => ::type_sitter::raw::QueryCapture {
                index: 14usize as u32,
                node: *node.raw(),
            },
            Self::VariableBuiltin(node) => ::type_sitter::raw::QueryCapture {
                index: 15usize as u32,
                node: *node.raw(),
            },
            Self::String(node) => ::type_sitter::raw::QueryCapture {
                index: 16usize as u32,
                node: *node.raw(),
            },
            Self::ConstantBuiltin(node) => ::type_sitter::raw::QueryCapture {
                index: 17usize as u32,
                node: *node.raw(),
            },
            Self::Escape(node) => ::type_sitter::raw::QueryCapture {
                index: 18usize as u32,
                node: *node.raw(),
            },
            Self::Attribute(node) => ::type_sitter::raw::QueryCapture {
                index: 19usize as u32,
                node: *node.raw(),
            },
            Self::Operator(node) => ::type_sitter::raw::QueryCapture {
                index: 20usize as u32,
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
            Self::Type(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::TypeBuiltin(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Property(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Constant(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Constructor(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Function(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::FunctionMethod(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::FunctionMacro(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Comment(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::CommentDocumentation(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::PunctuationBracket(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::PunctuationDelimiter(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::VariableParameter(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Label(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Keyword(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::VariableBuiltin(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::String(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::ConstantBuiltin(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Escape(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Attribute(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::Operator(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut ::type_sitter::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::Type(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::TypeBuiltin(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Property(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Constant(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Constructor(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Function(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::FunctionMethod(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::FunctionMacro(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Comment(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::CommentDocumentation(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::PunctuationBracket(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::PunctuationDelimiter(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::VariableParameter(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Label(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Keyword(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::VariableBuiltin(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::String(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::ConstantBuiltin(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Escape(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Attribute(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::Operator(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
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
#[doc = "Typed version of the query:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Injections;
#[doc = "Matches returned by a query cursor running the query [`Injections`]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type InjectionsMatches<'query, 'tree, Text, I> =
    ::type_sitter::QueryMatches<'query, 'tree, Injections, Text, I>;
#[doc = "Captures returned by a query cursor running the query [`Injections`]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type InjectionsCaptures<'query, 'tree, Text, I> =
    ::type_sitter::QueryCaptures<'query, 'tree, Injections, Text, I>;
#[doc = "A match returned by the query [`Injections`]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[repr(transparent)]
pub struct InjectionsMatch<'query, 'tree: 'query>(::type_sitter::raw::QueryMatch<'query, 'tree>);
#[doc = "A capture returned by the query [`Injections`]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[derive(Clone, Debug)]
pub enum InjectionsCapture<'tree> {
    #[doc = "A `injection.content` ([`anon_unions::InjectionContent`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "```"]
    InjectionContent(super::nodes::TokenTree<'tree>),
}
#[automatically_derived]
impl ::type_sitter::Query for Injections {
    type Match<'query, 'tree: 'query> = InjectionsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = InjectionsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n"
    }
    fn raw(&self) -> &'static ::type_sitter::raw::Query {
        #[allow(non_upper_case_globals)]
        static __Injections__: std::sync::OnceLock<::type_sitter::raw::Query> =
            std::sync::OnceLock::new();
        __Injections__ . get_or_init (|| { # [allow (unused_mut)] let mut query = :: type_sitter :: raw :: Query :: new (& tree_sitter_rust :: LANGUAGE . into () , "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ; query })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: ::type_sitter::raw::QueryMatch<'query, 'tree>,
    ) -> InjectionsMatch<'query, 'tree> {
        InjectionsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_match_ref<'m, 'query, 'tree>(
        &self,
        r#match: &'m ::type_sitter::raw::QueryMatch<'query, 'tree>,
    ) -> &'m InjectionsMatch<'query, 'tree> {
        &*(r#match as *const ::type_sitter::raw::QueryMatch<'query, 'tree>
            as *const InjectionsMatch<'query, 'tree>)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: ::type_sitter::raw::QueryCapture<'tree>,
    ) -> InjectionsCapture<'tree> {
        match capture.index as usize {
            0usize => InjectionsCapture::InjectionContent(
                <super::nodes::TokenTree<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                    capture.node,
                ),
            ),
            capture_index => unreachable!("Invalid capture index: {}", capture_index),
        }
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'query, 'tree: 'query> InjectionsMatch<'query, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `injection.content` ([`anon_unions::InjectionContent`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "```"]
    #[inline]
    pub fn injection_content(&self) -> super::nodes::TokenTree<'tree> {
        let result = { [0u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: TokenTree < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next () . expect ("one quantifier returned nothing") ;
        :: std :: debug_assert ! ({ [0u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: TokenTree < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next () . is_none () , "one quantifier returned more than one item");
        result
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for InjectionsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple(stringify!(InjectionsMatch))
            .field(&self.0)
            .finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter::QueryMatch<'query, 'tree>
    for InjectionsMatch<'query, 'tree>
{
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Injections
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
impl<'tree> InjectionsCapture<'tree> {
    #[doc = "Try to interpret this capture as a `injection.content` ([`anon_unions::InjectionContent`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "```"]
    #[inline]
    pub fn as_injection_content(&self) -> ::std::option::Option<&super::nodes::TokenTree<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::InjectionContent(node) = self {
            Some(node)
        } else {
            None
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter::QueryCapture<'query, 'tree>
    for InjectionsCapture<'tree>
{
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Injections
    }
    #[inline]
    fn raw(&self) -> ::type_sitter::raw::QueryCapture<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::InjectionContent(node) => ::type_sitter::raw::QueryCapture {
                index: 0usize as u32,
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
            Self::InjectionContent(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut ::type_sitter::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::InjectionContent(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
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
#[doc = "Typed version of the query:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
#[doc = "Matches returned by a query cursor running the query [`Tags`]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type TagsMatches<'query, 'tree, Text, I> =
    ::type_sitter::QueryMatches<'query, 'tree, Tags, Text, I>;
#[doc = "Captures returned by a query cursor running the query [`Tags`]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type TagsCaptures<'query, 'tree, Text, I> =
    ::type_sitter::QueryCaptures<'query, 'tree, Tags, Text, I>;
#[doc = "A match returned by the query [`Tags`]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[repr(transparent)]
pub struct TagsMatch<'query, 'tree: 'query>(::type_sitter::raw::QueryMatch<'query, 'tree>);
#[doc = "A capture returned by the query [`Tags`]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[derive(Clone, Debug)]
pub enum TagsCapture<'tree> {
    #[doc = "A `name` ([`anon_unions::Name`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(field_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "```"]
    Name(anon_unions::Name<'tree>),
    #[doc = "A `definition.class` ([`anon_unions::DefinitionClass`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(enum_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(union_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(type_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    DefinitionClass(anon_unions::DefinitionClass<'tree>),
    #[doc = "A `definition.method` ([`super::nodes::FunctionItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n        name: (identifier) @name) @definition.method"]
    #[doc = "```"]
    DefinitionMethod(super::nodes::FunctionItem<'tree>),
    #[doc = "A `definition.function` ([`super::nodes::FunctionItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n    name: (identifier) @name) @definition.function"]
    #[doc = "```"]
    DefinitionFunction(super::nodes::FunctionItem<'tree>),
    #[doc = "A `definition.interface` ([`super::nodes::TraitItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(trait_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    DefinitionInterface(super::nodes::TraitItem<'tree>),
    #[doc = "A `definition.module` ([`super::nodes::ModItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(mod_item\n    name: (identifier) @name) @definition.module"]
    #[doc = "```"]
    DefinitionModule(super::nodes::ModItem<'tree>),
    #[doc = "A `definition.macro` ([`super::nodes::MacroDefinition`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(macro_definition\n    name: (identifier) @name) @definition.macro"]
    #[doc = "```"]
    DefinitionMacro(super::nodes::MacroDefinition<'tree>),
    #[doc = "A `reference.call` ([`anon_unions::ReferenceCall`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(call_expression\n    function: (identifier) @name) @reference.call"]
    #[doc = "(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call"]
    #[doc = "(macro_invocation\n    macro: (identifier) @name) @reference.call"]
    #[doc = "```"]
    ReferenceCall(anon_unions::ReferenceCall<'tree>),
    #[doc = "A `reference.implementation` ([`anon_unions::ReferenceImplementation`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(impl_item\n    trait: (type_identifier) @name) @reference.implementation"]
    #[doc = "(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation"]
    #[doc = "```"]
    ReferenceImplementation(super::nodes::ImplItem<'tree>),
}
#[automatically_derived]
impl ::type_sitter::Query for Tags {
    type Match<'query, 'tree: 'query> = TagsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = TagsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n"
    }
    fn raw(&self) -> &'static ::type_sitter::raw::Query {
        #[allow(non_upper_case_globals)]
        static __Tags__: std::sync::OnceLock<::type_sitter::raw::Query> =
            std::sync::OnceLock::new();
        __Tags__ . get_or_init (|| { # [allow (unused_mut)] let mut query = :: type_sitter :: raw :: Query :: new (& tree_sitter_rust :: LANGUAGE . into () , "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name) @definition.method)\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ; query })
    }
    #[inline]
    unsafe fn wrap_match<'query, 'tree>(
        &self,
        r#match: ::type_sitter::raw::QueryMatch<'query, 'tree>,
    ) -> TagsMatch<'query, 'tree> {
        TagsMatch(r#match)
    }
    #[inline]
    unsafe fn wrap_match_ref<'m, 'query, 'tree>(
        &self,
        r#match: &'m ::type_sitter::raw::QueryMatch<'query, 'tree>,
    ) -> &'m TagsMatch<'query, 'tree> {
        &*(r#match as *const ::type_sitter::raw::QueryMatch<'query, 'tree>
            as *const TagsMatch<'query, 'tree>)
    }
    #[inline]
    unsafe fn wrap_capture<'query, 'tree: 'query>(
        &self,
        capture: ::type_sitter::raw::QueryCapture<'tree>,
    ) -> TagsCapture<'tree> {
        match capture . index as usize { 0usize => TagsCapture :: Name (< anon_unions :: Name < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 1usize => TagsCapture :: DefinitionClass (< anon_unions :: DefinitionClass < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 2usize => TagsCapture :: DefinitionMethod (< super :: nodes :: FunctionItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 3usize => TagsCapture :: DefinitionFunction (< super :: nodes :: FunctionItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 4usize => TagsCapture :: DefinitionInterface (< super :: nodes :: TraitItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 5usize => TagsCapture :: DefinitionModule (< super :: nodes :: ModItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 6usize => TagsCapture :: DefinitionMacro (< super :: nodes :: MacroDefinition < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 7usize => TagsCapture :: ReferenceCall (< anon_unions :: ReferenceCall < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , 8usize => TagsCapture :: ReferenceImplementation (< super :: nodes :: ImplItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (capture . node)) , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
#[allow(unused)]
impl<'query, 'tree: 'query> TagsMatch<'query, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `name` ([`anon_unions::Name`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(field_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "```"]
    #[inline]
    pub fn name(&self) -> ::std::option::Option<anon_unions::Name<'tree>> {
        {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::Name<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(n)
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.class` ([`anon_unions::DefinitionClass`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(enum_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(union_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(type_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    #[inline]
    pub fn definition_class(&self) -> ::std::option::Option<anon_unions::DefinitionClass<'tree>> {
        { [1u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: DefinitionClass < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.method` ([`super::nodes::FunctionItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n        name: (identifier) @name) @definition.method"]
    #[doc = "```"]
    #[inline]
    pub fn definition_method(&self) -> ::std::option::Option<super::nodes::FunctionItem<'tree>> {
        { [2u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FunctionItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.function` ([`super::nodes::FunctionItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n    name: (identifier) @name) @definition.function"]
    #[doc = "```"]
    #[inline]
    pub fn definition_function(&self) -> ::std::option::Option<super::nodes::FunctionItem<'tree>> {
        { [3u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FunctionItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.interface` ([`super::nodes::TraitItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(trait_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    #[inline]
    pub fn definition_interface(&self) -> ::std::option::Option<super::nodes::TraitItem<'tree>> {
        { [4u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: TraitItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.module` ([`super::nodes::ModItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(mod_item\n    name: (identifier) @name) @definition.module"]
    #[doc = "```"]
    #[inline]
    pub fn definition_module(&self) -> ::std::option::Option<super::nodes::ModItem<'tree>> {
        { [5u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: ModItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.macro` ([`super::nodes::MacroDefinition`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(macro_definition\n    name: (identifier) @name) @definition.macro"]
    #[doc = "```"]
    #[inline]
    pub fn definition_macro(&self) -> ::std::option::Option<super::nodes::MacroDefinition<'tree>> {
        { [6u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: MacroDefinition < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `reference.call` ([`anon_unions::ReferenceCall`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(call_expression\n    function: (identifier) @name) @reference.call"]
    #[doc = "(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call"]
    #[doc = "(macro_invocation\n    macro: (identifier) @name) @reference.call"]
    #[doc = "```"]
    #[inline]
    pub fn reference_call(&self) -> ::std::option::Option<anon_unions::ReferenceCall<'tree>> {
        { [7u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: ReferenceCall < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `reference.implementation` ([`anon_unions::ReferenceImplementation`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(impl_item\n    trait: (type_identifier) @name) @reference.implementation"]
    #[doc = "(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation"]
    #[doc = "```"]
    #[inline]
    pub fn reference_implementation(&self) -> ::std::option::Option<super::nodes::ImplItem<'tree>> {
        { [8u32] . into_iter () . flat_map (| i | self . 0 . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: ImplItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (n) }) } . next ()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> std::fmt::Debug for TagsMatch<'query, 'tree> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple(stringify!(TagsMatch)).field(&self.0).finish()
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter::QueryMatch<'query, 'tree> for TagsMatch<'query, 'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Tags
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
impl<'tree> TagsCapture<'tree> {
    #[doc = "Try to interpret this capture as a `name` ([`anon_unions::Name`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(field_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "```"]
    #[inline]
    pub fn as_name(&self) -> ::std::option::Option<&anon_unions::Name<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Name(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `definition.class` ([`anon_unions::DefinitionClass`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(enum_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(union_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(type_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    #[inline]
    pub fn as_definition_class(
        &self,
    ) -> ::std::option::Option<&anon_unions::DefinitionClass<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionClass(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `definition.method` ([`super::nodes::FunctionItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n        name: (identifier) @name) @definition.method"]
    #[doc = "```"]
    #[inline]
    pub fn as_definition_method(
        &self,
    ) -> ::std::option::Option<&super::nodes::FunctionItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionMethod(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `definition.function` ([`super::nodes::FunctionItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n    name: (identifier) @name) @definition.function"]
    #[doc = "```"]
    #[inline]
    pub fn as_definition_function(
        &self,
    ) -> ::std::option::Option<&super::nodes::FunctionItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionFunction(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `definition.interface` ([`super::nodes::TraitItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(trait_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    #[inline]
    pub fn as_definition_interface(
        &self,
    ) -> ::std::option::Option<&super::nodes::TraitItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionInterface(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `definition.module` ([`super::nodes::ModItem`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(mod_item\n    name: (identifier) @name) @definition.module"]
    #[doc = "```"]
    #[inline]
    pub fn as_definition_module(&self) -> ::std::option::Option<&super::nodes::ModItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionModule(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `definition.macro` ([`super::nodes::MacroDefinition`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(macro_definition\n    name: (identifier) @name) @definition.macro"]
    #[doc = "```"]
    #[inline]
    pub fn as_definition_macro(
        &self,
    ) -> ::std::option::Option<&super::nodes::MacroDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionMacro(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `reference.call` ([`anon_unions::ReferenceCall`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(call_expression\n    function: (identifier) @name) @reference.call"]
    #[doc = "(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call"]
    #[doc = "(macro_invocation\n    macro: (identifier) @name) @reference.call"]
    #[doc = "```"]
    #[inline]
    pub fn as_reference_call(&self) -> ::std::option::Option<&anon_unions::ReferenceCall<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ReferenceCall(node) = self {
            Some(node)
        } else {
            None
        }
    }
    #[doc = "Try to interpret this capture as a `reference.implementation` ([`anon_unions::ReferenceImplementation`])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(impl_item\n    trait: (type_identifier) @name) @reference.implementation"]
    #[doc = "(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation"]
    #[doc = "```"]
    #[inline]
    pub fn as_reference_implementation(
        &self,
    ) -> ::std::option::Option<&super::nodes::ImplItem<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ReferenceImplementation(node) = self {
            Some(node)
        } else {
            None
        }
    }
}
#[automatically_derived]
impl<'query, 'tree: 'query> ::type_sitter::QueryCapture<'query, 'tree> for TagsCapture<'tree> {
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'query Self::Query {
        &Tags
    }
    #[inline]
    fn raw(&self) -> ::type_sitter::raw::QueryCapture<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::Name(node) => ::type_sitter::raw::QueryCapture {
                index: 0usize as u32,
                node: *node.raw(),
            },
            Self::DefinitionClass(node) => ::type_sitter::raw::QueryCapture {
                index: 1usize as u32,
                node: *node.raw(),
            },
            Self::DefinitionMethod(node) => ::type_sitter::raw::QueryCapture {
                index: 2usize as u32,
                node: *node.raw(),
            },
            Self::DefinitionFunction(node) => ::type_sitter::raw::QueryCapture {
                index: 3usize as u32,
                node: *node.raw(),
            },
            Self::DefinitionInterface(node) => ::type_sitter::raw::QueryCapture {
                index: 4usize as u32,
                node: *node.raw(),
            },
            Self::DefinitionModule(node) => ::type_sitter::raw::QueryCapture {
                index: 5usize as u32,
                node: *node.raw(),
            },
            Self::DefinitionMacro(node) => ::type_sitter::raw::QueryCapture {
                index: 6usize as u32,
                node: *node.raw(),
            },
            Self::ReferenceCall(node) => ::type_sitter::raw::QueryCapture {
                index: 7usize as u32,
                node: *node.raw(),
            },
            Self::ReferenceImplementation(node) => ::type_sitter::raw::QueryCapture {
                index: 8usize as u32,
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
            Self::Name(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::DefinitionClass(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::DefinitionMethod(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::DefinitionFunction(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::DefinitionInterface(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::DefinitionModule(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::DefinitionMacro(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::ReferenceCall(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            Self::ReferenceImplementation(node) => ::type_sitter::UntypedNode::r#ref(node.raw()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut ::type_sitter::UntypedNode<'tree> {
        #[allow(unused_imports)]
        use ::type_sitter::Node;
        match self {
            Self::Name(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionClass(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionMethod(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionFunction(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionInterface(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionModule(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::DefinitionMacro(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::ReferenceCall(node) => ::type_sitter::UntypedNode::r#mut(node.raw_mut()),
            Self::ReferenceImplementation(node) => {
                ::type_sitter::UntypedNode::r#mut(node.raw_mut())
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
    #[doc = "One of `{attribute_item | inner_attribute_item}`:\n- [`AttributeItem`]\n- [`InnerAttributeItem`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute<'tree> {
        AttributeItem(AttributeItem<'tree>),
        InnerAttributeItem(InnerAttributeItem<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Attribute<'tree> {
        #[doc = "Returns the node if it is of type `attribute_item` ([`AttributeItem`]), otherwise returns `None`"]
        #[inline]
        pub fn as_attribute_item(self) -> ::std::option::Option<AttributeItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeItem(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `inner_attribute_item` ([`InnerAttributeItem`]), otherwise returns `None`"]
        #[inline]
        pub fn as_inner_attribute_item(self) -> ::std::option::Option<InnerAttributeItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::InnerAttributeItem(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Attribute<'tree> {
        type WithLifetime<'a> = Attribute<'a>;
        const KIND: &'static str = "{attribute_item | inner_attribute_item}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "attribute_item" => {
                    Ok(unsafe {
                        Self :: AttributeItem (< AttributeItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "inner_attribute_item" => {
                    Ok(unsafe {
                        Self :: InnerAttributeItem (< InnerAttributeItem < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeItem(x) => ::type_sitter::Node::raw(x),
                Self::InnerAttributeItem(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeItem(x) => ::type_sitter::Node::raw_mut(x),
                Self::InnerAttributeItem(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.into_raw(),
                Self::InnerAttributeItem(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{block_comment | line_comment}`:\n- [`BlockComment`]\n- [`LineComment`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comment<'tree> {
        BlockComment(BlockComment<'tree>),
        LineComment(LineComment<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comment<'tree> {
        #[doc = "Returns the node if it is of type `block_comment` ([`BlockComment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_block_comment(self) -> ::std::option::Option<BlockComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BlockComment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `line_comment` ([`LineComment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_line_comment(self) -> ::std::option::Option<LineComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LineComment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Get the optional field `doc`.\n\nThis child has type `doc_comment?` ([`DocComment`])"]
        #[inline]
        pub fn doc(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, DocComment<'tree>>> {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("doc")
                .map(<DocComment<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
        #[doc = "Get the optional field `inner`.\n\nThis child has type `inner_doc_comment_marker?` ([`InnerDocCommentMarker`])"]
        #[inline]
        pub fn inner(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, InnerDocCommentMarker<'tree>>>
        {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("inner")
                .map(<InnerDocCommentMarker<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
        #[doc = "Get the optional field `outer`.\n\nThis child has type `outer_doc_comment_marker?` ([`OuterDocCommentMarker`])"]
        #[inline]
        pub fn outer(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, OuterDocCommentMarker<'tree>>>
        {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("outer")
                .map(<OuterDocCommentMarker<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Comment<'tree> {
        type WithLifetime<'a> = Comment<'a>;
        const KIND: &'static str = "{block_comment | line_comment}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "block_comment" => Ok(unsafe {
                    Self::BlockComment(
                        <BlockComment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "line_comment" => Ok(unsafe {
                    Self::LineComment(
                        <LineComment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => ::type_sitter::Node::raw(x),
                Self::LineComment(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => ::type_sitter::Node::raw_mut(x),
                Self::LineComment(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.into_raw(),
                Self::LineComment(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{block_comment | line_comment}`:\n- [`BlockComment`]\n- [`LineComment`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CommentDocumentation<'tree> {
        BlockComment(BlockComment<'tree>),
        LineComment(LineComment<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CommentDocumentation<'tree> {
        #[doc = "Returns the node if it is of type `block_comment` ([`BlockComment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_block_comment(self) -> ::std::option::Option<BlockComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BlockComment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `line_comment` ([`LineComment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_line_comment(self) -> ::std::option::Option<LineComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LineComment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Get the optional field `doc`.\n\nThis child has type `doc_comment?` ([`DocComment`])"]
        #[inline]
        pub fn doc(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, DocComment<'tree>>> {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("doc")
                .map(<DocComment<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
        #[doc = "Get the optional field `inner`.\n\nThis child has type `inner_doc_comment_marker?` ([`InnerDocCommentMarker`])"]
        #[inline]
        pub fn inner(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, InnerDocCommentMarker<'tree>>>
        {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("inner")
                .map(<InnerDocCommentMarker<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
        #[doc = "Get the optional field `outer`.\n\nThis child has type `outer_doc_comment_marker?` ([`OuterDocCommentMarker`])"]
        #[inline]
        pub fn outer(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, OuterDocCommentMarker<'tree>>>
        {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("outer")
                .map(<OuterDocCommentMarker<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for CommentDocumentation<'tree> {
        type WithLifetime<'a> = CommentDocumentation<'a>;
        const KIND: &'static str = "{block_comment | line_comment}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "block_comment" => Ok(unsafe {
                    Self::BlockComment(
                        <BlockComment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "line_comment" => Ok(unsafe {
                    Self::LineComment(
                        <LineComment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => ::type_sitter::Node::raw(x),
                Self::LineComment(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => ::type_sitter::Node::raw_mut(x),
                Self::LineComment(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.into_raw(),
                Self::LineComment(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{boolean_literal | float_literal | integer_literal}`:\n- [`BooleanLiteral`]\n- [`FloatLiteral`]\n- [`IntegerLiteral`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstantBuiltin<'tree> {
        BooleanLiteral(BooleanLiteral<'tree>),
        FloatLiteral(FloatLiteral<'tree>),
        IntegerLiteral(IntegerLiteral<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ConstantBuiltin<'tree> {
        #[doc = "Returns the node if it is of type `boolean_literal` ([`BooleanLiteral`]), otherwise returns `None`"]
        #[inline]
        pub fn as_boolean_literal(self) -> ::std::option::Option<BooleanLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BooleanLiteral(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `float_literal` ([`FloatLiteral`]), otherwise returns `None`"]
        #[inline]
        pub fn as_float_literal(self) -> ::std::option::Option<FloatLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FloatLiteral(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `integer_literal` ([`IntegerLiteral`]), otherwise returns `None`"]
        #[inline]
        pub fn as_integer_literal(self) -> ::std::option::Option<IntegerLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::IntegerLiteral(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ConstantBuiltin<'tree> {
        type WithLifetime<'a> = ConstantBuiltin<'a>;
        const KIND: &'static str = "{boolean_literal | float_literal | integer_literal}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "boolean_literal" => {
                    Ok(unsafe {
                        Self::BooleanLiteral(<BooleanLiteral<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "float_literal" => Ok(unsafe {
                    Self::FloatLiteral(
                        <FloatLiteral<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "integer_literal" => {
                    Ok(unsafe {
                        Self::IntegerLiteral(<IntegerLiteral<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => ::type_sitter::Node::raw(x),
                Self::FloatLiteral(x) => ::type_sitter::Node::raw(x),
                Self::IntegerLiteral(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => ::type_sitter::Node::raw_mut(x),
                Self::FloatLiteral(x) => ::type_sitter::Node::raw_mut(x),
                Self::IntegerLiteral(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => x.into_raw(),
                Self::FloatLiteral(x) => x.into_raw(),
                Self::IntegerLiteral(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{identifier | type_identifier}`:\n- [`Identifier`]\n- [`TypeIdentifier`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Constructor<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Constructor<'tree> {
        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type_identifier(self) -> ::std::option::Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Constructor<'tree> {
        type WithLifetime<'a> = Constructor<'a>;
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self::Identifier(
                        <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(<TypeIdentifier<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::TypeIdentifier(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::TypeIdentifier(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{enum_item | struct_item | type_item | union_item}`:\n- [`EnumItem`]\n- [`StructItem`]\n- [`TypeItem`]\n- [`UnionItem`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefinitionClass<'tree> {
        EnumItem(EnumItem<'tree>),
        StructItem(StructItem<'tree>),
        TypeItem(TypeItem<'tree>),
        UnionItem(UnionItem<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DefinitionClass<'tree> {
        #[doc = "Returns the node if it is of type `enum_item` ([`EnumItem`]), otherwise returns `None`"]
        #[inline]
        pub fn as_enum_item(self) -> ::std::option::Option<EnumItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EnumItem(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `struct_item` ([`StructItem`]), otherwise returns `None`"]
        #[inline]
        pub fn as_struct_item(self) -> ::std::option::Option<StructItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StructItem(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `type_item` ([`TypeItem`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type_item(self) -> ::std::option::Option<TypeItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeItem(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `union_item` ([`UnionItem`]), otherwise returns `None`"]
        #[inline]
        pub fn as_union_item(self) -> ::std::option::Option<UnionItem<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionItem(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Get the field `name`.\n\nThis child has type `type_identifier` ([`TypeIdentifier`])"]
        #[inline]
        pub fn name(&self) -> ::type_sitter::NodeResult<'tree, TypeIdentifier<'tree>> {
            :: type_sitter :: Node :: raw (self) . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw) . expect ("required child not present, there should at least be a MISSING node in its place")
        }
        #[doc = "Get the optional field `type_parameters`.\n\nThis child has type `type_parameters?` ([`TypeParameters`])"]
        #[inline]
        pub fn type_parameters(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, TypeParameters<'tree>>>
        {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("type_parameters")
                .map(<TypeParameters<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DefinitionClass<'tree> {
        type WithLifetime<'a> = DefinitionClass<'a>;
        const KIND: &'static str = "{enum_item | struct_item | type_item | union_item}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "enum_item" => Ok(unsafe {
                    Self::EnumItem(
                        <EnumItem<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "struct_item" => Ok(unsafe {
                    Self::StructItem(
                        <StructItem<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "type_item" => Ok(unsafe {
                    Self::TypeItem(
                        <TypeItem<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "union_item" => Ok(unsafe {
                    Self::UnionItem(
                        <UnionItem<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::EnumItem(x) => ::type_sitter::Node::raw(x),
                Self::StructItem(x) => ::type_sitter::Node::raw(x),
                Self::TypeItem(x) => ::type_sitter::Node::raw(x),
                Self::UnionItem(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::EnumItem(x) => ::type_sitter::Node::raw_mut(x),
                Self::StructItem(x) => ::type_sitter::Node::raw_mut(x),
                Self::TypeItem(x) => ::type_sitter::Node::raw_mut(x),
                Self::UnionItem(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::EnumItem(x) => x.into_raw(),
                Self::StructItem(x) => x.into_raw(),
                Self::TypeItem(x) => x.into_raw(),
                Self::UnionItem(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{! | identifier}`:\n- [`symbols::Not`]\n- [`Identifier`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FunctionMacro<'tree> {
        Not(symbols::Not<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> FunctionMacro<'tree> {
        #[doc = "Returns the node if it is of type `!` ([`symbols::Not`]), otherwise returns `None`"]
        #[inline]
        pub fn as_not(self) -> ::std::option::Option<symbols::Not<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Not(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for FunctionMacro<'tree> {
        type WithLifetime<'a> = FunctionMacro<'a>;
        const KIND: &'static str = "{! | identifier}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "!" => Ok(unsafe {
                    Self::Not(
                        <symbols::Not<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "identifier" => Ok(unsafe {
                    Self::Identifier(
                        <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Not(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Not(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Not(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{as | async | await | break | const | continue | crate | default | dyn | else | enum | extern | fn | for | gen | if | impl | in | let | loop | macro_rules! | match | mod | move | mutable_specifier | pub | raw | ref | return | self | static | struct | super | trait | type | union | unsafe | use | where | while | yield}`:\n- [`unnamed::As`]\n- [`unnamed::Async`]\n- [`unnamed::Await`]\n- [`unnamed::Break`]\n- [`unnamed::Const`]\n- [`unnamed::Continue`]\n- [`Crate`]\n- [`unnamed::Default`]\n- [`unnamed::Dyn`]\n- [`unnamed::Else`]\n- [`unnamed::Enum`]\n- [`unnamed::Extern`]\n- [`unnamed::Fn`]\n- [`unnamed::For`]\n- [`unnamed::Gen`]\n- [`unnamed::If`]\n- [`unnamed::Impl`]\n- [`unnamed::In`]\n- [`unnamed::Let`]\n- [`unnamed::Loop`]\n- [`symbols::MacroRulesNot`]\n- [`unnamed::Match`]\n- [`unnamed::Mod`]\n- [`unnamed::Move`]\n- [`MutableSpecifier`]\n- [`unnamed::Pub`]\n- [`unnamed::Raw`]\n- [`unnamed::Ref`]\n- [`unnamed::Return`]\n- [`Self_`]\n- [`unnamed::Static`]\n- [`unnamed::Struct`]\n- [`Super`]\n- [`unnamed::Trait`]\n- [`unnamed::Type`]\n- [`unnamed::Union`]\n- [`unnamed::Unsafe`]\n- [`unnamed::Use`]\n- [`unnamed::Where`]\n- [`unnamed::While`]\n- [`unnamed::Yield`]"]
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
        Gen(unnamed::Gen<'tree>),
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
        Raw(unnamed::Raw<'tree>),
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
    #[allow(unused)]
    impl<'tree> Keyword<'tree> {
        #[doc = "Returns the node if it is of type `as` ([`unnamed::As`]), otherwise returns `None`"]
        #[inline]
        pub fn as_as(self) -> ::std::option::Option<unnamed::As<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::As(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `async` ([`unnamed::Async`]), otherwise returns `None`"]
        #[inline]
        pub fn as_async(self) -> ::std::option::Option<unnamed::Async<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Async(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `await` ([`unnamed::Await`]), otherwise returns `None`"]
        #[inline]
        pub fn as_await(self) -> ::std::option::Option<unnamed::Await<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Await(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `break` ([`unnamed::Break`]), otherwise returns `None`"]
        #[inline]
        pub fn as_break(self) -> ::std::option::Option<unnamed::Break<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Break(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `const` ([`unnamed::Const`]), otherwise returns `None`"]
        #[inline]
        pub fn as_const(self) -> ::std::option::Option<unnamed::Const<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Const(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `continue` ([`unnamed::Continue`]), otherwise returns `None`"]
        #[inline]
        pub fn as_continue(self) -> ::std::option::Option<unnamed::Continue<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Continue(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `crate` ([`Crate`]), otherwise returns `None`"]
        #[inline]
        pub fn as_crate(self) -> ::std::option::Option<Crate<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Crate(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `default` ([`unnamed::Default`]), otherwise returns `None`"]
        #[inline]
        pub fn as_default(self) -> ::std::option::Option<unnamed::Default<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Default(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `dyn` ([`unnamed::Dyn`]), otherwise returns `None`"]
        #[inline]
        pub fn as_dyn(self) -> ::std::option::Option<unnamed::Dyn<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Dyn(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `else` ([`unnamed::Else`]), otherwise returns `None`"]
        #[inline]
        pub fn as_else(self) -> ::std::option::Option<unnamed::Else<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Else(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `enum` ([`unnamed::Enum`]), otherwise returns `None`"]
        #[inline]
        pub fn as_enum(self) -> ::std::option::Option<unnamed::Enum<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Enum(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `extern` ([`unnamed::Extern`]), otherwise returns `None`"]
        #[inline]
        pub fn as_extern(self) -> ::std::option::Option<unnamed::Extern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Extern(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `fn` ([`unnamed::Fn`]), otherwise returns `None`"]
        #[inline]
        pub fn as_fn(self) -> ::std::option::Option<unnamed::Fn<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fn(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `for` ([`unnamed::For`]), otherwise returns `None`"]
        #[inline]
        pub fn as_for(self) -> ::std::option::Option<unnamed::For<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::For(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `gen` ([`unnamed::Gen`]), otherwise returns `None`"]
        #[inline]
        pub fn as_gen(self) -> ::std::option::Option<unnamed::Gen<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gen(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `if` ([`unnamed::If`]), otherwise returns `None`"]
        #[inline]
        pub fn as_if(self) -> ::std::option::Option<unnamed::If<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::If(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `impl` ([`unnamed::Impl`]), otherwise returns `None`"]
        #[inline]
        pub fn as_impl(self) -> ::std::option::Option<unnamed::Impl<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Impl(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `in` ([`unnamed::In`]), otherwise returns `None`"]
        #[inline]
        pub fn as_in(self) -> ::std::option::Option<unnamed::In<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::In(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `let` ([`unnamed::Let`]), otherwise returns `None`"]
        #[inline]
        pub fn as_let(self) -> ::std::option::Option<unnamed::Let<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Let(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `loop` ([`unnamed::Loop`]), otherwise returns `None`"]
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<unnamed::Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `macro_rules!` ([`symbols::MacroRulesNot`]), otherwise returns `None`"]
        #[inline]
        pub fn as_macro_rules_not(self) -> ::std::option::Option<symbols::MacroRulesNot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MacroRulesNot(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `match` ([`unnamed::Match`]), otherwise returns `None`"]
        #[inline]
        pub fn as_match(self) -> ::std::option::Option<unnamed::Match<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Match(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `mod` ([`unnamed::Mod`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mod_(self) -> ::std::option::Option<unnamed::Mod<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mod(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `move` ([`unnamed::Move`]), otherwise returns `None`"]
        #[inline]
        pub fn as_move(self) -> ::std::option::Option<unnamed::Move<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Move(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `mutable_specifier` ([`MutableSpecifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mutable_specifier(self) -> ::std::option::Option<MutableSpecifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MutableSpecifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `pub` ([`unnamed::Pub`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pub(self) -> ::std::option::Option<unnamed::Pub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pub(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `raw` ([`unnamed::Raw`]), otherwise returns `None`"]
        #[inline]
        pub fn as_raw(self) -> ::std::option::Option<unnamed::Raw<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Raw(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `ref` ([`unnamed::Ref`]), otherwise returns `None`"]
        #[inline]
        pub fn as_ref(self) -> ::std::option::Option<unnamed::Ref<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Ref(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `return` ([`unnamed::Return`]), otherwise returns `None`"]
        #[inline]
        pub fn as_return(self) -> ::std::option::Option<unnamed::Return<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Return(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `self` ([`Self_`]), otherwise returns `None`"]
        #[inline]
        pub fn as_self(self) -> ::std::option::Option<Self_<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Self_(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `static` ([`unnamed::Static`]), otherwise returns `None`"]
        #[inline]
        pub fn as_static(self) -> ::std::option::Option<unnamed::Static<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Static(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `struct` ([`unnamed::Struct`]), otherwise returns `None`"]
        #[inline]
        pub fn as_struct(self) -> ::std::option::Option<unnamed::Struct<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Struct(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `super` ([`Super`]), otherwise returns `None`"]
        #[inline]
        pub fn as_super(self) -> ::std::option::Option<Super<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Super(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `trait` ([`unnamed::Trait`]), otherwise returns `None`"]
        #[inline]
        pub fn as_trait(self) -> ::std::option::Option<unnamed::Trait<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Trait(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `type` ([`unnamed::Type`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type_(self) -> ::std::option::Option<unnamed::Type<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Type(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `union` ([`unnamed::Union`]), otherwise returns `None`"]
        #[inline]
        pub fn as_union(self) -> ::std::option::Option<unnamed::Union<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Union(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `unsafe` ([`unnamed::Unsafe`]), otherwise returns `None`"]
        #[inline]
        pub fn as_unsafe(self) -> ::std::option::Option<unnamed::Unsafe<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Unsafe(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `use` ([`unnamed::Use`]), otherwise returns `None`"]
        #[inline]
        pub fn as_use(self) -> ::std::option::Option<unnamed::Use<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Use(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `where` ([`unnamed::Where`]), otherwise returns `None`"]
        #[inline]
        pub fn as_where(self) -> ::std::option::Option<unnamed::Where<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Where(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `while` ([`unnamed::While`]), otherwise returns `None`"]
        #[inline]
        pub fn as_while(self) -> ::std::option::Option<unnamed::While<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::While(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `yield` ([`unnamed::Yield`]), otherwise returns `None`"]
        #[inline]
        pub fn as_yield(self) -> ::std::option::Option<unnamed::Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Keyword<'tree> {
        type WithLifetime<'a> = Keyword<'a>;
        const KIND : & 'static str = "{as | async | await | break | const | continue | crate | default | dyn | else | enum | extern | fn | for | gen | if | impl | in | let | loop | macro_rules! | match | mod | move | mutable_specifier | pub | raw | ref | return | self | static | struct | super | trait | type | union | unsafe | use | where | while | yield}" ;
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "as" => Ok(unsafe {
                    Self::As(
                        <unnamed::As<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "async" => {
                    Ok(unsafe {
                        Self :: Async (< unnamed :: Async < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "await" => {
                    Ok(unsafe {
                        Self :: Await (< unnamed :: Await < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "break" => {
                    Ok(unsafe {
                        Self :: Break (< unnamed :: Break < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "const" => {
                    Ok(unsafe {
                        Self :: Const (< unnamed :: Const < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "continue" => Ok(unsafe {
                    Self :: Continue (< unnamed :: Continue < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                }),
                "crate" => Ok(unsafe {
                    Self::Crate(
                        <Crate<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "default" => {
                    Ok(unsafe {
                        Self :: Default (< unnamed :: Default < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "dyn" => Ok(unsafe {
                    Self::Dyn(
                        <unnamed::Dyn<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "else" => {
                    Ok(unsafe {
                        Self :: Else (< unnamed :: Else < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "enum" => {
                    Ok(unsafe {
                        Self :: Enum (< unnamed :: Enum < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "extern" => {
                    Ok(unsafe {
                        Self :: Extern (< unnamed :: Extern < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "fn" => Ok(unsafe {
                    Self::Fn(
                        <unnamed::Fn<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "for" => Ok(unsafe {
                    Self::For(
                        <unnamed::For<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "gen" => Ok(unsafe {
                    Self::Gen(
                        <unnamed::Gen<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "if" => Ok(unsafe {
                    Self::If(
                        <unnamed::If<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "impl" => {
                    Ok(unsafe {
                        Self :: Impl (< unnamed :: Impl < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "in" => Ok(unsafe {
                    Self::In(
                        <unnamed::In<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "let" => Ok(unsafe {
                    Self::Let(
                        <unnamed::Let<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "loop" => {
                    Ok(unsafe {
                        Self :: Loop (< unnamed :: Loop < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "macro_rules!" => {
                    Ok(unsafe {
                        Self :: MacroRulesNot (< symbols :: MacroRulesNot < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "match" => {
                    Ok(unsafe {
                        Self :: Match (< unnamed :: Match < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "mod" => Ok(unsafe {
                    Self::Mod(
                        <unnamed::Mod<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "move" => {
                    Ok(unsafe {
                        Self :: Move (< unnamed :: Move < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "mutable_specifier" => Ok(unsafe {
                    Self::MutableSpecifier(<MutableSpecifier<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "pub" => Ok(unsafe {
                    Self::Pub(
                        <unnamed::Pub<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "raw" => Ok(unsafe {
                    Self::Raw(
                        <unnamed::Raw<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "ref" => Ok(unsafe {
                    Self::Ref(
                        <unnamed::Ref<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "return" => {
                    Ok(unsafe {
                        Self :: Return (< unnamed :: Return < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "self" => Ok(unsafe {
                    Self::Self_(
                        <Self_<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "static" => {
                    Ok(unsafe {
                        Self :: Static (< unnamed :: Static < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "struct" => {
                    Ok(unsafe {
                        Self :: Struct (< unnamed :: Struct < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "super" => Ok(unsafe {
                    Self::Super(
                        <Super<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "trait" => {
                    Ok(unsafe {
                        Self :: Trait (< unnamed :: Trait < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "type" => {
                    Ok(unsafe {
                        Self :: Type (< unnamed :: Type < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "union" => {
                    Ok(unsafe {
                        Self :: Union (< unnamed :: Union < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "unsafe" => {
                    Ok(unsafe {
                        Self :: Unsafe (< unnamed :: Unsafe < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "use" => Ok(unsafe {
                    Self::Use(
                        <unnamed::Use<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "where" => {
                    Ok(unsafe {
                        Self :: Where (< unnamed :: Where < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "while" => {
                    Ok(unsafe {
                        Self :: While (< unnamed :: While < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "yield" => {
                    Ok(unsafe {
                        Self :: Yield (< unnamed :: Yield < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::As(x) => ::type_sitter::Node::raw(x),
                Self::Async(x) => ::type_sitter::Node::raw(x),
                Self::Await(x) => ::type_sitter::Node::raw(x),
                Self::Break(x) => ::type_sitter::Node::raw(x),
                Self::Const(x) => ::type_sitter::Node::raw(x),
                Self::Continue(x) => ::type_sitter::Node::raw(x),
                Self::Crate(x) => ::type_sitter::Node::raw(x),
                Self::Default(x) => ::type_sitter::Node::raw(x),
                Self::Dyn(x) => ::type_sitter::Node::raw(x),
                Self::Else(x) => ::type_sitter::Node::raw(x),
                Self::Enum(x) => ::type_sitter::Node::raw(x),
                Self::Extern(x) => ::type_sitter::Node::raw(x),
                Self::Fn(x) => ::type_sitter::Node::raw(x),
                Self::For(x) => ::type_sitter::Node::raw(x),
                Self::Gen(x) => ::type_sitter::Node::raw(x),
                Self::If(x) => ::type_sitter::Node::raw(x),
                Self::Impl(x) => ::type_sitter::Node::raw(x),
                Self::In(x) => ::type_sitter::Node::raw(x),
                Self::Let(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::MacroRulesNot(x) => ::type_sitter::Node::raw(x),
                Self::Match(x) => ::type_sitter::Node::raw(x),
                Self::Mod(x) => ::type_sitter::Node::raw(x),
                Self::Move(x) => ::type_sitter::Node::raw(x),
                Self::MutableSpecifier(x) => ::type_sitter::Node::raw(x),
                Self::Pub(x) => ::type_sitter::Node::raw(x),
                Self::Raw(x) => ::type_sitter::Node::raw(x),
                Self::Ref(x) => ::type_sitter::Node::raw(x),
                Self::Return(x) => ::type_sitter::Node::raw(x),
                Self::Self_(x) => ::type_sitter::Node::raw(x),
                Self::Static(x) => ::type_sitter::Node::raw(x),
                Self::Struct(x) => ::type_sitter::Node::raw(x),
                Self::Super(x) => ::type_sitter::Node::raw(x),
                Self::Trait(x) => ::type_sitter::Node::raw(x),
                Self::Type(x) => ::type_sitter::Node::raw(x),
                Self::Union(x) => ::type_sitter::Node::raw(x),
                Self::Unsafe(x) => ::type_sitter::Node::raw(x),
                Self::Use(x) => ::type_sitter::Node::raw(x),
                Self::Where(x) => ::type_sitter::Node::raw(x),
                Self::While(x) => ::type_sitter::Node::raw(x),
                Self::Yield(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::As(x) => ::type_sitter::Node::raw_mut(x),
                Self::Async(x) => ::type_sitter::Node::raw_mut(x),
                Self::Await(x) => ::type_sitter::Node::raw_mut(x),
                Self::Break(x) => ::type_sitter::Node::raw_mut(x),
                Self::Const(x) => ::type_sitter::Node::raw_mut(x),
                Self::Continue(x) => ::type_sitter::Node::raw_mut(x),
                Self::Crate(x) => ::type_sitter::Node::raw_mut(x),
                Self::Default(x) => ::type_sitter::Node::raw_mut(x),
                Self::Dyn(x) => ::type_sitter::Node::raw_mut(x),
                Self::Else(x) => ::type_sitter::Node::raw_mut(x),
                Self::Enum(x) => ::type_sitter::Node::raw_mut(x),
                Self::Extern(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fn(x) => ::type_sitter::Node::raw_mut(x),
                Self::For(x) => ::type_sitter::Node::raw_mut(x),
                Self::Gen(x) => ::type_sitter::Node::raw_mut(x),
                Self::If(x) => ::type_sitter::Node::raw_mut(x),
                Self::Impl(x) => ::type_sitter::Node::raw_mut(x),
                Self::In(x) => ::type_sitter::Node::raw_mut(x),
                Self::Let(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::MacroRulesNot(x) => ::type_sitter::Node::raw_mut(x),
                Self::Match(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mod(x) => ::type_sitter::Node::raw_mut(x),
                Self::Move(x) => ::type_sitter::Node::raw_mut(x),
                Self::MutableSpecifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::Pub(x) => ::type_sitter::Node::raw_mut(x),
                Self::Raw(x) => ::type_sitter::Node::raw_mut(x),
                Self::Ref(x) => ::type_sitter::Node::raw_mut(x),
                Self::Return(x) => ::type_sitter::Node::raw_mut(x),
                Self::Self_(x) => ::type_sitter::Node::raw_mut(x),
                Self::Static(x) => ::type_sitter::Node::raw_mut(x),
                Self::Struct(x) => ::type_sitter::Node::raw_mut(x),
                Self::Super(x) => ::type_sitter::Node::raw_mut(x),
                Self::Trait(x) => ::type_sitter::Node::raw_mut(x),
                Self::Type(x) => ::type_sitter::Node::raw_mut(x),
                Self::Union(x) => ::type_sitter::Node::raw_mut(x),
                Self::Unsafe(x) => ::type_sitter::Node::raw_mut(x),
                Self::Use(x) => ::type_sitter::Node::raw_mut(x),
                Self::Where(x) => ::type_sitter::Node::raw_mut(x),
                Self::While(x) => ::type_sitter::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
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
                Self::Gen(x) => x.into_raw(),
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
                Self::Raw(x) => x.into_raw(),
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
    #[doc = "One of `{field_identifier | identifier | type_identifier}`:\n- [`FieldIdentifier`]\n- [`Identifier`]\n- [`TypeIdentifier`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Name<'tree> {
        FieldIdentifier(FieldIdentifier<'tree>),
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Name<'tree> {
        #[doc = "Returns the node if it is of type `field_identifier` ([`FieldIdentifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_field_identifier(self) -> ::std::option::Option<FieldIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FieldIdentifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type_identifier(self) -> ::std::option::Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Name<'tree> {
        type WithLifetime<'a> = Name<'a>;
        const KIND: &'static str = "{field_identifier | identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "field_identifier" => {
                    Ok(unsafe {
                        Self::FieldIdentifier(<FieldIdentifier<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "identifier" => Ok(unsafe {
                    Self::Identifier(
                        <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(<TypeIdentifier<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::TypeIdentifier(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::TypeIdentifier(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{& | ' | *}`:\n- [`symbols::And`]\n- [`symbols::Quote`]\n- [`symbols::Mul`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Operator<'tree> {
        And(symbols::And<'tree>),
        Quote(symbols::Quote<'tree>),
        Mul(symbols::Mul<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Operator<'tree> {
        #[doc = "Returns the node if it is of type `&` ([`symbols::And`]), otherwise returns `None`"]
        #[inline]
        pub fn as_and(self) -> ::std::option::Option<symbols::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `'` ([`symbols::Quote`]), otherwise returns `None`"]
        #[inline]
        pub fn as_quote(self) -> ::std::option::Option<symbols::Quote<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Quote(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `*` ([`symbols::Mul`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mul(self) -> ::std::option::Option<symbols::Mul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mul(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Operator<'tree> {
        type WithLifetime<'a> = Operator<'a>;
        const KIND: &'static str = "{& | ' | *}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "&" => Ok(unsafe {
                    Self::And(
                        <symbols::And<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "'" => {
                    Ok(unsafe {
                        Self :: Quote (< symbols :: Quote < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "*" => Ok(unsafe {
                    Self::Mul(
                        <symbols::Mul<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::And(x) => ::type_sitter::Node::raw(x),
                Self::Quote(x) => ::type_sitter::Node::raw(x),
                Self::Mul(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::And(x) => ::type_sitter::Node::raw_mut(x),
                Self::Quote(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::And(x) => x.into_raw(),
                Self::Quote(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{( | ) | < | > | [ | ] | { | }}`:\n- [`symbols::LParen`]\n- [`symbols::RParen`]\n- [`symbols::Lt`]\n- [`symbols::Gt`]\n- [`symbols::LBracket`]\n- [`symbols::RBracket`]\n- [`symbols::LBrace`]\n- [`symbols::RBrace`]"]
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
    #[allow(unused)]
    impl<'tree> PunctuationBracket<'tree> {
        #[doc = "Returns the node if it is of type `(` ([`symbols::LParen`]), otherwise returns `None`"]
        #[inline]
        pub fn as_l_paren(self) -> ::std::option::Option<symbols::LParen<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LParen(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `)` ([`symbols::RParen`]), otherwise returns `None`"]
        #[inline]
        pub fn as_r_paren(self) -> ::std::option::Option<symbols::RParen<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RParen(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lt(self) -> ::std::option::Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`"]
        #[inline]
        pub fn as_gt(self) -> ::std::option::Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `[` ([`symbols::LBracket`]), otherwise returns `None`"]
        #[inline]
        pub fn as_l_bracket(self) -> ::std::option::Option<symbols::LBracket<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LBracket(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `]` ([`symbols::RBracket`]), otherwise returns `None`"]
        #[inline]
        pub fn as_r_bracket(self) -> ::std::option::Option<symbols::RBracket<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RBracket(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `{` ([`symbols::LBrace`]), otherwise returns `None`"]
        #[inline]
        pub fn as_l_brace(self) -> ::std::option::Option<symbols::LBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LBrace(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `}` ([`symbols::RBrace`]), otherwise returns `None`"]
        #[inline]
        pub fn as_r_brace(self) -> ::std::option::Option<symbols::RBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RBrace(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for PunctuationBracket<'tree> {
        type WithLifetime<'a> = PunctuationBracket<'a>;
        const KIND: &'static str = "{( | ) | < | > | [ | ] | { | }}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "(" => {
                    Ok(unsafe {
                        Self :: LParen (< symbols :: LParen < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                ")" => {
                    Ok(unsafe {
                        Self :: RParen (< symbols :: RParen < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "<" => Ok(unsafe {
                    Self::Lt(
                        <symbols::Lt<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                ">" => Ok(unsafe {
                    Self::Gt(
                        <symbols::Gt<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "[" => Ok(unsafe {
                    Self :: LBracket (< symbols :: LBracket < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                }),
                "]" => Ok(unsafe {
                    Self :: RBracket (< symbols :: RBracket < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                }),
                "{" => {
                    Ok(unsafe {
                        Self :: LBrace (< symbols :: LBrace < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "}" => {
                    Ok(unsafe {
                        Self :: RBrace (< symbols :: RBrace < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::LParen(x) => ::type_sitter::Node::raw(x),
                Self::RParen(x) => ::type_sitter::Node::raw(x),
                Self::Lt(x) => ::type_sitter::Node::raw(x),
                Self::Gt(x) => ::type_sitter::Node::raw(x),
                Self::LBracket(x) => ::type_sitter::Node::raw(x),
                Self::RBracket(x) => ::type_sitter::Node::raw(x),
                Self::LBrace(x) => ::type_sitter::Node::raw(x),
                Self::RBrace(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::LParen(x) => ::type_sitter::Node::raw_mut(x),
                Self::RParen(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LBracket(x) => ::type_sitter::Node::raw_mut(x),
                Self::RBracket(x) => ::type_sitter::Node::raw_mut(x),
                Self::LBrace(x) => ::type_sitter::Node::raw_mut(x),
                Self::RBrace(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
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
    #[doc = "One of `{, | . | : | :: | ;}`:\n- [`symbols::Comma`]\n- [`symbols::Dot`]\n- [`symbols::Colon`]\n- [`symbols::ColonColon`]\n- [`symbols::Semicolon`]"]
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
    #[allow(unused)]
    impl<'tree> PunctuationDelimiter<'tree> {
        #[doc = "Returns the node if it is of type `,` ([`symbols::Comma`]), otherwise returns `None`"]
        #[inline]
        pub fn as_comma(self) -> ::std::option::Option<symbols::Comma<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comma(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `.` ([`symbols::Dot`]), otherwise returns `None`"]
        #[inline]
        pub fn as_dot(self) -> ::std::option::Option<symbols::Dot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Dot(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `:` ([`symbols::Colon`]), otherwise returns `None`"]
        #[inline]
        pub fn as_colon(self) -> ::std::option::Option<symbols::Colon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Colon(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `::` ([`symbols::ColonColon`]), otherwise returns `None`"]
        #[inline]
        pub fn as_colon_colon(self) -> ::std::option::Option<symbols::ColonColon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ColonColon(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `;` ([`symbols::Semicolon`]), otherwise returns `None`"]
        #[inline]
        pub fn as_semicolon(self) -> ::std::option::Option<symbols::Semicolon<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Semicolon(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for PunctuationDelimiter<'tree> {
        type WithLifetime<'a> = PunctuationDelimiter<'a>;
        const KIND: &'static str = "{, | . | : | :: | ;}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "," => {
                    Ok(unsafe {
                        Self :: Comma (< symbols :: Comma < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "." => Ok(unsafe {
                    Self::Dot(
                        <symbols::Dot<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                ":" => {
                    Ok(unsafe {
                        Self :: Colon (< symbols :: Colon < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "::" => Ok(unsafe {
                    Self :: ColonColon (< symbols :: ColonColon < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                }),
                ";" => Ok(unsafe {
                    Self :: Semicolon (< symbols :: Semicolon < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comma(x) => ::type_sitter::Node::raw(x),
                Self::Dot(x) => ::type_sitter::Node::raw(x),
                Self::Colon(x) => ::type_sitter::Node::raw(x),
                Self::ColonColon(x) => ::type_sitter::Node::raw(x),
                Self::Semicolon(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comma(x) => ::type_sitter::Node::raw_mut(x),
                Self::Dot(x) => ::type_sitter::Node::raw_mut(x),
                Self::Colon(x) => ::type_sitter::Node::raw_mut(x),
                Self::ColonColon(x) => ::type_sitter::Node::raw_mut(x),
                Self::Semicolon(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comma(x) => x.into_raw(),
                Self::Dot(x) => x.into_raw(),
                Self::Colon(x) => x.into_raw(),
                Self::ColonColon(x) => x.into_raw(),
                Self::Semicolon(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{call_expression | macro_invocation}`:\n- [`CallExpression`]\n- [`MacroInvocation`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ReferenceCall<'tree> {
        CallExpression(CallExpression<'tree>),
        MacroInvocation(MacroInvocation<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ReferenceCall<'tree> {
        #[doc = "Returns the node if it is of type `call_expression` ([`CallExpression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_call_expression(self) -> ::std::option::Option<CallExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CallExpression(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `macro_invocation` ([`MacroInvocation`]), otherwise returns `None`"]
        #[inline]
        pub fn as_macro_invocation(self) -> ::std::option::Option<MacroInvocation<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MacroInvocation(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ReferenceCall<'tree> {
        type WithLifetime<'a> = ReferenceCall<'a>;
        const KIND: &'static str = "{call_expression | macro_invocation}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "call_expression" => {
                    Ok(unsafe {
                        Self::CallExpression(<CallExpression<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "macro_invocation" => {
                    Ok(unsafe {
                        Self::MacroInvocation(<MacroInvocation<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::CallExpression(x) => ::type_sitter::Node::raw(x),
                Self::MacroInvocation(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CallExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::MacroInvocation(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CallExpression(x) => x.into_raw(),
                Self::MacroInvocation(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{char_literal | raw_string_literal | string_literal}`:\n- [`CharLiteral`]\n- [`RawStringLiteral`]\n- [`StringLiteral`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum String<'tree> {
        CharLiteral(CharLiteral<'tree>),
        RawStringLiteral(RawStringLiteral<'tree>),
        StringLiteral(StringLiteral<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> String<'tree> {
        #[doc = "Returns the node if it is of type `char_literal` ([`CharLiteral`]), otherwise returns `None`"]
        #[inline]
        pub fn as_char_literal(self) -> ::std::option::Option<CharLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CharLiteral(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `raw_string_literal` ([`RawStringLiteral`]), otherwise returns `None`"]
        #[inline]
        pub fn as_raw_string_literal(self) -> ::std::option::Option<RawStringLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RawStringLiteral(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `string_literal` ([`StringLiteral`]), otherwise returns `None`"]
        #[inline]
        pub fn as_string_literal(self) -> ::std::option::Option<StringLiteral<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StringLiteral(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for String<'tree> {
        type WithLifetime<'a> = String<'a>;
        const KIND: &'static str = "{char_literal | raw_string_literal | string_literal}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "char_literal" => Ok(unsafe {
                    Self::CharLiteral(
                        <CharLiteral<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "raw_string_literal" => Ok(unsafe {
                    Self::RawStringLiteral(<RawStringLiteral<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "string_literal" => {
                    Ok(unsafe {
                        Self :: StringLiteral (< StringLiteral < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::CharLiteral(x) => ::type_sitter::Node::raw(x),
                Self::RawStringLiteral(x) => ::type_sitter::Node::raw(x),
                Self::StringLiteral(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CharLiteral(x) => ::type_sitter::Node::raw_mut(x),
                Self::RawStringLiteral(x) => ::type_sitter::Node::raw_mut(x),
                Self::StringLiteral(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CharLiteral(x) => x.into_raw(),
                Self::RawStringLiteral(x) => x.into_raw(),
                Self::StringLiteral(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{identifier | type_identifier}`:\n- [`Identifier`]\n- [`TypeIdentifier`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Type<'tree> {
        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `type_identifier` ([`TypeIdentifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type_identifier(self) -> ::std::option::Option<TypeIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeIdentifier(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Type<'tree> {
        type WithLifetime<'a> = Type<'a>;
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self::Identifier(
                        <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self::TypeIdentifier(<TypeIdentifier<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::TypeIdentifier(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::TypeIdentifier(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::TypeIdentifier(x) => x.into_raw(),
            }
        }
    }
}
