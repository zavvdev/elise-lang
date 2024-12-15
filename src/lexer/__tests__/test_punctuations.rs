#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexemes,
        models::token::{Token, TokenKind, TokenSpan},
        tokenize,
    };

    // SUCCESS CASES

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

    #[test]
    fn test_newline() {
        assert_eq!(
            tokenize(
                "
"
            ),
            vec![Token {
                kind: TokenKind::Newline,
                span: TokenSpan::new(0, 1, lexemes::L_NEWLINE.to_string())
            }]
        )
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            tokenize(" "),
            vec![Token {
                kind: TokenKind::Whitespace,
                span: TokenSpan::new(0, 1, lexemes::L_WHITESPACE.to_string())
            }]
        )
    }

    #[test]
    fn test_should_reduce_whitespaces() {
        assert_eq!(
            tokenize("        "),
            vec![Token {
                kind: TokenKind::Whitespace,
                span: TokenSpan::new(0, 1, lexemes::L_WHITESPACE.to_string())
            }]
        )
    }

    #[test]
    fn test_should_reduce_newlines() {
        assert_eq!(
            tokenize(
                "

"
            ),
            vec![Token {
                kind: TokenKind::Newline,
                span: TokenSpan::new(0, 1, lexemes::L_NEWLINE.to_string())
            }]
        )
    }
}
