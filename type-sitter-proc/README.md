# type-sitter-proc: generate type-sitter wrappers via procedural macros

[![Crates.io](https://img.shields.io/crates/v/type-sitter-proc.svg)](https://crates.io/crates/type-sitter-proc)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This provides procedural macros which will generate typed node wrappers. However, it's highly recommended to use [type-sitter-cli](https://crates.io/crates/type-sitter-cli) instead, as it's more tested and will give your IDE at least as good inference.

If a project depends on this (and as with [type-sitter-cli](https://crates.io/crates/type-sitter-cli)), it *must also* depend on [type-sitter-lib](https://crates.io/crates/type-sitter-lib) and [tree-sitter](https://crates.io/crates/tree-sitter).

Note that you can't generate multiple node-type or query wrappers in the same module, due to the fact that they will each generate their own submodules with the same name. You have to put the wrapper-generating macros in separate modules and then `pub use` the generated definitions. Sorry!