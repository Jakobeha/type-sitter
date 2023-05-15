use std::fs::{create_dir, read_to_string, write};
use std::path::Path;
use std::str::FromStr;
use proc_macro2::TokenStream;
use type_sitter_gen::{generate_nodes, type_sitter_lib_wrapper};
use pretty_assertions::assert_eq;

#[test]
pub fn test_parse_node_types_json() {
    test_parse_node_types("json")
}

#[test]
pub fn test_parse_node_types_rust() {
    test_parse_node_types("rust")
}

// ???: Refactor common code out of this and parse_queries?
pub fn test_parse_node_types(lang: &str) {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../vendor/tree-sitter-{}", lang));
    let expected_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../type-sitter-lib/tests/{}", lang));
    if !expected_path.exists() {
        create_dir(&expected_path).expect("Failed to create expected directory");
        write(expected_path.join("mod.rs"), "pub mod nodes;\npub mod queries;")
            .expect("Failed to create expected mod.rs file");
    }
    let input_node_types_path = input_path.join("src/node-types.json");
    let expected_node_types_path = expected_path.join("nodes.rs");
    let node_types_code = generate_nodes(
        input_node_types_path,
        &type_sitter_lib_wrapper()
    ).expect("Failed to generate node types").collapse();

    if !expected_node_types_path.exists() {
        write(&expected_node_types_path, pretty_print(&node_types_code))
            .expect("Failed to create expected node types file");
        eprintln!("Created expected node types file: {}", expected_node_types_path.display());
    } else {
        let expected_node_types_code = TokenStream::from_str(&read_to_string(expected_node_types_path)
            .expect("Failed to read expected node types")).expect("Failed to parse expected node types");
        assert_eq!(pretty_print(&node_types_code), pretty_print(&expected_node_types_code), "Generated node types source code does not match expected node types source code");
    }
}

fn pretty_print(tokens: &TokenStream) -> String {
    let str = tokens.to_string();
    syn::parse_file(&str).map(|f| prettyplease::unparse(&f)).unwrap_or_else(|err| {
        eprintln!("Failed to pretty print tokens: {}", err);
        str
    })
}