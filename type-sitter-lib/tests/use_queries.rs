#![cfg(feature = "tree-sitter-wrapper")]

mod json;
mod rust;

use std::iter::zip;
use std::path::Path;
use type_sitter_lib::tree_sitter_wrapper::Parser;
use type_sitter_lib::{OptionNodeResultExtraOrExt, TypedNode, TypedQueryCursor};
use crate::rust::queries::Tags;

#[test]
pub fn text_use_queries_new() {
    // ???: Abstract with use_node_types?
    let mut parser = Parser::new(tree_sitter_rust::language()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_ast = parser.parse_file(&code_path, None).expect("Failed to parse code");
    let code_root = rust::nodes::SourceFile::try_from(code_ast.root_node()).expect("Failed to wrap code root node");

    let mut q = TypedQueryCursor::new();
    let mut q2 = TypedQueryCursor::new();
    q.set_match_limit(100);
    let matches = q.matches(&Tags, code_root);
    let mut matches_text = Vec::new();
    eprintln!("Matches:");
    for match_ in matches {
        eprintln!("  {:?}", match_);
        matches_text.push(format!("{:?}", match_));
    }
    let matches = q.matches(&Tags, code_root).collect::<Vec<_>>();
    assert_eq!(matches.len(), matches_text.len(), "# of matches changed when we collected them!");
    for (match_, old_match_text) in zip(&matches, &matches_text) {
        assert_eq!(&format!("{:?}", match_), old_match_text, "match changed when we stored it with others! (tree-sitter bug)")
    }
    let captures = q2.captures(&Tags, code_root).collect::<Vec<_>>();
    eprintln!("Captures:");
    for capture_ in &captures {
        eprintln!("  {:?}", capture_)
    }

    assert_eq!(matches[0].definition_function().unwrap().name().unwrap().identifier().unwrap().text(), "language");
    assert_eq!(matches[0].name().unwrap().identifier().unwrap().text(), "language");
    assert_eq!(matches[1].reference_call().unwrap().call_expression().unwrap().function().unwrap().identifier().unwrap().text(), "tree_sitter_rust");
    assert_eq!(matches[1].name().unwrap().identifier().unwrap().text(), "tree_sitter_rust");
    for i in 2..=5 {
        assert_eq!(matches[i].reference_call().unwrap().macro_invocation().unwrap().r#macro().unwrap().identifier().unwrap().text(), "include_str");
        assert_eq!(matches[i].name().unwrap().identifier().unwrap().text(), "include_str");
    }
    assert_eq!(matches[6].definition_module().unwrap().name().unwrap().text(), "tests");
    assert_eq!(matches[6].name().unwrap().identifier().unwrap().text(), "tests");
    assert_eq!(matches[7].definition_method().unwrap().child(0).unwrap3().function_item().unwrap().name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    assert_eq!(matches[7].name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    assert_eq!(matches[8].definition_function().unwrap().name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    assert_eq!(matches[8].name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    assert_eq!(matches[9].reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "set_language");
    assert_eq!(matches[9].name().unwrap().field_identifier().unwrap().text(), "set_language");
    assert_eq!(matches[10].reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "expect");
    assert_eq!(matches[10].name().unwrap().field_identifier().unwrap().text(), "expect");
    assert_eq!(matches.len(), 11);
    assert_eq!(captures.len(), 22);
}