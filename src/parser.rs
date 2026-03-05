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

const T_EXPON_MARKER: u8 = b'e';
const T_EXPON_MARKER_UPP: u8 = b'E';

// ==========================
//
// TOKEN DEFINITIONS END
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
    // Storing numbers as string in order to not
    // care about overflows at this stage. We can
    // then decide which type is better for specific
    // numeric value at the bytecole level.
    Number(String),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct AstNodeSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct AstNode {
    value: AstNodeValue,
    span: AstNodeSpan,
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

        while let Some(current_char) = self.peek() {
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
        let tok = self.peek();
        self.tok_pos += 1;
        tok
    }

    fn peek(&self) -> Option<u8> {
        if self.tok_pos >= self.source_code.len() {
            return None;
        }
        self.source_code.get(self.tok_pos).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        let next_pos = self.tok_pos + 1;
        if next_pos >= self.source_code.len() {
            return None;
        }
        self.source_code.get(next_pos).copied()
    }

    fn peek_prev(&self) -> Option<u8> {
        if self.tok_pos > 0 {
            let next_pos = self.tok_pos - 1;
            if next_pos >= self.source_code.len() {
                return None;
            }
            return self.source_code.get(next_pos).copied();
        } else {
            return None;
        }
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

    fn number_is_expon_marker(c: &u8) -> bool {
        *c == T_EXPON_MARKER || *c == T_EXPON_MARKER_UPP
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
        let mut float = false;
        let mut negative = false;
        let mut is_start = true;
        let mut is_scientific = false;
        let tok_start = self.tok_pos;

        while let Some(c) = self.peek() {
            let next_tok = self.peek_next();
            let prev_tok = self.peek_prev();

            if Self::number_is_end(&c) {
                break;
            // Do not allow 0 at the beginning
            } else if c == b'0'
                && is_start
                && next_tok.is_some()
                && Self::number_is_digit(&next_tok.unwrap())
            {
                self.number_invalid();
            // Always consume numeric value
            } else if Self::number_is_digit(&c) {
                is_start = false;
                self.advance();
            // Allow exponent if it's not a beginning and
            // next char is either digit or minus
            } else if Self::number_is_expon_marker(&c)
                && !is_start
                && !is_scientific
                && next_tok.is_some()
                && (Self::number_is_digit(&next_tok.unwrap()) || next_tok.unwrap() == T_MINUS)
            {
                is_scientific = true;
                self.advance();
            // Allow minus for negative numbers
            } else if c == T_MINUS
                && !negative
                && is_start
                && next_tok.is_some()
                && Self::number_is_digit(&next_tok.unwrap())
            {
                negative = true;
                is_start = false;
                self.advance();
            // Allow minus for scientific notation
            } else if c == T_MINUS
                && is_scientific
                && next_tok.is_some()
                && Self::number_is_digit(&next_tok.unwrap())
            {
                is_start = false;
                self.advance();
            // Allow period for floats
            } else if c == T_PERIOD
                && prev_tok.is_some()
                && Self::number_is_digit(&prev_tok.unwrap())
                && !is_scientific
                && !float
            {
                float = true;
                is_start = false;
                self.advance();
            } else {
                self.number_invalid();
            }
        }

        let tok_end = self.tok_pos;

        let value = from_utf8(&self.source_code[tok_start..tok_end])
            .unwrap_or_else(|_| self.number_invalid());

        AstNode {
            // allocate string in order to own value in AstNode
            value: AstNodeValue::Number(value.to_string()),
            span: AstNodeSpan {
                start: tok_start,
                end: tok_end,
            },
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
        parser::{AstNode, AstNodeSpan, AstNodeValue, Parser},
    };

    // ==========================
    // NUMBER TESTS START
    // ==========================

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
        let numbers = vec![
            ("2", 1),
            ("0", 1),
            ("123", 3),
            ("999999", 6),
            ("2.3", 3),
            ("23.23", 5),
            ("0.23", 4),
            ("9999.9999", 9),
            ("101", 3),
        ];
        for (number, end) in numbers {
            let ast = Parser::new(number).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode {
                    value: AstNodeValue::Number(number.to_string()),
                    span: AstNodeSpan { start: 0, end },
                    children: vec![],
                }
            );
        }
    }

    #[test]
    fn should_parse_negative_numbers() {
        let numbers = vec![
            ("-2", 2),
            ("-0", 2),
            ("-123", 4),
            ("-999999", 7),
            ("-2.3", 4),
            ("-23.23", 6),
            ("-0.23", 5),
            ("-9999.9999", 10),
            ("-101", 4),
        ];
        for (number, end) in numbers {
            let ast = Parser::new(number).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode {
                    value: AstNodeValue::Number(number.to_string()),
                    span: AstNodeSpan { start: 0, end },
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
                    value: AstNodeValue::Number("3".to_string()),
                    span: AstNodeSpan { start: 0, end: 1 },
                    children: vec![],
                },
                AstNode {
                    value: AstNodeValue::Number("56".to_string()),
                    span: AstNodeSpan { start: 2, end: 4 },
                    children: vec![],
                },
                AstNode {
                    value: AstNodeValue::Number("-9".to_string()),
                    span: AstNodeSpan { start: 6, end: 8 },
                    children: vec![],
                },
                AstNode {
                    value: AstNodeValue::Number("3.2".to_string()),
                    span: AstNodeSpan { start: 11, end: 14 },
                    children: vec![],
                }
            ]
        );
    }

    #[test]
    fn should_panic_if_scientific_notation_number_is_invalid() {
        let forbidded_tokens = vec!["1e1.2", "1e-", "1e"];

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
    fn should_parse_scientific_numbers_correctly() {
        let numbers = vec![
            ("1e3", 3),
            ("10e3", 4),
            ("102e302", 7),
            ("1E3", 3),
            ("1e-3", 4),
            ("10e-30", 6),
            ("102e-304", 8),
            ("1.5e10", 6),
            ("1.504e101", 9),
            ("-2.3e-5", 7),
            ("-2.30e-502", 10),
        ];
        for (number, end) in numbers {
            let ast = Parser::new(number).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode {
                    value: AstNodeValue::Number(number.to_string()),
                    span: AstNodeSpan { start: 0, end },
                    children: vec![],
                }
            );
        }
    }

    // ==========================
    // NUMBER TESTS FINISH
    // ==========================
}

// ==========================
//
//  TESTS END
//
// ==========================

// TODO:
// - [x] Add Span for AstNode instead of tok_start
// - [x] Remove vec! allocation for number parsing and use slice
// - [x] Store number as string in AST instead of f64
// - [x] Add support for numbers with scientific notation (1e10, 2e-10)
//      Valid: 1e3, 1E3, 1e-3, 1.5e10, -2.3e-5
//      Invalid: 1e1.2, 1e-, 1e
// - [x] Add tests for scientific number notation parsing
// - [ ] Improve number parsing function
// - [ ] Review AstNode design
