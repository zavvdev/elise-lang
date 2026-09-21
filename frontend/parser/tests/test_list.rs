use elise_ast::{AstNode, AstNodeExpr, AstNodeExprList, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_parse_empty() {
    let ast = Prelude::new("[]".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
            span: Span { start: 0, end: 2 },
            items: vec![],
        }))])
    );
}

#[test]
fn should_parse_nested_empty() {
    let ast = Prelude::new("[[]]".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
            span: Span { start: 0, end: 4 },
            items: vec![Box::new(AstNodeExpr::List(AstNodeExprList {
                span: Span { start: 1, end: 3 },
                items: vec![],
            }))],
        }))])
    );
}

#[test]
fn should_parse_non_empty() {
    let ast = Prelude::new("[1, \"hello\", null, false]".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
            span: Span { start: 0, end: 25 },
            items: vec![
                Box::new(AstNodeExpr::Int(AstNodeExprPrim {
                    lexeme: "1".to_string(),
                    span: Span { start: 1, end: 2 },
                })),
                Box::new(AstNodeExpr::Str(AstNodeExprPrim {
                    lexeme: "hello".to_string(),
                    span: Span { start: 4, end: 11 },
                })),
                Box::new(AstNodeExpr::Null(AstNodeExprPrim {
                    lexeme: "null".to_string(),
                    span: Span { start: 13, end: 17 },
                })),
                Box::new(AstNodeExpr::Bool(AstNodeExprPrim {
                    lexeme: "false".to_string(),
                    span: Span { start: 19, end: 24 },
                }))
            ],
        }))])
    );
}

#[test]
fn should_not_allow_non_closed() {
    let code = "[[1, 3]";
    assert_eq!(
        Prelude::new(code.as_bytes()).parse(),
        Err(ParserErr::UnexpEoFile(ParserErrInfo { pos: 7 }))
    );
}
