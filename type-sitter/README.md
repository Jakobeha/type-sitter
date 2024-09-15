# type-sitter: generate typed wrappers for [tree-sitter](https://tree-sitter.github.io) grammars from node-types.json and queries

[![Crates.io](https://img.shields.io/crates/v/type-sitter.svg)](https://crates.io/crates/type-sitter)

See [the monorepo workspace](https://github.com/Jakobeha/type-sitter#readme) for more information. This crate bundles [`type-sitter-lib`](https://crates.io/crates/type-sitter-lib), [`type-sitter-proc`](https://creates.io/crates/type-sitter-proc), and either [`yak-sitter`](https://crates.io/crates/yak-sitter) (if you enable the `yak-sitter` feature) or [`tree-sitter`](https://crates.io/tree-sitter) (otherwise), so you can depend on only it to use type-sitter and tree-sitter in your project. (It also includes [`StreamingIterator`](https://docs.rs/streaming-iterator/0.1.9/streaming_iterator/trait.StreamingIterator.html) and related traits that are necessary to iterate queries).

Note that you can't generate multiple node-type or query wrappers in the same module, due to the fact that they will each generate their own submodules with the same name. You have to put the wrapper-generating macros in separate modules and then `pub use` the generated definitions. Sorry!

## Usage

```ignore
use type_sitter::node_types;
use type_sitter::queries;

generate_nodes!("vendor/tree-sitter-json/src/node-types.json")

mod queries {
    generate_queries!("vendor/tree-sitter-json/queries", "vendor/tree-sitter-json", super);
}

// TODO: Example using the generated types
```


