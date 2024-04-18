pub mod models;

use self::models::{ast::AstNode, Parser};
use crate::lexer::models::token::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut parser = Parser::new(tokens);
    let mut nodes: Vec<AstNode> = Vec::new();

    while let Some(node) = parser.next_node() {
        nodes.push(node);
    }

    nodes
}

// ======== Tests ========

#[cfg(test)]
mod tests {
    use tests::models::ast::AstNodeKind;

    use crate::lexer::{
        lexemes::{self, fn_lexeme_to_string},
        models::token::{TokenKind, TokenSpan},
    };

    use super::*;

    // ==========================

    //          Numbers

    // ==========================

    #[test]
    fn test_parse_int() {
        assert_eq!(
            parse(vec![Token {
                kind: TokenKind::Int(42),
                span: TokenSpan::new(0, 2, "42".to_string())
            }]),
            vec![AstNode {
                kind: AstNodeKind::Int(42),
                args: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_int_negative() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::Minus,
                    span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string())
                },
                Token {
                    kind: TokenKind::Int(2),
                    span: TokenSpan::new(1, 2, "2".to_string())
                }
            ]),
            vec![AstNode {
                kind: AstNodeKind::Int(-2),
                args: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            parse(vec![Token {
                kind: TokenKind::Float(4.2),
                span: TokenSpan::new(0, 3, "4.2".to_string())
            }]),
            vec![AstNode {
                kind: AstNodeKind::Float(4.2),
                args: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_float_negative() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::Minus,
                    span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string())
                },
                Token {
                    kind: TokenKind::Float(5.6),
                    span: TokenSpan::new(1, 4, "5.6".to_string())
                }
            ]),
            vec![AstNode {
                kind: AstNodeKind::Float(-5.6),
                args: vec![],
            }]
        );
    }

    // ==========================

    //     Unexpected Tokens

    // ==========================

    #[test]
    #[should_panic(expected = "Unexpected token")]
    fn test_unexpected_token_trailing_minus() {
        parse(vec![Token {
            kind: TokenKind::Minus,
            span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string()),
        }]);
    }

    #[test]
    #[should_panic(expected = "Unexpected token")]
    fn test_unexpected_token_minus() {
        parse(vec![
            Token {
                kind: TokenKind::Minus,
                span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string()),
            },
            Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(1, 5, fn_lexeme_to_string(lexemes::L_FN_ADD)),
            },
        ]);
    }

    // ==========================

    //      Known functions

    // ==========================

    #[test]
    #[should_panic(expected = "Unexpected end of input")]
    fn test_known_function_unclosed_paren() {
        parse(vec![
            Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
            },
            Token {
                kind: TokenKind::LeftParen,
                span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
            },
            Token {
                kind: TokenKind::Int(2),
                span: TokenSpan::new(5, 6, "2".to_string()),
            },
        ]);
    }

    #[test]
    #[should_panic(expected = "Unexpected end of input")]
    fn test_known_function_unclosed_paren_no_args() {
        parse(vec![
            Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
            },
            Token {
                kind: TokenKind::LeftParen,
                span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
            },
        ]);
    }

    #[test]
    #[should_panic(expected = "Unexpected end of input")]
    fn test_known_function_unclosed_paren_nested() {
        parse(vec![
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
                kind: TokenKind::Float(3.4),
                span: TokenSpan::new(10, 13, "3.4".to_string()),
            },
            Token {
                kind: TokenKind::Int(1),
                span: TokenSpan::new(13, 14, "1".to_string()),
            },
            Token {
                kind: TokenKind::RightParen,
                span: TokenSpan::new(14, 15, lexemes::L_RIGHT_PAREN.to_string()),
            },
        ]);
    }

    #[test]
    fn test_known_function_no_args() {
        assert_eq!(
            parse(vec![
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
            ]),
            vec![AstNode {
                kind: AstNodeKind::FnAdd,
                args: vec![],
            }]
        );
    }

    #[test]
    fn test_known_function() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::LeftParen,
                    span: TokenSpan::new(4, 5, lexemes::L_LEFT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::Int(2),
                    span: TokenSpan::new(5, 6, "2".to_string()),
                },
                Token {
                    kind: TokenKind::Whitespace,
                    span: TokenSpan::new(6, 7, lexemes::L_WHITESPACE.to_string()),
                },
                Token {
                    kind: TokenKind::Float(3.4),
                    span: TokenSpan::new(8, 9, "3.4".to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(9, 10, lexemes::L_RIGHT_PAREN.to_string()),
                },
            ]),
            vec![AstNode {
                kind: AstNodeKind::FnAdd,
                args: vec![
                    Box::new(AstNode {
                        kind: AstNodeKind::Int(2),
                        args: vec![],
                    }),
                    Box::new(AstNode {
                        kind: AstNodeKind::Float(3.4),
                        args: vec![],
                    }),
                ],
            }]
        );
    }

    #[test]
    fn test_known_function_nested() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::FnAdd,
                    span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD)),
                },
                Token {
                    kind: TokenKind::Whitespace,
                    span: TokenSpan::new(4, 5, lexemes::L_WHITESPACE.to_string()),
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
                    kind: TokenKind::Float(3.4),
                    span: TokenSpan::new(11, 14, "3.4".to_string()),
                },
                Token {
                    kind: TokenKind::Int(1),
                    span: TokenSpan::new(14, 15, "1".to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(15, 16, lexemes::L_RIGHT_PAREN.to_string()),
                },
                Token {
                    kind: TokenKind::Int(2),
                    span: TokenSpan::new(16, 17, "2".to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(17, 18, lexemes::L_RIGHT_PAREN.to_string()),
                },
            ]),
            vec![AstNode {
                kind: AstNodeKind::FnAdd,
                args: vec![
                    Box::new(AstNode {
                        kind: AstNodeKind::FnAdd,
                        args: vec![
                            Box::new(AstNode {
                                kind: AstNodeKind::Float(3.4),
                                args: vec![],
                            }),
                            Box::new(AstNode {
                                kind: AstNodeKind::Int(1),
                                args: vec![],
                            }),
                        ],
                    }),
                    Box::new(AstNode {
                        kind: AstNodeKind::Int(2),
                        args: vec![],
                    }),
                ],
            }]
        );
    }
}
