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

    // SUCCESS CASES

    #[test]
    fn test_int() {
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

        assert_eq!(
            tokenize("1032"),
            vec![Token {
                kind: TokenKind::Number(1032 as types::Number),
                span: TokenSpan::new(0, 4, "1032".to_string()),
            }]
        );
    }

    #[test]
    fn test_float() {
        assert_eq!(
            tokenize("0.5"),
            vec![Token {
                kind: TokenKind::Number(0.5),
                span: TokenSpan::new(0, 3, "0.5".to_string()),
            }]
        );

        assert_eq!(
            tokenize("1.5231"),
            vec![Token {
                kind: TokenKind::Number(1.5231),
                span: TokenSpan::new(0, 6, "1.5231".to_string()),
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
    fn test_negative() {
        assert_eq!(
            tokenize("-0"),
            vec![Token {
                kind: TokenKind::Number(-0.0),
                span: TokenSpan::new(0, 2, "-0".to_string()),
            }]
        );

        assert_eq!(
            tokenize("-0.5"),
            vec![Token {
                kind: TokenKind::Number(-0.5),
                span: TokenSpan::new(0, 4, "-0.5".to_string()),
            }]
        );

        assert_eq!(
            tokenize("-99.9999"),
            vec![Token {
                kind: TokenKind::Number(-99.9999),
                span: TokenSpan::new(0, 8, "-99.9999".to_string()),
            }]
        );

        assert_eq!(
            tokenize("-99"),
            vec![Token {
                kind: TokenKind::Number(-99.0),
                span: TokenSpan::new(0, 3, "-99".to_string()),
            }]
        );

        assert_eq!(
            tokenize("-1032"),
            vec![Token {
                kind: TokenKind::Number(-1032.0),
                span: TokenSpan::new(0, 5, "-1032".to_string()),
            }]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_int_overflow() {
        tokenize(&format!("{}", Wrapping(BaseNumber::MAX) + Wrapping(1)));
    }

    #[test]
    #[should_panic]
    fn test_float_overflow() {
        #[allow(arithmetic_overflow)]
        let overflowed = types::Number::MAX + 0.1;
        #[deny(arithmetic_overflow)]
        tokenize(&format!("1.{}", overflowed));
    }

    #[test]
    #[should_panic]
    fn test_int_double_zero() {
        tokenize("00");
    }

    #[test]
    #[should_panic]
    fn test_int_starts_with_zero() {
        tokenize("01");
    }

    #[test]
    #[should_panic]
    fn test_float_starts_with_double_zero() {
        tokenize("00.123");
    }

    #[test]
    #[should_panic]
    fn test_double_dot() {
        tokenize("0.12.3");
    }

    #[test]
    #[should_panic]
    fn test_number_with_next_identifier() {
        tokenize("123x");
    }
}
