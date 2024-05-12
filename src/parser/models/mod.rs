pub mod ast;

use self::ast::{Expr, ExprKind};
use crate::{
    lexer::models::token::{Token, TokenKind},
    parser::messages,
    types,
};

/**
*
* Accepts tokens and returns a result of Grammar / Syntax Analysis
*
*/
pub struct Parser {
    tokens: Vec<Token>,
    token_pos: usize,
    seq_start_count: usize,
    seq_end_count: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            token_pos: 0,
            seq_start_count: 0,
            seq_end_count: 0,
        }
    }

    // ==========================

    //          Defaults

    // ==========================

    pub fn next_expr(&mut self) -> Option<Expr> {
        if self.seq_start_count != self.seq_end_count && self.get_current_token().is_none() {
            panic!("{}", messages::unexpected_end_of_input());
        } else if self.seq_start_count == self.seq_end_count && self.seq_start_count != 0 {
            self.seq_start_count = 0;
            self.seq_end_count = 0;
        }

        if self.token_pos > self.tokens.len() {
            return None;
        }

        let current_token = self.get_current_token()?;

        match &current_token.kind {
            TokenKind::Number(x) => self.consume_number(*x),
            TokenKind::Nil => self.consume_nil(),
            TokenKind::Identifier(x) => self.consume_identifier(x.to_string()),
            TokenKind::Minus => self.consume_negative_number(),
            TokenKind::FnAdd => self.consume_known_fn(ExprKind::FnAdd),
            TokenKind::FnSub => self.consume_known_fn(ExprKind::FnSub),
            TokenKind::FnMul => self.consume_known_fn(ExprKind::FnMul),
            TokenKind::FnDiv => self.consume_known_fn(ExprKind::FnDiv),
            TokenKind::FnPrint => self.consume_known_fn(ExprKind::FnPrint),
            TokenKind::FnPrintLn => self.consume_known_fn(ExprKind::FnPrintLn),
            TokenKind::FnLetBinding => self.consume_known_fn(ExprKind::FnLetBinding),
            TokenKind::RightParen => Some(Expr::new(ExprKind::_EndOfFn, vec![])),
            TokenKind::Comma => Some(Expr::new(ExprKind::_ArgumentSeparator, vec![])),
            TokenKind::LeftSqrBr => self.consume_list(),
            TokenKind::RightSqrBr => Some(Expr::new(ExprKind::_EndOfList, vec![])),
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
            messages::unexpected_token(&format!("{:?}", self.get_current_token()))
        );
    }

    /**
     *
     * Should be used for counting start of sequense
     *
     */
    fn capture_seq(&mut self) {
        self.seq_start_count += 1;
    }

    /**
     *
     * Should be used for counting end of sequense
     *
     */
    fn end_seq(&mut self) {
        self.seq_end_count += 1;
    }

    /**
     *
     * Can be used for consuming arguemnts of any sequentioan entity like function or list
     *
     */
    fn consume_seq_arguments(&mut self, seq_end_expr: ExprKind) -> Vec<Box<Expr>> {
        let mut arguments: Vec<Box<Expr>> = Vec::new();

        while let Some(expr) = self.next_expr() {
            if expr.kind == seq_end_expr {
                self.consume();
                self.end_seq();
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

    // ==========================

    //          Numbers

    // ==========================

    fn consume_number(&mut self, x: types::Number) -> Option<Expr> {
        self.consume();
        Some(Expr::new(ExprKind::Number(x), vec![]))
    }

    fn consume_negative_number(&mut self) -> Option<Expr> {
        let next = self.get_next_token();

        if next.is_none() {
            self.panic_at_current_token();
        }

        let next = next.unwrap();

        if let TokenKind::Number(x) = next.kind {
            self.skip_tokens(1);
            return Some(Expr::new(ExprKind::Number(x * -1.0), vec![]));
        } else {
            panic!("{}", messages::unexpected_token(&next.span.lexeme));
        }
    }

    // ==========================

    //      Known functions

    // ==========================

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
            self.capture_seq();

            return Some(Expr::new(
                known_fn_expr_kind,
                self.consume_seq_arguments(ExprKind::_EndOfFn),
            ));
        } else {
            self.panic_at_current_token();
        }
    }

    // ==========================

    //           List

    // ==========================

    fn consume_list(&mut self) -> Option<Expr> {
        let next = self.get_next_token();

        if next.is_none() {
            self.panic_at_current_token();
        }

        self.consume();
        self.capture_seq();

        Some(Expr::new(
            ExprKind::List,
            self.consume_seq_arguments(ExprKind::_EndOfList),
        ))
    }

    // ==========================

    //         Identifier

    // ==========================

    fn consume_identifier(&mut self, x: String) -> Option<Expr> {
        self.consume();
        Some(Expr::new(ExprKind::Identifier(x), vec![]))
    }

    // ==========================

    //           Nil

    // ==========================
    
    fn consume_nil(&mut self) -> Option<Expr> {
        self.consume();
        Some(Expr::new(ExprKind::Nil, vec![]))
    }
}
