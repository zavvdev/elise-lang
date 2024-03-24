pub mod number;
pub mod token;
pub mod token_span;

use crate::ast::lexer::{config::TokenKind, models::token_span::TokenSpan};

use self::{
    number::{Number, ParsedNumber},
    token::Token,
};

pub struct Lexer {
    input: String,
    char_pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: Self::prepare_input(input),
            char_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.char_pos > self.input.len() {
            return None;
        }

        let current_char = self.get_current_char();

        current_char.map(|char| {
            let start = self.char_pos;
            let mut token_kind = TokenKind::Unknown;

            if Self::is_number(&char) {
                let number = self.consume_number();
                let parsed_number = Self::parse_number(number);

                match parsed_number {
                    ParsedNumber::Int(int) => token_kind = TokenKind::Int(int),
                    ParsedNumber::Float(float) => token_kind = TokenKind::Float(float),
                }
            } else {
                self.consume(); // TODO: Should be removed
            }

            let end = self.char_pos;
            let literal = self.input[start..end].to_string();

            let token_span = TokenSpan {
                start,
                end,
                literal,
            };

            Token {
                kind: token_kind,
                span: token_span,
            }
        })
    }

    // TODO: Benchmark it and find faster solution if possible
    fn prepare_input(input: &str) -> String {
        let entries: Vec<&str> = input.split_whitespace().collect();
        entries.join(" ")
    }

    fn get_current_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.char_pos >= self.input.len() {
            return None;
        }

        let current_char = self.get_current_char();

        self.char_pos += 1;

        current_char
    }

    // Numbers

    // TODO: Move to trait

    fn is_number(char: &char) -> bool {
        char.is_digit(10)
    }

    fn parse_float(number: Number) -> f64 {
        // TODO: find a better way of converting to float
        format!("{}.{}", number.int, number.precision)
            .parse::<f64>()
            .unwrap()
    }

    fn parse_number(number: Number) -> ParsedNumber {
        if number.precision == 0 {
            ParsedNumber::Int(number.int)
        } else {
            ParsedNumber::Float(Self::parse_float(number))
        }
    }

    fn consume_number(&mut self) -> Number {
        let mut int: i64 = 0;
        let mut precision: u64 = 0;
        let mut is_int = true;

        while let Some(c) = self.get_current_char() {
            let is_digit = c.is_digit(10);

            if is_digit && is_int {
                self.consume();
                // TODO: Track numver overflow
                int = int * 10 + c.to_digit(10).unwrap() as i64;
            } else if is_digit && !is_int {
                self.consume();
                // TODO: Track numver overflow
                precision = precision * 10 + c.to_digit(10).unwrap() as u64;
            } else if c == '.' {
                self.consume();
                is_int = false;
            } else {
                break;
            }
        }

        Number { int, precision }
    }
}
