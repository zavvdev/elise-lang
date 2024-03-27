pub mod message;
pub mod models;

use self::models::{token::Token, Lexer};

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
    use std::num::Wrapping;

    use tests::models::{
        number::Integer,
        token::{TokenKind, TokenSpan},
    };

    use self::models::number::Float;

    use super::*;

    // ======== Number ========

    #[test]
    fn test_tokenize_int() {
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
                span: TokenSpan::new(0, 2, "99".to_string()),
            }]
        );
    }

    #[test]
    #[should_panic(expected = "Integer overflow")]
    fn test_tokenize_int_overflow() {
        tokenize(&format!("{}", Wrapping(Integer::MAX) + Wrapping(1)));
    }

    #[test]
    fn test_tokenize_float() {
        assert_eq!(
            tokenize("0.5"),
            vec![Token {
                kind: TokenKind::Float(0.5),
                span: TokenSpan::new(0, 3, "0.5".to_string()),
            }]
        );

        assert_eq!(
            tokenize("99.9999"),
            vec![Token {
                kind: TokenKind::Float(99.9999),
                span: TokenSpan::new(0, 7, "99.9999".to_string()),
            }]
        )
    }

    #[test]
    #[should_panic(expected = "Float overflow")]
    fn test_tokenize_float_overflow() {
        #[allow(arithmetic_overflow)]
        let overflowed = Float::MAX + 0.1;
        #[deny(arithmetic_overflow)]
        tokenize(&format!("1.{}", overflowed));
    }
}
