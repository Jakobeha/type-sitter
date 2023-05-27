# type-sitter-cli: Command-line utility to generate source files with type-sitter wrappers

[![Crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg)](https://crates.io/crates/type-sitter-cli)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This is the command-line tool which generates typed-safe wrappers for tree-sitter nodes.

If a project depends on this (and as with [type-sitter-proc](https://docs.rs/type-sitter-proc/latest/type_sitter_proc)), it *must also* depend on [type-sitter-lib](https://crates.io/crates/type-sitter-lib) and [tree-sitter](https://crates.io/crates/tree-sitter).

## Usage

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
# Add type-sitter-lib with the yak-sitter feature (see above section)
cargo add type-sitter-lib --features yak-sitter
# Specify a custom output directory and use yak-sitter
type-sitter-cli vendor/tree-sitter-foobar-lang/node-types.json -o generated_src --use-yak-sitter
# Specify a custom tree-sitter facade
type-sitter-cli vendor/tree-sitter-foobar-lang/node-types.json -o generated_src --use-yak-sitter --facade "crate::my_tree_sitter"
# Generate only node-types or queries
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src/rust_nodes.rs --use-yak-sitter
type-sitter-cli vendor/tree-sitter-rust/queries -o generated_src/rust_queries.rs --use-yak-sitter
# You can generate bindings for multiple grammars in the same project
type-sitter-cli vendor/tree-sitter-typescript/node-types.json -o generated_src --use-yak-sitter
# To see help for the CLI program
type-sitter-cli --help
```