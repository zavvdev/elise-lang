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

struct Parser<'a> {
    source_code: &'a str,
    tokens: Vec<Token>,
    token_pos: usize,

    /**
     * Sequence counting. Sequence is something
     * that has specific amount of elements within
     * distinct bounds (start and end tokens).
     * For example, function is a sequence of arguments,
     * list is a sequence of elements, etc.
     */
    seq_start_count: usize,
    seq_end_count: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>, source_code: &'a str) -> Self {
        Self {
            source_code,
            tokens,
            token_pos: 0,
            seq_start_count: 0,
            seq_end_count: 0,
        }
    }

    fn next_expr(&mut self) -> Option<Expr> {
        self.check_valid_eof();

        if self.token_pos > self.tokens.len() {
            return None;
        }

        let current_token = self.get_current_token()?;

        match &current_token.kind {
            // Internal Expressions
            // These exporessions are used for internal parser purposes
            // and should not be exposed to the user
            TokenKind::RightParen => self.internal_end_of_fn_consume(),
            TokenKind::RightSqrBr => self.internal_end_of_list_consume(),
            TokenKind::Comma => self.internal_separator_consume(),
            TokenKind::Whitespace => self.internal_separator_consume(),
            TokenKind::Newline => self.internal_separator_consume(),

            // Public Expressions
            // These expressions are exposed to the user
            TokenKind::Number(x) => self.number_consume(*x),
            TokenKind::String(x) => self.string_consume(x.to_string()),
            TokenKind::Boolean(x) => self.boolean_consume(*x),
            TokenKind::Identifier(x) => self.identifier_consume(x.to_string()),
            TokenKind::Nil => self.nil_consume(),
            TokenKind::LeftSqrBr => self.list_consume(),
            TokenKind::FnAdd => self.fn_consume(ExprKind::FnAdd),
            TokenKind::FnSub => self.fn_consume(ExprKind::FnSub),
            TokenKind::FnMul => self.fn_consume(ExprKind::FnMul),
            TokenKind::FnDiv => self.fn_consume(ExprKind::FnDiv),
            TokenKind::FnPrint => self.fn_consume(ExprKind::FnPrint),
            TokenKind::FnPrintLn => self.fn_consume(ExprKind::FnPrintLn),
            TokenKind::FnLetBinding => self.fn_consume(ExprKind::FnLetBinding),
            TokenKind::FnGreatr => self.fn_consume(ExprKind::FnGreatr),
            TokenKind::FnGreatrEq => self.fn_consume(ExprKind::FnGreatrEq),
            TokenKind::FnLess => self.fn_consume(ExprKind::FnLess),
            TokenKind::FnLessEq => self.fn_consume(ExprKind::FnLessEq),
            TokenKind::FnEq => self.fn_consume(ExprKind::FnEq),
            TokenKind::FnNotEq => self.fn_consume(ExprKind::FnNotEq),
            TokenKind::FnNot => self.fn_consume(ExprKind::FnNot),
            TokenKind::FnAnd => self.fn_consume(ExprKind::FnAnd),
            TokenKind::FnOr => self.fn_consume(ExprKind::FnOr),
            TokenKind::FnBool => self.fn_consume(ExprKind::FnBool),
            TokenKind::FnIf => self.fn_consume(ExprKind::FnIf),
            TokenKind::FnIsNil => self.fn_consume(ExprKind::FnIsNil),
            TokenKind::FnDefine => self.fn_consume(ExprKind::FnDefine),
            TokenKind::FnCustom(x) => self.fn_consume(ExprKind::FnCustom(x.to_string())),
            _ => None,
        }
    }

    fn check_valid_eof(&mut self) {
        if self.seq_start_count != self.seq_end_count && self.get_current_token().is_none() {
            self.error(&messages::unexpected_end_of_input(), self.tokens.last());
        } else if self.seq_start_count == self.seq_end_count && self.seq_start_count != 0 {
            self.seq_start_count = 0;
            self.seq_end_count = 0;
        }
    }

    /**
     *
     * Should be used if you want to skip `offset` amount of tokens.
     * For example, if I'm at token 4 and I call `self.skip_tokens(1)`
     * then the next token_pos will be 5. Useful in the case
     * when you parsed a sequence of tokens at once.
     *
     */
    fn skip_tokens(&mut self, offset: usize) {
        let tokens_len = self.tokens.len();

        if self.token_pos < tokens_len {
            self.token_pos += offset;
        }
    }

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

    // ==========================
    //
    // ERRORS START
    //
    // ==========================

    fn error(&self, message: &str, token: Option<&Token>) -> ! {
        if let Some(token) = token {
            print_error_message(message, self.source_code, token.span.start);
        } else if let Some(t) = self.get_current_token() {
            print_error_message(message, self.source_code, t.span.start);
        } else {
            ();
        }

        panic!("{}", messages::get_panic_message());
    }

    fn error_at_current(&self) -> ! {
        let token = self.get_current_token().unwrap();
        self.error(&messages::unexpected_token(&token.span.lexeme), Some(token));
    }

    // ==========================
    //
    // ERRORS END
    //
    // Generic
    //
    // ==========================

    // ==========================
    //
    // SEQUENCE START
    //
    // Generic
    //
    // Argument separator: Comma, Whitespace, Newline
    //
    // ==========================

    /**
     *
     * Should be used for counting start of sequence
     *
     */
    fn seq_capture(&mut self) {
        self.seq_start_count += 1;
    }

    /**
     *
     * Should be used for counting end of sequence
     *
     */
    fn seq_end(&mut self) {
        self.seq_end_count += 1;
    }

    /**
     *
     * Can be used for consuming arguemnts of any sequentional entity
     *
     */
    fn seq_consume_arguments(&mut self, seq_end_expr: ExprKind) -> Vec<Box<Expr>> {
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

    // ==========================
    //
    // SEQUENCE END
    //
    // ==========================

    // ==========================
    //
    // END OF FUNCTION START
    //
    // Internal
    //
    // ==========================

    fn internal_end_of_fn_consume(&mut self) -> Option<Expr> {
        self.consume();
        self.seq_end();
        Some(Expr::new(ExprKind::_EndOfFn, vec![]))
    }

    // ==========================
    //
    // END OF FUNCTION END
    //
    // ==========================

    // ==========================
    //
    // END OF LIST START
    //
    // Internal
    //
    // ==========================

    fn internal_end_of_list_consume(&mut self) -> Option<Expr> {
        self.consume();
        self.seq_end();
        Some(Expr::new(ExprKind::_EndOfList, vec![]))
    }

    // ==========================
    //
    // END OF LIST END
    //
    // ==========================

    // ==========================
    //
    // SEPARATOR START
    //
    // Internal
    //
    // ==========================

    fn internal_separator_consume(&mut self) -> Option<Expr> {
        self.consume();
        Some(Expr::new(ExprKind::_Separator, vec![]))
    }

    // ==========================
    //
    // SEPARATOR END
    //
    // ==========================

    // ==========================
    //
    // NUMBER START
    //
    // ==========================

    fn number_consume(&mut self, x: types::Number) -> Option<Expr> {
        self.consume();
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
        self.consume();
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
        self.consume();
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
        self.consume();
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
        self.consume();
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

    fn list_consume(&mut self) -> Option<Expr> {
        let next = self.get_next_token();

        if next.is_none() {
            self.error_at_current();
        }

        self.consume();
        self.seq_capture();

        Some(Expr::new(
            ExprKind::List,
            self.seq_consume_arguments(ExprKind::_EndOfList),
        ))
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

    fn fn_consume(&mut self, fn_expr_kind: ExprKind) -> Option<Expr> {
        let next = self.get_next_token();

        if next.is_none() {
            self.error_at_current();
        }

        let next = next.unwrap();

        if next.kind == TokenKind::Newline || next.kind == TokenKind::Whitespace {
            self.skip_tokens(1);
            return self.fn_consume(fn_expr_kind);
        } else if next.kind == TokenKind::LeftParen {
            self.skip_tokens(2);
            self.seq_capture();

            return Some(Expr::new(
                fn_expr_kind,
                self.seq_consume_arguments(ExprKind::_EndOfFn),
            ));
        } else {
            self.error_at_current();
        }
    }

    // ==========================
    //
    // FUNCTION END
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

    expressions
}
