pub mod number;
pub mod token;

use self::{
    number::{Float, FloatPrecision, Integer, Number, ParsedNumber, FLOAT_SEPARATOR},
    token::{Token, TokenKind, TokenSpan},
};

use super::{lexemes, messages};

pub struct Lexer {
    input: String,
    char_pos: usize,
}

impl Lexer {
    /**
     *
     * Create a new Lexer instance based on raw user input.
     * User input should be processed before analysis by `prepare_input` method
     *
     */
    pub fn new(input: &str) -> Self {
        Self {
            input: Self::prepare_input(input),
            char_pos: 0,
        }
    }

    // ==========================

    //          Defaults

    // ==========================

    /**
     *
     * Analyze current character and return a Token instance
     *
     */
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

                token_kind = match parsed_number {
                    ParsedNumber::Int(int) => TokenKind::Int(int),
                    ParsedNumber::Float(float) => TokenKind::Float(float),
                };
            } else if Self::is_whitespace(&char) {
                token_kind = TokenKind::Whitespace;
                self.consume();
            } else {
                if let Some(punctuation_token_kind) = self.consume_punctuation() {
                    token_kind = punctuation_token_kind
                }
            }

            let end = self.char_pos;
            let lexeme = self.input[start..end].to_string();

            if token_kind == TokenKind::Unknown {
                panic!("{} \"{}\"", messages::M_UNEXPECTED_TOKEN, &lexeme);
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
    fn prepare_input(input: &str) -> String {
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

    /**
     *
     * Should be used when current character has been identified as valid TokenKind
     * and you are ready to analyze next character
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
     *
     * Should be used for lexemes that consists of 2 characters
     *
     */
    fn lex_potential_pair(
        &mut self,
        expected: char,
        expected_token_kind: TokenKind,
        fallback_token_kind: TokenKind,
    ) -> TokenKind {
        if let Some(next) = self.get_current_char() {
            if next == expected {
                self.consume();
                expected_token_kind
            } else {
                fallback_token_kind
            }
        } else {
            fallback_token_kind
        }
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
     * Should be used for parsing `Number` instance as `Float` with its precision
     *
     * TODO: find a better way of converting to float
     *
     */
    fn parse_float(number: Number) -> Float {
        format!("{}.{}", number.int, number.precision)
            .parse::<Float>()
            .unwrap()
    }

    /**
     *
     * Should be used for parsing to `Integer` only. Precision is not included
     *
     */
    fn parse_number(number: Number) -> ParsedNumber {
        if number.precision == 0 {
            ParsedNumber::Int(number.int)
        } else {
            ParsedNumber::Float(Self::parse_float(number))
        }
    }

    /**
     *
     * Analysing numeric sequence as `Integer` or `Float`
     *
     */
    fn consume_number(&mut self) -> Number {
        let mut int: Integer = 0;
        let mut precision: FloatPrecision = 0;
        let mut is_int = true;

        while let Some(c) = self.get_current_char() {
            let is_digit = c.is_digit(10);

            if is_digit && is_int {
                int = int.checked_mul(10).expect(messages::M_INT_OVERFLOW);

                int = int
                    .checked_add(c.to_digit(10).unwrap() as Integer)
                    .expect(messages::M_INT_OVERFLOW);

                self.consume();
            } else if is_digit && !is_int {
                precision = precision.checked_mul(10).expect(messages::M_FLOAT_OVERFLOW);

                precision = precision
                    .checked_add(c.to_digit(10).unwrap() as FloatPrecision)
                    .expect(messages::M_FLOAT_OVERFLOW);

                self.consume();
            } else if c == FLOAT_SEPARATOR {
                is_int = false;
                self.consume();
            } else {
                break;
            }
        }

        Number { int, precision }
    }

    // ==========================

    //       Punctuations

    // ==========================

    /**
     *
     * Analyse all possible punctuations from `config.rs` prefixed by `P_*`
     *
     */
    fn consume_punctuation(&mut self) -> Option<TokenKind> {
        let char = self.consume().unwrap();

        match char {
            lexemes::L_MINUS => Some(self.lex_potential_pair(
                lexemes::L_RETURN_TYPE.1,
                TokenKind::ReturnType,
                TokenKind::Minus,
            )),
            lexemes::L_LEFT_PAREN => Some(TokenKind::LeftParen),
            lexemes::L_RIGHT_PAREN => Some(TokenKind::RightParen),
            lexemes::L_LEFT_SQR_BR => Some(TokenKind::LeftSqrBr),
            lexemes::L_RIGHT_SQR_BR => Some(TokenKind::RightSqrBr),
            lexemes::L_COLON => Some(TokenKind::Colon),
            lexemes::L_COMMA => Some(TokenKind::Comma),
            _ => None,
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
}
