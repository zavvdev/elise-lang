use elise_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::shared_types::Span;

mod common;

#[test]
fn should_parse_true() {
    let ast = Prelude::new("true".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Bool(AstNodeExprPrim {
            lexeme: "true".to_string(),
            span: Span { start: 0, end: 4 }
        }))])
    )
}

#[test]
fn should_parse_false() {
    let ast = Prelude::new("false".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Bool(AstNodeExprPrim {
            lexeme: "false".to_string(),
            span: Span { start: 0, end: 5 }
        }))])
    )
}
