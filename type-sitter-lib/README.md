# type-sitter-lib: type-sitter code for generated / downstream code

[![Crates.io](https://img.shields.io/crates/v/type-sitter-lib.svg)](https://crates.io/crates/type-sitter-lib)
[![Docs.rs](https://docs.rs/type-sitter-lib/badge.svg)](https://docs.rs/type-sitter-lib)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This is the library which code generated from type-sitter depends on.

This library also contains the [`tree-sitter-wrapper`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/tree_sitter_wrapper) feature.module, enabled by default, which provides lightweight general-purpose wrappers over tree-sitter data-structures, to the extent you won't need to explicitly `use tree-sitter` at all. These wrappers have convenience functions like getting a node's text as a `&str` and assigning arbitrary bitmasks ("marks") to each node, at the cost of slightly lower performance. `tree-sitter-wrapper` also has a [`QueryMatches`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/tree_sitter_wrapper/struct.QueryMatches.html) which is a streaming iterator [because it's not a real iterator](https://github.com/tree-sitter/tree-sitter/issues/608).

**Documentation Note:** All documentation is generated with the `tree-sitter-wrapper` flag enabled, which is enabled by default