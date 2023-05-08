use type_sitter_lib::{OptionNodeResultExt, OptionNodeResultExtraOrExt, TypedNode};

mod rust {
    use type_sitter_proc::generate_nodes;

    generate_nodes!("../vendor/tree-sitter-rust/src/node-types.json", tree_sitter);
}

#[test]
fn test_node_types() {
    let rust_str = "\
    /// Foo bar
    fn main() {
        let json = \"\
        {
            \\\"type\\\": \\\"array\\\",
            \\\"content\\\": \\\"value\\\"
        }\";
        let mut parser = tree_sitter::Parser::new();
        todo!(\"baz\")
    }";
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    let rust_tree = parser.parse(&rust_str, None).unwrap();
    let rust_source_file = rust::SourceFile::try_from(rust_tree.root_node()).unwrap();
    assert_eq!(rust_source_file.children(&mut rust_tree.walk()).len(), 2);
    let rust_doc = rust::LineComment::try_from(rust_source_file.child(0).unwrap2().unwrap_extra()).unwrap();
    assert_eq!(rust_doc.utf8_text(rust_str.as_bytes()).unwrap(), "/// Foo bar");
    let rust_main_fn = rust_source_file.child(1).unwrap3().declaration_statement().unwrap().function_item().unwrap();
    assert_eq!(rust_main_fn.name().unwrap().identifier().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "main");
    assert_eq!(rust_main_fn.parameters().unwrap().children(&mut rust_tree.walk()).len(), 0);
    let rust_main_fn_body = rust_main_fn.body().unwrap();
    assert_eq!(rust_main_fn_body.children(&mut rust_tree.walk()).len(), 3);
    let rust_let_json = rust_main_fn_body.child(0).unwrap3().declaration_statement().unwrap().let_declaration().unwrap();
    assert_eq!(rust_let_json.pattern().unwrap().identifier().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "json");
    assert!(!rust_let_json.children(&mut rust_tree.walk()).any(|child| child.unwrap().is_regular_and(|c| c.mutable_specifier().is_some())));
    let rust_json_str = rust_let_json.value().unwrap2().literal().unwrap().string_literal().unwrap();
    assert_eq!(rust_json_str.utf8_text(rust_str.as_bytes()).unwrap(), "\"{\n            \\\"type\\\": \\\"array\\\",\n            \\\"content\\\": \\\"value\\\"\n        }\"");
    let rust_let_mut_parser = rust_main_fn_body.child(1).unwrap3().declaration_statement().unwrap().let_declaration().unwrap();
    assert_eq!(rust_let_mut_parser.pattern().unwrap().identifier().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "parser");
    assert!(rust_let_mut_parser.children(&mut rust_tree.walk()).any(|child| child.unwrap().is_regular_and(|c| c.mutable_specifier().is_some())));
    let rust_parser_new = rust_let_mut_parser.value().unwrap2().call_expression().unwrap();
    assert_eq!(rust_parser_new.function().unwrap().scoped_identifier().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "tree_sitter::Parser::new");
    assert_eq!(rust_parser_new.arguments().unwrap().children(&mut rust_tree.walk()).len(), 0);
    use rust::anon_unions::DeclarationStatement_Expression_ExpressionStatement::*;
    let rust_todo = match rust_main_fn_body.child(2).unwrap3() {
        DeclarationStatement(decl) => decl.macro_invocation(),
        Expression(expr) => expr.macro_invocation(),
        ExpressionStatement(expr) => expr.child().unwrap().macro_invocation()
    }.unwrap();
    assert_eq!(rust_todo.children(&mut rust_tree.walk()).len(), 2);
    assert_eq!(rust_todo.r#macro().unwrap().identifier().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "todo");
    assert_eq!(rust_todo.child(0).unwrap3().identifier().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "todo");
    let rust_todo_arg = rust_todo.child(1).unwrap3().token_tree().unwrap();
    assert_eq!(rust_todo_arg.children(&mut rust_tree.walk()).len(), 1);
    assert_eq!(rust_todo_arg.child(0).unwrap3().literal().unwrap().string_literal().unwrap().utf8_text(rust_str.as_bytes()).unwrap(), "\"baz\"");
}
