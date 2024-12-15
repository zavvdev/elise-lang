#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexemes,
        models::token::{Token, TokenKind, TokenSpan},
        tokenize,
    };

    // SUCCESS CASES

    #[test]
    fn test_nil() {
        assert_eq!(
            tokenize("nil"),
            vec![Token {
                kind: TokenKind::Nil,
                span: TokenSpan::new(0, 3, lexemes::L_NIL.to_string())
            }]
        )
    }
}
