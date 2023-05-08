#![cfg(feature = "tree-sitter-wrapper")]

mod json;
mod rust;

use std::path::Path;
use type_sitter_lib::tree_sitter_wrapper::Parser;
use type_sitter_lib::TypedNode;

#[test]
pub fn test_use_node_types_rust() {
    let mut parser = Parser::new(tree_sitter_rust::language()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_ast = parser.parse_file(&code_path, None).expect("Failed to parse code");
    let code_root = rust::SourceFile::try_from(code_ast.root_node()).expect("Failed to wrap code root node");
    let statements = code_root.children(&mut code_root.walk())
        .filter_map(|child| child.unwrap().regular())
        .collect::<Vec<_>>();
    eprintln!("Statements:");
    for statement in &statements {
        eprintln!("  {}", statement.node().to_sexp());
    }
    assert!(matches!(statements[0], rust::anon_unions::DeclarationStatement_ExpressionStatement::DeclarationStatement(rust::DeclarationStatement::UseDeclaration(_))));
    assert!(statements[0].declaration_statement().unwrap().use_declaration().is_some(), "Expected 1st statement to be a use declaration");
    assert!(statements[1].declaration_statement().unwrap().foreign_mod_item().is_some(), "Expected 2nd statement to be a foreign mod item");
    assert!(statements[2].declaration_statement().unwrap().function_item().is_some(), "Expected 3rd statement to be a function item");
    assert!(statements[3].declaration_statement().unwrap().const_item().is_some(), "Expected 4th statement to be a const item");
    assert!(statements[4].declaration_statement().unwrap().const_item().is_some(), "Expected 5th statement to be a const item");
    assert!(statements[5].declaration_statement().unwrap().const_item().is_some(), "Expected 6th statement to be a const item");
    assert!(statements[6].declaration_statement().unwrap().const_item().is_some(), "Expected 7th statement to be a const item");
    assert!(statements[7].declaration_statement().unwrap().attribute_item().is_some(), "Expected 8th statement to be an attribute item");
    assert!(statements[8].declaration_statement().unwrap().mod_item().is_some(), "Expected 9th statement to be a mod item");
    assert_eq!(
        statements[0]
            .declaration_statement().expect("Expected declaration statement")
            .use_declaration().expect("Expected use declaration")
            .argument().unwrap()
            .scoped_identifier().expect("Expected use declaration to have a scoped identifier")
            .path().expect("Expected use declaration's scoped identifier to have a path").unwrap()
            .identifier().expect("Expected use declaration's scope identifier to have an identifier")
            .text(),
        "tree_sitter"
    );
    assert_eq!(
        statements[0]
            .declaration_statement().expect("Expected declaration statement")
            .use_declaration().expect("Expected use declaration")
            .argument().unwrap()
            .scoped_identifier().expect("Expected use declaration to have a scoped identifier")
            .name().unwrap()
            .text(),
        "Language"
    );
    assert!(
        statements[8]
            .declaration_statement().expect("Expected declaration statement")
            .mod_item().expect("Expected mod item")
            .body().expect("Expected mod item to have a body").unwrap()
            .children(&mut code_root.walk()).skip(1).next().expect("Expected mod item's body to have a second child").unwrap().expect("Expected mod item's body's second child to be a regular node")
            .function_item().expect("Expected mod item's body's second child to be a function item")
            .body().unwrap()
            .children(&mut code_root.walk()).skip(1).next().expect("Expected function item's body to have a second child").unwrap().expect("Expected function item's body's second child to be a regular node")
            .expression_statement().expect("Expected function item's body's second child to be an expression statement").child().unwrap()
            .call_expression().expect("Expected function item's body's second child to be a call expression")
            .function().unwrap()
            .field_expression().expect("Expected function item's body's second child's call expression's function to be a field expression")
            .value().unwrap()
            .call_expression().expect("Expected function item's body's second child's call expression's function's field expression's value to be a call expression")
            .arguments().unwrap()
            .children(&mut code_root.walk()).next().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression to have a child").unwrap().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child to be a regular node")
            .expression().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child to be an expression")
            .call_expression().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression to be a call expression")
            .function().unwrap()
            .scoped_identifier().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's call expression's function to be a scoped identifier")
            .path().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's call expression's function's scoped identifier to have a path").unwrap()
            .super_().is_some(),
        "Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's call expression's function's scoped identifier's path to be a super_"
    );
    assert!(
        statements[8]
            .declaration_statement().expect("Expected declaration statement")
            .mod_item().expect("Expected mod item")
            .children(&mut code_root.walk()).all(|child| child.unwrap()
                .expect("Expected mod item to not have an extra")
                .visibility_modifier().is_none()),
        "Expected mod item to not have a visibility modifier"
    )
}