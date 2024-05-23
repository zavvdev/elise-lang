#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::lexer::{models::token::{Token, TokenKind, TokenSpan}, tokenize};

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
    }

    #[test]
    fn test_invalid_identifiers() {
        let invalid_identifiers = vec![
            "?hello",
            "!hello",
            "hello@world",
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
                format!(
                    "Lexing error. Invalid identifier name \"{}\".",
                    invalid_identifier
                )
            );
        }
    }
}
