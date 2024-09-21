# type-sitter-proc: generate type-sitter wrappers via procedural macros

[![crates.io](https://img.shields.io/crates/v/type-sitter-proc.svg)](https://crates.io/crates/type-sitter-proc)

See [`type-sitter`](https://github.com/Jakobeha/type-sitter#readme) for background information. This exports procedural macros which generate typed nodes. Typed nodes can also be generated from a built-script using [`type-sitter-gen`](https://crates.io/crates/type-sitter-gen), or from a command-line interface using [`type-sitter-cli`](https://crates.io/crates/type-sitter-cli).

Instead of depending on this, typically you should depend on `type-sitter` instead with the `proc` feature (enabled by default). `type-sitter` with the `proc` feature re-exports this crate, but also `type-sitter-lib`, which the generated code depends on at runtime.