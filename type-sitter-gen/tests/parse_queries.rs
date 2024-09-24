mod common;

use crate::common::{setup_common, Common};
use std::fs::write;
use type_sitter_gen::{generate_queries_with_custom_module_paths, super_nodes, type_sitter_lib, yak_sitter};

#[test]
pub fn test_parse_queries_json() {
    test_parse_queries("json")
}

#[test]
pub fn test_parse_queries_rust() {
    test_parse_queries("rust")
}

pub fn test_parse_queries(lang: &str) {
    let Common { input_dir, expected_dir } = setup_common(lang);
    let input_queries_dir = input_dir.join("queries");
    let expected_queries_path = expected_dir.join("queries.rs");

    let queries_code = generate_queries_with_custom_module_paths(
        input_queries_dir,
        input_dir,
        &super_nodes(),
        true,
        &yak_sitter(),
        &type_sitter_lib(),
    ).expect("Failed to generate queries");

    write(&expected_queries_path, queries_code.into_string())
        .expect("Failed to create expected queries file");
}