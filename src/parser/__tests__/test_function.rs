#[cfg(test)]
mod tests {
    use crate::{
        lexer::{
            lexemes::{self, fn_lexeme_to_string},
            models::token::{Token, TokenKind, TokenSpan},
        },
        parser::{
            models::expression::{Expr, ExprKind},
            parse,
        },
        types,
    };

    // SUCCESS CASES

    #[test]
    fn test_function_no_children() {
        assert_eq!(
            parse(
                vec![
                    Token {
                        kind: TokenKind::FnAdd,
                        span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                    },
                    Token {
                        kind: TokenKind::LeftParen,
                        span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                    },
                    Token {
                        kind: TokenKind::RightParen,
                        span: TokenSpan::new(5, 6, lexemes::L_RIGHT_PAREN.to_string()),
                    }
                ],
                ".add()"
            ),
            vec![Expr {
                kind: ExprKind::FnAdd,
                children: vec![],
            }]
        );
    }

    #[test]
    fn test_function() {
        assert_eq!(
            parse(
                vec![
                    Token {
                        kind: TokenKind::FnAdd,
                        span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                    },
                    Token {
                        kind: TokenKind::LeftParen,
                        span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                    },
                    Token {
                        kind: TokenKind::Number(2 as types::Number),
                        span: TokenSpan::new(5, 6, "2".to_string()),
                    },
                    Token {
                        kind: TokenKind::Number(3.4),
                        span: TokenSpan::new(8, 9, "3.4".to_string()),
                    },
                    Token {
                        kind: TokenKind::RightParen,
                        span: TokenSpan::new(9, 10, lexemes::L_RIGHT_PAREN.to_string()),
                    },
                ],
                ".add(2 3.4)"
            ),
            vec![Expr {
                kind: ExprKind::FnAdd,
                children: vec![
                    Box::new(Expr {
                        kind: ExprKind::Number(2 as types::Number),
                        children: vec![],
                    }),
                    Box::new(Expr {
                        kind: ExprKind::Number(3.4),
                        children: vec![],
                    }),
                ],
            }]
        );
    }

    #[test]
    fn test_function_nested() {
        assert_eq!(
            parse(
                vec![
                    Token {
                        kind: TokenKind::FnAdd,
                        span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                    },
                    Token {
                        kind: TokenKind::LeftParen,
                        span: TokenSpan::new(5, 6, lexemes::L_LEFT_PAREN.to_string()),
                    },
                    Token {
                        kind: TokenKind::FnAdd,
                        span: TokenSpan::new(6, 10, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                    },
                    Token {
                        kind: TokenKind::LeftParen,
                        span: TokenSpan::new(10, 11, lexemes::L_LEFT_PAREN.to_string()),
                    },
                    Token {
                        kind: TokenKind::Number(3.4),
                        span: TokenSpan::new(11, 14, "3.4".to_string()),
                    },
                    Token {
                        kind: TokenKind::Number(1 as types::Number),
                        span: TokenSpan::new(14, 15, "1".to_string()),
                    },
                    Token {
                        kind: TokenKind::RightParen,
                        span: TokenSpan::new(15, 16, lexemes::L_RIGHT_PAREN.to_string()),
                    },
                    Token {
                        kind: TokenKind::Number(2 as types::Number),
                        span: TokenSpan::new(16, 17, "2".to_string()),
                    },
                    Token {
                        kind: TokenKind::RightParen,
                        span: TokenSpan::new(17, 18, lexemes::L_RIGHT_PAREN.to_string()),
                    },
                ],
                ".add(.add(3.4 1) 2)"
            ),
            vec![Expr {
                kind: ExprKind::FnAdd,
                children: vec![
                    Box::new(Expr {
                        kind: ExprKind::FnAdd,
                        children: vec![
                            Box::new(Expr {
                                kind: ExprKind::Number(3.4),
                                children: vec![],
                            }),
                            Box::new(Expr {
                                kind: ExprKind::Number(1 as types::Number),
                                children: vec![],
                            }),
                        ],
                    }),
                    Box::new(Expr {
                        kind: ExprKind::Number(2 as types::Number),
                        children: vec![],
                    }),
                ],
            }]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_function_unclosed_paren() {
        parse(
            vec![
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::Number(2 as types::Number),
                    span: TokenSpan::new(5, 6, "2".to_string()),
                },
            ],
            ".add(2",
        );
    }

    #[test]
    #[should_panic]
    fn test_function_unclosed_paren_no_children() {
        parse(
            vec![
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                },
            ],
            ".add(",
        );
    }

    #[test]
    #[should_panic]
    fn test_function_unclosed_paren_nested() {
        parse(
            vec![
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(5, 9, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(9, 10, lexemes::L_LEFT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::Number(3.4),
                    span: TokenSpan::new(10, 13, "3.4".to_string()),
                },
                Token {
                    kind: TokenKind::Number(1 as types::Number),
                    span: TokenSpan::new(13, 14, "1".to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(14, 15, lexemes::L_RIGHT_PAREN.to_string()),
                },
            ],
            ".add(.add(3.4 1)",
        );
    }

    #[test]
    #[should_panic]
    fn test_unmatched_closed() {
        parse(
            vec![
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(5, 9, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(9, 10, lexemes::L_LEFT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::Number(3.4),
                    span: TokenSpan::new(10, 13, "3.4".to_string()),
                },
                Token {
                    kind: TokenKind::Number(1 as types::Number),
                    span: TokenSpan::new(13, 14, "1".to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(14, 15, lexemes::L_RIGHT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(15, 16, lexemes::L_RIGHT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(16, 17, lexemes::L_RIGHT_PAREN.to_string()),
                },
            ],
            ".add(.add(3.4 1)))",
        );
    }
}
