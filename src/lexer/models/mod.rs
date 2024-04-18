pub mod number;
pub mod token;

use crate::{messages, types};

use self::{
    number::{FloatPrecision, Number, ParsedNumber, FLOAT_SEPARATOR},
    token::{Token, TokenKind, TokenSpan},
};

use super::lexemes;

/**
 *
 * Accepts an input as a string with custom formattings and char position to start with.
 *
 */
pub struct Lexer {
    input: String,
    char_pos: usize,
}

impl Lexer {
    /**
     *
     * Create a new Lexer instance based on raw user input.
     * User input should be processed before analysis by `preprocess` method
     *
     */
    pub fn new(input: &str) -> Self {
        Self {
            input: Self::preprocess(input),
            char_pos: 0,
        }
    }

    // ==========================

    //          Defaults

    // ==========================

    /**
     *
     * Analyze current character/character seq and return a Token instance
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

            // ===============
            // Number
            // ===============
            if Self::is_number(&char) {
                let number = self.consume_number();
                let parsed_number = Self::parse_number(number);

                token_kind = match parsed_number {
                    ParsedNumber::Int(int) => TokenKind::Int(int),
                    ParsedNumber::Float(float) => TokenKind::Float(float),
                };

            // ===============
            // Whitespace
            // ===============
            } else if Self::is_whitespace(&char) {
                token_kind = TokenKind::Whitespace;
                self.consume();

            // ===============
            // Functions
            // ===============
            } else if Self::is_fn_start(&char) {
                self.consume();
                let fn_name = self.consume_known_fn_name();
                token_kind = Self::distinguish_known_fn(&fn_name);

            // ===============
            // Punctuations
            // ===============
            } else {
                if let Some(punctuation_token_kind) = self.consume_punctuation() {
                    token_kind = punctuation_token_kind
                }
            }

            // ===============
            // Construct Token
            // ===============

            let end = self.char_pos;
            let lexeme = self.input[start..end].to_string();

            if token_kind == TokenKind::Unknown {
                panic!("{}", messages::m_unexpected_token(&lexeme));
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
    fn parse_float(number: Number) -> types::Float {
        format!("{}.{}", number.int, number.precision)
            .parse::<types::Float>()
            .unwrap()
    }

    /**
     *
     * Should be used for parsing to either `Integer` or `Float` based on available precision
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
        let mut int: types::Integer = 0;
        let mut precision: FloatPrecision = 0;
        let mut is_int = true;

        while let Some(c) = self.get_current_char() {
            let is_digit = c.is_digit(10);

            if is_digit && is_int {
                int = int.checked_mul(10).expect(&messages::m_int_overflow());

                int = int
                    .checked_add(c.to_digit(10).unwrap() as types::Integer)
                    .expect(&messages::m_int_overflow());

                self.consume();
            } else if is_digit && !is_int {
                precision = precision
                    .checked_mul(10)
                    .expect(&messages::m_float_overflow());

                precision = precision
                    .checked_add(c.to_digit(10).unwrap() as FloatPrecision)
                    .expect(&messages::m_float_overflow());

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
     * Analyse all possible punctuations
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

        TokenKind::Unknown
    }
}
