mod json;
mod rust;

use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::Parser;

#[test]
pub fn test_use_node_types_rust() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_str = read_to_string(code_path).expect("Failed to read code");
    let code_ast = parser.parse(code_str, None).expect("Failed to parse code");
    let code_root = rust::SourceFile::try_from(code_ast.root_node()).expect("Failed to wrap code root node");
    let code_first_child = code_root.child(0).expect("Failed to get first child of code root node").expect("First child of code root node is error");
    todo!("{:?}", code_first_child)
}