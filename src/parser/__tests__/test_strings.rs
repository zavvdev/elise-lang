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
    fn test_string() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::String("Hello, World!".to_string()),
                    span: TokenSpan::new(0, 15, "\"Hello, World!\"".to_string()),
                }],
                "\"Hello, World!\""
            ),
            vec![Expr {
                kind: ExprKind::String("Hello, World!".to_string()),
                children: vec![],
            }]
        );
    }
}
