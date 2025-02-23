mod common;

use crate::common::{setup_common, Common};
use std::fs::write;
use type_sitter_gen::{dylib_path, generate_queries_with_custom_module_paths, super_nodes, type_sitter_lib, yak_sitter};

#[test]
pub fn test_parse_queries_json() {
    test_parse_queries("json")
}

#[test]
pub fn test_parse_queries_rust() {
    test_parse_queries("rust")
}

#[test]
pub fn test_parse_queries_c() {
    test_parse_queries("c")
}

pub fn test_parse_queries(lang: &str) {
    let Common { input_dir, output_dir } = setup_common(lang);
    let queries_scm_dir = input_dir.join("queries");
    let queries_code_path = output_dir.join("queries.rs");

    // Regenerate dylib if old, to prevent stale and ensure generation works.
    let dylib_path = dylib_path(&input_dir);
    if dylib_path.exists() {
        std::fs::remove_file(&dylib_path).expect("Failed to remove old dylib");
    }

    let queries_code = generate_queries_with_custom_module_paths(
        queries_scm_dir,
        input_dir,
        &super_nodes(),
        true,
        &yak_sitter(),
        &type_sitter_lib(),
    ).expect("Failed to generate queries");

    write(&queries_code_path, queries_code.into_string())
        .expect("Failed to create expected queries file");
}