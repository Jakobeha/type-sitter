# type-sitter-lib: type-sitter code for generated / downstream code

[![Crates.io](https://img.shields.io/crates/v/type-sitter-lib.svg)](https://crates.io/crates/type-sitter-lib)
[![Docs.rs](https://docs.rs/type-sitter-lib/badge.svg)](https://docs.rs/type-sitter-lib)

See [`type-sitter`](https://github.com/Jakobeha/type-sitter#readme) for background information. This is the library which code generated from type-sitter depends on at runtime.

Instead of depending on this, typically you should depend on `type-sitter` instead and, if necessary, `use-default-features = false` to avoid depending on `type-sitter-proc`. `type-sitter` with no features simply re-exports this crate.