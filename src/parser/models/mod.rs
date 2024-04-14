pub mod ast;

use self::ast::AstNode;
use crate::{
    lexer::{
        lexemes,
        models::token::{Token, TokenKind},
    },
    messages, types,
};

/**
*
* Accepts tokens and returns a result of Grammar / Syntax Analysis
*
*/
pub struct Parser {
    tokens: Vec<Token>,
    token_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_pos: 0,
        }
    }

    // ==========================

    //          Defaults

    // ==========================

    pub fn next_node(&mut self) -> Option<AstNode> {
        if self.token_pos > self.tokens.len() {
            return None;
        }

        let current_token = self.tokens.get(self.token_pos)?;

        match current_token.kind {
            TokenKind::Int(x) => self.consume_int(x),
            TokenKind::Float(x) => self.consume_float(x),
            TokenKind::Minus => self.consume_negative_number(),
            _ => None,
        }
    }

    /**
     *
     * Should be used if you want to skip `offset` amount of tokens that are
     * next to the current token. For example, if I'm at token 4 and I call
     * `self.skip_tokens(1)` then the next token_pos will be 6. Useful in the case
     * when you parsed a sequence of tokens at once.
     *
     */
    fn skip_tokens(&mut self, offset: usize) {
        let tokens_len = self.tokens.len();

        if self.token_pos < tokens_len {
            self.token_pos += offset + 1;
        }
    }

    /**
     *
     * Should be used when current token is required but with
     * addition move to the next token
     *
     */
    fn consume(&mut self) -> Option<&Token> {
        if self.token_pos >= self.tokens.len() {
            return None;
        }

        let current_token = self.tokens.get(self.token_pos);

        self.token_pos += 1;

        current_token
    }

    // ==========================

    //          Numbers

    // ==========================

    fn consume_int(&mut self, int: types::Integer) -> Option<AstNode> {
        self.consume();
        Some(AstNode::new(TokenKind::Int(int), vec![]))
    }

    fn consume_float(&mut self, float: types::Float) -> Option<AstNode> {
        self.consume();
        Some(AstNode::new(TokenKind::Float(float), vec![]))
    }

    fn consume_negative_number(&mut self) -> Option<AstNode> {
        let next = self.tokens.get(self.token_pos + 1);

        if next.is_none() {
            panic!(
                "{}",
                messages::m_parse_error_unexpected_token(&lexemes::L_MINUS.to_string())
            );
        }

        let next = next.unwrap();

        if let TokenKind::Int(x) = next.kind {
            self.skip_tokens(1);
            return Some(AstNode::new(TokenKind::Int(x * -1), vec![]));
        } else if let TokenKind::Float(x) = next.kind {
            self.skip_tokens(1);
            return Some(AstNode::new(TokenKind::Float(x * -1.0), vec![]));
        } else {
            panic!("{}", messages::m_parse_error_unexpected_token(&next.span.lexeme));
        }
    }
}
