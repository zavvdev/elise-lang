use std::collections::VecDeque;

use regex::Regex;

use crate::types;

use self::models::{number::{BaseNumber, ConsumedNumber, FLOAT_SEPARATOR}, token::{Token, TokenKind, TokenSpan}};

pub mod config;
pub mod lexemes;
pub mod messages;
pub mod models;
pub mod __tests__;

struct Lexer {
    input: String,
    char_pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: Self::preprocess(input),
            char_pos: 0,
        }
    }

    fn distinguish_token_kind(&mut self, c: &char) -> TokenKind {
        if Self::is_number(&c) {
            let number = self.consume_number();
            self.construct_number_token(number)
        } else if Self::is_whitespace(&c) {
            self.consume();
            TokenKind::Whitespace
        } else if Self::is_fn_start(&c) {
            self.consume();
            let fn_name = self.consume_known_fn_name();
            Self::distinguish_known_fn(&fn_name)
        } else if Self::is_string_literal(&c) {
            self.consume();
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
            let token_kind = self.distinguish_token_kind(&char);

            let end = self.char_pos;
            let lexeme = self.input[start..end].to_string();

            if token_kind == TokenKind::Unknown {
                panic!("{}", messages::unknown_lexeme(&lexeme));
            }

            let token_span = TokenSpan { start, end, lexeme };

            Token {
                kind: token_kind,
                span: token_span,
            }
        })
    }

    /**
     *
     * Should be used for processing raw user input during Lexer instance construction.
     * Should remove multiple Unicode whitespace characters
     *
     * TODO: Benchmark it and find faster solution if possible
     *
     */
    fn preprocess(input: &str) -> String {
        let entries: Vec<&str> = input.split_whitespace().collect();
        entries.join(&lexemes::L_WHITESPACE.to_string())
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

    fn is_separator(c: &char) -> bool {
        *c == lexemes::L_WHITESPACE || *c == lexemes::L_COMMA
    }

    // ==========================

    //          Numbers

    // ==========================

    /**
     *
     * Identify Base 10 numeric character
     *
     */
    fn is_number(char: &char) -> bool {
        char.is_digit(10)
    }

    /**
     *
     * Analysing numeric sequence as `Number`
     *
     */
    fn consume_number(&mut self) -> ConsumedNumber {
        let mut int: BaseNumber = 0;
        let mut precision: BaseNumber = 0;
        let mut is_int = true;

        while let Some(c) = self.get_current_char() {
            let is_digit = c.is_digit(10);

            if is_digit && is_int {
                int = int.checked_mul(10).expect(&messages::number_overflow());

                int = int
                    .checked_add(c.to_digit(10).unwrap() as BaseNumber)
                    .expect(&messages::number_overflow());

                self.consume();
            } else if is_digit && !is_int {
                precision = precision
                    .checked_mul(10)
                    .expect(&messages::number_overflow());

                precision = precision
                    .checked_add(c.to_digit(10).unwrap() as BaseNumber)
                    .expect(&messages::number_overflow());

                self.consume();
            } else if c == FLOAT_SEPARATOR {
                is_int = false;
                self.consume();
            } else {
                break;
            }
        }

        ConsumedNumber {
            int,
            precision,
            is_int,
        }
    }

    fn construct_number_token(&self, number: ConsumedNumber) -> TokenKind {
        if number.is_int {
            TokenKind::Number(number.int as types::Number)
        } else {
            TokenKind::Number(
                format!("{}{}{}", number.int, FLOAT_SEPARATOR, number.precision)
                    .parse::<types::Number>()
                    .unwrap(),
            )
        }
    }

    // ==========================

    //       Punctuations

    // ==========================

    /**
     *
     * Analyse all possible punctuations
     *
     */
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

    //           Other

    // ==========================

    /**
     *
     * Checks if given character is a Unicode whitespace
     *
     */
    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    // ==========================

    //        Known functions

    // ==========================

    /**
     *
     * Determine the start of the custom or known function
     *
     */
    fn is_fn_start(c: &char) -> bool {
        *c == lexemes::L_FN
    }

    /**
     *
     * Consume only known function names
     *
     */
    fn consume_known_fn_name(&mut self) -> String {
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

    /**
     *
     * Match known function lexeme
     *
     */
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

        TokenKind::Unknown
    }

    // ==========================

    //         Identifier

    // ==========================

    fn is_identifier_end(c: &char) -> bool {
        Self::is_separator(c) || *c == lexemes::L_RIGHT_PAREN || *c == lexemes::L_RIGHT_SQR_BR
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
        if token.kind == TokenKind::Whitespace {
            continue;
        }
        tokens.push(token);
    }

    tokens
}
