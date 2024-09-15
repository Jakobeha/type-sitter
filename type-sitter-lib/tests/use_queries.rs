#![cfg(feature = "yak-sitter")]

mod json;
mod rust;

use std::path::Path;
use streaming_iterator::StreamingIterator;
use yak_sitter::Parser;
use type_sitter_lib::{OptionNodeResultExtraOrExt, TypedNode, TypedQueryCursor};
use crate::rust::queries::Tags;

#[test]
pub fn test_use_queries_new() {
    // ???: Abstract with use_node_types?
    let mut parser = Parser::new(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_ast = parser.parse_file(&code_path, None, ()).expect("Failed to parse code");
    let code_root = rust::nodes::SourceFile::try_from(code_ast.root_node()).expect("Failed to wrap code root node");

    let mut q = TypedQueryCursor::new();
    let mut q2 = TypedQueryCursor::new();
    q.set_match_limit(100);
    let mut matches = q.matches(&Tags, code_root);
    eprintln!("Matches:");
    while let Some(r#match) = matches.next() {
        eprintln!("  {:?}", r#match);
    }
    let captures = q2.captures(&Tags, code_root).collect::<Vec<_>>();
    eprintln!("Captures:");
    for capture_ in &captures {
        eprintln!("  {:?}", capture_)
    }

    let mut matches = q.matches(&Tags, code_root);
    for _ in 0..=3 {
        let r#match = matches.next().unwrap();
        assert_eq!(r#match.reference_call().unwrap().macro_invocation().unwrap().r#macro().unwrap().identifier().unwrap().text(), "include_str");
        assert_eq!(r#match.name().unwrap().identifier().unwrap().text(), "include_str");
    }
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.definition_module().unwrap().name().unwrap().text(), "tests");
    assert_eq!(r#match.name().unwrap().identifier().unwrap().text(), "tests");
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.definition_method().unwrap().child(0).unwrap3().attribute_item().unwrap().child().unwrap().child(0).unwrap3().identifier().unwrap().text(), "test");
    assert!(r#match.definition_method().unwrap().child(0).unwrap3().attribute_item().unwrap().child().unwrap().child(1).is_none());
    assert_eq!(r#match.definition_method().unwrap().child(1).unwrap3().function_item().unwrap().name().unwrap().identifier().unwrap().text(), "test_can_load_grammar");
    assert!(r#match.definition_method().unwrap().child(2).is_none());
    assert_eq!(r#match.name().unwrap().identifier().unwrap().text(), "test_can_load_grammar");
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.definition_function().unwrap().name().unwrap().identifier().unwrap().text(), "test_can_load_grammar");
    assert_eq!(r#match.name().unwrap().identifier().unwrap().text(), "test_can_load_grammar");
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "set_language");
    assert_eq!(r#match.name().unwrap().field_identifier().unwrap().text(), "set_language");
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "into");
    assert_eq!(r#match.name().unwrap().field_identifier().unwrap().text(), "into");
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.reference_call().unwrap().call_expression().unwrap().function().unwrap().field_expression().unwrap().field().unwrap().text(), "expect");
    assert_eq!(r#match.name().unwrap().field_identifier().unwrap().text(), "expect");
    assert!(matches.next().is_none());
    // ???: Individual captures tests?
    assert_eq!(captures.len(), 20);
}