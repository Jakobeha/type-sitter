use type_sitter::{OptionNodeResultExt, StreamingIterator, Node, QueryCursor, Parser, Tree};

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

fn json_tree() -> Tree<json::Document<'static>> {
    let mut parser = Parser::<json::Document>::new(&tree_sitter_json::LANGUAGE.into()).unwrap();
    parser.parse(&JSON_STR, None).unwrap()
}

#[test]
fn test_node_types() {
    let json_tree = json_tree();
    let json_document = json_tree.root_node().unwrap();
    let mut cursor = json_tree.walk();

    assert_eq!(json_document.values(&mut cursor).count(), 1);
    let json_root = json_document.values(&mut cursor).next().unwrap2().as_object().unwrap();
    assert_eq!(json_root.pairs(&mut cursor).count(), 2);
    let mut json_fields = json_root.pairs(&mut cursor);
    let json_type = json_fields.next().unwrap2();
    assert_eq!(json_type.key().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"type\"");
    assert_eq!(json_type.value().unwrap().as_string().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"array\"");
    let json_content = json_fields.next().unwrap2();
    assert_eq!(json_content.key().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"content\"");
    assert_eq!(json_content.value().unwrap().as_string().unwrap().utf8_text(JSON_STR.as_bytes()).unwrap(), "\"value\"");
}

#[test]
fn test_queries() {
    let json_tree = json_tree();
    let json_document = json_tree.root_node().unwrap();

    let mut q = QueryCursor::new();
    let mut matches = q.matches(&json::queries::Highlights, json_document, JSON_STR.as_bytes());
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string_special_key().unwrap().downcast::<json::String>().unwrap().byte_range(), 6..12);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 6..12);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 14..21);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string_special_key().unwrap().downcast::<json::String>().unwrap().byte_range(), 27..36);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 27..36);
    let r#match = matches.next().unwrap();
    assert_eq!(r#match.string().unwrap().byte_range(), 38..45);
    assert!(matches.next().is_none());

    let captures = q.captures(&json::queries::Highlights, json_document, JSON_STR.as_bytes()).collect::<Vec<_>>();
    assert_eq!(captures[0].as_string_special_key().unwrap().downcast::<json::String>().unwrap().byte_range(), 6..12);
    assert_eq!(captures[1].as_string().unwrap().byte_range(), 6..12);
    assert_eq!(captures[2].as_string().unwrap().byte_range(), 14..21);
    assert_eq!(captures[3].as_string_special_key().unwrap().downcast::<json::String>().unwrap().byte_range(), 27..36);
    assert_eq!(captures[4].as_string().unwrap().byte_range(), 27..36);
    assert_eq!(captures[5].as_string().unwrap().byte_range(), 38..45);
    assert_eq!(captures.len(), 6);
}