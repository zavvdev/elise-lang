#[cfg(test)]
mod tests {
    use crate::lexer::{
        collect_tokens, lexemes,
        models::token::{Token, TokenKind, TokenSpan},
    };

    #[test]
    fn test_should_include_ignored_tokens() {
        assert_eq!(
            collect_tokens(
                "true, false
42"
            ),
            vec![
                Token {
                    kind: TokenKind::Boolean(true),
                    span: TokenSpan::new(0, 4, lexemes::L_TRUE.to_string()),
                },
                Token {
                    kind: TokenKind::Comma,
                    span: TokenSpan::new(4, 5, lexemes::L_COMMA.to_string()),
                },
                Token {
                    kind: TokenKind::Whitespace,
                    span: TokenSpan::new(5, 6, lexemes::L_WHITESPACE.to_string()),
                },
                Token {
                    kind: TokenKind::Boolean(false),
                    span: TokenSpan::new(6, 11, lexemes::L_FALSE.to_string()),
                },
                Token {
                    kind: TokenKind::Newline,
                    span: TokenSpan::new(11, 12, lexemes::L_NEWLINE.to_string()),
                },
                Token {
                    kind: TokenKind::Number(42.0),
                    span: TokenSpan::new(12, 14, "42".to_string()),
                },
            ]
        )
    }
}
