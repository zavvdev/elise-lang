#[cfg(test)]

mod tests {
    use assert_panic::assert_panic;

    use crate::lexer::{
        lexemes::{self, fn_lexeme_to_string, to_fn_string},
        models::token::{Token, TokenKind, TokenSpan},
        tokenize,
    };

    // SUCCESS CASES

    #[test]
    fn test_add() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_ADD)),
            vec![Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_ADD))
            }]
        )
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_SUB)),
            vec![Token {
                kind: TokenKind::FnSub,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_SUB))
            }]
        )
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_MUL)),
            vec![Token {
                kind: TokenKind::FnMul,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_MUL))
            }]
        )
    }

    #[test]
    fn test_div() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_DIV)),
            vec![Token {
                kind: TokenKind::FnDiv,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_DIV))
            }]
        )
    }

    #[test]
    fn test_print() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_PRINT)),
            vec![Token {
                kind: TokenKind::FnPrint,
                span: TokenSpan::new(0, 6, fn_lexeme_to_string(lexemes::L_FN_PRINT))
            }]
        )
    }

    #[test]
    fn test_println() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_PRINTLN)),
            vec![Token {
                kind: TokenKind::FnPrintLn,
                span: TokenSpan::new(0, 8, fn_lexeme_to_string(lexemes::L_FN_PRINTLN))
            }]
        )
    }

    #[test]
    fn test_let() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_LET_BINDING)),
            vec![Token {
                kind: TokenKind::FnLetBinding,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_LET_BINDING))
            }]
        )
    }

    #[test]
    fn test_greatr() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_GREATR)),
            vec![Token {
                kind: TokenKind::FnGreatr,
                span: TokenSpan::new(0, 7, fn_lexeme_to_string(lexemes::L_FN_GREATR))
            }]
        )
    }

    #[test]
    fn test_greatr_eq() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_GREATR_EQ)),
            vec![Token {
                kind: TokenKind::FnGreatrEq,
                span: TokenSpan::new(0, 10, fn_lexeme_to_string(lexemes::L_FN_GREATR_EQ))
            }]
        )
    }

    #[test]
    fn test_less() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_LESS)),
            vec![Token {
                kind: TokenKind::FnLess,
                span: TokenSpan::new(0, 5, fn_lexeme_to_string(lexemes::L_FN_LESS))
            }]
        )
    }

    #[test]
    fn test_less_eq() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_LESS_EQ)),
            vec![Token {
                kind: TokenKind::FnLessEq,
                span: TokenSpan::new(0, 8, fn_lexeme_to_string(lexemes::L_FN_LESS_EQ))
            }]
        )
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_EQ)),
            vec![Token {
                kind: TokenKind::FnEq,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_EQ))
            }]
        )
    }

    #[test]
    fn test_not_eq() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_NOT_EQ)),
            vec![Token {
                kind: TokenKind::FnNotEq,
                span: TokenSpan::new(0, 7, fn_lexeme_to_string(lexemes::L_FN_NOT_EQ))
            }]
        )
    }

    #[test]
    fn test_not() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_NOT)),
            vec![Token {
                kind: TokenKind::FnNot,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_NOT))
            }]
        )
    }

    #[test]
    fn test_and() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_AND)),
            vec![Token {
                kind: TokenKind::FnAnd,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_AND))
            }]
        )
    }

    #[test]
    fn test_or() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_OR)),
            vec![Token {
                kind: TokenKind::FnOr,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_OR))
            }]
        )
    }

    #[test]
    fn test_bool() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_BOOL)),
            vec![Token {
                kind: TokenKind::FnBool,
                span: TokenSpan::new(0, 5, fn_lexeme_to_string(lexemes::L_FN_BOOL))
            }]
        )
    }

    #[test]
    fn test_if() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_IF)),
            vec![Token {
                kind: TokenKind::FnIf,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_IF))
            }]
        )
    }

    #[test]
    fn test_is_nil() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_IS_NIL)),
            vec![Token {
                kind: TokenKind::FnIsNil,
                span: TokenSpan::new(0, 5, fn_lexeme_to_string(lexemes::L_FN_IS_NIL))
            }]
        )
    }

    #[test]
    fn test_define_fn() {
        assert_eq!(
            tokenize(&fn_lexeme_to_string(lexemes::L_FN_DEFINE)),
            vec![Token {
                kind: TokenKind::FnDefine,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_DEFINE))
            }]
        )
    }

    #[test]
    fn test_fn_custom() {
        assert_eq!(
            tokenize(&to_fn_string("custom")),
            vec![Token {
                kind: TokenKind::FnCustom("custom".to_string()),
                span: TokenSpan::new(
                    0,
                    7,
                    lexemes::fn_lexeme_to_string((lexemes::L_FN, "custom"))
                )
            }]
        )
    }

    // FAILURE CASES

    #[test]
    #[should_panic(expected = "Invalid function name \"\"")]
    fn test_empty_name() {
        tokenize(&to_fn_string(""));
    }

    #[test]
    fn test_invalid_name() {
        let invalid_names = vec![
            ".",
            "?hello",
            "!hello",
            "@hello",
            "-hello",
            ".hello",
            "2hello",
            "hello.",
            "hello@world",
            "hello.world",
            "hello~world",
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

        for invalid_name in invalid_names {
            assert_panic!(
                {
                    tokenize(&to_fn_string(&invalid_name));
                },
                String,
                format!("Lexing error. Invalid function name \"{}\".", invalid_name)
            );
        }
    }
}
