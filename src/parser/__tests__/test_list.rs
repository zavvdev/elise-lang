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
    fn test_empty_list() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::LeftSqrBr,
                    span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string()),
                },
                Token {
                    kind: TokenKind::RightSqrBr,
                    span: TokenSpan::new(1, 2, lexemes::L_RIGHT_SQR_BR.to_string()),
                },
            ]),
            vec![Expr {
                kind: ExprKind::List,
                children: vec![],
            }]
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::LeftSqrBr,
                    span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string()),
                },
                Token {
                    kind: TokenKind::Number(2.2),
                    span: TokenSpan::new(1, 4, "2.2".to_string()),
                },
                Token {
                    kind: TokenKind::RightSqrBr,
                    span: TokenSpan::new(4, 5, lexemes::L_RIGHT_SQR_BR.to_string()),
                },
            ]),
            vec![Expr {
                kind: ExprKind::List,
                children: vec![Box::new(Expr {
                    kind: ExprKind::Number(2.2),
                    children: vec![],
                })],
            }]
        );
    }

    #[test]
    fn test_nested_list() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::LeftSqrBr,
                    span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string()),
                },
                Token {
                    kind: TokenKind::Number(2.2),
                    span: TokenSpan::new(1, 4, "2.2".to_string()),
                },
                Token {
                    kind: TokenKind::LeftSqrBr,
                    span: TokenSpan::new(4, 5, lexemes::L_LEFT_SQR_BR.to_string()),
                },
                Token {
                    kind: TokenKind::Number(4.2),
                    span: TokenSpan::new(5, 8, "4.2".to_string()),
                },
                Token {
                    kind: TokenKind::Number(4.6),
                    span: TokenSpan::new(8, 11, "4.6".to_string()),
                },
                Token {
                    kind: TokenKind::RightSqrBr,
                    span: TokenSpan::new(11, 12, lexemes::L_RIGHT_SQR_BR.to_string()),
                },
                Token {
                    kind: TokenKind::RightSqrBr,
                    span: TokenSpan::new(12, 13, lexemes::L_RIGHT_SQR_BR.to_string()),
                },
            ]),
            vec![Expr {
                kind: ExprKind::List,
                children: vec![
                    Box::new(Expr {
                        kind: ExprKind::Number(2.2),
                        children: vec![],
                    }),
                    Box::new(Expr {
                        kind: ExprKind::List,
                        children: vec![
                            Box::new(Expr {
                                kind: ExprKind::Number(4.2),
                                children: vec![],
                            }),
                            Box::new(Expr {
                                kind: ExprKind::Number(4.6),
                                children: vec![],
                            })
                        ],
                    })
                ],
            }]
        );
    }

    #[test]
    #[should_panic(expected = "Unexpected end of input")]
    fn test_unclosed_list() {
        parse(vec![
            Token {
                kind: TokenKind::LeftSqrBr,
                span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string()),
            },
            Token {
                kind: TokenKind::Number(2.2),
                span: TokenSpan::new(1, 4, "2.2".to_string()),
            },
        ]);
    }

    #[test]
    #[should_panic(expected = "Parse error. Unexpected token")]
    fn test_unclosed_list_2() {
        parse(vec![Token {
            kind: TokenKind::LeftSqrBr,
            span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string()),
        }]);
    }
}
