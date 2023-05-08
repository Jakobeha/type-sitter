# type-sitter-proc: generate type-sitter wrappers via procedural macros

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This provides procedural macros which will generate typed node wrappers. However, it's highly recommended to use [type-sitter-cli](https://crates.io/crates/type-sitter-cli) instead, as it's more tested and will give your IDE better inference.

If a project depends on this (and as with [type-sitter-cli](https://crates.io/crates/type-sitter-cli)), it *must also* depend on [type-sitter-lib](https://crates.io/crates/type-sitter-lib) and [tree-sitter](https://crates.io/crates/tree-sitter).