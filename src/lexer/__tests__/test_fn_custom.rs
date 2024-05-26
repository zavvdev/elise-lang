#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexemes,
        models::token::{Token, TokenKind, TokenSpan},
        tokenize,
    };

    #[test]
    fn test_fn_custom() {
        assert_eq!(
            tokenize("@custom"),
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
}
