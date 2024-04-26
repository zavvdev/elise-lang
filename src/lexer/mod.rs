pub mod lexemes;
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

    use tests::{
        lexemes::fn_lexeme_to_string,
        models::token::{TokenKind, TokenSpan},
    };

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
        tokenize(&format!("{}", Wrapping(types::Integer::MAX) + Wrapping(1)));
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
        let overflowed = types::Float::MAX + 0.1;
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
    fn test_tokenize_colon() {
        assert_eq!(
            tokenize(":"),
            vec![Token {
                kind: TokenKind::Colon,
                span: TokenSpan::new(0, 1, lexemes::L_COLON.to_string())
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

    //          Other

    // ==========================

    #[test]
    fn test_tokenize_return_type() {
        assert_eq!(
            tokenize("->"),
            vec![Token {
                kind: TokenKind::ReturnType,
                span: TokenSpan::new(
                    0,
                    2,
                    format!("{}{}", lexemes::L_RETURN_TYPE.0, lexemes::L_RETURN_TYPE.1)
                )
            }]
        )
    }

    // ==========================

    //      Unexpected Token

    // ==========================

    #[test]
    #[should_panic(expected = "Unexpected token \"~\"")]
    fn test_tokenize_unknown() {
        tokenize("~");
    }

    // ==========================

    //         Known functions

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
}
