use type_sitter::{Node, OptionNodeResultExt, Parser, Tree};

mod rust {
    type_sitter::generate_nodes!("../vendor/tree-sitter-rust/src/node-types.json");
}

fn parse_rust(input: &str) -> Tree<rust::SourceFile<'static>> {
    let mut parser = Parser::<rust::SourceFile>::new(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    parser.parse(&input, None).unwrap()
}

#[test]
fn test_nonfield_children_iterator() {
    let code = "fn main() { let _ = [1, 2, 3, 4, 5]; }";

    let tree = parse_rust(code);
    let source_file = tree.root_node().unwrap();

    let array = source_file
        .children(&mut tree.walk()).next().unwrap2()
        .as_function_item().unwrap()
        .body().unwrap()
        .children(&mut tree.walk()).next().unwrap2()
        .as_let_declaration().unwrap()
        .value().unwrap2()
        .as_array_expression().unwrap();

    let elements = array.others(&mut tree.walk())
        .collect::<Result<Vec<_>, _>>().unwrap();

    assert_eq!(elements.len(), 5);
    for (i, element) in elements.into_iter().enumerate() {
        assert!(element.as_integer_literal().is_some());

        // text = "1", "2", "3", "4", or "5" depending on `i`.
        let element_code = element.utf8_text(code.as_bytes()).unwrap();
        assert_eq!(element_code, (i + 1).to_string());
    }
}

#[test]
fn test_nonfield_child_getter() {
    let code = "fn main() { match () { () if true => () } }";

    let tree = parse_rust(code);
    let source_file = tree.root_node().unwrap();

    let pattern_and_if = source_file
        .children(&mut tree.walk()).next().unwrap2()
        .as_function_item().unwrap()
        .body().unwrap()
        .children(&mut tree.walk()).next().unwrap2()
        .as_expression_statement().unwrap()
        .expression().unwrap()
        .as_match_expression().unwrap()
        .body().unwrap()
        .match_arms(&mut tree.walk()).next().unwrap2()
        .pattern().unwrap();

    assert_eq!(pattern_and_if.condition().unwrap().utf8_text(code.as_bytes()).unwrap(), "true");
    assert_eq!(pattern_and_if.pattern().unwrap().utf8_text(code.as_bytes()).unwrap(), "()");
}