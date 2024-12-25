use crate::types;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Nil,
    Number(types::Number),
    Minus,
    LeftParen,
    RightParen,
    LeftSqrBr,
    RightSqrBr,
    Comma,
    Whitespace,
    Newline,
    Identifier(String),
    Boolean(bool),
    String(String),
    FnAdd,
    FnSub,
    FnMul,
    FnDiv,
    FnPrint,
    FnPrintLn,
    FnLetBinding,
    FnGreatr,
    FnLess,
    FnGreatrEq,
    FnLessEq,
    FnEq,
    FnNotEq,
    FnNot,
    FnAnd,
    FnOr,
    FnBool,
    FnIf,
    FnIsNil,
    FnDefine,
    FnCustom(String),
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

pub fn is_reduceable_token(token: &Token) -> bool {
    vec![TokenKind::Whitespace, TokenKind::Newline].contains(&token.kind)
}
