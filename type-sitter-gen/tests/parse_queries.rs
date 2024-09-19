use std::fs::{create_dir, read_to_string, write};
use std::path::Path;
use std::str::FromStr;
use proc_macro2::TokenStream;
use type_sitter_gen::{generate_queries_from_dir, super_nodes, type_sitter_lib, yak_sitter};
use pretty_assertions::assert_eq;

#[test]
pub fn test_parse_queries_json() {
    test_parse_queries("json")
}

#[test]
pub fn test_parse_queries_rust() {
    test_parse_queries("rust")
}

// ???: Refactor common code out of this and parse_node_types?
pub fn test_parse_queries(lang: &str) {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../vendor/tree-sitter-{}", lang));
    let expected_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../type-sitter-lib/tests/{}", lang));
    if !expected_path.exists() {
        create_dir(&expected_path).expect("Failed to create expected directory");
        write(expected_path.join("mod.rs"), "pub mod nodes;\npub mod queries;")
            .expect("Failed to create expected mod.rs file");
    }
    let input_queries_path = input_path.join("queries");
    let expected_queries_path = expected_path.join("queries.rs");
    let queries_code = generate_queries_from_dir(
        input_queries_path,
        input_path,
        &super_nodes(),
        true,
        &yak_sitter(),
        &type_sitter_lib(),
    ).expect("Failed to generate queries").collapse(&super_nodes());

    if !expected_queries_path.exists() {
        write(&expected_queries_path, pretty_print(&queries_code))
            .expect("Failed to create expected queries file");
        eprintln!("Created expected queries file: {}", expected_queries_path.display());
    } else {
        let expected_queries_code = TokenStream::from_str(&read_to_string(expected_queries_path)
            .expect("Failed to read expected queries")).expect("Failed to parse expected queries");
        assert_eq!(pretty_print(&queries_code), pretty_print(&expected_queries_code), "Generated queries source code does not match expected queries source code");
    }
}

fn pretty_print(tokens: &TokenStream) -> String {
    let str = tokens.to_string();
    syn::parse_file(&str).map(|f| prettyplease::unparse(&f)).unwrap_or_else(|err| {
        eprintln!("Failed to pretty print tokens: {}", err);
        str
    })
}