# type-sitter-cli: Command-line utility to generate source files with type-sitter wrappers

[![Crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg)](https://crates.io/crates/type-sitter-cli)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This is the command-line tool which generates typed-safe wrappers for tree-sitter nodes.

If a project depends on this (and as with [type-sitter-proc](https://docs.rs/type-sitter-proc/latest/type_sitter_proc)), it *must also* depend on [type-sitter-lib](https://crates.io/crates/type-sitter-lib) and [tree-sitter](https://crates.io/crates/tree-sitter).

## Usage

Be aware that this will generate 

```shell
# If not already installed
cargo install type-sitter-cli
# In your cargo project root directory
type-sitter-cli path/to/tree-sitter-foobar-lang
# The default output directory is `src/type_sitter`. You'll need to create the root module of this directory yourself
echo "pub mod foobar_lang;\n" > src/type_sitter.rs
# To add type-sitter-lib as a dependency (also in cargo root)
cargo add type-sitter-lib
```

### Advanced usage

```shell
# Add type-sitter-lib with the yak-sitter feature (see above section)
cargo add type-sitter-lib --features yak-sitter
# Specify a custom output directory and use yak-sitter
type-sitter-cli vendor/tree-sitter-foobar-lang -o generated_src --use-yak-sitter
echo "pub mod foobar_lang;\n" > generated_src.rs
# Specify a custom tree-sitter facade
type-sitter-cli vendor/tree-sitter-foobar-lang -o generated_src --use-yak-sitter --facade "crate::my_tree_sitter"
# You can generate bindings for multiple grammars in the same directory
type-sitter-cli vendor/tree-sitter-typescript -o generated_src --use-yak-sitter
echo "pub mod typescript;\n" >> generated_src.rs
# Generate only node types or queries
type-sitter-cli vendor/tree-sitter-rust/src/node-types.json -o generated_src --use-yak-sitter
echo "pub mod rust;\n" >> generated_src.rs  # generated_src/rust.rs contains only node types
type-sitter-cli vendor/tree-sitter-json/queries -o generated_src/json --use-yak-sitter
echo "pub mod json;\n" >> generated_src.rs  # generated_src/json.rs contains only queries
# To see help for the CLI program
type-sitter-cli --help
```

## Issues

- **Q:** I can't import the generated sources.
- **A:** `type-sitter-cli` doesn't generate the root module for generated sources itself, you must do so manually. e.g. if the root module is the default (`src/type_sitter`), either create `src/type_sitter.rs` or `src/type_sitter/mod.rs` and add `pub mod <my_language>;` to it.


- **Q:** I get build errors for missing symbols within `type_sitter-lib`.
- **A:** Make sure you have the `type-sitter-lib` dependency in your `Cargo.toml`


- **Q:** I get build errors for symbols within `tree-sitter`.
- **A:** Make sure you are using tree-sitter `0.22`.
