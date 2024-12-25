#[cfg(test)]
mod tests {
    use crate::{
        lexer::models::token::{Token, TokenKind, TokenSpan},
        parser::{
            models::expression::{Expr, ExprKind},
            parse,
        },
    };

    // SUCCESS CASES

    #[test]
    fn test_identifier() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::Identifier("x".to_string()),
                    span: TokenSpan::new(0, 1, "x".to_string()),
                }],
                "x"
            ),
            vec![Expr {
                kind: ExprKind::Identifier("x".to_string()),
                children: vec![],
            }]
        );
    }
}
