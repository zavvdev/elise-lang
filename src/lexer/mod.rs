use std::collections::VecDeque;

use regex::Regex;

use crate::types;

use self::models::{
    number::{BaseNumber, ConsumedNumber, FLOAT_SEPARATOR},
    token::{Token, TokenKind, TokenSpan},
};

pub mod __tests__;
pub mod config;
pub mod lexemes;
pub mod messages;
pub mod models;

struct Lexer {
    input: String,
    char_pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            char_pos: 0,
        }
    }

    fn get_token_kind(&mut self, c: &char) -> TokenKind {
        if Self::number_is_start(&c) {
            self.number_consume(false)
        } else if Self::whitespace_is_match(&c) {
            self.consume_whitespace()
        } else if Self::is_fn_start(&c) {
            self.consume_fn()
        } else if Self::is_string_literal(&c) {
            self.consume_string_literal()
        } else if let Some(punctuation_token_kind) = self.consume_punctuation() {
            punctuation_token_kind
        } else {
            self.consume_identifier()
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.char_pos > self.input.len() {
            return None;
        }

        let current_char = self.get_current_char();

        current_char.map(|char| {
            let start = self.char_pos;
            let token_kind = self.get_token_kind(&char);

            let end = self.char_pos;
            let lexeme = self.input[start..end].to_string();
            let token_span = TokenSpan { start, end, lexeme };

            Token {
                kind: token_kind,
                span: token_span,
            }
        })
    }

    /**
     *
     * Should be used when current character is required withoud consuming
     *
     */
    fn get_current_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos)
    }

    fn get_prev_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos - 1)
    }

    fn get_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos + 1)
    }

    fn is_whitespace_like(c: &char) -> bool {
        c.is_whitespace()
    }

    /**
     *
     * Should be used when current character is required but with
     * addition move to the next character
     *
     */
    fn consume(&mut self) -> Option<char> {
        if self.char_pos >= self.input.len() {
            return None;
        }

        let current_char = self.get_current_char();

        self.char_pos += 1;

        current_char
    }

    /**
     * ==========================
     *
     * NUMBER START
     *
     * 1. Can start with: Minus, Digit
     * 2. Can contain: Digit, Dot, Minus
     * 3. Can end with: Comma, Whitespace-like
     *
     * ==========================
     */

    fn number_is_start(char: &char) -> bool {
        Self::number_is_digit(char) || Self::number_is_minus(char)
    }

    fn number_is_end(char: &char) -> bool {
        *char == lexemes::L_COMMA || Self::is_whitespace_like(char)
    }

    fn number_is_digit(char: &char) -> bool {
        char.is_digit(10)
    }

    fn number_is_minus(char: &char) -> bool {
        *char == lexemes::L_MINUS
    }

    fn number_append(prev: BaseNumber, next_char_digit: char) -> BaseNumber {
        let mut res = prev.checked_mul(10).expect(&messages::number_overflow());

        res = res
            .checked_add(next_char_digit.to_digit(10).unwrap() as BaseNumber)
            .expect(&messages::number_overflow());

        res as BaseNumber
    }

    fn number_construct_token(number: ConsumedNumber) -> TokenKind {
        let sig: types::Number = if number.is_negative { -1.0 } else { 1.0 };

        if number.is_int {
            TokenKind::Number((number.int * sig as BaseNumber) as types::Number)
        } else {
            TokenKind::Number(
                format!("{}{}{}", number.int, FLOAT_SEPARATOR, number.precision)
                    .parse::<types::Number>()
                    .unwrap()
                    * sig,
            )
        }
    }

    fn number_consume(&mut self, is_negative: bool) -> TokenKind {
        let mut int: BaseNumber = 0;
        let mut precision: BaseNumber = 0;
        let mut is_int = true;

        while let Some(c) = self.get_current_char() {
            let is_digit = Self::number_is_digit(&c);

            if Self::number_is_minus(&c) {
                return self.number_maybe_signed();
            } else if is_digit && is_int {
                int = Self::number_append(int, c);
                self.consume();
            } else if is_digit && !is_int {
                precision = Self::number_append(precision, c);
                self.consume();
            } else if c == FLOAT_SEPARATOR {
                is_int = false;
                self.consume();
            } else if Self::number_is_end(&c) {
                break;
            } else {
                panic!("{}", messages::invalid_number());
            }
        }

        Self::number_construct_token(ConsumedNumber {
            int,
            precision,
            is_int,
            is_negative,
        })
    }

    fn number_maybe_signed(&mut self) -> TokenKind {
        let next = self.get_next_char();

        match next {
            Some(c) if Self::number_is_digit(&c) => {
                self.consume();
                self.number_consume(true)
            }
            _ => self.consume_punctuation().unwrap(),
        }
    }

    /**
     * ==========================
     *
     * NUMBER END
     *
     * ==========================
     */

    // ==========================

    //        Whitespace

    // ==========================

    fn whitespace_is_match(c: &char) -> bool {
        c.is_whitespace()
    }

    fn consume_whitespace(&mut self) -> TokenKind {
        self.consume();
        TokenKind::Whitespace
    }

    // ==========================

    //       Punctuations

    // ==========================

    fn consume_punctuation(&mut self) -> Option<TokenKind> {
        let char = self.get_current_char()?;

        if let Some(c) = match char {
            lexemes::L_MINUS => Some(TokenKind::Minus),
            lexemes::L_LEFT_PAREN => Some(TokenKind::LeftParen),
            lexemes::L_RIGHT_PAREN => Some(TokenKind::RightParen),
            lexemes::L_LEFT_SQR_BR => Some(TokenKind::LeftSqrBr),
            lexemes::L_RIGHT_SQR_BR => Some(TokenKind::RightSqrBr),
            lexemes::L_COMMA => Some(TokenKind::Comma),
            _ => None,
        } {
            self.consume();
            Some(c)
        } else {
            None
        }
    }

    // ==========================

    //         Functions

    // ==========================

    fn is_fn_start(c: &char) -> bool {
        *c == lexemes::L_FN
    }

    fn consume_fn_name(&mut self) -> String {
        let mut result = String::new();

        while let Some(c) = self.get_current_char() {
            if c == lexemes::L_LEFT_PAREN || c == lexemes::L_WHITESPACE {
                break;
            }

            result.push(c);
            self.consume();
        }

        result
    }

    fn distinguish_known_fn(fn_name: &str) -> TokenKind {
        if fn_name == lexemes::L_FN_ADD.1 {
            return TokenKind::FnAdd;
        }

        if fn_name == lexemes::L_FN_SUB.1 {
            return TokenKind::FnSub;
        }

        if fn_name == lexemes::L_FN_MUL.1 {
            return TokenKind::FnMul;
        }

        if fn_name == lexemes::L_FN_DIV.1 {
            return TokenKind::FnDiv;
        }

        if fn_name == lexemes::L_FN_PRINT.1 {
            return TokenKind::FnPrint;
        }

        if fn_name == lexemes::L_FN_PRINTLN.1 {
            return TokenKind::FnPrintLn;
        }

        if fn_name == lexemes::L_FN_LET_BINDING.1 {
            return TokenKind::FnLetBinding;
        }

        if fn_name == lexemes::L_FN_GREATR.1 {
            return TokenKind::FnGreatr;
        }

        if fn_name == lexemes::L_FN_LESS.1 {
            return TokenKind::FnLess;
        }

        if fn_name == lexemes::L_FN_GREATR_EQ.1 {
            return TokenKind::FnGreatrEq;
        }

        if fn_name == lexemes::L_FN_LESS_EQ.1 {
            return TokenKind::FnLessEq;
        }

        if fn_name == lexemes::L_FN_EQ.1 {
            return TokenKind::FnEq;
        }

        if fn_name == lexemes::L_FN_NOT_EQ.1 {
            return TokenKind::FnNotEq;
        }

        if fn_name == lexemes::L_FN_NOT.1 {
            return TokenKind::FnNot;
        }

        if fn_name == lexemes::L_FN_AND.1 {
            return TokenKind::FnAnd;
        }

        if fn_name == lexemes::L_FN_OR.1 {
            return TokenKind::FnOr;
        }

        if fn_name == lexemes::L_FN_BOOL.1 {
            return TokenKind::FnBool;
        }

        if fn_name == lexemes::L_FN_IF.1 {
            return TokenKind::FnIf;
        }

        if fn_name == lexemes::L_FN_IS_NIL.1 {
            return TokenKind::FnIsNil;
        }

        if fn_name == lexemes::L_FN_DEFINE.1 {
            return TokenKind::FnDefine;
        }

        TokenKind::FnCustom(fn_name.to_string())
    }

    fn consume_fn(&mut self) -> TokenKind {
        self.consume();
        let fn_name = self.consume_fn_name();
        Self::distinguish_known_fn(&fn_name)
    }

    // ==========================

    //         Identifier

    // ==========================

    fn is_identifier_end(c: &char) -> bool {
        *c == lexemes::L_WHITESPACE
            || *c == lexemes::L_COMMA
            || *c == lexemes::L_RIGHT_PAREN
            || *c == lexemes::L_RIGHT_SQR_BR
    }

    fn distinguish_identifier(&self, identifier: &str) -> TokenKind {
        if identifier == lexemes::L_NIL {
            return TokenKind::Nil;
        }

        if identifier == lexemes::L_TRUE {
            return TokenKind::Boolean(true);
        }

        if identifier == lexemes::L_FALSE {
            return TokenKind::Boolean(false);
        }

        TokenKind::Identifier(identifier.to_string())
    }

    fn consume_identifier(&mut self) -> TokenKind {
        let re = Regex::new(config::IDENTIFIER_REGEX).unwrap();
        let mut result = String::new();

        while let Some(c) = self.get_current_char() {
            if Self::is_identifier_end(&c) {
                break;
            }

            result.push(c);
            self.consume();
        }

        if !re.is_match(&result) {
            panic!("{}", messages::invalid_identifier_name(&result));
        }

        self.distinguish_identifier(&result)
    }

    // ==========================

    //          String

    // ==========================

    fn is_string_literal(char: &char) -> bool {
        *char == lexemes::L_STRING_LITERAL
    }

    fn is_current_char_escaped(&self) -> bool {
        self.get_prev_char() == Some(lexemes::L_STRING_LITERAL_ESCAPE)
    }

    fn replace_escape_chars(s: &str) -> Option<String> {
        let mut queue = String::from(s).chars().collect::<VecDeque<_>>();
        let mut result = String::new();

        while let Some(c) = queue.pop_front() {
            if c != '\\' {
                result.push(c);
                continue;
            }

            match queue.pop_front() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\"') => result.push('\"'),
                Some('\\') => result.push('\\'),
                Some('0') => result.push('\0'),
                _ => return None,
            };
        }

        Some(result)
    }

    fn consume_string_literal(&mut self) -> TokenKind {
        self.consume();
        let mut result = String::new();

        while let Some(c) = self.get_current_char() {
            if c == lexemes::L_STRING_LITERAL && !self.is_current_char_escaped() {
                self.consume();
                break;
            }

            result.push(c);
            self.consume();
        }

        TokenKind::String(Self::replace_escape_chars(&result).unwrap())
    }
}

// ==============================================

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(&input);

    while let Some(token) = lexer.next_token() {
        if token.kind == TokenKind::Whitespace
            && tokens
                .last()
                .map_or(false, |t| t.kind == TokenKind::Whitespace)
        {
            continue;
        }
        tokens.push(token);
    }

    tokens
}
