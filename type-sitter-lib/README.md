# type-sitter-lib: type-sitter code for generated / downstream code

See [type-sitter](../README.md) for more information. type-sitter is a proc-macro so it executes during "macro time" AKA "phase 1". This means that downstream code can't depend on any of its data-structures or functions. This crate contains those which downstream code can depend on.