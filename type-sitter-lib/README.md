# type-sitter-lib: type-sitter code for generated / downstream code

[![Crates.io](https://img.shields.io/crates/v/type-sitter-lib.svg)](https://crates.io/crates/type-sitter-lib)
[![Docs.rs](https://docs.rs/type-sitter-lib/badge.svg)](https://docs.rs/type-sitter-lib)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This is the library code generated from type-sitter depends on. It also contains the `tree-sitter-wrapper` feature, which provides a lightweight general-purpose wrapper over tree-sitter nodes: these wrappers have convenience functions like getting a node's text as a `&str` and assigning arbitrary bitmasks ("marks") to each node, at the cost of slightly lower performance.