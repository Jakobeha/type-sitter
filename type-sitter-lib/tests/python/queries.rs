/**Typed version of the query:

```sexp
; Identifier naming conventions

(identifier) @variable

((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_]*$"))

; Function calls

(decorator) @function
(decorator
  (identifier) @function)

(call
  function: (attribute attribute: (identifier) @function.method))
(call
  function: (identifier) @function)

; Builtin functions

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$"))

; Function definitions

(function_definition
  name: (identifier) @function)

(attribute attribute: (identifier) @property)
(type (identifier) @type)

; Literals

[
  (none)
  (true)
  (false)
] @constant.builtin

[
  (integer)
  (float)
] @number

(comment) @comment
(string) @string
(escape_sequence) @escape

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator

[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
/**Matches returned by a query cursor running the query [`Highlights`]:

```sexp
; Identifier naming conventions

(identifier) @variable

((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_]*$"))

; Function calls

(decorator) @function
(decorator
  (identifier) @function)

(call
  function: (attribute attribute: (identifier) @function.method))
(call
  function: (identifier) @function)

; Builtin functions

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$"))

; Function definitions

(function_definition
  name: (identifier) @function)

(attribute attribute: (identifier) @property)
(type (identifier) @type)

; Literals

[
  (none)
  (true)
  (false)
] @constant.builtin

[
  (integer)
  (float)
] @number

(comment) @comment
(string) @string
(escape_sequence) @escape

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator

[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

```*/
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'query, 'tree> = ::type_sitter_lib::QueryMatches<
    'query,
    'tree,
    Highlights,
