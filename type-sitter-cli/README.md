# type-sitter-cli: Command-line utility to generate source files with type-sitter wrappers

[![crates.io](https://img.shields.io/crates/v/type-sitter-cli.svg)](https://crates.io/crates/type-sitter-cli)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for background information,
and [type-sitter\#cli-tool](https://github.com/Jakobeha/type-sitter#cli-tool) for a guide on how to use that applied to
most cases. This is the command-line tool which generates typed-safe wrappers for tree-sitter nodes.

## Advanced usage

Remember, if `-o` isn't specified it defaults to `src/type_sitter`.

```shell
# Generate bindings for the `yak-sitter` feature
type-sitter-cli vendor/tree-sitter-foobar-lang --use-yak-sitter
echo "pub mod foobar_lang;\n" > src/type_sitter.rs
# You can generate bindings for multiple grammars in the same directory
type-sitter-cli vendor/tree-sitter-typescript --use-yak-sitter
echo "pub mod typescript;\n" >> src/type_sitter.rs
# Generate only node types or queries
type-sitter-cli vendor/tree-sitter-rust/src/node-types.json --use-yak-sitter
echo "pub mod rust;\n" >> src/type_sitter.rs  # src/type_sitter/rust.rs contains only node types
type-sitter-cli vendor/tree-sitter-json/queries/json --use-yak-sitter
echo "pub mod json;\n" >> src/type_sitter.rs  # src/type_sitter/json.rs contains only queries
# To see help for the CLI program
type-sitter-cli --help
```

## Issues

- **Q:** I can't import the generated sources.
- **A:** `type-sitter-cli` doesn't generate the root module for generated sources itself, you must do so manually. e.g.
  if the root module is the default (`src/type_sitter`), either create `src/type_sitter.rs` or `src/type_sitter/mod.rs`
  and add `pub mod <my_language>;` to it.

<br/>

- **Q:** I get build errors for missing symbols within `type_sitter`.
- **A:** Make sure you have the `type-sitter` dependency in your `Cargo.toml`. Also, make sure `use-default-features` is
  set to `false`, because the default features includes the proc macros, which the CLI is an alternative to.

<br/>

- **Q:** I get build errors for symbols within `tree-sitter`.
- **A:** Make sure you are using tree-sitter `0.22`.
