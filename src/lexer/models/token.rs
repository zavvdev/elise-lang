use super::number::{Float, Integer};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,

    // Data Types
    Int(Integer),
    Float(Float),

    // Punctuation
    Minus,
    LeftParen,
    RightParen,
    LeftSqrBr,
    RightSqrBr,
    Colon,

    // Functions
    Add,

    // Other
    Whitespace,
    ReturnType,
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TokenSpan {
            start,
            end,
            literal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TokenSpan) -> Self {
        Token { kind, span }
    }
}
