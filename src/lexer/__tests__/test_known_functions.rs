#[cfg(test)]

mod tests {
    use crate::lexer::{
        lexemes::{self, fn_lexeme_to_string},
        models::token::{Token, TokenKind, TokenSpan},
        tokenize,
    };

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

    #[test]
    fn test_fn_greatr() {
        assert_eq!(
            tokenize("@greatr"),
            vec![Token {
                kind: TokenKind::FnGreatr,
                span: TokenSpan::new(0, 7, fn_lexeme_to_string(lexemes::L_FN_GREATR))
            }]
        )
    }

    #[test]
    fn test_fn_greatr_eq() {
        assert_eq!(
            tokenize("@greatr-eq"),
            vec![Token {
                kind: TokenKind::FnGreatrEq,
                span: TokenSpan::new(0, 10, fn_lexeme_to_string(lexemes::L_FN_GREATR_EQ))
            }]
        )
    }

    #[test]
    fn test_fn_less() {
        assert_eq!(
            tokenize("@less"),
            vec![Token {
                kind: TokenKind::FnLess,
                span: TokenSpan::new(0, 5, fn_lexeme_to_string(lexemes::L_FN_LESS))
            }]
        )
    }

    #[test]
    fn test_fn_less_eq() {
        assert_eq!(
            tokenize("@less-eq"),
            vec![Token {
                kind: TokenKind::FnLessEq,
                span: TokenSpan::new(0, 8, fn_lexeme_to_string(lexemes::L_FN_LESS_EQ))
            }]
        )
    }

    #[test]
    fn test_fn_eq() {
        assert_eq!(
            tokenize("@eq"),
            vec![Token {
                kind: TokenKind::FnEq,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_EQ))
            }]
        )
    }

    #[test]
    fn test_fn_not_eq() {
        assert_eq!(
            tokenize("@not-eq"),
            vec![Token {
                kind: TokenKind::FnNotEq,
                span: TokenSpan::new(0, 7, fn_lexeme_to_string(lexemes::L_FN_NOT_EQ))
            }]
        )
    }

    #[test]
    fn test_fn_not() {
        assert_eq!(
            tokenize("@not"),
            vec![Token {
                kind: TokenKind::FnNot,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_NOT))
            }]
        )
    }

    #[test]
    fn test_fn_and() {
        assert_eq!(
            tokenize("@and"),
            vec![Token {
                kind: TokenKind::FnAnd,
                span: TokenSpan::new(0, 4, fn_lexeme_to_string(lexemes::L_FN_AND))
            }]
        )
    }

    #[test]
    fn test_fn_or() {
        assert_eq!(
            tokenize("@or"),
            vec![Token {
                kind: TokenKind::FnOr,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_OR))
            }]
        )
    }

    #[test]
    fn test_fn_bool() {
        assert_eq!(
            tokenize("@bool"),
            vec![Token {
                kind: TokenKind::FnBool,
                span: TokenSpan::new(0, 5, fn_lexeme_to_string(lexemes::L_FN_BOOL))
            }]
        )
    }

    #[test]
    fn test_fn_if() {
        assert_eq!(
            tokenize("@if"),
            vec![Token {
                kind: TokenKind::FnIf,
                span: TokenSpan::new(0, 3, fn_lexeme_to_string(lexemes::L_FN_IF))
            }]
        )
    }
}
