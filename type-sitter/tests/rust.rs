use type_sitter::{Node, OptionNodeResultExt, Parser, QueryCursor, StreamingIterator, Tree};

mod rust {
    use type_sitter::generate_nodes;

    generate_nodes!("../vendor/tree-sitter-rust/src/node-types.json");

    pub mod queries {
        use type_sitter::generate_queries;

        generate_queries!("../vendor/tree-sitter-rust/queries", "../vendor/tree-sitter-rust", super);
    }
}

const RUST_STR: &'static str = "\
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

fn rust_tree() -> Tree<rust::SourceFile<'static>> {
    let mut parser = Parser::<rust::SourceFile>::new(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    parser.parse(&RUST_STR, None).unwrap()
}

#[test]
fn test_node_types() {
    let rust_tree = rust_tree();
    let rust_source_file = rust_tree.root_node().unwrap();
    let mut cursor = rust_tree.walk();
    let mut cursor2 = rust_tree.walk();
    let mut cursor3 = rust_tree.walk();

    assert_eq!(rust_source_file.children(&mut cursor).count(), 1);
    let rust_source_file_child = rust_source_file.children(&mut cursor).next().unwrap2();
    assert_eq!(rust_source_file_child.prefixes().count(), 1);
    assert_eq!(rust_source_file_child.suffixes().count(), 0);
    let rust_doc = rust_source_file_child.prefixes().next().unwrap();
    assert_eq!(rust_doc.utf8_text(RUST_STR.as_bytes()).unwrap(), "/// Foo bar\n");
    let rust_main_fn = rust_source_file_child.as_declaration_statement().unwrap().as_function_item().unwrap();
    assert_eq!(rust_main_fn.name().unwrap().as_identifier().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "main");
    assert_eq!(rust_main_fn.parameters().unwrap().children(&mut cursor).count(), 0);
    let rust_main_fn_body = rust_main_fn.body().unwrap();
    assert_eq!(rust_main_fn_body.children(&mut cursor).count(), 3);
    let mut rust_main_fn_body_children = rust_main_fn_body.children(&mut cursor);
    let rust_let_json = rust_main_fn_body_children.next().unwrap2().as_declaration_statement().unwrap().as_let_declaration().unwrap();
    assert_eq!(rust_let_json.pattern().unwrap().as_identifier().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "json");
    assert!(!rust_let_json.children(&mut cursor2).any(|child| child.unwrap().as_mutable_specifier().is_ok()));
    let rust_json_str = rust_let_json.value().unwrap2().as_literal().unwrap().as_string_literal().unwrap();
    assert_eq!(rust_json_str.utf8_text(RUST_STR.as_bytes()).unwrap(), "\"{\n        \\\"type\\\": \\\"array\\\",\n        \\\"content\\\": \\\"value\\\"\n    }\"");
    let rust_let_mut_parser = rust_main_fn_body_children.next().unwrap2().as_declaration_statement().unwrap().as_let_declaration().unwrap();
    assert_eq!(rust_let_mut_parser.pattern().unwrap().as_identifier().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "parser");
    assert!(rust_let_mut_parser.children(&mut cursor2).any(|child| child.unwrap().as_mutable_specifier().is_ok()));
    let rust_parser_new = rust_let_mut_parser.value().unwrap2().as_call_expression().unwrap();
    assert_eq!(rust_parser_new.function().unwrap().as_scoped_identifier().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "tree_sitter::Parser::new");
    assert_eq!(rust_parser_new.arguments().unwrap().children(&mut cursor2).count(), 0);
    use rust::anon_unions::DeclarationStatement_Expression_ExpressionStatement_Label::*;
    let rust_todo = match rust_main_fn_body_children.next().unwrap2() {
        DeclarationStatement(decl) => decl.as_macro_invocation(),
        Expression(expr) => expr.as_macro_invocation(),
        ExpressionStatement(expr) => expr.child().unwrap().as_macro_invocation(),
        Label(label) => panic!("Expected declaration statement, expression, or expression statement, but got label: {:?}", label),
    }.unwrap();
    let mut rust_todo_children = rust_todo.children(&mut cursor2);
    assert_eq!(rust_todo.children(&mut cursor3).count(), 2);
    assert_eq!(rust_todo.r#macro().unwrap().as_identifier().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "todo");
    assert_eq!(rust_todo_children.next().unwrap2().as_identifier().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "todo");
    let rust_todo_arg = rust_todo_children.next().unwrap2().as_token_tree().unwrap();
    assert_eq!(rust_todo_arg.children(&mut cursor3).count(), 1);
    assert_eq!(rust_todo_arg.children(&mut cursor3).next().unwrap2().as_literal().unwrap().as_string_literal().unwrap().utf8_text(RUST_STR.as_bytes()).unwrap(), "\"baz\"");
}

#[test]
fn test_queries() {
    let rust_tree = rust_tree();
    let rust_source_file = rust_tree.root_node().unwrap();

    let mut q = QueryCursor::new();
    let matches_str = q
        .matches(&rust::queries::Highlights, rust_source_file, RUST_STR.as_bytes())
        .map_deref(|r#match| format!("{:?}\n", r#match))
        .collect::<String>();
    // println!("---\n{}\n---", matches_str);
    assert_eq!(matches_str, r#"
HighlightsMatch(QueryMatch { id: 0, pattern_index: 19, captures: [QueryCapture { node: {Node line_comment (0, 0) - (1, 0)}, index: 8 }] })
HighlightsMatch(QueryMatch { id: 1, pattern_index: 21, captures: [QueryCapture { node: {Node line_comment (0, 0) - (1, 0)}, index: 9 }] })
HighlightsMatch(QueryMatch { id: 2, pattern_index: 49, captures: [QueryCapture { node: {Node fn (1, 0) - (1, 2)}, index: 14 }] })
HighlightsMatch(QueryMatch { id: 3, pattern_index: 17, captures: [QueryCapture { node: {Node identifier (1, 3) - (1, 7)}, index: 5 }] })
HighlightsMatch(QueryMatch { id: 6, pattern_index: 23, captures: [QueryCapture { node: {Node ( (1, 7) - (1, 8)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 7, pattern_index: 24, captures: [QueryCapture { node: {Node ) (1, 8) - (1, 9)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 8, pattern_index: 27, captures: [QueryCapture { node: {Node { (1, 10) - (1, 11)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 9, pattern_index: 54, captures: [QueryCapture { node: {Node let (2, 4) - (2, 7)}, index: 14 }] })
HighlightsMatch(QueryMatch { id: 12, pattern_index: 81, captures: [QueryCapture { node: {Node string_literal (2, 15) - (5, 6)}, index: 16 }] })
HighlightsMatch(QueryMatch { id: 13, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (3, 8) - (3, 10)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 14, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (3, 14) - (3, 16)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 15, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (3, 18) - (3, 20)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 16, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (3, 25) - (3, 27)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 17, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (4, 8) - (4, 10)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 18, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (4, 17) - (4, 19)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 19, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (4, 21) - (4, 23)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 20, pattern_index: 86, captures: [QueryCapture { node: {Node escape_sequence (4, 28) - (4, 30)}, index: 18 }] })
HighlightsMatch(QueryMatch { id: 21, pattern_index: 35, captures: [QueryCapture { node: {Node ; (5, 6) - (5, 7)}, index: 11 }] })
HighlightsMatch(QueryMatch { id: 22, pattern_index: 54, captures: [QueryCapture { node: {Node let (6, 4) - (6, 7)}, index: 14 }] })
HighlightsMatch(QueryMatch { id: 23, pattern_index: 74, captures: [QueryCapture { node: {Node mutable_specifier (6, 8) - (6, 11)}, index: 14 }] })
HighlightsMatch(QueryMatch { id: 29, pattern_index: 31, captures: [QueryCapture { node: {Node :: (6, 32) - (6, 34)}, index: 11 }] })
HighlightsMatch(QueryMatch { id: 30, pattern_index: 6, captures: [QueryCapture { node: {Node identifier (6, 34) - (6, 40)}, index: 0 }] })
HighlightsMatch(QueryMatch { id: 32, pattern_index: 4, captures: [QueryCapture { node: {Node identifier (6, 34) - (6, 40)}, index: 4 }] })
HighlightsMatch(QueryMatch { id: 33, pattern_index: 31, captures: [QueryCapture { node: {Node :: (6, 40) - (6, 42)}, index: 11 }] })
HighlightsMatch(QueryMatch { id: 34, pattern_index: 12, captures: [QueryCapture { node: {Node identifier (6, 42) - (6, 45)}, index: 5 }] })
HighlightsMatch(QueryMatch { id: 37, pattern_index: 23, captures: [QueryCapture { node: {Node ( (6, 45) - (6, 46)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 38, pattern_index: 24, captures: [QueryCapture { node: {Node ) (6, 46) - (6, 47)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 39, pattern_index: 35, captures: [QueryCapture { node: {Node ; (6, 47) - (6, 48)}, index: 11 }] })
HighlightsMatch(QueryMatch { id: 42, pattern_index: 16, captures: [QueryCapture { node: {Node identifier (7, 4) - (7, 8)}, index: 7 }, QueryCapture { node: {Node ! (7, 8) - (7, 9)}, index: 7 }] })
HighlightsMatch(QueryMatch { id: 43, pattern_index: 23, captures: [QueryCapture { node: {Node ( (7, 9) - (7, 10)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 44, pattern_index: 81, captures: [QueryCapture { node: {Node string_literal (7, 10) - (7, 15)}, index: 16 }] })
HighlightsMatch(QueryMatch { id: 45, pattern_index: 24, captures: [QueryCapture { node: {Node ) (7, 15) - (7, 16)}, index: 10 }] })
HighlightsMatch(QueryMatch { id: 46, pattern_index: 28, captures: [QueryCapture { node: {Node } (8, 0) - (8, 1)}, index: 10 }] })
"#[1..]);
    let captures_str = q
        .captures(&rust::queries::Highlights, rust_source_file, RUST_STR.as_bytes())
        .map(|capture| format!("{:?}\n", capture))
        .collect::<String>();
    // println!("---\n{captures_str}\n---");
    assert_eq!(captures_str, r#"
HighlightsCapture::Comment { node: LineComment(LineComment({Node line_comment (0, 0) - (1, 0)})) }
HighlightsCapture::CommentDocumentation { node: LineComment(LineComment({Node line_comment (0, 0) - (1, 0)})) }
HighlightsCapture::Keyword { node: Fn(Fn({Node fn (1, 0) - (1, 2)})) }
HighlightsCapture::Function { node: Identifier({Node identifier (1, 3) - (1, 7)}) }
HighlightsCapture::PunctuationBracket { node: LParen(LParen({Node ( (1, 7) - (1, 8)})) }
HighlightsCapture::PunctuationBracket { node: RParen(RParen({Node ) (1, 8) - (1, 9)})) }
HighlightsCapture::PunctuationBracket { node: LBrace(LBrace({Node { (1, 10) - (1, 11)})) }
HighlightsCapture::Keyword { node: Let(Let({Node let (2, 4) - (2, 7)})) }
HighlightsCapture::String { node: StringLiteral(StringLiteral({Node string_literal (2, 15) - (5, 6)})) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (3, 8) - (3, 10)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (3, 14) - (3, 16)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (3, 18) - (3, 20)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (3, 25) - (3, 27)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (4, 8) - (4, 10)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (4, 17) - (4, 19)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (4, 21) - (4, 23)}) }
HighlightsCapture::Escape { node: EscapeSequence({Node escape_sequence (4, 28) - (4, 30)}) }
HighlightsCapture::PunctuationDelimiter { node: Semicolon(Semicolon({Node ; (5, 6) - (5, 7)})) }
HighlightsCapture::Keyword { node: Let(Let({Node let (6, 4) - (6, 7)})) }
HighlightsCapture::Keyword { node: MutableSpecifier(MutableSpecifier({Node mutable_specifier (6, 8) - (6, 11)})) }
HighlightsCapture::PunctuationDelimiter { node: ColonColon(ColonColon({Node :: (6, 32) - (6, 34)})) }
HighlightsCapture::Constructor { node: Identifier(Identifier({Node identifier (6, 34) - (6, 40)})) }
HighlightsCapture::Type { node: Identifier(Identifier({Node identifier (6, 34) - (6, 40)})) }
HighlightsCapture::PunctuationDelimiter { node: ColonColon(ColonColon({Node :: (6, 40) - (6, 42)})) }
HighlightsCapture::Function { node: Identifier({Node identifier (6, 42) - (6, 45)}) }
HighlightsCapture::PunctuationBracket { node: LParen(LParen({Node ( (6, 45) - (6, 46)})) }
HighlightsCapture::PunctuationBracket { node: RParen(RParen({Node ) (6, 46) - (6, 47)})) }
HighlightsCapture::PunctuationDelimiter { node: Semicolon(Semicolon({Node ; (6, 47) - (6, 48)})) }
HighlightsCapture::FunctionMacro { node: Identifier(Identifier({Node identifier (7, 4) - (7, 8)})) }
HighlightsCapture::FunctionMacro { node: Not(Not({Node ! (7, 8) - (7, 9)})) }
HighlightsCapture::PunctuationBracket { node: LParen(LParen({Node ( (7, 9) - (7, 10)})) }
HighlightsCapture::String { node: StringLiteral(StringLiteral({Node string_literal (7, 10) - (7, 15)})) }
HighlightsCapture::PunctuationBracket { node: RParen(RParen({Node ) (7, 15) - (7, 16)})) }
HighlightsCapture::PunctuationBracket { node: RBrace(RBrace({Node } (8, 0) - (8, 1)})) }
"#[1..]);
}