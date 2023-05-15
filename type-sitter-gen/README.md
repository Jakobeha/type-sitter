# type-sitter-gen: base functionality to generate typed wrapper nodes for type-sitter

[![Crates.io](https://img.shields.io/crates/v/type-sitter-gen.svg)](https://crates.io/crates/type-sitter-gen)
[![Docs.rs](https://docs.rs/type-sitter-gen/badge.svg)](https://docs.rs/type-sitter-gen)

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This library allows you to generate typed tree-sitter wrappers with even more flexibility than [type-sitter-cli](https://crates.io/crates/type-sitter-cli) and [type-sitter-proc](https://crates.io/crates/type-sitter-proc). For instance, you can generate wrappers which use a custom underlying `tree-sitter` wrapper, or generate queries without some of their patterns and captures. You can also combine multiple generations into one file.