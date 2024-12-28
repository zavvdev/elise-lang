#[cfg(test)]
mod tests {
    use crate::{
        lexer::{
            lexemes,
            models::token::{Token, TokenKind, TokenSpan},
        },
        parser::{
            models::expression::{Expr, ExprKind},
            parse,
        },
    };

    // SUCCESS CASES

    #[test]
    fn test_nil() {
        assert_eq!(
            parse(
                vec![Token {
                    kind: TokenKind::Nil,
                    span: TokenSpan::new(0, 3, lexemes::L_NIL.to_string()),
                }],
                "nil"
            ),
            vec![Expr {
                kind: ExprKind::Nil,
                children: vec![],
                start_at: 0,
            }]
        );
    }
}
