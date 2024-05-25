#[cfg(test)]
mod tests {
    use crate::lexer::{lexemes, models::token::{Token, TokenKind, TokenSpan}, tokenize};

    #[test]
    fn test_minus() {
        assert_eq!(
            tokenize("-"),
            vec![Token {
                kind: TokenKind::Minus,
                span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string())
            }]
        )
    }

    #[test]
    fn test_left_paren() {
        assert_eq!(
            tokenize("("),
            vec![Token {
                kind: TokenKind::LeftParen,
                span: TokenSpan::new(0, 1, lexemes::L_LEFT_PAREN.to_string())
            }]
        )
    }

    #[test]
    fn test_right_paren() {
        assert_eq!(
            tokenize(")"),
            vec![Token {
                kind: TokenKind::RightParen,
                span: TokenSpan::new(0, 1, lexemes::L_RIGHT_PAREN.to_string())
            }]
        )
    }

    #[test]
    fn test_left_sqr_br() {
        assert_eq!(
            tokenize("["),
            vec![Token {
                kind: TokenKind::LeftSqrBr,
                span: TokenSpan::new(0, 1, lexemes::L_LEFT_SQR_BR.to_string())
            }]
        )
    }

    #[test]
    fn test_right_sqr_br() {
        assert_eq!(
            tokenize("]"),
            vec![Token {
                kind: TokenKind::RightSqrBr,
                span: TokenSpan::new(0, 1, lexemes::L_RIGHT_SQR_BR.to_string())
            }]
        )
    }

    #[test]
    fn test_comma() {
        assert_eq!(
            tokenize(","),
            vec![Token {
                kind: TokenKind::Comma,
                span: TokenSpan::new(0, 1, lexemes::L_COMMA.to_string())
            }]
        )
    }
}
