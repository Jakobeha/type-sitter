# type-sitter-cli: Command-line utility to generate source files with type-sitter wrappers

See [type-sitter](https://github.com/Jakobeha/type-sitter#readme) for more information. This is the command-line tool which generates typed-safe wrappers for tree-sitter nodes.

## Usage

```shell
# If not already installed
cargo install type-sitter-cli
# In your cargo project root directory
type-sitter-cli path/to/node-types.json
```

#### Advanced usage

```shell
# Specify a custom output directory and use tree-sitter-wrapper
type-sitter-cli vendor/tree-sitter-rust/node-types.json -o generated_src --use-wrapper
# See help for the CLI program
type-sitter-cli --help
```