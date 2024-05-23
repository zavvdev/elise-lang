#[cfg(test)]
mod tests {
    use crate::{
        lexer::{
            lexemes::{self, fn_lexeme_to_string},
            models::token::{Token, TokenKind, TokenSpan},
        },
        parser::parse,
    };

    #[test]
    #[should_panic(expected = "Unexpected token")]
    fn test_unexpected_token_trailing_minus() {
        parse(vec![Token {
            kind: TokenKind::Minus,
            span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string()),
        }]);
    }

    #[test]
    #[should_panic(expected = "Unexpected token")]
    fn test_unexpected_token_minus() {
        parse(vec![
            Token {
                kind: TokenKind::Minus,
                span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string()),
            },
            Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(1, 5, fn_lexeme_to_string(lexemes::L_FN_ADD)),
            },
        ]);
    }
}
