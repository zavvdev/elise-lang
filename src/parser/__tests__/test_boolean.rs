#[cfg(test)]
mod tests {
    use crate::{
        lexer::{
            lexemes,
            models::token::{Token, TokenKind, TokenSpan},
        },
        parser::{models::expression::{Expr, ExprKind}, parse},
    };

    #[test]
    fn test_true() {
        assert_eq!(
            parse(vec![Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(0, 4, lexemes::L_TRUE.to_string()),
            }]),
            vec![Expr {
                kind: ExprKind::Boolean(true),
                children: vec![],
            }]
        );
    }

    #[test]
    fn test_false() {
        assert_eq!(
            parse(vec![Token {
                kind: TokenKind::Boolean(false),
                span: TokenSpan::new(0, 5, lexemes::L_FALSE.to_string()),
            }]),
            vec![Expr {
                kind: ExprKind::Boolean(false),
                children: vec![],
            }]
        );
    }
}
