pub mod config;
pub mod lexemes;
pub mod messages;
pub mod models;

use self::models::{
    token::{Token, TokenKind},
    Lexer,
};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(&input);

    while let Some(token) = lexer.next_token() {
        if token.kind == TokenKind::Whitespace {
            continue;
        }
        tokens.push(token);
    }

    tokens
}

// ======== Tests ========

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use assert_panic::assert_panic;
    use tests::{
        lexemes::fn_lexeme_to_string,
        models::token::{TokenKind, TokenSpan},
    };

    use self::models::number::BaseNumber;

    use super::*;
    use crate::types;

    // ==========================

    //          Numbers

    // ==========================

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

    // ==========================

    //        Punctuation

    // ==========================

    #[test]
    fn test_tokenize_minus() {
        assert_eq!(
            tokenize("-"),
            vec![Token {
                kind: TokenKind::Minus,
                span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string())
            }]
        )
    }

    #[test]
    fn test_tokenize_left_paren() {
        assert_eq!(
            tokenize("("),
            vec![Token {
                kind: TokenKind::LeftParen,
                span: TokenSpan::new(0, 1, lexemes::L_LEFT_PAREN.to_string())
            }]
        )
    }

    #[test]
    fn test_tokenize_right_paren() {
        assert_eq!(
            tokenize(")"),
            vec![Token {
                kind: TokenKind::RightParen,
                span: TokenSpan::new(0, 1, lexemes::L_RIGHT_PAREN.to_string())
            }]
        )
    }

    #[test]
    fn test_tokenize_left_sqr_br() {
        assert_eq!(
            tokenize("["),
            vec![Token {
                kind: TokenKind::LeftSqrBr,
                span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string())
            }]
        )
    }

    #[test]
    fn test_tokenize_right_sqr_br() {
        assert_eq!(
            tokenize("]"),
            vec![Token {
                kind: TokenKind::RightSqrBr,
                span: TokenSpan::new(0, 1, lexemes::L_RIGHT_SQR_BR.to_string())
            }]
        )
    }

    #[test]
    fn test_tokenize_comma() {
        assert_eq!(
            tokenize(","),
            vec![Token {
                kind: TokenKind::Comma,
                span: TokenSpan::new(0, 1, lexemes::L_COMMA.to_string())
            }]
        )
    }

    // ==========================

    //      Unexpected Token

    // ==========================

    #[test]
    #[should_panic(expected = "Lexing error. Unknown lexeme \"@klk\"")]
    fn test_tokenize_unknown() {
        tokenize("@klk");
    }

    // ==========================

    //      Known functions

    // ==========================

    #[test]
    fn test_fn_add() {
        assert_eq!(
            tokenize("@add"),
            vec![Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD))
            }]
        )
    }

    #[test]
    fn test_fn_sub() {
        assert_eq!(
            tokenize("@sub"),
            vec![Token {
                kind: TokenKind::FnSub,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_SUB))
            }]
        )
    }

    #[test]
    fn test_fn_mul() {
        assert_eq!(
            tokenize("@mul"),
            vec![Token {
                kind: TokenKind::FnMul,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_MUL))
            }]
        )
    }

    #[test]
    fn test_fn_div() {
        assert_eq!(
            tokenize("@div"),
            vec![Token {
                kind: TokenKind::FnDiv,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_DIV))
            }]
        )
    }

    #[test]
    fn test_fn_print() {
        assert_eq!(
            tokenize("@print"),
            vec![Token {
                kind: TokenKind::FnPrint,
                span: TokenSpan::new(0, 6, fn_lexeme_to_string(lexemes::L_FN_PRINT))
            }]
        )
    }

    #[test]
    fn test_fn_let_binding() {
        assert_eq!(
            tokenize("@let"),
            vec![Token {
                kind: TokenKind::FnLetBinding,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_LET_BINDING))
            }]
        )
    }

    // ==========================

    //        Identifier

    // ==========================

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
