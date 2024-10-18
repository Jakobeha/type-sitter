# yak-sitter: opinionated tree-sitter facade which lets you store data visible to nodes and provides other improvements

[![crates.io](https://img.shields.io/crates/v/yak-sitter.svg)](https://crates.io/crates/yak-sitter)
[![docs.rs](https://img.shields.io/docsrs/yak-sitter)](https://docs.rs/yak-sitter)

This library provides an API almost identical to [`tree-sitter`](https://crates.io/crates/tree-sitter), but with the following changes:

- Trees can optionally store `Custom` data of arbitrary type, which each node can access via shared reference. Typically the custom data will contain a map of nodes to additional metadata, or a diagnostic logger (assuming all diagnostics have at least one source location). Then, you won't need to pass around this extra information to functions which deal with nodes, you can retrieve it from the nodes directly.
- Trees store their original source code, and nodes can access their text
- Trees also store the path they were parsed from, and nodes can access this path
- Nodes are ordered based on their location in the tree, and nodes from different trees are ordered based on their paths.
- `TreeCursor` is split into [`LocalTreeCursor`](https://docs.rs/yak-sitter/latest/yak_sitter/struct.LocalTreeCursor.html) and [`GlobalTreeCursor`](https://docs.rs/yak-sitter/latest/yak_sitter/struct.GlobalTreeCursor.html). When `GlobalTreeCursor` is reset to a node it can still go to parents and siblings, albeit with a performance penalty. `LocalTreeCursor` is the original [`TreeCursor`](https://docs.rs/tree-sitter/latest/tree_sitter/struct.TreeCursor.html) [since its behavior a bit confusing](https://github.com/tree-sitter/tree-sitter/issues/567).
- All parsed text must be valid UTF-8

The library is an attempt to integrate tree-sitter with an existing project (https://github.com/Jakobeha/nominalscript), hence the [name](https://www.techtarget.com/whatis/definition/yak-shaving). Regardless, if you want any changes or have any suggestions, feel free to submit a github issue or PR