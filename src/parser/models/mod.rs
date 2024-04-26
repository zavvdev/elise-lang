pub mod ast;

use self::ast::{Expr, ExprKind};
use crate::{
    lexer::models::token::{Token, TokenKind},
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
    fn_start_count: usize,
    fn_end_count: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_pos: 0,
            fn_start_count: 0,
            fn_end_count: 0,
        }
    }

    // ==========================

    //          Defaults

    // ==========================

    pub fn next_expr(&mut self) -> Option<Expr> {
        if self.fn_start_count != self.fn_end_count && self.get_current_token().is_none() {
            panic!("{}", messages::m_unexpected_end_of_input());
        }

        if self.token_pos > self.tokens.len() {
            return None;
        }

        let current_token = self.get_current_token()?;

        match current_token.kind {
            TokenKind::Int(x) => self.consume_int(x),
            TokenKind::Float(x) => self.consume_float(x),
            TokenKind::Minus => self.consume_negative_number(),
            TokenKind::FnAdd => self.consume_known_fn(ExprKind::FnAdd),
            TokenKind::FnSub => self.consume_known_fn(ExprKind::FnSub),
            TokenKind::FnMul => self.consume_known_fn(ExprKind::FnMul),
            TokenKind::FnDiv => self.consume_known_fn(ExprKind::FnDiv),
            TokenKind::RightParen => Some(Expr::new(ExprKind::_EndOfFn, vec![])),
            TokenKind::Comma => Some(Expr::new(ExprKind::_ArgumentSeparator, vec![])),
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

    fn get_current_token(&self) -> Option<&Token> {
        self.tokens.get(self.token_pos)
    }

    fn get_next_token(&self) -> Option<&Token> {
        self.tokens.get(self.token_pos + 1)
    }

    fn panic_at_current_token(&self) -> ! {
        panic!(
            "{}",
            messages::m_unexpected_token(&format!("{:?}", self.get_current_token()))
        );
    }

    /**
     *
     * Should be used for counting function start
     *
     */
    fn capture_fn(&mut self) {
        self.fn_start_count += 1;
    }

    /**
     *
     * Should be used for counting function end
     *
     */
    fn end_fn(&mut self) {
        self.fn_end_count += 1;
    }

    // ==========================

    //          Numbers

    // ==========================

    fn consume_int(&mut self, x: types::Integer) -> Option<Expr> {
        self.consume();
        Some(Expr::new(ast::ExprKind::Int(x), vec![]))
    }

    fn consume_float(&mut self, x: types::Float) -> Option<Expr> {
        self.consume();
        Some(Expr::new(ExprKind::Float(x), vec![]))
    }

    fn consume_negative_number(&mut self) -> Option<Expr> {
        let next = self.get_next_token();

        if next.is_none() {
            self.panic_at_current_token();
        }

        let next = next.unwrap();

        if let TokenKind::Int(x) = next.kind {
            self.skip_tokens(1);
            return Some(Expr::new(ExprKind::Int(x * -1), vec![]));
        } else if let TokenKind::Float(x) = next.kind {
            self.skip_tokens(1);
            return Some(Expr::new(ExprKind::Float(x * -1.0), vec![]));
        } else {
            panic!("{}", messages::m_unexpected_token(&next.span.lexeme));
        }
    }

    // ==========================

    //      Known functions

    // ==========================

    /**
     *
     * Can be used for consuming any function arguments
     *
     */
    fn consume_fn_arguments(&mut self) -> Vec<Box<Expr>> {
        let mut arguments: Vec<Box<Expr>> = Vec::new();

        while let Some(expr) = self.next_expr() {
            if expr.kind == ExprKind::_EndOfFn {
                self.consume();
                self.end_fn();
                return arguments;
            }

            if expr.kind == ExprKind::_ArgumentSeparator {
                self.consume();
                continue;
            }

            arguments.push(Box::new(expr));
        }

        arguments
    }

    /**
     *
     * Should be used for consuming known functions only
     *
     */
    fn consume_known_fn(&mut self, known_fn_expr_kind: ExprKind) -> Option<Expr> {
        let next = self.get_next_token();

        if next.is_none() {
            self.panic_at_current_token();
        }

        let next = next.unwrap();

        if next.kind == TokenKind::LeftParen {
            self.skip_tokens(1);
            self.capture_fn();

            return Some(Expr::new(known_fn_expr_kind, self.consume_fn_arguments()));
        } else {
            self.panic_at_current_token();
        }
    }
}
