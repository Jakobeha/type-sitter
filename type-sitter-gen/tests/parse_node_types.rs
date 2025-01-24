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

#[test]
pub fn test_parse_node_types_c() {
    test_parse_node_types("c")
}

pub fn test_parse_node_types(lang: &str) {
    let Common { input_dir, output_dir } = setup_common(lang);
    let node_types_json_path = input_dir.join("src/node-types.json");
    let node_types_code_path = output_dir.join("nodes.rs");

    let node_types_code = generate_nodes_with_custom_module_paths(
        node_types_json_path.as_path(),
        &yak_sitter(),
        &type_sitter_lib(),
    ).expect("Failed to generate node types");

    write(&node_types_code_path, node_types_code.into_string())
        .expect("Failed to create expected node types file");
}