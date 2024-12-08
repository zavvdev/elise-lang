#[cfg(test)]
mod tests {
    use crate::lexer::{
        filter_tokens, lexemes,
        models::token::{Token, TokenKind, TokenSpan},
    };

    #[test]
    fn test_should_remove_whitespace_tokens() {
        let tokens = filter_tokens(vec![
            Token {
                kind: TokenKind::Whitespace,
                span: TokenSpan::new(0, 1, lexemes::L_WHITESPACE.to_string()),
            },
            Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string()),
            },
            Token {
                kind: TokenKind::Whitespace,
                span: TokenSpan::new(5, 6, lexemes::L_WHITESPACE.to_string()),
            },
        ]);

        assert_eq!(
            filter_tokens(tokens),
            vec![Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string())
            }]
        )
    }

    #[test]
    fn test_should_remove_newline_tokens() {
        let tokens = filter_tokens(vec![
            Token {
                kind: TokenKind::Newline,
                span: TokenSpan::new(0, 1, lexemes::L_NEWLINE.to_string()),
            },
            Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string()),
            },
            Token {
                kind: TokenKind::Newline,
                span: TokenSpan::new(5, 6, lexemes::L_NEWLINE.to_string()),
            },
        ]);

        assert_eq!(
            filter_tokens(tokens),
            vec![Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string())
            }]
        )
    }

    #[test]
    fn test_should_remove_comma_tokens() {
        let tokens = filter_tokens(vec![
            Token {
                kind: TokenKind::Comma,
                span: TokenSpan::new(0, 1, lexemes::L_COMMA.to_string()),
            },
            Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string()),
            },
            Token {
                kind: TokenKind::Comma,
                span: TokenSpan::new(5, 6, lexemes::L_COMMA.to_string()),
            },
        ]);

        assert_eq!(
            filter_tokens(tokens),
            vec![Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string())
            }]
        )
    }

    #[test]
    fn test_should_remove_all_ignored_tokens() {
        let tokens = filter_tokens(vec![
            Token {
                kind: TokenKind::Comma,
                span: TokenSpan::new(0, 1, lexemes::L_COMMA.to_string()),
            },
            Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string()),
            },
            Token {
                kind: TokenKind::Whitespace,
                span: TokenSpan::new(5, 6, lexemes::L_WHITESPACE.to_string()),
            },
            Token {
                kind: TokenKind::Newline,
                span: TokenSpan::new(6, 7, lexemes::L_NEWLINE.to_string()),
            },
        ]);

        assert_eq!(
            filter_tokens(tokens),
            vec![Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(1, 5, lexemes::L_TRUE.to_string())
            }]
        )
    }
}
