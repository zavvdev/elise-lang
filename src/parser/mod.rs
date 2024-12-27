use std::collections::VecDeque;

use models::expression::is_expr_internal;

use crate::{
    lexer::models::token::{Token, TokenKind},
    messages::print_error_message,
    types,
};

use self::models::expression::{Expr, ExprKind};

pub mod __tests__;
pub mod messages;
pub mod models;

struct Parser<'source_code> {
    source_code: &'source_code str,
    tokens: Vec<Token>,
    token_pos: usize,
    seq_stack: VecDeque<TokenKind>,
}

impl<'source_code> Parser<'source_code> {
    fn new(tokens: Vec<Token>, source_code: &'source_code str) -> Self {
        Self {
            source_code,
            tokens,
            token_pos: 0,
            seq_stack: VecDeque::new(),
        }
    }

    fn next_expr(&mut self) -> Option<Expr> {
        if self.token_pos > self.tokens.len() {
            return None;
        }

        let token = self.token_current()?;

        match &token.kind {
            TokenKind::Number(x) => self.number_consume(*x),
            TokenKind::String(x) => self.string_consume(x.to_string()),
            TokenKind::Boolean(x) => self.boolean_consume(*x),
            TokenKind::Identifier(x) => self.identifier_consume(x.to_string()),
            TokenKind::Nil => self.nil_consume(),

            TokenKind::LeftSqrBr => self.list_consume_start(),
            TokenKind::RightSqrBr => self.list_consume_end(),

            TokenKind::FnAdd => self.fn_consume_start(ExprKind::FnAdd),
            TokenKind::FnSub => self.fn_consume_start(ExprKind::FnSub),
            TokenKind::FnMul => self.fn_consume_start(ExprKind::FnMul),
            TokenKind::FnDiv => self.fn_consume_start(ExprKind::FnDiv),
            TokenKind::FnPrint => self.fn_consume_start(ExprKind::FnPrint),
            TokenKind::FnPrintLn => self.fn_consume_start(ExprKind::FnPrintLn),
            TokenKind::FnLetBinding => self.fn_consume_start(ExprKind::FnLetBinding),
            TokenKind::FnGreatr => self.fn_consume_start(ExprKind::FnGreatr),
            TokenKind::FnGreatrEq => self.fn_consume_start(ExprKind::FnGreatrEq),
            TokenKind::FnLess => self.fn_consume_start(ExprKind::FnLess),
            TokenKind::FnLessEq => self.fn_consume_start(ExprKind::FnLessEq),
            TokenKind::FnEq => self.fn_consume_start(ExprKind::FnEq),
            TokenKind::FnNotEq => self.fn_consume_start(ExprKind::FnNotEq),
            TokenKind::FnNot => self.fn_consume_start(ExprKind::FnNot),
            TokenKind::FnAnd => self.fn_consume_start(ExprKind::FnAnd),
            TokenKind::FnOr => self.fn_consume_start(ExprKind::FnOr),
            TokenKind::FnBool => self.fn_consume_start(ExprKind::FnBool),
            TokenKind::FnIf => self.fn_consume_start(ExprKind::FnIf),
            TokenKind::FnIsNil => self.fn_consume_start(ExprKind::FnIsNil),
            TokenKind::FnDefine => self.fn_consume_start(ExprKind::FnDefine),
            TokenKind::FnCustom(x) => self.fn_consume_start(ExprKind::FnCustom(x.to_string())),
            TokenKind::RightParen => self.fn_consume_end(),

            TokenKind::Comma => self.separator_consume(),
            TokenKind::Whitespace => self.separator_consume(),
            TokenKind::Newline => self.separator_consume(),

            _ => None,
        }
    }

    // ==========================
    //
    // TOKEN START
    //
    // ==========================

    /**
     * If I'm at token 4 and I call `self.token_skip(1)`
     * then the next token_pos will be 5.
     */
    fn token_skip(&mut self, offset: usize) {
        let tokens_len = self.tokens.len();

        if self.token_pos < tokens_len {
            self.token_pos += offset;
        }
    }

    fn token_consume(&mut self) -> Option<&Token> {
        if self.token_pos >= self.tokens.len() {
            return None;
        }

        let current_token = self.tokens.get(self.token_pos);

        self.token_pos += 1;

        current_token
    }

    fn token_current(&self) -> Option<&Token> {
        self.tokens.get(self.token_pos)
    }

    fn token_next(&self) -> Option<&Token> {
        self.tokens.get(self.token_pos + 1)
    }

    // ==========================
    //
    // TOKEN END
    //
    // ==========================

    // ==========================
    //
    // ERROR START
    //
    // ==========================

    fn error(&self, message: &str, token: Option<&Token>) -> ! {
        if let Some(token) = token {
            print_error_message(message, self.source_code, token.span.start);
        } else if let Some(current) = self.token_current() {
            print_error_message(message, self.source_code, current.span.start);
        } else if self.tokens.len() > 0 {
            let last = self.tokens.last().unwrap();
            print_error_message(message, self.source_code, last.span.start);
        }

        panic!("{}", messages::get_panic_message());
    }

    fn error_unexpected(&self) -> ! {
        let token = self.token_current().unwrap();
        self.error(&messages::unexpected_token(&token.span.lexeme), Some(token));
    }

