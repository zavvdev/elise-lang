use crate::lexer::models::token::TokenKind;

#[derive(Debug, PartialEq)]
pub struct AstNode {
    pub token_kind: TokenKind,
    pub branches: Vec<Box<AstNode>>,
}

impl AstNode {
    pub fn new(token_kind: TokenKind, branches: Vec<Box<AstNode>>) -> Self {
        Self {
            token_kind,
            branches,
        }
    }
}
