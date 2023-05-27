#![cfg(feature = "yak-sitter")]

mod json;
mod rust;

use std::path::Path;
use streaming_iterator::StreamingIterator;
use yak_sitter::Parser;
use type_sitter_lib::{OptionNodeResultExtraOrExt, TypedNode, TypedQueryCursor};
use crate::rust::queries::Tags;

#[test]
pub fn text_use_queries_new() {
    // ???: Abstract with use_node_types?
    let mut parser = Parser::new(tree_sitter_rust::language()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_ast = parser.parse_file(&code_path, None, ()).expect("Failed to parse code");
    let code_root = rust::nodes::SourceFile::try_from(code_ast.root_node()).expect("Failed to wrap code root node");

    let mut q = TypedQueryCursor::new();
    let mut q2 = TypedQueryCursor::new();
    q.set_match_limit(100);
    let mut matches = q.matches(&Tags, code_root);
    eprintln!("Matches:");
    while let Some(match_) = matches.next() {
        eprintln!("  {:?}", match_);
    }
    let captures = q2.captures(&Tags, code_root).collect::<Vec<_>>();
    eprintln!("Captures:");
    for capture_ in &captures {
        eprintln!("  {:?}", capture_)
    }

    let mut matches = q.matches(&Tags, code_root);
    let match_ = matches.next().unwrap();
    assert_eq!(match_.definition_function().unwrap().name().unwrap().identifier().unwrap().text(), "language");
    assert_eq!(match_.name().unwrap().identifier().unwrap().text(), "language");
    let match_ = matches.next().unwrap();
    assert_eq!(match_.reference_call().unwrap().call_expression().unwrap().function().unwrap().identifier().unwrap().text(), "tree_sitter_rust");
    assert_eq!(match_.name().unwrap().identifier().unwrap().text(), "tree_sitter_rust");
    for _ in 2..=5 {
        let match_ = matches.next().unwrap();
        assert_eq!(match_.reference_call().unwrap().macro_invocation().unwrap().r#macro().unwrap().identifier().unwrap().text(), "include_str");
        assert_eq!(match_.name().unwrap().identifier().unwrap().text(), "include_str");
    }
    let match_ = matches.next().unwrap();
    assert_eq!(match_.definition_module().unwrap().name().unwrap().text(), "tests");
    assert_eq!(match_.name().unwrap().identifier().unwrap().text(), "tests");
    let match_ = matches.next().unwrap();
    assert_eq!(match_.definition_method().unwrap().child(0).unwrap3().attribute_item().unwrap().child().unwrap().child(0).unwrap3().identifier().unwrap().text(), "test");
    assert!(match_.definition_method().unwrap().child(0).unwrap3().attribute_item().unwrap().child().unwrap().child(1).is_none());
    assert_eq!(match_.definition_method().unwrap().child(1).unwrap3().function_item().unwrap().name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    assert!(match_.definition_method().unwrap().child(2).is_none());
    assert_eq!(match_.name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    let match_ = matches.next().unwrap();
    assert_eq!(match_.definition_function().unwrap().name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    assert_eq!(match_.name().unwrap().identifier().unwrap().text(), "can_load_grammar");
    let match_ = matches.next().unwrap();
    assert_eq!(match_.reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "set_language");
    assert_eq!(match_.name().unwrap().field_identifier().unwrap().text(), "set_language");
    let match_ = matches.next().unwrap();
    assert_eq!(match_.reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "expect");
    assert_eq!(match_.name().unwrap().field_identifier().unwrap().text(), "expect");
    assert!(matches.next().is_none());
    // ???: Individual captures tests?
    assert_eq!(captures.len(), 22);
}