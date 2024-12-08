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

// TODO:
//
// - [ ] Add newline lexeme and token
// - [ ] ? Remove whitespace and newline from the tokenizer result
// - [ ] Add source code to error messages
// - [ ] ? Add source code to tokenizer result
// - [ ] Update tests

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            char_pos: 0,
        }
    }

    /**
     * Distunguish token kind based on the character.
     * First, check cases that are not require any speculations
     * about the next characters to determine the token kind.
     *
     * Non-speculative token - does not require any speculations about the next characters.
     * This type of token has only one possible outcome. For example, string token.
     * It starts with a double quote and ends with a double quote. There is no other
     * possible outcome.
     *
     * Speculative token - requires speculations about the next characters. This type of token
     * has multiple possible outcomes. For example, number token can be either integer
     * or float, positive or negative, so we need to check the next characters to determine.
     * It also can redirect to another function for further distinguishing if speculation failed,
     * for example, minus can be a part of the number token or it can be a separate token kind.
     * So, if the next character is a digit, it is a part of the number token, otherwise,
     * cunsumption of this token will be redirected to a separate function that will return the
     * minus token kind.
     *
     * Fallback group - if the token kind is not found in the previous groups, it should be a
     * punctuation token or identifier.
     */
    fn get_token_kind(&mut self, c: &char) -> TokenKind {
        /*
         * Non-speculative
         */
        if Self::whitespace_is_match(&c) {
            self.whitespace_consume()
        } else if Self::string_is_start(&c) {
            self.string_consume()
        } else if Self::function_is_start(&c) {
            self.function_consume()

        /*
         * Speculative
         */
        } else if Self::number_is_start(&c) {
            self.number_consume(false)

        /*
         * Fallback group
         */
        } else if let Some(punctuation_token_kind) = self.punctuation_consume() {
            punctuation_token_kind
        } else {
            self.identifier_consume()
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

    fn get_current_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos)
    }

    fn get_prev_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos - 1)
    }

    fn get_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.char_pos + 1)
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

    // ==========================
    //
    // WHITESPACE START
    //
    // Non-speculative
    //
    // ==========================

    /*
     * Whitespace-like
     */
    fn whitespace_is_match(c: &char) -> bool {
        c.is_whitespace()
    }

    fn whitespace_consume(&mut self) -> TokenKind {
        self.consume();
        TokenKind::Whitespace
    }

    // ==========================
    //
    // WHITESPACE END
    //
    // ==========================

    // ==========================
    //
    // STRING START
    //
    // Non-speculative
    //
    // 1. Starts with: Double quote
    // 2. Contains: Any character, Escape character
    // 3. Ends with: Double quote
    //
    // ==========================

    fn string_is_start(char: &char) -> bool {
        *char == lexemes::L_STRING_LITERAL
    }

    fn string_is_escape_char(c: Option<char>) -> bool {
        c == Some(lexemes::L_STRING_LITERAL_ESCAPE)
    }

    fn string_replace_escape_chars(s: &str) -> Option<String> {
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

    fn string_consume(&mut self) -> TokenKind {
        self.consume();
        let mut result = String::new();

        while let Some(c) = self.get_current_char() {
            if c == lexemes::L_STRING_LITERAL && !Self::string_is_escape_char(self.get_prev_char())
            {
                self.consume();
                break;
            }

            result.push(c);
            self.consume();
        }

        TokenKind::String(Self::string_replace_escape_chars(&result).unwrap())
    }

    // ==========================
    //
    // STRING END
    //
    // ==========================

    // ==========================
    //
    // FUNCTION START
    //
    // Non-speculative
    //
    // 1. Starts with: lexemes::L_FN
    // 2. Contains: Predefined function name
    // 3. Ends with: Left Paren, Whitespace-like
    //
    // ==========================

    fn function_is_start(c: &char) -> bool {
        *c == lexemes::L_FN
    }

    fn function_is_end(c: &char) -> bool {
        *c == lexemes::L_LEFT_PAREN || Self::whitespace_is_match(c)
    }

    fn function_consume_name(&mut self) -> String {
        let mut result = String::new();

        while let Some(c) = self.get_current_char() {
            if Self::function_is_end(&c) {
                break;
            }

            result.push(c);
            self.consume();
        }

        result
    }

    fn function_distinguish_known(fn_name: &str) -> TokenKind {
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

    fn function_consume(&mut self) -> TokenKind {
        self.consume();
        let fn_name = self.function_consume_name();
        Self::function_distinguish_known(&fn_name)
    }

    // ==========================
    //
    // FUNCTION END
    //
    // ==========================

    // ==========================
    //
    // NUMBER START
    //
    // Speculative
    //
    // 1. Starts with: Minus, Digit
    // 2. Contains: Digit, Dot, Minus
    // 3. Ends with: Whitespace-like, Comma, Right Paren, Right Sqr Br
    //
    // ==========================

    fn number_is_start(char: &char) -> bool {
        Self::number_is_digit(char) || Self::number_is_minus(char)
    }

    fn number_is_end(char: &char) -> bool {
        Self::whitespace_is_match(char)
            || *char == lexemes::L_COMMA
            || *char == lexemes::L_RIGHT_PAREN
            || *char == lexemes::L_RIGHT_SQR_BR
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

            println!("c: {}", c);
            if Self::number_is_minus(&c) {
                // Speculation start. It can be a part of the number token
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
            // Speculation failed, it is a minus token
            // from the punctuation group
            _ => self.punctuation_consume().unwrap(),
        }
    }

    // ==========================
    //
    // NUMBER END
    //
    // ==========================

    // ==========================
    //
    // PUNCTUATION START
    //
    // Fallback group
    //
    // 1. Consists of: Minus, Left Paren, Right Paren,
    // Left Sqr Br, Right Sqr Br, Comma
    //
    // ==========================

    fn punctuation_consume(&mut self) -> Option<TokenKind> {
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
    //
    // PUNCTUATION END
    //
    // ==========================

    // ==========================
    //
    // IDENTIFIER START
    //
    // Fallback group
    //
    // Rules: config::IDENTIFIER_REGEX
    // 1. Ends with: Whitespace-like, Comma, Right Paren, Right Sqr Br
    //
    // ==========================

    fn identifier_is_end(c: &char) -> bool {
        Self::whitespace_is_match(c)
            || *c == lexemes::L_COMMA
            || *c == lexemes::L_RIGHT_PAREN
            || *c == lexemes::L_RIGHT_SQR_BR
    }

    fn identifier_distinguish_known(identifier: &str) -> TokenKind {
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

    fn identifier_consume(&mut self) -> TokenKind {
        let re = Regex::new(config::IDENTIFIER_REGEX).unwrap();
        let mut result = String::new();

        while let Some(c) = self.get_current_char() {
            if Self::identifier_is_end(&c) {
                break;
            }

            result.push(c);
            self.consume();
        }

        if !re.is_match(&result) {
            panic!("{}", messages::invalid_identifier_name(&result));
        }

        Self::identifier_distinguish_known(&result)
    }

    // ==========================
    //
    // IDENTIFIER END
    //
    // ==========================
}

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
