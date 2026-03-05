use std::str::from_utf8;

use crate::{messages, out};

// ==========================
//
// TOKEN DEFINITIONS START
//
// ==========================

const T_FN_PREFIX: u8 = b'.';
const T_FN_DECLARE: &str = "declare";

const T_LEFT_PAREN: u8 = b'(';
const T_RIGHT_PAREN: u8 = b')';

const T_LEFT_SQR_BRACKET: u8 = b'[';
const T_RIGHT_SQR_BRACKET: u8 = b']';

const T_MINUS: u8 = b'-';
const T_PERIOD: u8 = b'.';
const T_COMMA: u8 = b',';

// ==========================
//
// TOKEN DEFINITIONS END
//
// ==========================

// ==========================
//
// CUSTOM TYPES START
//
// ==========================

type TNumber = f64;
type TString = String;

// ==========================
//
// CUSTOM TYPES END
//
// ==========================

// ==========================
//
//  PARSER START
//
// ==========================

#[derive(Debug, PartialEq)]
enum AstNodeValue {
    Function,
    Identifier,
    Number(TNumber),
    String(TString),
}

#[derive(Debug, PartialEq)]
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

    // ==========================
    //
    // TOKEN UTILITIES START
    //
    // ==========================

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

    // ==========================
    //
    // TOKEN UTILITIES END
    //
    // ==========================

    // ==========================
    //
    // COMMON UTILITIES START
    //
    // ==========================

    fn is_whitespace(c: &u8) -> bool {
        matches!(c, b' ' | b'\n' | b'\t' | b'\r')
    }

    // ==========================
    //
    // COMMON UTILITIES END
    //
    // ==========================

    // ==========================
    //
    // NUMBER START
    //
    // 1. Can start with: minus or digit
    // 2. Can contain: digit, only one dot if float, only one minus at the start
    // 3. Ends with: Whitespace-like, Comma, Right Paren, Right Sqr Br
    //
    // ==========================

    fn number_is_digit(c: &u8) -> bool {
        (b'0'..=b'9').contains(c)
    }

    fn number_is_start(c: &u8) -> bool {
        Self::number_is_digit(c) || *c == T_MINUS
    }

    fn number_is_end(c: &u8) -> bool {
        Self::is_whitespace(c) || *c == T_COMMA || *c == T_RIGHT_PAREN || *c == T_RIGHT_SQR_BRACKET
    }

    fn number_invalid(&self) -> ! {
        out::crash_at_token_pos(
            messages::M_INVALID_NUMBER,
            self.source_code,
            self.tok_pos,
            messages::M_PARSING_ERROR,
        );
    }

    fn number_consume(&mut self) -> AstNode {
        let mut value: Vec<u8> = vec![];
        let mut float = false;
        let tok_start = self.tok_pos;

        while let Some(c) = self.peek_at(self.tok_pos) {
            let next_tok = self.peek_at(self.tok_pos + 1);

            if Self::number_is_end(&c) {
                break;
            } else if c == b'0' && next_tok.is_some() && next_tok.unwrap() != T_PERIOD {
                self.number_invalid();
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
                self.number_invalid();
            }
        }

        let value = from_utf8(&value);

        if value.is_err() {
            self.number_invalid();
        }

        let numeric = value.unwrap().parse::<TNumber>();

        if numeric.is_err() {
            self.number_invalid();
        }

        AstNode {
            value: AstNodeValue::Number(numeric.unwrap()),
            tok_start,
            children: vec![],
        }
    }

    // ==========================
    //
    // NUMBER END
    //
    // ==========================
}

// ==========================
//
//  PARSER END
//
// ==========================

// ==========================
//
//  TESTS START
//
// ==========================

#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        messages,
        parser::{AstNode, AstNodeValue, Parser, TNumber},
    };

    // Number

    #[test]
    fn should_panic_if_number_contains_non_numeric_token() {
        let forbidded_tokens = vec!["1a", "12a2", "0.2a", "-1a"];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Parser::new(token).parse();
                },
                String,
                messages::M_PARSING_ERROR
            );
        }
    }

    #[test]
    fn should_panic_if_number_contains_more_than_one_minus_token() {
        let forbidded_tokens = vec!["--1", "-1-2", "-2-3-"];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Parser::new(token).parse();
                },
                String,
                messages::M_PARSING_ERROR
            );
        }
    }

    #[test]
    fn should_panic_if_number_contains_more_than_one_period_token() {
        let forbidded_tokens = vec!["0.2.3", "0.3."];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Parser::new(token).parse();
                },
                String,
                messages::M_PARSING_ERROR
            );
        }
    }

    #[test]
    fn should_panic_if_number_starts_with_zero_and_not_float() {
        assert_panic!(
            {
                Parser::new("02").parse();
            },
            String,
            messages::M_PARSING_ERROR
        );
    }

    #[test]
    fn should_panic_if_we_start_from_minus_and_nothing_follows() {
        assert_panic!(
            {
                Parser::new("-").parse();
            },
            String,
            messages::M_PARSING_ERROR
        );
    }

    #[test]
    fn should_parse_positive_numbers() {
        let numbers = vec!["2", "123", "999999", "2.3", "23.23", "0.23", "9999.9999"];
        for number in numbers {
            let ast = Parser::new(number).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode {
                    value: AstNodeValue::Number(number.parse::<TNumber>().unwrap()),
                    tok_start: 0,
                    children: vec![],
                }
            );
        }
    }

    #[test]
    fn should_parse_negative_numbers() {
        let numbers = vec![
            "-2",
            "-123",
            "-999999",
            "-2.3",
            "-23.23",
            "-0.23",
            "-9999.9999",
        ];
        for number in numbers {
            let ast = Parser::new(number).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode {
                    value: AstNodeValue::Number(number.parse::<TNumber>().unwrap()),
                    tok_start: 0,
                    children: vec![],
                }
            );
        }
    }

    #[test]
    fn should_parse_numbers_correctly_that_are_separated() {
        let ast = Parser::new(
            "3
56  -9   3.2",
        )
        .parse();
        assert_eq!(
            *ast,
            vec![
                AstNode {
                    value: AstNodeValue::Number(3 as TNumber),
                    tok_start: 0,
                    children: vec![],
                },
                AstNode {
                    value: AstNodeValue::Number(56 as TNumber),
                    tok_start: 2,
                    children: vec![],
                },
                AstNode {
                    value: AstNodeValue::Number(-9 as TNumber),
                    tok_start: 6,
                    children: vec![],
                },
                AstNode {
                    value: AstNodeValue::Number(3.2 as TNumber),
                    tok_start: 11,
                    children: vec![],
                }
            ]
        );
    }
}

// ==========================
//
//  TESTS END
//
// ==========================