>;
/**Captures returned by a query cursor running the query [`Highlights`]:

```sexp
; Identifier naming conventions

(identifier) @variable

((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_]*$"))

; Function calls

(decorator) @function
(decorator
  (identifier) @function)

(call
  function: (attribute attribute: (identifier) @function.method))
(call
  function: (identifier) @function)

; Builtin functions

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$"))

; Function definitions

(function_definition
  name: (identifier) @function)

(attribute attribute: (identifier) @property)
(type (identifier) @type)

; Literals

[
  (none)
  (true)
  (false)
] @constant.builtin

[
  (integer)
  (float)
] @number

(comment) @comment
(string) @string
(escape_sequence) @escape

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator

[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

```*/
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'query, 'tree> = ::type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Highlights,
>;
/**A match returned by the query [`Highlights`]:

```sexp
; Identifier naming conventions

(identifier) @variable

((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_]*$"))

; Function calls

(decorator) @function
(decorator
  (identifier) @function)

(call
  function: (attribute attribute: (identifier) @function.method))
(call
  function: (identifier) @function)

; Builtin functions

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$"))

; Function definitions

(function_definition
  name: (identifier) @function)

(attribute attribute: (identifier) @property)
(type (identifier) @type)

; Literals

[
  (none)
  (true)
  (false)
] @constant.builtin

[
  (integer)
  (float)
] @number

(comment) @comment
(string) @string
(escape_sequence) @escape

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator

[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

```*/
#[repr(transparent)]
pub struct HighlightsMatch<'query, 'tree: 'query>(
    ::yak_sitter::QueryMatch<'query, 'tree>,
);
/**A capture returned by the query [`Highlights`]:

```sexp
; Identifier naming conventions

(identifier) @variable

((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_]*$"))

; Function calls

(decorator) @function
(decorator
  (identifier) @function)

(call
  function: (attribute attribute: (identifier) @function.method))
(call
  function: (identifier) @function)

; Builtin functions

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$"))

; Function definitions

(function_definition
  name: (identifier) @function)

(attribute attribute: (identifier) @property)
(type (identifier) @type)

; Literals

[
  (none)
  (true)
  (false)
] @constant.builtin

[
  (integer)
  (float)
] @number

(comment) @comment
(string) @string
(escape_sequence) @escape

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator

[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

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
    ///A `constructor` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constructor
    ///```
    Constructor(super::nodes::Identifier<'tree>),
    ///A `constant` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///```
    Constant(super::nodes::Identifier<'tree>),
    ///A `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(decorator) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///```
    Function(anon_unions::Function<'tree>),
    ///A `function.method` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.method
    ///```
    FunctionMethod(super::nodes::Identifier<'tree>),
    ///A `function.builtin` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.builtin
    ///```
    FunctionBuiltin(super::nodes::Identifier<'tree>),
    ///A `property` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @property
    ///```
    Property(super::nodes::Identifier<'tree>),
    ///A `type` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @type
    ///```
    Type(super::nodes::Identifier<'tree>),
    ///A `constant.builtin` ([`anon_unions::ConstantBuiltin`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (none)
  (true)
  (false)
] @constant.builtin*/
    ///```
    ConstantBuiltin(anon_unions::ConstantBuiltin<'tree>),
    ///A `number` ([`anon_unions::Number`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (integer)
  (float)
] @number*/
    ///```
    Number(anon_unions::Number<'tree>),
    ///A `comment` ([`super::nodes::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    Comment(super::nodes::Comment<'tree>),
    ///A `string` ([`super::nodes::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string) @string
    ///```
    String(super::nodes::String<'tree>),
    ///A `escape` ([`super::nodes::EscapeSequence`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    Escape(super::nodes::EscapeSequence<'tree>),
    ///A `punctuation.special` ([`anon_unions::PunctuationSpecial`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"{" @punctuation.special
    ///"}" @punctuation.special
    ///```
    PunctuationSpecial(anon_unions::PunctuationSpecial<'tree>),
    ///A `embedded` ([`super::nodes::Interpolation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded*/
    ///```
    Embedded(super::nodes::Interpolation<'tree>),
    ///A `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator*/
    ///```
    Operator(anon_unions::Operator<'tree>),
    ///A `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword*/
    ///```
    Keyword(anon_unions::Keyword<'tree>),
}
#[automatically_derived]
impl ::type_sitter_lib::Query for Highlights {
    type Match<'query, 'tree: 'query> = HighlightsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = HighlightsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "; Identifier naming conventions\n\n(identifier) @variable\n\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z_]*$\"))\n\n; Function calls\n\n(decorator) @function\n(decorator\n  (identifier) @function)\n\n(call\n  function: (attribute attribute: (identifier) @function.method))\n(call\n  function: (identifier) @function)\n\n; Builtin functions\n\n((call\n  function: (identifier) @function.builtin)\n (#match?\n   @function.builtin\n   \"^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$\"))\n\n; Function definitions\n\n(function_definition\n  name: (identifier) @function)\n\n(attribute attribute: (identifier) @property)\n(type (identifier) @type)\n\n; Literals\n\n[\n  (none)\n  (true)\n  (false)\n] @constant.builtin\n\n[\n  (integer)\n  (float)\n] @number\n\n(comment) @comment\n(string) @string\n(escape_sequence) @escape\n\n(interpolation\n  \"{\" @punctuation.special\n  \"}\" @punctuation.special) @embedded\n\n[\n  \"-\"\n  \"-=\"\n  \"!=\"\n  \"*\"\n  \"**\"\n  \"**=\"\n  \"*=\"\n  \"/\"\n  \"//\"\n  \"//=\"\n  \"/=\"\n  \"&\"\n  \"&=\"\n  \"%\"\n  \"%=\"\n  \"^\"\n  \"^=\"\n  \"+\"\n  \"->\"\n  \"+=\"\n  \"<\"\n  \"<<\"\n  \"<<=\"\n  \"<=\"\n  \"<>\"\n  \"=\"\n  \":=\"\n  \"==\"\n  \">\"\n  \">=\"\n  \">>\"\n  \">>=\"\n  \"|\"\n  \"|=\"\n  \"~\"\n  \"@=\"\n  \"and\"\n  \"in\"\n  \"is\"\n  \"not\"\n  \"or\"\n  \"is not\"\n  \"not in\"\n] @operator\n\n[\n  \"as\"\n  \"assert\"\n  \"async\"\n  \"await\"\n  \"break\"\n  \"class\"\n  \"continue\"\n  \"def\"\n  \"del\"\n  \"elif\"\n  \"else\"\n  \"except\"\n  \"exec\"\n  \"finally\"\n  \"for\"\n  \"from\"\n  \"global\"\n  \"if\"\n  \"import\"\n  \"lambda\"\n  \"nonlocal\"\n  \"pass\"\n  \"print\"\n  \"raise\"\n  \"return\"\n  \"try\"\n  \"while\"\n  \"with\"\n  \"yield\"\n  \"match\"\n  \"case\"\n] @keyword\n"
    }
    fn raw(&self) -> &'static ::yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Highlights__: std::sync::OnceLock<::yak_sitter::Query> = std::sync::OnceLock::new();
        __Highlights__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = ::yak_sitter::Query::new(
                        &tree_sitter_python::LANGUAGE.into(),
                        "; Identifier naming conventions\n\n(identifier) @variable\n\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z_]*$\"))\n\n; Function calls\n\n(decorator) @function\n(decorator\n  (identifier) @function)\n\n(call\n  function: (attribute attribute: (identifier) @function.method))\n(call\n  function: (identifier) @function)\n\n; Builtin functions\n\n((call\n  function: (identifier) @function.builtin)\n (#match?\n   @function.builtin\n   \"^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$\"))\n\n; Function definitions\n\n(function_definition\n  name: (identifier) @function)\n\n(attribute attribute: (identifier) @property)\n(type (identifier) @type)\n\n; Literals\n\n[\n  (none)\n  (true)\n  (false)\n] @constant.builtin\n\n[\n  (integer)\n  (float)\n] @number\n\n(comment) @comment\n(string) @string\n(escape_sequence) @escape\n\n(interpolation\n  \"{\" @punctuation.special\n  \"}\" @punctuation.special) @embedded\n\n[\n  \"-\"\n  \"-=\"\n  \"!=\"\n  \"*\"\n  \"**\"\n  \"**=\"\n  \"*=\"\n  \"/\"\n  \"//\"\n  \"//=\"\n  \"/=\"\n  \"&\"\n  \"&=\"\n  \"%\"\n  \"%=\"\n  \"^\"\n  \"^=\"\n  \"+\"\n  \"->\"\n  \"+=\"\n  \"<\"\n  \"<<\"\n  \"<<=\"\n  \"<=\"\n  \"<>\"\n  \"=\"\n  \":=\"\n  \"==\"\n  \">\"\n  \">=\"\n  \">>\"\n  \">>=\"\n  \"|\"\n  \"|=\"\n  \"~\"\n  \"@=\"\n  \"and\"\n  \"in\"\n  \"is\"\n  \"not\"\n  \"or\"\n  \"is not\"\n  \"not in\"\n] @operator\n\n[\n  \"as\"\n  \"assert\"\n  \"async\"\n  \"await\"\n  \"break\"\n  \"class\"\n  \"continue\"\n  \"def\"\n  \"del\"\n  \"elif\"\n  \"else\"\n  \"except\"\n  \"exec\"\n  \"finally\"\n  \"for\"\n  \"from\"\n  \"global\"\n  \"if\"\n  \"import\"\n  \"lambda\"\n  \"nonlocal\"\n  \"pass\"\n  \"print\"\n  \"raise\"\n  \"return\"\n  \"try\"\n  \"while\"\n  \"with\"\n  \"yield\"\n  \"match\"\n  \"case\"\n] @keyword\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_python' correct, and did you use the same tree-sitter / tree_sitter_python version?",
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
                HighlightsCapture::Constructor(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            2usize => {
                HighlightsCapture::Constant(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            3usize => {
                HighlightsCapture::Function(
                    <anon_unions::Function<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            4usize => {
                HighlightsCapture::FunctionMethod(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            5usize => {
                HighlightsCapture::FunctionBuiltin(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            6usize => {
                HighlightsCapture::Property(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            7usize => {
                HighlightsCapture::Type(
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            8usize => {
                HighlightsCapture::ConstantBuiltin(
                    <anon_unions::ConstantBuiltin<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            9usize => {
                HighlightsCapture::Number(
                    <anon_unions::Number<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            10usize => {
                HighlightsCapture::Comment(
                    <super::nodes::Comment<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            11usize => {
                HighlightsCapture::String(
                    <super::nodes::String<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            12usize => {
                HighlightsCapture::Escape(
                    <super::nodes::EscapeSequence<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            13usize => {
                HighlightsCapture::PunctuationSpecial(
                    <anon_unions::PunctuationSpecial<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            14usize => {
                HighlightsCapture::Embedded(
                    <super::nodes::Interpolation<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            15usize => {
                HighlightsCapture::Operator(
                    <anon_unions::Operator<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            16usize => {
                HighlightsCapture::Keyword(
                    <anon_unions::Keyword<
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
    ///Returns an iterator over the nodes captured by `constructor` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constructor
    ///```
    #[inline]
    pub fn constructor(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [1u32]
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
    ///Returns an iterator over the nodes captured by `constant` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///```
    #[inline]
    pub fn constant(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [2u32]
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
    ///Returns an iterator over the nodes captured by `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(decorator) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///```
    #[inline]
    pub fn function(&self) -> ::std::option::Option<anon_unions::Function<'tree>> {
        {
            [3u32]
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
    ///Returns an iterator over the nodes captured by `function.method` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.method
    ///```
    #[inline]
    pub fn function_method(
        &self,
    ) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [4u32]
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
    ///Returns an iterator over the nodes captured by `function.builtin` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.builtin
    ///```
    #[inline]
    pub fn function_builtin(
        &self,
    ) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [5u32]
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
    ///Returns an iterator over the nodes captured by `property` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @property
    ///```
    #[inline]
    pub fn property(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [6u32]
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
    ///Returns an iterator over the nodes captured by `type` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @type
    ///```
    #[inline]
    pub fn r#type(&self) -> ::std::option::Option<super::nodes::Identifier<'tree>> {
        {
            [7u32]
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
    ///Returns an iterator over the nodes captured by `constant.builtin` ([`anon_unions::ConstantBuiltin`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (none)
  (true)
  (false)
] @constant.builtin*/
    ///```
    #[inline]
    pub fn constant_builtin(
        &self,
    ) -> ::std::option::Option<anon_unions::ConstantBuiltin<'tree>> {
        {
            [8u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::ConstantBuiltin<
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
    /**[
  (integer)
  (float)
] @number*/
    ///```
    #[inline]
    pub fn number(&self) -> ::std::option::Option<anon_unions::Number<'tree>> {
        {
            [9u32]
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
    ///Returns an iterator over the nodes captured by `comment` ([`super::nodes::Comment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(comment) @comment
    ///```
    #[inline]
    pub fn comment(&self) -> ::std::option::Option<super::nodes::Comment<'tree>> {
        {
            [10u32]
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
    ///Returns an iterator over the nodes captured by `string` ([`super::nodes::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string) @string
    ///```
    #[inline]
    pub fn string(&self) -> ::std::option::Option<super::nodes::String<'tree>> {
        {
            [11u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::String<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
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
    pub fn escape(&self) -> ::std::option::Option<super::nodes::EscapeSequence<'tree>> {
        {
            [12u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::EscapeSequence<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `punctuation.special` ([`anon_unions::PunctuationSpecial`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"{" @punctuation.special
    ///"}" @punctuation.special
    ///```
    #[inline]
    pub fn punctuation_special(
        &self,
    ) -> impl ::std::iter::Iterator<Item = anon_unions::PunctuationSpecial<'tree>> + '_ {
        {
            [13u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::PunctuationSpecial<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
    }
    ///Returns an iterator over the nodes captured by `embedded` ([`super::nodes::Interpolation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded*/
    ///```
    #[inline]
    pub fn embedded(&self) -> ::std::option::Option<super::nodes::Interpolation<'tree>> {
        {
            [14u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Interpolation<
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
    /**[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator*/
    ///```
    #[inline]
    pub fn operator(&self) -> ::std::option::Option<anon_unions::Operator<'tree>> {
        {
            [15u32]
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
    ///Returns an iterator over the nodes captured by `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword*/
    ///```
    #[inline]
    pub fn keyword(&self) -> ::std::option::Option<anon_unions::Keyword<'tree>> {
        {
            [16u32]
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
    ///Try to interpret this capture as a `constructor` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constructor
    ///```
    #[inline]
    pub fn as_constructor(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constructor(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `constant` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @constant
    ///```
    #[inline]
    pub fn as_constant(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Constant(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function` ([`anon_unions::Function`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(decorator) @function
    ///(identifier) @function
    ///(identifier) @function
    ///(identifier) @function
    ///```
    #[inline]
    pub fn as_function(&self) -> ::std::option::Option<&anon_unions::Function<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Function(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function.method` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.method
    ///```
    #[inline]
    pub fn as_function_method(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionMethod(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `function.builtin` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @function.builtin
    ///```
    #[inline]
    pub fn as_function_builtin(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionBuiltin(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `property` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @property
    ///```
    #[inline]
    pub fn as_property(
        &self,
    ) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Property(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `type` ([`super::nodes::Identifier`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @type
    ///```
    #[inline]
    pub fn as_type(&self) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Type(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `constant.builtin` ([`anon_unions::ConstantBuiltin`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (none)
  (true)
  (false)
] @constant.builtin*/
    ///```
    #[inline]
    pub fn as_constant_builtin(
        &self,
    ) -> ::std::option::Option<&anon_unions::ConstantBuiltin<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConstantBuiltin(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `number` ([`anon_unions::Number`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  (integer)
  (float)
] @number*/
    ///```
    #[inline]
    pub fn as_number(&self) -> ::std::option::Option<&anon_unions::Number<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Number(node) = self { Some(node) } else { None }
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
    ///Try to interpret this capture as a `string` ([`super::nodes::String`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(string) @string
    ///```
    #[inline]
    pub fn as_string(&self) -> ::std::option::Option<&super::nodes::String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `escape` ([`super::nodes::EscapeSequence`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(escape_sequence) @escape
    ///```
    #[inline]
    pub fn as_escape(
        &self,
    ) -> ::std::option::Option<&super::nodes::EscapeSequence<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Escape(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `punctuation.special` ([`anon_unions::PunctuationSpecial`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///"{" @punctuation.special
    ///"}" @punctuation.special
    ///```
    #[inline]
    pub fn as_punctuation_special(
        &self,
    ) -> ::std::option::Option<&anon_unions::PunctuationSpecial<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PunctuationSpecial(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `embedded` ([`super::nodes::Interpolation`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded*/
    ///```
    #[inline]
    pub fn as_embedded(
        &self,
    ) -> ::std::option::Option<&super::nodes::Interpolation<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Embedded(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `operator` ([`anon_unions::Operator`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<<="
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "~"
  "@="
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @operator*/
    ///```
    #[inline]
    pub fn as_operator(&self) -> ::std::option::Option<&anon_unions::Operator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Operator(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `keyword` ([`anon_unions::Keyword`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword*/
    ///```
    #[inline]
    pub fn as_keyword(&self) -> ::std::option::Option<&anon_unions::Keyword<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Keyword(node) = self { Some(node) } else { None }
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
            Self::Constructor(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 1usize,
                    name: "constructor",
                }
            }
            Self::Constant(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 2usize,
                    name: "constant",
                }
            }
            Self::Function(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 3usize,
                    name: "function",
                }
            }
            Self::FunctionMethod(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 4usize,
                    name: "function.method",
                }
            }
            Self::FunctionBuiltin(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 5usize,
                    name: "function.builtin",
                }
            }
            Self::Property(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 6usize,
                    name: "property",
                }
            }
            Self::Type(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 7usize,
                    name: "type",
                }
            }
            Self::ConstantBuiltin(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 8usize,
                    name: "constant.builtin",
                }
            }
            Self::Number(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 9usize,
                    name: "number",
                }
            }
            Self::Comment(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 10usize,
                    name: "comment",
                }
            }
            Self::String(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 11usize,
                    name: "string",
                }
            }
            Self::Escape(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 12usize,
                    name: "escape",
                }
            }
            Self::PunctuationSpecial(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 13usize,
                    name: "punctuation.special",
                }
            }
            Self::Embedded(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 14usize,
                    name: "embedded",
                }
            }
            Self::Operator(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 15usize,
                    name: "operator",
                }
            }
            Self::Keyword(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 16usize,
                    name: "keyword",
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
            Self::Constructor(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Constant(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Function(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::FunctionMethod(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::FunctionBuiltin(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Property(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Type(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::ConstantBuiltin(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Number(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Comment(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::String(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Escape(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::PunctuationSpecial(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::Embedded(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Operator(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
            Self::Keyword(node) => ::type_sitter_lib::UntypedNode::r#ref(node.raw()),
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
            Self::Constructor(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Constant(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Function(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::FunctionMethod(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::FunctionBuiltin(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Property(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Type(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::ConstantBuiltin(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Number(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Comment(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::String(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Escape(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::PunctuationSpecial(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::Embedded(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Operator(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            Self::Keyword(node) => ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn name(&self) -> &'query str {
        match self {
            Self::Variable { .. } => "variable",
            Self::Constructor { .. } => "constructor",
            Self::Constant { .. } => "constant",
            Self::Function { .. } => "function",
            Self::FunctionMethod { .. } => "function.method",
            Self::FunctionBuiltin { .. } => "function.builtin",
            Self::Property { .. } => "property",
            Self::Type { .. } => "type",
            Self::ConstantBuiltin { .. } => "constant.builtin",
            Self::Number { .. } => "number",
            Self::Comment { .. } => "comment",
            Self::String { .. } => "string",
            Self::Escape { .. } => "escape",
            Self::PunctuationSpecial { .. } => "punctuation.special",
            Self::Embedded { .. } => "embedded",
            Self::Operator { .. } => "operator",
            Self::Keyword { .. } => "keyword",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Variable { .. } => 0usize,
            Self::Constructor { .. } => 1usize,
            Self::Constant { .. } => 2usize,
            Self::Function { .. } => 3usize,
            Self::FunctionMethod { .. } => 4usize,
            Self::FunctionBuiltin { .. } => 5usize,
            Self::Property { .. } => 6usize,
            Self::Type { .. } => 7usize,
            Self::ConstantBuiltin { .. } => 8usize,
            Self::Number { .. } => 9usize,
            Self::Comment { .. } => 10usize,
            Self::String { .. } => 11usize,
            Self::Escape { .. } => 12usize,
            Self::PunctuationSpecial { .. } => 13usize,
            Self::Embedded { .. } => 14usize,
            Self::Operator { .. } => 15usize,
            Self::Keyword { .. } => 16usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
/**Typed version of the query:

```sexp
(module (expression_statement (assignment left: (identifier) @name) @definition.constant))

(class_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call

```*/
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
/**Matches returned by a query cursor running the query [`Tags`]:

```sexp
(module (expression_statement (assignment left: (identifier) @name) @definition.constant))

(class_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call

```*/
#[allow(unused, non_camel_case_types)]
pub type TagsMatches<'query, 'tree> = ::type_sitter_lib::QueryMatches<
    'query,
    'tree,
    Tags,
>;
/**Captures returned by a query cursor running the query [`Tags`]:

```sexp
(module (expression_statement (assignment left: (identifier) @name) @definition.constant))

(class_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call

```*/
#[allow(unused, non_camel_case_types)]
pub type TagsCaptures<'query, 'tree> = ::type_sitter_lib::QueryCaptures<
    'query,
    'tree,
    Tags,
>;
/**A match returned by the query [`Tags`]:

```sexp
(module (expression_statement (assignment left: (identifier) @name) @definition.constant))

(class_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call

```*/
#[repr(transparent)]
pub struct TagsMatch<'query, 'tree: 'query>(::yak_sitter::QueryMatch<'query, 'tree>);
/**A capture returned by the query [`Tags`]:

```sexp
(module (expression_statement (assignment left: (identifier) @name) @definition.constant))

(class_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call

```*/
#[derive(Clone, Debug)]
pub enum TagsCapture<'tree> {
    ///A `name` ([`anon_unions::Name`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///```
    Name(super::nodes::Identifier<'tree>),
    ///A `definition.constant` ([`super::nodes::Assignment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(assignment left: (identifier) @name) @definition.constant
    ///```
    DefinitionConstant(super::nodes::Assignment<'tree>),
    ///A `definition.class` ([`super::nodes::ClassDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(class_definition
  name: (identifier) @name) @definition.class*/
    ///```
    DefinitionClass(super::nodes::ClassDefinition<'tree>),
    ///A `definition.function` ([`super::nodes::FunctionDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(function_definition
  name: (identifier) @name) @definition.function*/
    ///```
    DefinitionFunction(super::nodes::FunctionDefinition<'tree>),
    ///A `reference.call` ([`super::nodes::Call`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call*/
    ///```
    ReferenceCall(super::nodes::Call<'tree>),
}
#[automatically_derived]
impl ::type_sitter_lib::Query for Tags {
    type Match<'query, 'tree: 'query> = TagsMatch<'query, 'tree>;
    type Capture<'query, 'tree: 'query> = TagsCapture<'tree>;
    fn as_str(&self) -> &'static str {
        "(module (expression_statement (assignment left: (identifier) @name) @definition.constant))\n\n(class_definition\n  name: (identifier) @name) @definition.class\n\n(function_definition\n  name: (identifier) @name) @definition.function\n\n(call\n  function: [\n      (identifier) @name\n      (attribute\n        attribute: (identifier) @name)\n  ]) @reference.call\n"
    }
    fn raw(&self) -> &'static ::yak_sitter::Query {
        #[allow(non_upper_case_globals)]
        static __Tags__: std::sync::OnceLock<::yak_sitter::Query> = std::sync::OnceLock::new();
        __Tags__
            .get_or_init(|| {
                #[allow(unused_mut)]
                let mut query = ::yak_sitter::Query::new(
                        &tree_sitter_python::LANGUAGE.into(),
                        "(module (expression_statement (assignment left: (identifier) @name) @definition.constant))\n\n(class_definition\n  name: (identifier) @name) @definition.class\n\n(function_definition\n  name: (identifier) @name) @definition.function\n\n(call\n  function: [\n      (identifier) @name\n      (attribute\n        attribute: (identifier) @name)\n  ]) @reference.call\n",
                    )
                    .expect(
                        "query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_python' correct, and did you use the same tree-sitter / tree_sitter_python version?",
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
                    <super::nodes::Identifier<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            1usize => {
                TagsCapture::DefinitionConstant(
                    <super::nodes::Assignment<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            2usize => {
                TagsCapture::DefinitionClass(
                    <super::nodes::ClassDefinition<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            3usize => {
                TagsCapture::DefinitionFunction(
                    <super::nodes::FunctionDefinition<
                        'tree,
                    > as ::type_sitter_lib::Node<
                        'tree,
                    >>::from_raw_unchecked(capture.node),
                )
            }
            4usize => {
                TagsCapture::ReferenceCall(
                    <super::nodes::Call<
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
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///```
    #[inline]
    pub fn name(&self) -> super::nodes::Identifier<'tree> {
        let mut iterator = {
            [0u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Identifier<
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
    ///Returns an iterator over the nodes captured by `definition.constant` ([`super::nodes::Assignment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(assignment left: (identifier) @name) @definition.constant
    ///```
    #[inline]
    pub fn definition_constant(
        &self,
    ) -> ::std::option::Option<super::nodes::Assignment<'tree>> {
        {
            [1u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Assignment<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.class` ([`super::nodes::ClassDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(class_definition
  name: (identifier) @name) @definition.class*/
    ///```
    #[inline]
    pub fn definition_class(
        &self,
    ) -> ::std::option::Option<super::nodes::ClassDefinition<'tree>> {
        {
            [2u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::ClassDefinition<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `definition.function` ([`super::nodes::FunctionDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(function_definition
  name: (identifier) @name) @definition.function*/
    ///```
    #[inline]
    pub fn definition_function(
        &self,
    ) -> ::std::option::Option<super::nodes::FunctionDefinition<'tree>> {
        {
            [3u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::FunctionDefinition<
                        'tree,
                    > as ::type_sitter_lib::Node<'tree>>::from_raw_unchecked(n)
                })
        }
            .next()
    }
    ///Returns an iterator over the nodes captured by `reference.call` ([`super::nodes::Call`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call*/
    ///```
    #[inline]
    pub fn reference_call(&self) -> ::std::option::Option<super::nodes::Call<'tree>> {
        {
            [4u32]
                .into_iter()
                .flat_map(|i| self.0.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <super::nodes::Call<
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
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///(identifier) @name
    ///```
    #[inline]
    pub fn as_name(&self) -> ::std::option::Option<&super::nodes::Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Name(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.constant` ([`super::nodes::Assignment`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    ///(assignment left: (identifier) @name) @definition.constant
    ///```
    #[inline]
    pub fn as_definition_constant(
        &self,
    ) -> ::std::option::Option<&super::nodes::Assignment<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionConstant(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.class` ([`super::nodes::ClassDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(class_definition
  name: (identifier) @name) @definition.class*/
    ///```
    #[inline]
    pub fn as_definition_class(
        &self,
    ) -> ::std::option::Option<&super::nodes::ClassDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionClass(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `definition.function` ([`super::nodes::FunctionDefinition`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(function_definition
  name: (identifier) @name) @definition.function*/
    ///```
    #[inline]
    pub fn as_definition_function(
        &self,
    ) -> ::std::option::Option<&super::nodes::FunctionDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefinitionFunction(node) = self { Some(node) } else { None }
    }
    ///Try to interpret this capture as a `reference.call` ([`super::nodes::Call`])
    ///
    ///The full capture including pattern(s) is:
    ///```sexp
    /**(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call*/
    ///```
    #[inline]
    pub fn as_reference_call(
        &self,
    ) -> ::std::option::Option<&super::nodes::Call<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ReferenceCall(node) = self { Some(node) } else { None }
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
            Self::DefinitionConstant(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 1usize,
                    name: "definition.constant",
                }
            }
            Self::DefinitionClass(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 2usize,
                    name: "definition.class",
                }
            }
            Self::DefinitionFunction(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 3usize,
                    name: "definition.function",
                }
            }
            Self::ReferenceCall(node) => {
                yak_sitter::QueryCapture {
                    node: *node.raw(),
                    index: 4usize,
                    name: "reference.call",
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
            Self::DefinitionConstant(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionClass(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::DefinitionFunction(node) => {
                ::type_sitter_lib::UntypedNode::r#ref(node.raw())
            }
            Self::ReferenceCall(node) => {
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
            Self::DefinitionConstant(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionClass(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::DefinitionFunction(node) => {
                ::type_sitter_lib::UntypedNode::r#mut(node.raw_mut())
            }
            Self::ReferenceCall(node) => {
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
            Self::DefinitionConstant { .. } => "definition.constant",
            Self::DefinitionClass { .. } => "definition.class",
            Self::DefinitionFunction { .. } => "definition.function",
            Self::ReferenceCall { .. } => "reference.call",
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Name { .. } => 0usize,
            Self::DefinitionConstant { .. } => 1usize,
            Self::DefinitionClass { .. } => 2usize,
            Self::DefinitionFunction { .. } => 3usize,
            Self::ReferenceCall { .. } => 4usize,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::super::nodes::*;
    /**One of `{false | none | true}`:
- [`False`]
- [`None`]
- [`True`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstantBuiltin<'tree> {
        False(False<'tree>),
        None(None<'tree>),
        True(True<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ConstantBuiltin<'tree> {
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
    impl<'tree> ::type_sitter_lib::Node<'tree> for ConstantBuiltin<'tree> {
        type WithLifetime<'a> = ConstantBuiltin<'a>;
        const KIND: &'static str = "{false | none | true}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "false" => {
                    Ok(unsafe {
                        Self::False(
                            <False<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "none" => {
                    Ok(unsafe {
                        Self::None(
                            <None<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "true" => {
                    Ok(unsafe {
                        Self::True(
                            <True<
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
                Self::False(x) => ::type_sitter_lib::Node::raw(x),
                Self::None(x) => ::type_sitter_lib::Node::raw(x),
                Self::True(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::False(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::None(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::True(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::False(x) => x.into_raw(),
                Self::None(x) => x.into_raw(),
                Self::True(x) => x.into_raw(),
            }
        }
    }
    /**One of `{decorator | identifier}`:
- [`Decorator`]
- [`Identifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Function<'tree> {
        Decorator(Decorator<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Function<'tree> {
        ///Returns the node if it is of type `decorator` ([`Decorator`]), otherwise returns `None`
        #[inline]
        pub fn as_decorator(self) -> ::std::option::Option<Decorator<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Decorator(x) = self {
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
        const KIND: &'static str = "{decorator | identifier}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "decorator" => {
                    Ok(unsafe {
                        Self::Decorator(
                            <Decorator<
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
                Self::Decorator(x) => ::type_sitter_lib::Node::raw(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::Decorator(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::Decorator(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{as | assert | async | await | break | case | class | continue | def | del | elif | else | except | exec | finally | for | from | global | if | import | lambda | match | nonlocal | pass | print | raise | return | try | while | with | yield}`:
- [`unnamed::As`]
- [`unnamed::Assert`]
- [`unnamed::Async`]
- [`unnamed::Await`]
- [`unnamed::Break`]
- [`unnamed::Case`]
- [`unnamed::Class`]
- [`unnamed::Continue`]
- [`unnamed::Def`]
- [`unnamed::Del`]
- [`unnamed::Elif`]
- [`unnamed::Else`]
- [`unnamed::Except`]
- [`unnamed::Exec`]
- [`unnamed::Finally`]
- [`unnamed::For`]
- [`unnamed::From`]
- [`unnamed::Global`]
- [`unnamed::If`]
- [`unnamed::Import`]
- [`unnamed::Lambda`]
- [`unnamed::Match`]
- [`unnamed::Nonlocal`]
- [`unnamed::Pass`]
- [`unnamed::Print`]
- [`unnamed::Raise`]
- [`unnamed::Return`]
- [`unnamed::Try`]
- [`unnamed::While`]
- [`unnamed::With`]
- [`unnamed::Yield`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Keyword<'tree> {
        As(unnamed::As<'tree>),
        Assert(unnamed::Assert<'tree>),
        Async(unnamed::Async<'tree>),
        Await(unnamed::Await<'tree>),
        Break(unnamed::Break<'tree>),
        Case(unnamed::Case<'tree>),
        Class(unnamed::Class<'tree>),
        Continue(unnamed::Continue<'tree>),
        Def(unnamed::Def<'tree>),
        Del(unnamed::Del<'tree>),
        Elif(unnamed::Elif<'tree>),
        Else(unnamed::Else<'tree>),
        Except(unnamed::Except<'tree>),
        Exec(unnamed::Exec<'tree>),
        Finally(unnamed::Finally<'tree>),
        For(unnamed::For<'tree>),
        From(unnamed::From<'tree>),
        Global(unnamed::Global<'tree>),
        If(unnamed::If<'tree>),
        Import(unnamed::Import<'tree>),
        Lambda(unnamed::Lambda<'tree>),
        Match(unnamed::Match<'tree>),
        Nonlocal(unnamed::Nonlocal<'tree>),
        Pass(unnamed::Pass<'tree>),
        Print(unnamed::Print<'tree>),
        Raise(unnamed::Raise<'tree>),
        Return(unnamed::Return<'tree>),
        Try(unnamed::Try<'tree>),
        While(unnamed::While<'tree>),
        With(unnamed::With<'tree>),
        Yield(unnamed::Yield<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Keyword<'tree> {
        ///Returns the node if it is of type `as` ([`unnamed::As`]), otherwise returns `None`
        #[inline]
        pub fn as_as(self) -> ::std::option::Option<unnamed::As<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::As(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `assert` ([`unnamed::Assert`]), otherwise returns `None`
        #[inline]
        pub fn as_assert(self) -> ::std::option::Option<unnamed::Assert<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Assert(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `async` ([`unnamed::Async`]), otherwise returns `None`
        #[inline]
        pub fn as_async(self) -> ::std::option::Option<unnamed::Async<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Async(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `await` ([`unnamed::Await`]), otherwise returns `None`
        #[inline]
        pub fn as_await_(self) -> ::std::option::Option<unnamed::Await<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Await(x) = self {
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
        ///Returns the node if it is of type `class` ([`unnamed::Class`]), otherwise returns `None`
        #[inline]
        pub fn as_class(self) -> ::std::option::Option<unnamed::Class<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Class(x) = self {
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
        ///Returns the node if it is of type `def` ([`unnamed::Def`]), otherwise returns `None`
        #[inline]
        pub fn as_def(self) -> ::std::option::Option<unnamed::Def<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Def(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `del` ([`unnamed::Del`]), otherwise returns `None`
        #[inline]
        pub fn as_del(self) -> ::std::option::Option<unnamed::Del<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Del(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `elif` ([`unnamed::Elif`]), otherwise returns `None`
        #[inline]
        pub fn as_elif(self) -> ::std::option::Option<unnamed::Elif<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Elif(x) = self {
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
        ///Returns the node if it is of type `except` ([`unnamed::Except`]), otherwise returns `None`
        #[inline]
        pub fn as_except(self) -> ::std::option::Option<unnamed::Except<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Except(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `exec` ([`unnamed::Exec`]), otherwise returns `None`
        #[inline]
        pub fn as_exec(self) -> ::std::option::Option<unnamed::Exec<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Exec(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `finally` ([`unnamed::Finally`]), otherwise returns `None`
        #[inline]
        pub fn as_finally(self) -> ::std::option::Option<unnamed::Finally<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Finally(x) = self {
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
        ///Returns the node if it is of type `from` ([`unnamed::From`]), otherwise returns `None`
        #[inline]
        pub fn as_from(self) -> ::std::option::Option<unnamed::From<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::From(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `global` ([`unnamed::Global`]), otherwise returns `None`
        #[inline]
        pub fn as_global(self) -> ::std::option::Option<unnamed::Global<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Global(x) = self {
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
        ///Returns the node if it is of type `import` ([`unnamed::Import`]), otherwise returns `None`
        #[inline]
        pub fn as_import(self) -> ::std::option::Option<unnamed::Import<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Import(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `lambda` ([`unnamed::Lambda`]), otherwise returns `None`
        #[inline]
        pub fn as_lambda_(self) -> ::std::option::Option<unnamed::Lambda<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lambda(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `match` ([`unnamed::Match`]), otherwise returns `None`
        #[inline]
        pub fn as_match(self) -> ::std::option::Option<unnamed::Match<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Match(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `nonlocal` ([`unnamed::Nonlocal`]), otherwise returns `None`
        #[inline]
        pub fn as_nonlocal(self) -> ::std::option::Option<unnamed::Nonlocal<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Nonlocal(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `pass` ([`unnamed::Pass`]), otherwise returns `None`
        #[inline]
        pub fn as_pass(self) -> ::std::option::Option<unnamed::Pass<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pass(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `print` ([`unnamed::Print`]), otherwise returns `None`
        #[inline]
        pub fn as_print(self) -> ::std::option::Option<unnamed::Print<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Print(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `raise` ([`unnamed::Raise`]), otherwise returns `None`
        #[inline]
        pub fn as_raise(self) -> ::std::option::Option<unnamed::Raise<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Raise(x) = self {
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
        ///Returns the node if it is of type `try` ([`unnamed::Try`]), otherwise returns `None`
        #[inline]
        pub fn as_try(self) -> ::std::option::Option<unnamed::Try<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Try(x) = self {
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
        ///Returns the node if it is of type `with` ([`unnamed::With`]), otherwise returns `None`
        #[inline]
        pub fn as_with(self) -> ::std::option::Option<unnamed::With<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::With(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `yield` ([`unnamed::Yield`]), otherwise returns `None`
        #[inline]
        pub fn as_yield_(self) -> ::std::option::Option<unnamed::Yield<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Yield(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for Keyword<'tree> {
        type WithLifetime<'a> = Keyword<'a>;
        const KIND: &'static str = "{as | assert | async | await | break | case | class | continue | def | del | elif | else | except | exec | finally | for | from | global | if | import | lambda | match | nonlocal | pass | print | raise | return | try | while | with | yield}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "as" => {
                    Ok(unsafe {
                        Self::As(
                            <unnamed::As<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "assert" => {
                    Ok(unsafe {
                        Self::Assert(
                            <unnamed::Assert<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "async" => {
                    Ok(unsafe {
                        Self::Async(
                            <unnamed::Async<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "await" => {
                    Ok(unsafe {
                        Self::Await(
                            <unnamed::Await<
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
                "class" => {
                    Ok(unsafe {
                        Self::Class(
                            <unnamed::Class<
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
                "def" => {
                    Ok(unsafe {
                        Self::Def(
                            <unnamed::Def<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "del" => {
                    Ok(unsafe {
                        Self::Del(
                            <unnamed::Del<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "elif" => {
                    Ok(unsafe {
                        Self::Elif(
                            <unnamed::Elif<
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
                "except" => {
                    Ok(unsafe {
                        Self::Except(
                            <unnamed::Except<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "exec" => {
                    Ok(unsafe {
                        Self::Exec(
                            <unnamed::Exec<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "finally" => {
                    Ok(unsafe {
                        Self::Finally(
                            <unnamed::Finally<
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
                "from" => {
                    Ok(unsafe {
                        Self::From(
                            <unnamed::From<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "global" => {
                    Ok(unsafe {
                        Self::Global(
                            <unnamed::Global<
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
                "import" => {
                    Ok(unsafe {
                        Self::Import(
                            <unnamed::Import<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "lambda" => {
                    Ok(unsafe {
                        Self::Lambda(
                            <unnamed::Lambda<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "match" => {
                    Ok(unsafe {
                        Self::Match(
                            <unnamed::Match<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "nonlocal" => {
                    Ok(unsafe {
                        Self::Nonlocal(
                            <unnamed::Nonlocal<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "pass" => {
                    Ok(unsafe {
                        Self::Pass(
                            <unnamed::Pass<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "print" => {
                    Ok(unsafe {
                        Self::Print(
                            <unnamed::Print<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "raise" => {
                    Ok(unsafe {
                        Self::Raise(
                            <unnamed::Raise<
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
                "try" => {
                    Ok(unsafe {
                        Self::Try(
                            <unnamed::Try<
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
                "with" => {
                    Ok(unsafe {
                        Self::With(
                            <unnamed::With<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "yield" => {
                    Ok(unsafe {
                        Self::Yield(
                            <unnamed::Yield<
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
                Self::As(x) => ::type_sitter_lib::Node::raw(x),
                Self::Assert(x) => ::type_sitter_lib::Node::raw(x),
                Self::Async(x) => ::type_sitter_lib::Node::raw(x),
                Self::Await(x) => ::type_sitter_lib::Node::raw(x),
                Self::Break(x) => ::type_sitter_lib::Node::raw(x),
                Self::Case(x) => ::type_sitter_lib::Node::raw(x),
                Self::Class(x) => ::type_sitter_lib::Node::raw(x),
                Self::Continue(x) => ::type_sitter_lib::Node::raw(x),
                Self::Def(x) => ::type_sitter_lib::Node::raw(x),
                Self::Del(x) => ::type_sitter_lib::Node::raw(x),
                Self::Elif(x) => ::type_sitter_lib::Node::raw(x),
                Self::Else(x) => ::type_sitter_lib::Node::raw(x),
                Self::Except(x) => ::type_sitter_lib::Node::raw(x),
                Self::Exec(x) => ::type_sitter_lib::Node::raw(x),
                Self::Finally(x) => ::type_sitter_lib::Node::raw(x),
                Self::For(x) => ::type_sitter_lib::Node::raw(x),
                Self::From(x) => ::type_sitter_lib::Node::raw(x),
                Self::Global(x) => ::type_sitter_lib::Node::raw(x),
                Self::If(x) => ::type_sitter_lib::Node::raw(x),
                Self::Import(x) => ::type_sitter_lib::Node::raw(x),
                Self::Lambda(x) => ::type_sitter_lib::Node::raw(x),
                Self::Match(x) => ::type_sitter_lib::Node::raw(x),
                Self::Nonlocal(x) => ::type_sitter_lib::Node::raw(x),
                Self::Pass(x) => ::type_sitter_lib::Node::raw(x),
                Self::Print(x) => ::type_sitter_lib::Node::raw(x),
                Self::Raise(x) => ::type_sitter_lib::Node::raw(x),
                Self::Return(x) => ::type_sitter_lib::Node::raw(x),
                Self::Try(x) => ::type_sitter_lib::Node::raw(x),
                Self::While(x) => ::type_sitter_lib::Node::raw(x),
                Self::With(x) => ::type_sitter_lib::Node::raw(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::As(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Assert(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Async(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Await(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Break(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Case(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Class(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Continue(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Def(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Del(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Elif(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Else(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Except(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Exec(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Finally(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::For(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::From(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Global(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::If(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Import(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Lambda(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Match(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Nonlocal(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Pass(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Print(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Raise(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Return(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Try(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::While(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::With(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Yield(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::As(x) => x.into_raw(),
                Self::Assert(x) => x.into_raw(),
                Self::Async(x) => x.into_raw(),
                Self::Await(x) => x.into_raw(),
                Self::Break(x) => x.into_raw(),
                Self::Case(x) => x.into_raw(),
                Self::Class(x) => x.into_raw(),
                Self::Continue(x) => x.into_raw(),
                Self::Def(x) => x.into_raw(),
                Self::Del(x) => x.into_raw(),
                Self::Elif(x) => x.into_raw(),
                Self::Else(x) => x.into_raw(),
                Self::Except(x) => x.into_raw(),
                Self::Exec(x) => x.into_raw(),
                Self::Finally(x) => x.into_raw(),
                Self::For(x) => x.into_raw(),
                Self::From(x) => x.into_raw(),
                Self::Global(x) => x.into_raw(),
                Self::If(x) => x.into_raw(),
                Self::Import(x) => x.into_raw(),
                Self::Lambda(x) => x.into_raw(),
                Self::Match(x) => x.into_raw(),
                Self::Nonlocal(x) => x.into_raw(),
                Self::Pass(x) => x.into_raw(),
                Self::Print(x) => x.into_raw(),
                Self::Raise(x) => x.into_raw(),
                Self::Return(x) => x.into_raw(),
                Self::Try(x) => x.into_raw(),
                Self::While(x) => x.into_raw(),
                Self::With(x) => x.into_raw(),
                Self::Yield(x) => x.into_raw(),
            }
        }
    }
    /**One of `{float | integer}`:
- [`Float`]
- [`Integer`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Number<'tree> {
        Float(Float<'tree>),
        Integer(Integer<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Number<'tree> {
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
    impl<'tree> ::type_sitter_lib::Node<'tree> for Number<'tree> {
        type WithLifetime<'a> = Number<'a>;
        const KIND: &'static str = "{float | integer}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "float" => {
                    Ok(unsafe {
                        Self::Float(
                            <Float<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "integer" => {
                    Ok(unsafe {
                        Self::Integer(
                            <Integer<
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
    /**One of `{!= | % | %= | & | &= | * | ** | **= | *= | + | += | - | -= | -> | / | // | //= | /= | := | < | << | <<= | <= | <> | = | == | > | >= | >> | >>= | @= | ^ | ^= | and | in | is | is not | not | not in | or | | | |= | ~}`:
- [`symbols::NotEq`]
- [`symbols::Mod`]
- [`symbols::ModEq`]
- [`symbols::And`]
- [`symbols::AndEq`]
- [`symbols::Mul`]
- [`symbols::MulMul`]
- [`symbols::MulMulEq`]
- [`symbols::MulEq`]
- [`symbols::Add`]
- [`symbols::AddEq`]
- [`symbols::Sub`]
- [`symbols::SubEq`]
- [`symbols::SubGt`]
- [`symbols::Div`]
- [`symbols::DivDiv`]
- [`symbols::DivDivEq`]
- [`symbols::DivEq`]
- [`symbols::ColonEq`]
- [`symbols::Lt`]
- [`symbols::LtLt`]
- [`symbols::LtLtEq`]
- [`symbols::LtEq`]
- [`symbols::LtGt`]
- [`symbols::Eq`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
- [`symbols::GtGt`]
- [`symbols::GtGtEq`]
- [`symbols::AtEq`]
- [`symbols::BitXor`]
- [`symbols::BitXorEq`]
- [`unnamed::And`]
- [`unnamed::In`]
- [`unnamed::Is`]
- [`symbols::IsSpacenot`]
- [`unnamed::Not`]
- [`symbols::NotSpacein`]
- [`unnamed::Or`]
- [`symbols::Or`]
- [`symbols::OrEq`]
- [`symbols::BitNot`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Operator<'tree> {
        NotEq(symbols::NotEq<'tree>),
        Mod(symbols::Mod<'tree>),
        ModEq(symbols::ModEq<'tree>),
        And(symbols::And<'tree>),
        AndEq(symbols::AndEq<'tree>),
        Mul(symbols::Mul<'tree>),
        MulMul(symbols::MulMul<'tree>),
        MulMulEq(symbols::MulMulEq<'tree>),
        MulEq(symbols::MulEq<'tree>),
        Add(symbols::Add<'tree>),
        AddEq(symbols::AddEq<'tree>),
        Sub(symbols::Sub<'tree>),
        SubEq(symbols::SubEq<'tree>),
        SubGt(symbols::SubGt<'tree>),
        Div(symbols::Div<'tree>),
        DivDiv(symbols::DivDiv<'tree>),
        DivDivEq(symbols::DivDivEq<'tree>),
        DivEq(symbols::DivEq<'tree>),
        ColonEq(symbols::ColonEq<'tree>),
        Lt(symbols::Lt<'tree>),
        LtLt(symbols::LtLt<'tree>),
        LtLtEq(symbols::LtLtEq<'tree>),
        LtEq(symbols::LtEq<'tree>),
        LtGt(symbols::LtGt<'tree>),
        Eq(symbols::Eq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
        GtGt(symbols::GtGt<'tree>),
        GtGtEq(symbols::GtGtEq<'tree>),
        AtEq(symbols::AtEq<'tree>),
        BitXor(symbols::BitXor<'tree>),
        BitXorEq(symbols::BitXorEq<'tree>),
        And_(unnamed::And<'tree>),
        In(unnamed::In<'tree>),
        Is(unnamed::Is<'tree>),
        IsSpacenot(symbols::IsSpacenot<'tree>),
        Not(unnamed::Not<'tree>),
        NotSpacein(symbols::NotSpacein<'tree>),
        Or(unnamed::Or<'tree>),
        Or_(symbols::Or<'tree>),
        OrEq(symbols::OrEq<'tree>),
        BitNot(symbols::BitNot<'tree>),
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
        ///Returns the node if it is of type `:=` ([`symbols::ColonEq`]), otherwise returns `None`
        #[inline]
        pub fn as_colon_eq(self) -> ::std::option::Option<symbols::ColonEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ColonEq(x) = self {
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
        ///Returns the node if it is of type `and` ([`unnamed::And`]), otherwise returns `None`
        #[inline]
        pub fn as_and_(self) -> ::std::option::Option<unnamed::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And_(x) = self {
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
        ///Returns the node if it is of type `not` ([`unnamed::Not`]), otherwise returns `None`
        #[inline]
        pub fn as_not(self) -> ::std::option::Option<unnamed::Not<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Not(x) = self {
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
        ///Returns the node if it is of type `|` ([`symbols::Or`]), otherwise returns `None`
        #[inline]
        pub fn as_or_(self) -> ::std::option::Option<symbols::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or_(x) = self {
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
    impl<'tree> ::type_sitter_lib::Node<'tree> for Operator<'tree> {
        type WithLifetime<'a> = Operator<'a>;
        const KIND: &'static str = "{!= | % | %= | & | &= | * | ** | **= | *= | + | += | - | -= | -> | / | // | //= | /= | := | < | << | <<= | <= | <> | = | == | > | >= | >> | >>= | @= | ^ | ^= | and | in | is | is not | not | not in | or | | | |= | ~}";
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
                "%" => {
                    Ok(unsafe {
                        Self::Mod(
                            <symbols::Mod<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "%=" => {
                    Ok(unsafe {
                        Self::ModEq(
                            <symbols::ModEq<
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
                "&=" => {
                    Ok(unsafe {
                        Self::AndEq(
                            <symbols::AndEq<
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
                "**" => {
                    Ok(unsafe {
                        Self::MulMul(
                            <symbols::MulMul<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "**=" => {
                    Ok(unsafe {
                        Self::MulMulEq(
                            <symbols::MulMulEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "*=" => {
                    Ok(unsafe {
                        Self::MulEq(
                            <symbols::MulEq<
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
                "/" => {
                    Ok(unsafe {
                        Self::Div(
                            <symbols::Div<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "//" => {
                    Ok(unsafe {
                        Self::DivDiv(
                            <symbols::DivDiv<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "//=" => {
                    Ok(unsafe {
                        Self::DivDivEq(
                            <symbols::DivDivEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "/=" => {
                    Ok(unsafe {
                        Self::DivEq(
                            <symbols::DivEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                ":=" => {
                    Ok(unsafe {
                        Self::ColonEq(
                            <symbols::ColonEq<
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
                "<<" => {
                    Ok(unsafe {
                        Self::LtLt(
                            <symbols::LtLt<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "<<=" => {
                    Ok(unsafe {
                        Self::LtLtEq(
                            <symbols::LtLtEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "<=" => {
                    Ok(unsafe {
                        Self::LtEq(
                            <symbols::LtEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "<>" => {
                    Ok(unsafe {
                        Self::LtGt(
                            <symbols::LtGt<
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
                ">=" => {
                    Ok(unsafe {
                        Self::GtEq(
                            <symbols::GtEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                ">>" => {
                    Ok(unsafe {
                        Self::GtGt(
                            <symbols::GtGt<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                ">>=" => {
                    Ok(unsafe {
                        Self::GtGtEq(
                            <symbols::GtGtEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "@=" => {
                    Ok(unsafe {
                        Self::AtEq(
                            <symbols::AtEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "^" => {
                    Ok(unsafe {
                        Self::BitXor(
                            <symbols::BitXor<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "^=" => {
                    Ok(unsafe {
                        Self::BitXorEq(
                            <symbols::BitXorEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "and" => {
                    Ok(unsafe {
                        Self::And_(
                            <unnamed::And<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "in" => {
                    Ok(unsafe {
                        Self::In(
                            <unnamed::In<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "is" => {
                    Ok(unsafe {
                        Self::Is(
                            <unnamed::Is<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "is not" => {
                    Ok(unsafe {
                        Self::IsSpacenot(
                            <symbols::IsSpacenot<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "not" => {
                    Ok(unsafe {
                        Self::Not(
                            <unnamed::Not<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "not in" => {
                    Ok(unsafe {
                        Self::NotSpacein(
                            <symbols::NotSpacein<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "or" => {
                    Ok(unsafe {
                        Self::Or(
                            <unnamed::Or<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "|" => {
                    Ok(unsafe {
                        Self::Or_(
                            <symbols::Or<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "|=" => {
                    Ok(unsafe {
                        Self::OrEq(
                            <symbols::OrEq<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "~" => {
                    Ok(unsafe {
                        Self::BitNot(
                            <symbols::BitNot<
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
                Self::Mod(x) => ::type_sitter_lib::Node::raw(x),
                Self::ModEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::And(x) => ::type_sitter_lib::Node::raw(x),
                Self::AndEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Mul(x) => ::type_sitter_lib::Node::raw(x),
                Self::MulMul(x) => ::type_sitter_lib::Node::raw(x),
                Self::MulMulEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::MulEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Add(x) => ::type_sitter_lib::Node::raw(x),
                Self::AddEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw(x),
                Self::SubEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::SubGt(x) => ::type_sitter_lib::Node::raw(x),
                Self::Div(x) => ::type_sitter_lib::Node::raw(x),
                Self::DivDiv(x) => ::type_sitter_lib::Node::raw(x),
                Self::DivDivEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::DivEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::ColonEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Lt(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtLt(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtLtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::LtGt(x) => ::type_sitter_lib::Node::raw(x),
                Self::Eq(x) => ::type_sitter_lib::Node::raw(x),
                Self::EqEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::Gt(x) => ::type_sitter_lib::Node::raw(x),
                Self::GtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::GtGt(x) => ::type_sitter_lib::Node::raw(x),
                Self::GtGtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::AtEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::BitXor(x) => ::type_sitter_lib::Node::raw(x),
                Self::BitXorEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::And_(x) => ::type_sitter_lib::Node::raw(x),
                Self::In(x) => ::type_sitter_lib::Node::raw(x),
                Self::Is(x) => ::type_sitter_lib::Node::raw(x),
                Self::IsSpacenot(x) => ::type_sitter_lib::Node::raw(x),
                Self::Not(x) => ::type_sitter_lib::Node::raw(x),
                Self::NotSpacein(x) => ::type_sitter_lib::Node::raw(x),
                Self::Or(x) => ::type_sitter_lib::Node::raw(x),
                Self::Or_(x) => ::type_sitter_lib::Node::raw(x),
                Self::OrEq(x) => ::type_sitter_lib::Node::raw(x),
                Self::BitNot(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Mod(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ModEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::And(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AndEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MulMul(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MulMulEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::MulEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AddEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SubEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::SubGt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Div(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DivDiv(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DivDivEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::DivEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::ColonEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtLt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtLtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::LtGt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Eq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GtGt(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::GtGtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::AtEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::BitXor(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::BitXorEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::And_(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::In(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Is(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::IsSpacenot(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Not(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::NotSpacein(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Or(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::Or_(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::OrEq(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::BitNot(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::Mod(x) => x.into_raw(),
                Self::ModEq(x) => x.into_raw(),
                Self::And(x) => x.into_raw(),
                Self::AndEq(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
                Self::MulMul(x) => x.into_raw(),
                Self::MulMulEq(x) => x.into_raw(),
                Self::MulEq(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::AddEq(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::SubEq(x) => x.into_raw(),
                Self::SubGt(x) => x.into_raw(),
                Self::Div(x) => x.into_raw(),
                Self::DivDiv(x) => x.into_raw(),
                Self::DivDivEq(x) => x.into_raw(),
                Self::DivEq(x) => x.into_raw(),
                Self::ColonEq(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::LtLt(x) => x.into_raw(),
                Self::LtLtEq(x) => x.into_raw(),
                Self::LtEq(x) => x.into_raw(),
                Self::LtGt(x) => x.into_raw(),
                Self::Eq(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::GtEq(x) => x.into_raw(),
                Self::GtGt(x) => x.into_raw(),
                Self::GtGtEq(x) => x.into_raw(),
                Self::AtEq(x) => x.into_raw(),
                Self::BitXor(x) => x.into_raw(),
                Self::BitXorEq(x) => x.into_raw(),
                Self::And_(x) => x.into_raw(),
                Self::In(x) => x.into_raw(),
                Self::Is(x) => x.into_raw(),
                Self::IsSpacenot(x) => x.into_raw(),
                Self::Not(x) => x.into_raw(),
                Self::NotSpacein(x) => x.into_raw(),
                Self::Or(x) => x.into_raw(),
                Self::Or_(x) => x.into_raw(),
                Self::OrEq(x) => x.into_raw(),
                Self::BitNot(x) => x.into_raw(),
            }
        }
    }
    /**One of `{{ | }}`:
- [`symbols::LBrace`]
- [`symbols::RBrace`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PunctuationSpecial<'tree> {
        LBrace(symbols::LBrace<'tree>),
        RBrace(symbols::RBrace<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> PunctuationSpecial<'tree> {
        ///Returns the node if it is of type `{` ([`symbols::LBrace`]), otherwise returns `None`
        #[inline]
        pub fn as_l_brace(self) -> ::std::option::Option<symbols::LBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LBrace(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `}` ([`symbols::RBrace`]), otherwise returns `None`
        #[inline]
        pub fn as_r_brace(self) -> ::std::option::Option<symbols::RBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RBrace(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter_lib::Node<'tree> for PunctuationSpecial<'tree> {
        type WithLifetime<'a> = PunctuationSpecial<'a>;
        const KIND: &'static str = "{{ | }}";
        #[inline]
        fn try_from_raw(
            node: ::yak_sitter::Node<'tree>,
        ) -> ::type_sitter_lib::NodeResult<'tree, Self> {
            match node.kind() {
                "{" => {
                    Ok(unsafe {
                        Self::LBrace(
                            <symbols::LBrace<
                                'tree,
                            > as ::type_sitter_lib::Node<
                                'tree,
                            >>::from_raw_unchecked(node),
                        )
                    })
                }
                "}" => {
                    Ok(unsafe {
                        Self::RBrace(
                            <symbols::RBrace<
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
                Self::LBrace(x) => ::type_sitter_lib::Node::raw(x),
                Self::RBrace(x) => ::type_sitter_lib::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::yak_sitter::Node<'tree> {
            match self {
                Self::LBrace(x) => ::type_sitter_lib::Node::raw_mut(x),
                Self::RBrace(x) => ::type_sitter_lib::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::yak_sitter::Node<'tree> {
            match self {
                Self::LBrace(x) => x.into_raw(),
                Self::RBrace(x) => x.into_raw(),
            }
        }
    }
}