    // ==========================
    //
    // ERROR END
    //
    // ==========================

    // ==========================
    //
    // SEQUENCE START
    //
    // ==========================

    fn seq_consume(&mut self, seq_end_expr: ExprKind) -> Vec<Box<Expr>> {
        let mut arguments: Vec<Box<Expr>> = Vec::new();

        while let Some(expr) = self.next_expr() {
            if expr.kind == seq_end_expr {
                return arguments;
            }

            if expr.kind == ExprKind::_Separator {
                continue;
            }

            arguments.push(Box::new(expr));
        }

        arguments
    }

    fn seq_capture(&mut self, token_kind: TokenKind) {
        self.seq_stack.push_back(token_kind);
    }

    fn seq_match_end(&mut self, token_kind: TokenKind) -> bool {
        self.seq_stack.pop_back() == Some(token_kind)
    }

    // ==========================
    //
    // SEQUENCE END
    //
    // ==========================

    // ==========================
    //
    // NUMBER START
    //
    // ==========================

    fn number_consume(&mut self, x: types::Number) -> Option<Expr> {
        self.token_consume();
        Some(Expr::new(ExprKind::Number(x), vec![]))
    }

    // ==========================
    //
    // NUMBER END
    //
    // ==========================

    // ==========================
    //
    // STRING START
    //
    // ==========================

    fn string_consume(&mut self, x: String) -> Option<Expr> {
        self.token_consume();
        Some(Expr::new(ExprKind::String(x), vec![]))
    }

    // ==========================
    //
    // STRING END
    //
    // ==========================

    // ==========================
    //
    // BOOLEAN START
    //
    // ==========================

    fn boolean_consume(&mut self, x: bool) -> Option<Expr> {
        self.token_consume();
        Some(Expr::new(ExprKind::Boolean(x), vec![]))
    }

    // ==========================
    //
    // BOOLEAN END
    //
    // ==========================

    // ==========================
    //
    // IDENTIFIER START
    //
    // ==========================

    fn identifier_consume(&mut self, x: String) -> Option<Expr> {
        self.token_consume();
        Some(Expr::new(ExprKind::Identifier(x), vec![]))
    }

    // ==========================
    //
    // IDENTIFIER END
    //
    // ==========================

    // ==========================
    //
    // NIL START
    //
    // ==========================

    fn nil_consume(&mut self) -> Option<Expr> {
        self.token_consume();
        Some(Expr::new(ExprKind::Nil, vec![]))
    }

    // ==========================
    //
    // NIL END
    //
    // ==========================

    // ==========================
    //
    // LIST START
    //
    // ==========================

    fn list_consume_start(&mut self) -> Option<Expr> {
        let next = self.token_consume();

        if next.is_none() {
            self.error_unexpected();
        }

        self.seq_capture(TokenKind::LeftSqrBr);

        Some(Expr::new(
            ExprKind::List,
            self.seq_consume(ExprKind::_EndOfList),
        ))
    }

    fn list_consume_end(&mut self) -> Option<Expr> {
        if !self.seq_match_end(TokenKind::LeftSqrBr) {
            self.error(&messages::unmatched_sqr_bracket(), None);
        }

        self.token_consume();

        Some(Expr::new(ExprKind::_EndOfList, vec![]))
    }

    // ==========================
    //
    // LIST END
    //
    // ==========================

    // ==========================
    //
    // FUNCTION START
    //
    // ==========================

    fn fn_consume_start(&mut self, fn_expr_kind: ExprKind) -> Option<Expr> {
        let next = self.token_next();

        if next.is_none() {
            self.error_unexpected();
        }

        let next = next.unwrap();

        if next.kind == TokenKind::Newline || next.kind == TokenKind::Whitespace {
            self.token_skip(1);
            return self.fn_consume_start(fn_expr_kind);
        } else if next.kind == TokenKind::LeftParen {
            self.token_skip(2);
            self.seq_capture(TokenKind::LeftParen);

            Some(Expr::new(
                fn_expr_kind,
                self.seq_consume(ExprKind::_EndOfFn),
            ))
        } else {
            self.error_unexpected();
        }
    }

    fn fn_consume_end(&mut self) -> Option<Expr> {
        if !self.seq_match_end(TokenKind::LeftParen) {
            self.error(&messages::unmatched_parenthesis(), None);
        }

        self.token_consume();

        Some(Expr::new(ExprKind::_EndOfFn, vec![]))
    }

    // ==========================
    //
    // FUNCTION END
    //
    // ==========================

    // ==========================
    //
    // SEPARATOR START
    //
    // ==========================

    fn separator_consume(&mut self) -> Option<Expr> {
        self.token_consume();
        Some(Expr::new(ExprKind::_Separator, vec![]))
    }

    // ==========================
    //
    // SEPARATOR END
    //
    // ==========================
}

pub fn parse<'a>(tokens: Vec<Token>, source_code: &str) -> Vec<Expr> {
    let mut parser = Parser::new(tokens, source_code);
    let mut expressions: Vec<Expr> = Vec::new();

    while let Some(expr) = parser.next_expr() {
        if is_expr_internal(&expr) {
            continue;
        }
        expressions.push(expr);
    }

    if parser.seq_stack.len() > 0 {
        parser.error(&messages::unclosed_opening_symbols(), None);
    }

    expressions
}
