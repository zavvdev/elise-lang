#[cfg(test)]
mod tests {
    use crate::lexer::{lexemes, models::token::{Token, TokenKind, TokenSpan}, tokenize};

    #[test]
    fn test_true() {
        assert_eq!(
            tokenize("true"),
            vec![Token {
                kind: TokenKind::Boolean(true),
                span: TokenSpan::new(0, 4, lexemes::L_TRUE.to_string())
            }]
        )
    }

    #[test]
    fn test_false() {
        assert_eq!(
            tokenize("false"),
            vec![Token {
                kind: TokenKind::Boolean(false),
                span: TokenSpan::new(0, 5, lexemes::L_FALSE.to_string())
            }]
        )
    }
}
