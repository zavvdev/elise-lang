use self::models::{token::Token, Lexer};

pub mod config;
pub mod models;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(&input);

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    tokens
}

// ======== Tests ========

#[cfg(test)]
mod tests {
    use self::config::TokenKind;
    use self::models::token_span::TokenSpan;

    use super::*;

    #[test]
    fn test_tokenize_int() {
        assert_eq!(
            tokenize("-99"),
            vec![Token {
                kind: TokenKind::Int(-99),
                span: TokenSpan::new(0, 3, "-99".to_string()),
            }]
        );

        assert_eq!(
            tokenize("-1"),
            vec![Token {
                kind: TokenKind::Int(-1),
                span: TokenSpan::new(0, 2, "-1".to_string()),
            }]
        );

        assert_eq!(
            tokenize("0"),
            vec![Token {
                kind: TokenKind::Int(0),
                span: TokenSpan::new(0, 1, "0".to_string()),
            }]
        );

        assert_eq!(
            tokenize("99"),
            vec![Token {
                kind: TokenKind::Int(99),
                span: TokenSpan::new(0, 2, "0".to_string()),
            }]
        );
    }
}
