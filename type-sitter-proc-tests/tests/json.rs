use type_sitter_lib::{OptionNodeResultExt, OptionNodeResultExtraOrExt, TypedNode};

mod json {
    use type_sitter_proc::generate_nodes;

    generate_nodes!("../vendor/tree-sitter-json/src/node-types.json", tree_sitter);
}

#[test]
fn test_node_types() {
    let json_str = "\
    {
        \"type\": \"array\",
        \"content\": \"value\"
    }";
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_json::language()).unwrap();
    let json_tree = parser.parse(&json_str, None).unwrap();
    let json_document = json::Document::try_from(json_tree.root_node()).unwrap();
    assert_eq!(json_document.children(&mut json_tree.walk()).len(), 1);
    let json_root = json_document.child(0).unwrap3().object().unwrap();
    assert_eq!(json_root.children(&mut json_tree.walk()).len(), 2);
    let json_type = json_root.child(0).unwrap3();
    assert_eq!(json_type.key().unwrap().string().unwrap().child().unwrap2().utf8_text(json_str.as_bytes()).unwrap(), "type");
    assert_eq!(json_type.value().unwrap().string().unwrap().child().unwrap2().utf8_text(json_str.as_bytes()).unwrap(), "array");
    let json_content = json_root.child(1).unwrap3();
    assert_eq!(json_content.key().unwrap().string().unwrap().child().unwrap2().utf8_text(json_str.as_bytes()).unwrap(), "content");
    assert_eq!(json_content.value().unwrap().string().unwrap().child().unwrap2().utf8_text(json_str.as_bytes()).unwrap(), "value");
}
