use elise_frontend_parser::Prelude;
use elise_ir_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_shared_types::Span;

mod common;

#[test]
fn should_parse() {
    let ast = Prelude::new("null".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Null(AstNodeExprPrim {
            lexeme: "null".to_string(),
            span: Span { start: 0, end: 4 }
        }))])
    )
}
