#[cfg(test)]

mod tests {
    use crate::lexer::{models::token::{Token, TokenKind, TokenSpan}, tokenize};

    #[test]
    fn test_string() {
        assert_eq!(
            tokenize(r#""hello world""#),
            vec![Token {
                kind: TokenKind::String("hello world".to_string()),
                span: TokenSpan::new(0, 13, r#""hello world""#.to_string())
            }]
        )
    }

    #[test]
    fn test_string_empty() {
        assert_eq!(
            tokenize(r#""""#),
            vec![Token {
                kind: TokenKind::String("".to_string()),
                span: TokenSpan::new(0, 2, r#""""#.to_string())
            }]
        )
    }

    #[test]
    fn test_string_with_escape() {
        assert_eq!(
            tokenize(&r#""hello\nworld""#),
            vec![Token {
                kind: TokenKind::String("hello\nworld".to_string()),
                span: TokenSpan::new(0, 14, r#""hello\nworld""#.to_string())
            }]
        );

        assert_eq!(
            tokenize(&r#""hello\rworld""#),
            vec![Token {
                kind: TokenKind::String("hello\rworld".to_string()),
                span: TokenSpan::new(0, 14, r#""hello\rworld""#.to_string())
            }]
        );

        assert_eq!(
            tokenize(&r#""hello\tworld""#),
            vec![Token {
                kind: TokenKind::String("hello\tworld".to_string()),
                span: TokenSpan::new(0, 14, r#""hello\tworld""#.to_string())
            }]
        );

        assert_eq!(
            tokenize(&r#""hello\"world""#),
            vec![Token {
                kind: TokenKind::String("hello\"world".to_string()),
                span: TokenSpan::new(0, 14, r#""hello\"world""#.to_string())
            }]
        );

        assert_eq!(
            tokenize(&r#""hello\\world""#),
            vec![Token {
                kind: TokenKind::String("hello\\world".to_string()),
                span: TokenSpan::new(0, 14, r#""hello\\world""#.to_string())
            }]
        );

        assert_eq!(
            tokenize(&r#""hello\0world""#),
            vec![Token {
                kind: TokenKind::String("hello\0world".to_string()),
                span: TokenSpan::new(0, 14, r#""hello\0world""#.to_string())
            }]
        );
    }
}
