use crate::types;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,

    // Data Types
    Number(types::Number),

    // Punctuation
    Minus,
    LeftParen,
    RightParen,
    LeftSqrBr,
    RightSqrBr,
    Colon,
    Comma,

    // Known functions
    FnAdd,
    FnSub,
    FnMul,
    FnDiv,
    FnPrint,
    FnPrintLn,
    FnLetBinding,

    // Other
    Whitespace,
    ReturnType,
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub lexeme: String,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize, lexeme: String) -> Self {
        TokenSpan { start, end, lexeme }
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
