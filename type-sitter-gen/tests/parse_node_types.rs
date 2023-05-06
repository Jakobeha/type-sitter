use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use proc_macro2::TokenStream;
use type_sitter_gen::generate_nodes;

#[test]
pub fn test_parse_node_types_json() {
    test_parse_node_types("json")
}

#[test]
pub fn test_parse_node_types_rust() {
    test_parse_node_types("rust")
}

pub fn test_parse_node_types(lang: &str) {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../vendor/tree-sitter-{}", lang));
    let expected_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("examples/{}", lang));
    let node_types_code = generate_nodes(input_path.join("src/node-types.json")).expect("Failed to generate nodes");
    let expected_node_types_code = TokenStream::from_str(&read_to_string(expected_path.join("mod.rs")).expect("Failed to read expected node types")).expect("Failed to parse expected node types");
    assert_eq!(pretty_print(&node_types_code), pretty_print(&expected_node_types_code), "Generated node types source code does not match expected node types source code");
}

fn pretty_print(tokens: &TokenStream) -> String {
    let str = tokens.to_string();
    syn::parse_file(&str).map(|f| prettyplease::unparse(&f)).unwrap_or_else(|err| {
        eprintln!("Failed to pretty print tokens: {}", err);
        str
    })
}