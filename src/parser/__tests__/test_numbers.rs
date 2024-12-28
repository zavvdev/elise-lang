#[cfg(test)]
mod tests {
    use crate::{
        lexer::models::token::{Token, TokenKind, TokenSpan},
        parser::{
            models::expression::{Expr, ExprKind},
            parse,
        },
        types,
    };

    // SUCCESS CASES

    #[test]
    fn test_int() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::Number(42 as types::Number),
                    span: TokenSpan::new(0, 2, "42".to_string())
                }],
                "42"
            ),
            vec![Expr {
                kind: ExprKind::Number(42 as types::Number),
                children: vec![],
                start_at: 0,
            }]
        );
    }

    #[test]
    fn test_int_negative() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::Number(-2 as types::Number),
                    span: TokenSpan::new(0, 2, "-2".to_string())
                }],
                "-2"
            ),
            vec![Expr {
                kind: ExprKind::Number(-2 as types::Number),
                children: vec![],
                start_at: 0,
            }]
        );
    }

    #[test]
    fn test_float() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::Number(4.2),
                    span: TokenSpan::new(0, 3, "4.2".to_string())
                }],
                "4.2"
            ),
            vec![Expr {
                kind: ExprKind::Number(4.2),
                children: vec![],
                start_at: 0,
            }]
        );
    }

    #[test]
    fn test_float_negative() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::Number(-5.6),
                    span: TokenSpan::new(0, 4, "-5.6".to_string())
                }],
                "-5.6"
            ),
            vec![Expr {
                kind: ExprKind::Number(-5.6),
                children: vec![],
                start_at: 0,
            }]
        );
    }
}
