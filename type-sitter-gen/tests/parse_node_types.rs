mod common;

use crate::common::{setup_common, Common};
use std::fs::write;
use type_sitter_gen::{generate_nodes_with_custom_module_paths, type_sitter_lib, yak_sitter};

#[test]
pub fn test_parse_node_types_json() {
    test_parse_node_types("json")
}

#[test]
pub fn test_parse_node_types_rust() {
    test_parse_node_types("rust")
}

pub fn test_parse_node_types(lang: &str) {
    let Common { input_dir, expected_dir } = setup_common(lang);
    let input_node_types_path = input_dir.join("src/node-types.json");
    let expected_node_types_path = expected_dir.join("nodes.rs");

    let node_types_code = generate_nodes_with_custom_module_paths(
        input_node_types_path,
        &yak_sitter(),
        &type_sitter_lib(),
    ).expect("Failed to generate node types");

    write(&expected_node_types_path, node_types_code.into_string())
        .expect("Failed to create expected node types file");
}