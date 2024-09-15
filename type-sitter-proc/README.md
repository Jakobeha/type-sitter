# type-sitter-proc: generate type-sitter wrappers via procedural macros

[![Crates.io](https://img.shields.io/crates/v/type-sitter-proc.svg)](https://crates.io/crates/type-sitter-proc)

You should probably use [`type-sitter`](https://crates.io/crates/type-sitter) instead of this crate directly. This crate is a dependency of `type-sitter`, but `type-sitter` also provides [`type-sitter-lib`](https://crates.io/crates/type-sitter-lib) (and [`yak-sitter`](https://crates.io/crates/yak-sitter) with the feature) which otherwise you have to manually add to your project's dependency list.

This provides procedural macros which will generate typed node wrappers.

Note that you can't generate multiple node-type or query wrappers in the same module, due to the fact that they will each generate their own submodules with the same name. You have to put the wrapper-generating macros in separate modules and then `pub use` the generated definitions. Sorry!