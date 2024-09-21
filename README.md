# type-sitter: generate typed wrappers for [tree-sitter](https://tree-sitter.github.io) nodes and queries

***Note:** type-sitter is in alpha, therefore the API is subject to change.*

[![Build status](https://github.com/Jakobeha/type-sitter/actions/workflows/ci.yml/badge.svg)](https://github.com/Jakobeha/type-sitter/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/type-sitter)](https://crates.io/crates/type-sitter)
[![docs.rs | type-sitter-lib](https://img.shields.io/docsrs/type-sitter-lib?label=docs%20%7C%20type-sitter-lib)](https://docs.rs/type-sitter-lib)
[![docs.rs | type-sitter-gen](https://img.shields.io/docsrs/type-sitter-gen?label=docs%20%7C%20type-sitter-gen)](https://docs.rs/type-sitter-gen)
[![docs.rs | yak-sitter](https://img.shields.io/docsrs/yak-sitter?label=docs%20%7C%20yak-sitter)](https://docs.rs/yak-sitter)

## Overview

`type-sitter` generates type-safe wrappers for tree-sitter nodes and queries in a specific language. Nodes are generated from [`node-types.json`](https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types), and queries from [query s-expressions](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries).

"Type-safe" here means that:

- Instead of representing all tree-sitter nodes by [`tree_sitter::Node`](https://docs.rs/tree-sitter/latest/tree_sitter/struct.Node.html), each node type has its own data-type which wraps `tree_sitter::Node`.
  - Nodes with supertypes are `enum`s, so their 
  - Each node data-type implements [`type_sitter::Node`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/trait.Node.html). You can use generics and convert to/from [`type_sitter::UntypedNode`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/struct.UntypedNode.html) to write methods that take or return arbitrary-typed nodes.
- Instead of accessing fields by `field("field_name")`, you access by specific methods like `field_name()`.
  - These methods, and every other generated method, also return typed nodes.
- Queries also have their own data-types, you access captures by specific methods instead of `capture("capture_name")`, and query methods return typed nodes.

`type-sitter` has other useful features:

- Typed error, missing, and extra nodes.
- From a typed node you can lookup the "extra" nodes before and after, e.g. to handle comments.
- [`Option<NodeResult<'_>>.flatten()`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/trait.NodeResultExtraOrExt.html#tymethod.flatten).

Lastly, there's an optional feature, `yak-sitter`, which re-exports the `tree-sitter` API with a few small changes, most notably nodes being able to access their text and filepath directly. The [`yak-sitter`](./yak-sitter/README.md) library is a drop-in replacement for `tree-sitter` and can by used by itself without `type-sitter` (and `yak-sitter` is optional in `type-sitter`).

## Usage

There are three ways to use `type-sitter`: procedural macros, build script, or the CLI tool. Procedural macros is the easiest. Build script is much faster because it's less redundant. The CLI tool is as fast as the build script, and lets you edit the generated code, but requires you to run it manually.

Every method requires that you **vendor** the tree-sitter grammar you want to generate bindings for: you cannot just include it as a dependency in `Cargo.toml`, because the node generator needs a hard-coded (relative) path to the grammar's `node-types.json`, and the query generator needs a hard-coded path to the grammar's root folder (containing `src/node-types.json`), which must also contain a built shared object (at `build/tree_sitter_foobar_binding.dylib` or `build/tree_sitter_foobar_binding.so`).

### Procedural macros

```shell
cargo add type-sitter  # Or add to Cargo.toml manually
```

To generate typed nodes:

```rust
// Assume this code is in `src/foobar_nodes.rs`
use type_sitter_proc::generate_nodes;

generate_nodes! {
    // Replace this with the path to the node-types.json file
    "vendor/path/to/tree-sitter-foobar-lang/src/node-types.json"
}
```

To generate typed queries:

```rust
// Assume this code is in `src/foobar_queries.rs`
use type_sitter_proc::generate_queries;

generate_queries! {
    // Replace this with the path to the queries folder
    "vendor/path/to/tree-sitter-foobar-lang/src/queries",
    // Replace this with the path to the grammar's root
    "vendor/path/to/tree-sitter-foobar-lang/src",
    // Replace with a different path if the nodes don't exist in a sibling module named `foobar_nodes`.
    super::foobar_nodes,
}
```

### Build script

```shell
cargo add type-sitter --no-default-features  # Or add to Cargo.toml manually
cargo add --build type-sitter-gen  # Notice `cargo add --build`
```

Then, in `build.rs`

```rust
use std::fs;
use type_sitter_gen::{generate_nodes, generate_queries, super_nodes};

fn main() {
    // Common setup
    let out_dir = Path::new(env::var_os("OUT_DIR").unwrap());
    println!("cargo::rerun-if-changed=build.rs");

    // Obligatory: in this and future lines, replace `vendor/path/to/tree-sitter-foobar-lang`
    // with the path to your grammar's folder, relative to the folder containing `Cargo.toml`
    println!("cargo::rerun-if-changed=vendor/path/to/tree-sitter-foobar-lang");
    
    // To generate nodes
    let dest_path = out_dir.join("nodes.rs");
    fs::write(
        &dest_path,
        generate_nodes("vendor/path/to/tree-sitter-foobar-lang/src/node-types.json").unwrap().to_string()
    ).unwrap();
  
    // To generate queries
    let dest_path = out_dir.join("queries.rs");
    fs::write(
        &dest_path,
        generate_queries(
            "vendor/path/to/tree-sitter-foobar-lang/queries",
            "vendor/path/to/tree-sitter-foobar-lang",
            // Replace with a different `syn::Path` if the nodes don't exist in a subling to `dest_path` named `nodes`
            &super_nodes(),
            // Replace with `true` if you are using the `yak-sitter` feature (by default, no)
            false
        ).unwrap().to_string()
    ).unwrap();
}
```

### CLI tool

```shell
cargo add type-sitter --no-default-features  # Or add to Cargo.toml manually
cargo add --dev type-sitter-cli  # Notice `cargo add --dev`
```

Then, *manually* generate typed nodes and queries with the CLI tool:

```shell
# Replace `vendor/path/to/tree-sitter-foobar-lang` and `src/parent/of/generated/module` with the path to the grammar's
# root folder (containing `src/node-types.json` and `queries`) and the directory where you want the generated module's
# source files to be placed, respectively.
> cargo run -p type-sitter-cli vendor/path/to/tree-sitter-foobar-lang -o src/parent/of/generated/module
```

Additionally, you must pass `--use-yak-sitter` if the `yak-sitter` feature is enabled. If you skip `-o`, it defaults to `src/type_sitter`.

Alternatively, instead of the path to the grammar's root folder, if you specify the path to the `node-types.json` directly, the CLI tool will only generate node types; or if you specify the path to the `queries` directory, it will only generate queries.

A downside with the CLI approach is that you need to manually re-generate the nodes if the grammar changes. An upside is that, if you know the grammar won't change and you won't have to manually re-generate, you can edit the generated code and the edits will persist.

## Example

```rust
pub fn get_import_paths_untyped<'a>(source: &'a str, tree: &tree_sitter::Tree) -> Vec<&'a str> {
    // BAD: what if we spell the field names wrong? What if a new variant is added with the same field name? 
    tree.root_node().children(&mut tree.walk())
        .filter(|n| n.kind() == "use_declaration")
        .filter_map(|n| n.child_by_field_name("argument"))
        .filter_map(|n| n.child_by_field_name("path"))
        .filter_map(|n| n.utf8_text(source.as_bytes()).unwrap())
        .collect()
}

pub fn get_import_paths_typed<'a>(source: &'a str, tree: &type_sitter::Tree<rust::SourceFile<'static>>) -> Vec<&'a str> {
    // GOOD: fields are type-safe, variant selectors are explicit, and we get IDE inference
    tree.root_node().unwrap().children(&mut tree.walk())
        .filter_map(|n| n.as_declaration_statement().and_then(|n| n.as_use_declaration()))
        .filter_map(|n| n.argument().map(|r| r.unwrap()))
        .filter_map(|n| n.as_scoped_identifier())
        .filter_map(|n| n.path().map(|r| r.unwrap()))
        .filter_map(|n| n.as_identifier())
        .filter_map(|n| n.utf8_text(source.as_bytes()).unwrap())
        .collect()
}

// We can also define methods which only take nodes of certain types
pub fn process_declaration(decl: rust::DeclarationStatement<'_>) {
    // ...
}
```

## Drawbacks

Be aware that the generated wrapper code is very large: the [generated node wrappers for `tree-sitter-rust`](type-sitter-lib/tests/rust/nodes.rs) are 32180 LOC, and [queries](type-sitter-lib/tests/rust/queries.rs) are 6929 LOC. In my usage it seems analyzers are pretty good at handling this, but it's something to keep in mind.

## Naming Rules

`type-sitter` generates data-types based on the names of the nodes in the grammar. However, these nodes are in snake-case and contain punctuation which is illegal in Rust, so we convert them to camel-case and perform the following illegal-character substitutions:

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
- Any other character ⇒ `U` + the character's Unicode codepoint in upper-hex.

For method names (variant selectors), we simply convert back to snake case.

Additionally, if a node is implicit (starts with `_`), we remove the prepended `_`.

If a type or method name is an illegal definition identifier (`Self`, `self`, `super`, `crate`, `_`, or anything which starts with a number), `type-sitter` prepends an `_`. If it's a Rust keyword, `type-sitter` prepends `r#`.

Naming rules also determine the module. Unnamed nodes and symbols are in modules specifically to reduce naming conflicts without having to actually rename the nodes.

- Unnamed and contains symbols: `symbol::`.
- Unnamed and doesn't contain symbols: `unnamed::`.
- Otherwise the node is at the toplevel of the generated source.

Lastly, if a type, method, or variant, after applying the conversions, has the same name as another, it will have an underscore appended to disambiguate it. For example, if there are two unnamed nodes `Fn` and `fn`, one of them will have type `Fn`, and the other will have type `Fn_`. You can see which node is which by looking at the documentation, which contains the original tree-sitter name.

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

## Comparison to [rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter)

[rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter) is the primary alternative which also provides convenience over tree-sitter's Rust API. However, rust-sitter takes a much different approach by fully generating the tree-sitter grammar from a Rust file.

Advantages of type-sitter:

- arbitrary tree-sitter grammars, not only ones written in Rust
- Error node and incremental parsing support, since typed nodes directly wrap `tree-sitter` nodes
- Less API difference from the native tree-sitter API: if you don't use the `yak-sitter` feature it only provides typed wrappers for nodes (and even `yak-sitter` isn't much different)
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
