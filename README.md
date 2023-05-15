# type-sitter: generate typed wrappers for [tree-sitter](https://tree-sitter.github.io) grammars from node-types.json and queries

***Note:** type-sitter is still in the early stages and as such the API is subject to change.*

<!-- [![Build status](https://github.com/Jakobeha/type-sitter/workflows/ci/badge.svg)](https://github.com/Jakobeha/type-sitter/actions) -->
[![Crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg?label=type-sitter-cli)](https://crates.io/crates/type-sitter-cli)
[![Crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg?label=type-sitter-proc)](https://crates.io/crates/type-sitter-proc)
[![Crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg?label=type-sitter-gen)](https://crates.io/crates/type-sitter-gen)
[![Docs.rs](https://docs.rs/type-sitter-gen/badge.svg)](https://docs.rs/type-sitter-gen)
[![Crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg?label=type-sitter-lib)](https://crates.io/crates/type-sitter-lib)
[![Docs.rs](https://docs.rs/type-sitter-lib/badge.svg)](https://docs.rs/type-sitter-lib)

## Overview

type-sitter is a library, CLI tool, and procedural-macro which generates type-safe wrappers for tree-sitter nodes from a [tree-sitter grammar](https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types), and queries from [tree-sitter query s-expressions](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries).

These wrappers contain methods to access the node's fields and children, and query's captures, as well as pattern-matching and selectors for union and supertype nodes. They even have documentation! The wrappers also encourage good practices by explicitly handling "error" and "extra" nodes, so you won't forget; but also provide convenience methods like [`unwrap2()`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/trait.NodeResultExtraOrExt.html#tymethod.unwrap2) and [`flatten()`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/trait.NodeResultExtraOrExt.html#tymethod.flatten) to ease some of the verbosity.

### tree-sitter-wrapper

Additionally, in `type-sitter-lib`, the [`tree-sitter-wrapper`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/tree_sitter_wrapper) feature/module provides *another* wrapper over tree-sitter nodes and queries. This wrapper is general-purpose: it provides the ability to get node's text directly from the node itself as a `&str`, assign arbitrary bitmasks ("marks") to each node, and other convenience methods. It also makes [`QueryMatches`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/tree_sitter_wrapper/struct.QueryMatches.html) a streaming iterator [because it's not a real iterator](https://github.com/tree-sitter/tree-sitter/issues/608) ([`TypedQueryMatches`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/struct.TypedQueryMatches.html) is also only a streaming iterator).

`type-sitter` can generate typed wrappers for `tree_sitter_wrapper::Node` *or* plain `tree_sitter::Node`. Using the library `type-sitter-gen`, have much more customization options, including the ability to generate nodes which wrap an arbitrary `...::Node` datatype.

## Drawbacks

`type-sitter`'s main drawback is that as of now, the generated wrapper code is very large: the generated node wrappers for `tree-sitter-rust` are 33217 LOC. There are potential future steps to reduce code size such as replacing enums with generic types, but these have their own drawbacks (more complex resolution, may not be effective). Though on my M1 Macbook Air running IntelliJ, building and IntelliJ code analysis is still pretty fast: cold starts are a few seconds, incremental builds are <1 second and hints are not sluggish. Your mileage may vary.

Another issue is that some grammars may generate duplicate datatype definitions (see [Naming Rules](#naming-rules)): this should be very uncommon because it will only happen if their names are weirdly similar, but if it does, there is currently no workaround as there is no way to rename generated datatypes.

`tree-sitter-wrapper`'s drawbacks are slightly lower performance and limitations. The lower performance is because `tree-sitter-wrapper` requires all nodes and most other datatypes to have an additional reference to `tree_sitter_wrapper::Tree` for capabilities like the ability for nodes to get their own text. The limitations are that `tree_sitter_wrapper` requires the parsed source code to be UTF-8 compliant and be stored along with the tree itself, and certain aspects of incremental parsing may not work with the convenience methods (this is planned to be fixed to be fixed in the future).

Lastly, keep in mind that both libraries are still in the early stages of development, so they will have bugs and API may change (though will try to preserve naming rules).

## Naming Rules

`type-sitter` generates datatype based on the names of the nodes in the grammar. However, these nodes are in snake-case and contain punctuation which is illegal in Rust, so we convert them to camel-case and perform the following illegal-character substitutions:

- `&` ⇒ `And`
- `|` ⇒ `Or`
- `!` ⇒ `Not`
- `=` ⇒ `Eq`
- `<` ⇒ `Lt`
- `>` ⇒ `Gt`
- `+` ⇒ `Add`
- `-` ⇒ `Sub`
- `*` ⇒ `Mul`
- `/` ⇒ `Div`
- `~` ⇒ `BitNot`
- `%` ⇒ `Mod`
- `^` ⇒ `BitXor`
- `?` ⇒ `Question`
- `:` ⇒ `Colon`
- `.` ⇒ `Dot`
- `,` ⇒ `Comma`
- `;` ⇒ `Semicolon`
- `(` ⇒ `LParen`
- `)` ⇒ `RParen`
- `[` ⇒ `LBracket`
- `]` ⇒ `RBracket`
- `{` ⇒ `LBrace`
- `}` ⇒ `RBrace`
- `\` ⇒ `Backslash`
- `'` ⇒ `Quote`
- `"` ⇒ `DoubleQuote`
- `#` ⇒ `Hash`
- `@` ⇒ `At`
- `$` ⇒ `Dollar`
- `` ` `` ⇒ `Backtick`
- ` ` ⇒ `Space`
- `\t` ⇒ `Tab`
- `\n` ⇒ `Newline`
- `\r` ⇒ `CarriageReturn`
- Any other character ⇒ `U` + the character's Unicode codepoint in upper-hex

For method names (variant selectors), we simply convert back to snake case.

Additionally, if a node is implicit (starts with `_`), we remove the prepended `_`

Lastly, if a type or method name is an illegal definition identifier (`Self`, `self`, `super`, `crate`, `_`, or anything which starts with a number), `type-sitter` prepends an `_`. If it's a Rust keyword, `type-sitter` prepends `r#`.

Naming rules also determine the module. Unnamed nodes and symbols are in modules specifically to reduce naming conflicts without having to actually rename the nodes.

- Unnamed and contains symbols: `symbol::`
- Unnamed and doesn't contain symbols: `unnamed::`
- Otherwise the node is at the toplevel of the generated source

The source for all this is at [`type-sitter-gen/src/names.rs`](type-sitter-gen/src/names.rs).

### Naming Rule Examples

- `_declaration_statement` ⇒ `DeclarationStatement`
- `use_declaration` ⇒ `UseDeclaration`
- `self` ⇒ `unnamed::_Self`
- `%` ⇒ `symbols::Mod`
- `mod` ⇒ `unnamed::Mod`
- `true` selector ⇒ `r#true` (`true` ⇒ `unnamed::True`)

### Query Capture Naming Rules

Query capture naming rules are the exact same as node rules, except that in captures, `.` is interpreted as `_` when converting to camel-case (e.g. `method.definition` => `MethodDefinition` and `method_definition`).

## Example

```rust
use tree_sitter::{Parser, Tree};
use type_sitter_lib::{Either2, TypedNode};

pub fn get_import_paths_unsafe(tree: &Tree, text: &str) -> Vec<String> {
    // BAD: what if we spell the field names wrong?
    tree.root_node().children(&mut tree.walk())
        .filter(|n| n.kind() == "use_declaration")
        .filter_map(|n| n.child_by_field_name("argument"))
        .filter_map(|n| n.child_by_field_name("path"))
        .filter_map(|n| n.utf8_text(text.as_bytes()))
        .map(|s| s.to_string())
        .collect()
}

pub fn get_import_paths_safe(tree: &Tree, text: &str) -> Vec<String> {
    // GOOD: fields are type-safe, and we get IDE inference
    rust::SourceFile::try_from(tree.root_node()).unwrap().children(&mut tree.walk())
        .filter_map(|n| n.declaration_statement())
        .filter_map(|n| n.use_declaration())
        .filter_map(|n| n.argument())
        .filter_map(|n| n.scoped_identifier())
        .filter_map(|n| n.path().flatten())
        .filter_map(|n| n.identifier())
        .filter_mao(|n| n.utf8_text(code_str.as_bytes()))
        .map(|s| s.to_string())
        .collect()
}

// We can also define methods which only take nodes of certain types
pub fn process_declaration(decl: rust::DeclarationStatement<'_>) {
    // ...
}
```

## Usage

In order to generate the bindings, you can either invoke `type-sitter-cli` directly, or use the procedural macros in `type-sitter-proc`. The CLI tool is recommended, as it's more flexible and will give your IDE better inference.

The generated code depends on `type-sitter-lib`, so you must include `type-sitter-lib` as a dependency.

### Basic usage

```shell
# If not already installed
cargo install type-sitter-cli
# In your cargo project root directory
type-sitter-cli path/to/tree-sitter-foobar-lang
# To add type-sitter-lib as a dependency (also in cargo root)
cargo add type-sitter-lib
```

#### Advanced usage

```shell
# Add type-sitter-lib with the tree-sitter-wrapper feature (see above section)
cargo add type-sitter-lib --features tree-sitter-wrapper
# Specify a custom output directory and use tree-sitter-wrapper
type-sitter-cli vendor/tree-sitter-foobar-lang/node-types.json -o generated_src --use-wrapper
# Generate only node-types or queries
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src/rust_nodes.rs --use-wrapper
type-sitter-cli vendor/tree-sitter-rust/queries -o generated_src/rust_queries.rs --use-wrapper
# You can generate bindings for multiple grammars in the same project
type-sitter-cli vendor/tree-sitter-typescript/node-types.json -o generated_src --use-wrapper
# To see help for the CLI program
type-sitter-cli --help
```

## Comparison to [rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter)

[rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter) is the primary alternative which also provides convenience over tree-sitter's Rust API. However, rust-sitter takes a much different approach by fully generating the tree-sitter grammar from a Rust file.

Advantages of type-sitter:

- arbitrary tree-sitter grammars, not only ones written in Rust
- Error node and incremental parsing support, since typed nodes directly wrap `tree-sitter` nodes
- Less API difference from the native tree-sitter API: if you don't use `tree-sitter-wrapper` feature it only provides typed wrappers for nodes, and even `tree-sitter-wrapper` really only provides convenience methods on top of the base API
- Less complexity because of the above

Advantages of rust-sitter:

- More control over the typed nodes, since you define them yourself
- May generate less boilerplate especially because of the extra control
- Less verbosity since extra and error nodes are implicitly handled
- type-sitter is in the much earlier stages

## Contributing

Feel free to submit an issue or pull request if you want a new feature or anything is missing, and don't hesitate to submit an issue if you encounter any bugs or have any questions.

## Licence

The code is licensed under MIT or Apache 2.0 (you choose), which is the norm for Rust packages.
