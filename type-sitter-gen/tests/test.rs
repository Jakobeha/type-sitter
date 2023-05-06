use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use proc_macro2::TokenStream;
use type_sitter_gen::generate_nodes;
use rust_format::{Formatter, RustFmt};
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_PATH: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust");
    static ref EXPECTED_PATH: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/rust");
    static ref FORMATTER: RustFmt = RustFmt::default();
}

#[test]
pub fn test_parse_node_types() {
    let node_types_code = generate_nodes(INPUT_PATH.join("src/node-types.json")).expect("Failed to generate nodes");
    let expected_node_types_code = TokenStream::from_str(&read_to_string(EXPECTED_PATH.join("mod.rs")).expect("Failed to read expected node types")).expect("Failed to parse expected node types");
    assert_eq!(pretty_print(node_types_code), pretty_print(expected_node_types_code), "Generated node types source code does not match expected node types source code");
}

fn pretty_print(tokens: TokenStream) -> String {
    FORMATTER.format_tokens(tokens).expect("Failed to format tokens")
}