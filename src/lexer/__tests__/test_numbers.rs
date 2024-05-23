#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use crate::{
        lexer::{
            models::{
                number::BaseNumber,
                token::{Token, TokenKind, TokenSpan},
            },
            tokenize,
        },
        types,
    };

    #[test]
    fn test_tokenize_int() {
        assert_eq!(
            tokenize("0"),
            vec![Token {
                kind: TokenKind::Number(0 as types::Number),
                span: TokenSpan::new(0, 1, "0".to_string()),
            }]
        );

        assert_eq!(
            tokenize("99"),
            vec![Token {
                kind: TokenKind::Number(99 as types::Number),
                span: TokenSpan::new(0, 2, "99".to_string()),
            }]
        );
    }

    #[test]
    #[should_panic(expected = "Number overflow")]
    fn test_tokenize_int_overflow() {
        tokenize(&format!("{}", Wrapping(BaseNumber::MAX) + Wrapping(1)));
    }

    #[test]
    fn test_tokenize_float() {
        assert_eq!(
            tokenize("0.5"),
            vec![Token {
                kind: TokenKind::Number(0.5),
                span: TokenSpan::new(0, 3, "0.5".to_string()),
            }]
        );

        assert_eq!(
            tokenize("99.9999"),
            vec![Token {
                kind: TokenKind::Number(99.9999),
                span: TokenSpan::new(0, 7, "99.9999".to_string()),
            }]
        )
    }

    #[test]
    #[should_panic(expected = "Number overflow")]
    fn test_tokenize_float_overflow() {
        #[allow(arithmetic_overflow)]
        let overflowed = types::Number::MAX + 0.1;
        #[deny(arithmetic_overflow)]
        tokenize(&format!("1.{}", overflowed));
    }
}
