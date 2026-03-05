use std::str::from_utf8;

use crate::messages;

// =======================
// Token Definitions
// =======================

const T_FN_PREFIX: u8 = b'.';
const T_FN_DECLARE: &str = "declare";

const T_LEFT_PAREN: u8 = b'(';
const T_RIGHT_PAREN: u8 = b')';

const T_LEFT_SQR_BRACKET: u8 = b'[';
const T_RIGHT_SQR_BRACKET: u8 = b']';

const T_MINUS: u8 = b'-';
const T_PERIOD: u8 = b'.';
const T_COMMA: u8 = b',';

// =======================
// Custom Types
// =======================

type TNumber = f64;
type TString = String;

// =======================
// Parser
// =======================

#[derive(Debug)]
enum AstNodeValue {
    Function,
    Identifier,
    Number(TNumber),
    String(TString),
}

#[derive(Debug)]
pub struct AstNode {
    value: AstNodeValue,
    tok_start: usize,
    children: Vec<Box<AstNode>>,
}

// Since strings and chars in Rust are UTF-8 encoded,
// which means that even if our char fits into 1 byte (ASCII)
// we still have 4 bytes allocated for it. So we split our
// source code input into raw bytes which takes less memory
// space and compare tokens as bytes.
// For preserving UTF-8 encodings for Elise Strings we just
// slice string bytes and convert to UTF-8 that particular
// slice of bytes.

pub struct Parser<'a> {
    source_code: &'a [u8],
    tok_pos: usize,
    depth_stack: Vec<u8>,
}

impl<'a> Parser<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code: &source_code.as_bytes(),
            tok_pos: 0,
            depth_stack: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut ast: Vec<AstNode> = vec![];

        while let Some(current_char) = self.peek_at(self.tok_pos) {
            if Self::is_whitespace(&current_char) {
                self.advance();
            } else if Self::number_is_start(&current_char) {
                ast.push(self.number_consume());
            }
        }

        ast
    }

    // Token

    fn advance(&mut self) -> Option<u8> {
        let tok = self.peek_at(self.tok_pos);
        self.tok_pos += 1;
        tok
    }

    fn peek_at(&mut self, pos: usize) -> Option<u8> {
        if pos >= self.source_code.len() {
            return None;
        }
        self.source_code.get(pos).copied()
    }

    // Utilities

    fn is_whitespace(c: &u8) -> bool {
        matches!(c, b' ' | b'\n' | b'\t' | b'\r')
    }

    // Number

    fn number_is_digit(c: &u8) -> bool {
        (b'0'..=b'9').contains(c)
    }

    fn number_is_start(c: &u8) -> bool {
        Self::number_is_digit(c) || *c == T_MINUS
    }

    fn number_is_end(c: &u8) -> bool {
        Self::is_whitespace(c) || *c == T_COMMA || *c == T_RIGHT_PAREN || *c == T_RIGHT_SQR_BRACKET
    }

    fn number_consume(&mut self) -> AstNode {
        let mut value: Vec<u8> = vec![];
        let mut float = false;
        let tok_start = self.tok_pos;

        while let Some(c) = self.peek_at(self.tok_pos) {
            let next_tok = self.peek_at(self.tok_pos + 1);

            if Self::number_is_end(&c) {
                break;
            } else if Self::number_is_digit(&c) {
                value.push(c);
                self.advance();
            } else if c == T_MINUS
                && value.is_empty()
                && next_tok.is_some()
                && Self::number_is_digit(&next_tok.unwrap())
            {
                value.push(c);
                self.advance();
            } else if c == T_PERIOD && !value.is_empty() && !float {
                float = true;
                value.push(c);
                self.advance();
            } else {
                messages::error_at_char_pos(
                    messages::M_INVALID_NUMBER,
                    self.source_code,
                    self.tok_pos,
                );
            }
        }

        let value = from_utf8(&value);

        if value.is_err() {
            messages::error_at_char_pos(messages::M_INVALID_NUMBER, self.source_code, self.tok_pos);
        }

        let numeric = value.unwrap().parse::<TNumber>();

        if numeric.is_err() {
            messages::error_at_char_pos(messages::M_INVALID_NUMBER, self.source_code, self.tok_pos);
        }

        AstNode {
            value: AstNodeValue::Number(numeric.unwrap()),
            tok_start,
            children: vec![],
        }
    }
}

// =======================
// Tests
// =======================

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_none_if_source_code_is_empty_string() {
        panic!("TODO");
    }

    #[test]
    fn should_return_none_if_source_code_is_string_with_spaces() {
        panic!("TODO");
    }

    // Number

    #[test]
    #[should_panic]
    fn should_panic_if_number_contains_non_numeric_token() {
        // 1a, 12a2, 0.2a, -1a
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_contains_more_than_one_minus_token() {
        // --1, -1-2, -2-3-
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_contains_more_than_one_period_token() {
        // 0.2.3, 0.3.
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_starts_with_zero_and_not_float() {
        // 023
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_starts_with_period() {
        // .23
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_we_start_from_minus_and_nothing_follows() {
        // -
        panic!("TODO");
    }

    #[test]
    fn should_parse_positive_numbers() {
        // 2, 33, 444, 9999
        panic!("TODO");
    }

    #[test]
    fn should_parse_negative_numbers() {
        // -2, -33, -444, -9999
        panic!("TODO");
    }

    #[test]
    fn should_parse_positive_float_numbers() {
        // 2.0, 3.3, 4.44, 99.99, 0.234
        panic!("TODO");
    }

    #[test]
    fn should_parse_negative_float_numbers() {
        // -2.0, -3.3, -4.44, -99.99, -0.234
        panic!("TODO");
    }

    #[test]
    fn should_parse_numbers_correctly_that_are_separated() {
        // separated with space, multiple spaces, new lines, tabs
        panic!("TODO");
    }
}

// TODO:
// - [x] Migrate source code to vec<u8>
// - [x] Move messages to separate module
// - [x] Move message pring to a separate module
// - [x] Review number parsing
// - [x] Fix an issue with message print
// - [ ] Write tests for number parsing
