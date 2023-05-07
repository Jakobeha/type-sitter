mod json;
mod rust;

use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::Parser;
use type_sitter_lib::{Either2, TypedNode};

#[test]
pub fn test_use_node_types_rust() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_str = read_to_string(code_path).expect("Failed to read code");
    let code_ast = parser.parse(&code_str, None).expect("Failed to parse code");
    let code_root = rust::SourceFile::try_from(code_ast.root_node()).expect("Failed to wrap code root node");
    let statements = code_root.children(&mut code_root.walk())
        .filter_map(|child| child.unwrap().regular())
        .collect::<Vec<_>>();
    eprintln!("Statements:");
    for statement in &statements {
        eprintln!("  {}", statement.node().to_sexp());
    }
    assert!(matches!(statements[0], Either2::A(rust::DeclarationStatement::UseDeclaration(_))));
    assert!(matches!(statements[1], Either2::A(rust::DeclarationStatement::ForeignModItem(_))));
    assert!(matches!(statements[2], Either2::A(rust::DeclarationStatement::FunctionItem(_))));
    assert!(matches!(statements[3], Either2::A(rust::DeclarationStatement::ConstItem(_))));
    assert!(matches!(statements[4], Either2::A(rust::DeclarationStatement::ConstItem(_))));
    assert!(matches!(statements[5], Either2::A(rust::DeclarationStatement::ConstItem(_))));
    assert!(matches!(statements[6], Either2::A(rust::DeclarationStatement::ConstItem(_))));
    assert!(matches!(statements[7], Either2::A(rust::DeclarationStatement::AttributeItem(_))));
    assert!(matches!(statements[8], Either2::A(rust::DeclarationStatement::ModItem(_))));
    assert_eq!(
        statements[0]
            .a().expect("Expected declaration statement")
            .use_declaration().expect("Expected use declaration")
            .argument().unwrap()
            .d().expect("Expected use declaration to have a scoped identifier")
            .path().expect("Expected use declaration's scoped identifier to have a path").unwrap()
            .d().expect("Expected use declaration's scope identifier to have an identifier")
            .utf8_text(code_str.as_bytes()).unwrap(),
        "tree_sitter"
    );
    assert_eq!(
        statements[0]
            .a().expect("Expected declaration statement")
            .use_declaration().expect("Expected use declaration")
            .argument().unwrap()
            .d().expect("Expected use declaration to have a scoped identifier")
            .name().unwrap()
            .utf8_text(code_str.as_bytes()).unwrap(),
        "Language"
    );
}