# type-sitter: generate typed wrappers for tree-sitter grammars from node-types.json and (WIP) ~~queries~~

***Note:** type-sitter is still in the very early stages and as such the API is subject to change.*

## Overview

type-sitter is a library, CLI tool, and procedural-macro which generates type-safe wrappers for tree-sitter nodes from a tree-sitter grammar. These wrappers contain methods to access the node's fields and children, and nodes with subtypes are represented as `enum`s. The wrappers also encourage good practices by explicitly handling "error" and "extra" nodes, while also providing convenience methods like subtype selectors and `flatten` to ease some of the verbosity.

### tree-sitter-wrapper

Also, in `type-sitter-lib`, the `tree-sitter-wrapper` feature provides another wrapper over tree-sitter nodes. This wrapper is more general-purpose, and allows you to get node's text directly from the node itself as a `&str`, and assign arbitrary bitmasks ("marks") to each node, among other convenience methods, at the cost of slightly lower performance. `type-sitter` can generate typed nodes which use these wrappers, or it can generate nodes which only wrap `tree_sitter::Node`.

## Usage

In order to generate the bindings, you can either invoke `type-sitter-cli` directly, or use the procedural macros in `type-sitter-proc`. The CLI tool is recommended, as it's more flexible and will give your IDE better inference.

The generated code depends on `type-sitter-lib`, so you must include `type-sitter-lib` as a dependency.

### Basic usage

```shell
# If not already installed
cargo install type-sitter-cli
# In your cargo project root directory
type-sitter-cli path/to/node-types.json
# To add type-sitter-lib as a dependency (also in cargo root)
cargo add type-sitter-lib
```

#### Advanced usage

```shell
# Add type-sitter-lib with the tree-sitter-wrapper feature (see above section)
cargo add type-sitter-lib --features tree-sitter-wrapper
# Specify a custom output directory and use tree-sitter-wrapper
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src --use-wrapper
# You can generate bindings for multiple grammars in the same project
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src --use-wrapper
# To see help for the CLI program
type-sitter-cli --help
```

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
    // GOOD: fields are type-safe
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
```

## Comparison to [rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter)

[rust-sitter](https://www.shadaj.me/writing/introducing-rust-sitter) is the primary alternative which also provides convenience over tree-sitter's Rust API. However, rust-sitter takes a much different approach by fully generating the tree-sitter grammar from a Rust file.

Advantages of type-sitter:

- arbitrary tree-sitter grammars, not only ones written in Rust
- Error node and incremental parsing support, since typed nodes directly wrap `tree-sitter` nodes
- Much more lightweight at runtime
- Less API difference from the native tree-sitter API: if you don't use `tree-sitter-wrapper` feature it only provides typed wrappers for nodes, and even `tree-sitter-wrapper` really only provides convenience methods on top of the base API

Advantages of rust-sitter:

- More control over the typed nodes, since you define them yourself
- May generate less boilerplate especially because of the extra control
- type-sitter is in the much earlier stages, and it's more likely to have bugs and API changes

## Contributing

Feel free to submit an issue or pull request if you want a new feature or anything is missing, and don't hesitate to submit an issue if you encounter any bugs or have any questions.

## Licence

The code is licensed under MIT or Apache 2.0 (you choose), which is the norm for Rust packages.
