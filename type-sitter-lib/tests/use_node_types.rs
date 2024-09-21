#![cfg(feature = "yak-sitter")]

mod json;
mod rust;

use std::path::Path;
use type_sitter_lib::{Node, Parser};

#[test]
pub fn test_use_node_types_rust() {
    let mut parser = Parser::<rust::nodes::SourceFile>::new(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    let code_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../vendor/tree-sitter-rust/bindings/rust/lib.rs");
    let code_ast = parser.parse_file(&code_path, None).expect("Failed to parse code");
    let code_root = code_ast.root_node().expect("Failed to wrap code root node");
    let statements = code_root.children(&mut code_root.walk())
        .map(|child| child.unwrap())
        .collect::<Vec<_>>();
    eprintln!("Statements:");
    for statement in &statements {
        eprintln!("  {}", statement.to_sexp());
    }
    assert!(matches!(statements[0], rust::nodes::anon_unions::DeclarationStatement_ExpressionStatement_Shebang::DeclarationStatement(rust::nodes::DeclarationStatement::UseDeclaration(_))));
    assert!(statements[0].as_declaration_statement().unwrap().as_use_declaration().is_some(), "Expected 1st statement to be a use declaration");
    assert!(statements[1].as_declaration_statement().unwrap().as_foreign_mod_item().is_some(), "Expected 2nd statement to be a foreign mod item");
    assert!(statements[2].as_declaration_statement().unwrap().as_const_item().is_some(), "Expected 3rd statement to be a const item");
    assert!(statements[3].as_declaration_statement().unwrap().as_const_item().is_some(), "Expected 4th statement to be a const item");
    assert!(statements[4].as_declaration_statement().unwrap().as_const_item().is_some(), "Expected 5th statement to be a const item");
    assert!(statements[5].as_declaration_statement().unwrap().as_const_item().is_some(), "Expected 6th statement to be a const item");
    assert!(statements[6].as_declaration_statement().unwrap().as_const_item().is_some(), "Expected 7th statement to be a const item");
    assert!(statements[7].as_declaration_statement().unwrap().as_attribute_item().is_some(), "Expected 8th statement to be an attribute item");
    assert!(statements[8].as_declaration_statement().unwrap().as_mod_item().is_some(), "Expected 9th statement to be a mod item");
    assert_eq!(
        statements[0]
            .as_declaration_statement().expect("Expected declaration statement")
            .as_use_declaration().expect("Expected use declaration")
            .argument().unwrap()
            .as_scoped_identifier().expect("Expected use declaration to have a scoped identifier")
            .path().expect("Expected use declaration's scoped identifier to have a path").unwrap()
            .as_identifier().expect("Expected use declaration's scope identifier to have an identifier")
            .text(),
        "tree_sitter_language"
    );
    assert_eq!(
        statements[0]
            .as_declaration_statement().expect("Expected declaration statement")
            .as_use_declaration().expect("Expected use declaration")
            .argument().unwrap()
            .as_scoped_identifier().expect("Expected use declaration to have a scoped identifier")
            .name().unwrap()
            .text(),
        "LanguageFn"
    );
    assert!(
        statements[8]
            .as_declaration_statement().expect("Expected declaration statement")
            .as_mod_item().expect("Expected mod item")
            .body().expect("Expected mod item to have a body").unwrap()
            .children(&mut code_root.walk()).skip(1).next().expect("Expected mod item's body to have a second child").unwrap()
            .as_function_item().expect("Expected mod item's body's second child to be a function item")
            .body().unwrap()
            .children(&mut code_root.walk()).skip(1).next().expect("Expected function item's body to have a second child").unwrap()
            .as_expression_statement().expect("Expected function item's body's second child to be an expression statement").child().unwrap()
            .as_call_expression().expect("Expected function item's body's second child to be a call expression")
            .function().unwrap()
            .as_field_expression().expect("Expected function item's body's second child's call expression's function to be a field expression")
            .value().unwrap()
            .as_call_expression().expect("Expected function item's body's second child's call expression's function's field expression's value to be a call expression")
            .arguments().unwrap()
            .children(&mut code_root.walk()).next().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression to have a child").unwrap()
            .as_expression().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child to be an expression")
            .as_reference_expression().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression to be a reference expression")
            .value().unwrap()
            .as_call_expression().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's value to be a call expression")
            .function().unwrap()
            .as_field_expression().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's value's call expression's function to be a field expression")
            .value().unwrap()
            .as_scoped_identifier().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's value's call expression's function's value to be a scoped identifier")
            .path().expect("Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's value's call expression's function's value's scoped identifier to have a path").unwrap()
            .as_super().is_some(),
        "Expected function item's body's second child's call expression's function's field expression's value's call expression's child's expression's value's call expression's function's scoped identifier's path to be a super_"
    );
    assert!(
        statements[8]
            .as_declaration_statement().expect("Expected declaration statement")
            .as_mod_item().expect("Expected mod item")
            .children(&mut code_root.walk()).all(|child|
                child.unwrap().as_visibility_modifier().is_none()),
        "Expected mod item to not have a visibility modifier"
    )
}