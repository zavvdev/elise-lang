#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::lexer::{
        lexemes::FORBIDDEN_IDENTIFIER_NAMES,
        models::token::{Token, TokenKind, TokenSpan},
        tokenize,
    };

    // SUCCESS CASES

    #[test]
    fn test_valid_identifiers() {
        assert_eq!(
            tokenize("x"),
            vec![Token {
                kind: TokenKind::Identifier("x".to_string()),
                span: TokenSpan::new(0, 1, "x".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello"),
            vec![Token {
                kind: TokenKind::Identifier("hello".to_string()),
                span: TokenSpan::new(0, 5, "hello".to_string())
            }]
        );

        assert_eq!(
            tokenize("HELLO"),
            vec![Token {
                kind: TokenKind::Identifier("HELLO".to_string()),
                span: TokenSpan::new(0, 5, "HELLO".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello123"),
            vec![Token {
                kind: TokenKind::Identifier("hello123".to_string()),
                span: TokenSpan::new(0, 8, "hello123".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello_world"),
            vec![Token {
                kind: TokenKind::Identifier("hello_world".to_string()),
                span: TokenSpan::new(0, 11, "hello_world".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello-world"),
            vec![Token {
                kind: TokenKind::Identifier("hello-world".to_string()),
                span: TokenSpan::new(0, 11, "hello-world".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello?"),
            vec![Token {
                kind: TokenKind::Identifier("hello?".to_string()),
                span: TokenSpan::new(0, 6, "hello?".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello!"),
            vec![Token {
                kind: TokenKind::Identifier("hello!".to_string()),
                span: TokenSpan::new(0, 6, "hello!".to_string())
            }]
        );

        assert_eq!(
            tokenize("hello_world-42"),
            vec![Token {
                kind: TokenKind::Identifier("hello_world-42".to_string()),
                span: TokenSpan::new(0, 14, "hello_world-42".to_string())
            }]
        );

        assert_eq!(
            tokenize("_hello"),
            vec![Token {
                kind: TokenKind::Identifier("_hello".to_string()),
                span: TokenSpan::new(0, 6, "_hello".to_string())
            }]
        );

        assert_eq!(
            tokenize("~hello"),
            vec![Token {
                kind: TokenKind::Identifier("~hello".to_string()),
                span: TokenSpan::new(0, 6, "~hello".to_string())
            }]
        );
    }

    #[test]
    fn ends_with_whitespace() {
        assert_eq!(
            tokenize("hello "),
            vec![
                Token {
                    kind: TokenKind::Identifier("hello".to_string()),
                    span: TokenSpan::new(0, 5, "hello".to_string()),
                },
                Token {
                    kind: TokenKind::Whitespace,
                    span: TokenSpan::new(5, 6, " ".to_string()),
                }
            ]
        );
    }

    #[test]
    fn ends_with_newline() {
        assert_eq!(
            tokenize(
                "hello
"
            ),
            vec![
                Token {
                    kind: TokenKind::Identifier("hello".to_string()),
                    span: TokenSpan::new(0, 5, "hello".to_string()),
                },
                Token {
                    kind: TokenKind::Newline,
                    span: TokenSpan::new(5, 6, "\n".to_string()),
                }
            ]
        );
    }

    #[test]
    fn ends_with_comma() {
        assert_eq!(
            tokenize("hello,"),
            vec![
                Token {
                    kind: TokenKind::Identifier("hello".to_string()),
                    span: TokenSpan::new(0, 5, "hello".to_string()),
                },
                Token {
                    kind: TokenKind::Comma,
                    span: TokenSpan::new(5, 6, ",".to_string()),
                }
            ]
        );
    }

    #[test]
    fn ends_with_right_paren() {
        assert_eq!(
            tokenize("hello)"),
            vec![
                Token {
                    kind: TokenKind::Identifier("hello".to_string()),
                    span: TokenSpan::new(0, 5, "hello".to_string()),
                },
                Token {
                    kind: TokenKind::RightParen,
                    span: TokenSpan::new(5, 6, ")".to_string()),
                }
            ]
        );
    }

    #[test]
    fn ends_with_right_sqr_br() {
        assert_eq!(
            tokenize("hello]"),
            vec![
                Token {
                    kind: TokenKind::Identifier("hello".to_string()),
                    span: TokenSpan::new(0, 5, "hello".to_string()),
                },
                Token {
                    kind: TokenKind::RightSqrBr,
                    span: TokenSpan::new(5, 6, "]".to_string()),
                }
            ]
        );
    }

    // FAILURE CASES

    #[test]
    fn test_invalid_identifiers() {
        let invalid_identifiers = vec![
            "?hello",
            "!hello",
            "@hello",
            "hello@world",
            "hello.world",
            "hello~world",
            "hello#world",
            "hello$world",
            "hello%world",
            "hello^world",
            "hello&world",
            "hello*world",
            "hello+world",
            "hello=world",
            "hello/world",
            "hello\\world",
            "hello\"world",
            "hello'world",
            "hello>world",
            "hello<world",
            "hello;world",
            "hello:world",
        ];

        for invalid_identifier in invalid_identifiers {
            assert_panic!(
                {
                    tokenize(invalid_identifier);
                },
                String,
            );
        }
    }

    #[test]
    fn test_forbidden_identifier_names() {
        for forbidden_name in FORBIDDEN_IDENTIFIER_NAMES.iter() {
            assert_panic!(
                {
                    tokenize(format!(".let ([{} 42])", forbidden_name).as_str());
                },
                String,
            );
        }
    }
}
