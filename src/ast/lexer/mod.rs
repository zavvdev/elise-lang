pub mod config;
pub mod models;

use self::models::Token;

struct Lexer<'a> {
    input: &'a str,
    char_pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            char_pos: 0,
        }
    }

    fn next_token(&self) -> Option<Token> {
        None
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); 
    let lexer = Lexer::new(&input);
            
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    tokens
}

// ======== Tests ========

#[cfg(test)]
mod tests {
    use self::models::{TokenKind, TokenSpan};

    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let input = "-99, -2.45, -1, 0, 1, 2.45, 99";

        let expected = vec![
            Token {
                kind: TokenKind::Number(-99 as f64),
                span: TokenSpan::new(0, 3),
            },
            Token {
                kind: TokenKind::Number(-2.45),
                span: TokenSpan::new(5, 10),
            },
        ];

        let result = tokenize(input);

        assert_eq!(result, expected);
    }
}
