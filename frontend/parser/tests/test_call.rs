use elise_ast::{AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_parse_with_no_arguments() {
    let ast = Prelude::new(".some-fn()".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
            lexeme: "some-fn".to_string(),
            span: Span { start: 0, end: 10 },
            body: vec![],
        }))])
    );
}

#[test]
fn should_parse_with_arguments() {
    let ast = Prelude::new(".add(2 .div(4 2))".as_bytes()).parse();
    let nested_children = vec![
        Box::new(AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
            lexeme: "4".to_string(),
            span: Span { start: 12, end: 13 },
        }))),
        Box::new(AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
            lexeme: "2".to_string(),
            span: Span { start: 14, end: 15 },
        }))),
    ];
    let body = vec![
        Box::new(AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
            lexeme: "2".to_string(),
            span: Span { start: 5, end: 6 },
        }))),
        Box::new(AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
            lexeme: "div".to_string(),
            span: Span { start: 7, end: 16 },
            body: nested_children,
        }))),
    ];
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
            lexeme: "add".to_string(),
            span: Span { start: 0, end: 17 },
            body,
        }))])
    );
}

#[test]
fn should_parse_with_separators_after_name() {
    let inputs = vec![
        (".test ()", 8),
        (".test  ()", 9),
        (
            ".test
                 ()",
            25,
        ),
        (
            ".test
                             ()",
            37,
        ),
    ];
    for (input, end) in inputs {
        assert_eq!(
            Prelude::new(input.as_bytes()).parse(),
            Ok(vec![AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
                lexeme: "test".to_string(),
                span: Span { start: 0, end },
                body: vec![],
            }))])
        );
    }
}

#[test]
fn should_not_allow_non_closed() {
    let code = ".some-fn(2 2 3))";
    assert_eq!(
        Prelude::new(code.as_bytes()).parse(),
        Err(ParserErr::UnexpTok(ParserErrInfo { pos: 15 }))
    );
}

#[test]
fn should_not_allow_separator_after_call_symbol() {
    let code = ". some-fn()";
    assert_eq!(
        Prelude::new(code.as_bytes()).parse(),
        Err(ParserErr::InvalFnName(ParserErrInfo { pos: 9 }))
    );
}

#[test]
fn should_reject_invalid_names() {
    let identifiers = vec![
        ("1asd", 5),
        ("!asd", 5),
        ("@asd", 5),
        ("#asd", 5),
        ("$asd", 5),
        ("%asd", 5),
        ("^asd", 5),
        ("&asd", 5),
        ("*asd", 5),
        ("-asd", 5),
        ("_asd", 5),
        ("=asd", 5),
        ("+asd", 5),
        ("?asd", 5),
        ("?asd", 5),
        (">asd", 5),
        ("<asd", 5),
        ("/asd", 5),
        ("asd<", 5),
        ("asd>", 5),
        ("asd%", 5),
    ];
    for (identifier, pos) in identifiers {
        assert_eq!(
            Prelude::new(&format!(".{}()", identifier).as_bytes()).parse(),
            Err(ParserErr::InvalFnName(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_not_allow_standalone_parens() {
    let code = "()";
    assert_eq!(
        Prelude::new(code.as_bytes()).parse(),
        Err(ParserErr::UnexpTok(ParserErrInfo { pos: 0 }))
    );
}

#[test]
fn should_not_allow_missing_names() {
    assert_eq!(
        Prelude::new(".()".as_bytes()).parse(),
        Err(ParserErr::InvalFnName(ParserErrInfo { pos: 1 }))
    );
}
