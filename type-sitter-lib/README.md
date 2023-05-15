# type-sitter-lib: type-sitter code for generated / downstream code

[![Crates.io](https://img.shields.io/crates/v/type-sitter-lib.svg)](https://crates.io/crates/type-sitter-lib)
[![Docs.rs](https://docs.rs/type-sitter-lib/badge.svg)](https://docs.rs/type-sitter-lib)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This is the library which code generated from type-sitter depends on.

This library also contains the [`tree-sitter-wrapper`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/tree_sitter_wrapper) feature.module, which provides a lightweight general-purpose wrapper over tree-sitter nodes. These wrappers have convenience functions like getting a node's text as a `&str` and assigning arbitrary bitmasks ("marks") to each node, at the cost of slightly lower performance. `tree-sitter-wrapper` also has a [`QueryMatches`](https://docs.rs/type-sitter-lib/latest/type_sitter_lib/tree_sitter_wrapper/struct.QueryMatches.html) which is a streaming iterator [because it's not a real iterator](https://github.com/tree-sitter/tree-sitter/issues/608).