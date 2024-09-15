use type_sitter::{OptionNodeResultExtraOrExt, StreamingIterator, Tree, TypedNode, TypedQueryCursor};

mod json {
    use type_sitter::generate_nodes;

    generate_nodes!("../vendor/tree-sitter-json/src/node-types.json");

    pub mod queries {
        use type_sitter::generate_queries;

        generate_queries!("../vendor/tree-sitter-json/queries", "../vendor/tree-sitter-json", super);
    }
}

const JSON_STR: &'static str = "\
{
    \"type\": \"array\",
    \"content\": \"value\"
}";

fn json_tree() -> Tree {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_json::LANGUAGE.into()).unwrap();
    parser.parse(&JSON_STR, None).unwrap()
}

#[test]
fn test_node_types() {
    let json_tree = json_tree();
    let json_document = json::Document::try_from(json_tree.root_node()).unwrap();

    assert_eq!(json_document.children(&mut json_tree.walk()).len(), 1);
    let json_root = json_document.child(0).unwrap3().object().unwrap();
    assert_eq!(json_root.children(&mut json_tree.walk()).len(), 2);
    let json_type = json_root.child(0).unwrap3();
    assert_eq!(json_type.key().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"type\"");
    assert_eq!(json_type.value().unwrap().string().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"array\"");
    let json_content = json_root.child(1).unwrap3();
    assert_eq!(json_content.key().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"content\"");
    assert_eq!(json_content.value().unwrap().string().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"value\"");
}

#[test]
fn test_queries() {
    let json_tree = json_tree();
    let json_document = json::Document::try_from(json_tree.root_node()).unwrap();

    let mut q = TypedQueryCursor::new();
    let mut matches = q.matches(&json::queries::Highlights, json_document, JSON_STR.as_bytes());
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string_special_key().unwrap().to::<json::String>().unwrap().byte_range(), 6..12);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 6..12);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 14..21);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string_special_key().unwrap().to::<json::String>().unwrap().byte_range(), 27..36);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 27..36);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 38..45);
    assert!(matches.next().is_none());

    let captures = q.captures(&json::queries::Highlights, json_document, JSON_STR.as_bytes()).collect::<Vec<_>>();
    assert_eq!(captures[0].string_special_key().unwrap().to::<json::String>().unwrap().byte_range(), 6..12);
    assert_eq!(captures[1].string().unwrap().byte_range(), 6..12);
    assert_eq!(captures[2].string().unwrap().byte_range(), 14..21);
    assert_eq!(captures[3].string_special_key().unwrap().to::<json::String>().unwrap().byte_range(), 27..36);
    assert_eq!(captures[4].string().unwrap().byte_range(), 27..36);
    assert_eq!(captures[5].string().unwrap().byte_range(), 38..45);
    assert_eq!(captures.len(), 6);
}
