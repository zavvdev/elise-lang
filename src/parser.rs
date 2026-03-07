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
const T_COMMA: u8 = b',';
const T_DOUBLE_QT: u8 = b'"';

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

#[derive(Debug)]
enum FstNumState {
    Start,
    Sign,
    Zero,
    Int,
    Frac,
    Dot,
    Scient,
    ScientMinus,
    Expon,
}

#[derive(Debug, PartialEq)]
pub struct TokSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub struct Primitive {
    value: String,
    span: TokSpan,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Call {
        name: &'static str,
        span: TokSpan,
        arguments: Vec<Box<AstNode>>,
    },
    Number(Primitive),
    String(Primitive),
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
            } else if Self::string_is_start(&current_char) {
                ast.push(self.string_consume());
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

    fn number_crash_invalid(&self) -> ! {
        out::crash_at_token_pos(
            messages::M_NUMBER_INVALID,
            self.source_code,
            self.tok_pos,
            messages::M_PARSING_ERROR,
        );
    }

    fn number_consume(&mut self) -> AstNode {
        let mut state = FstNumState::Start;
        let tok_start = self.tok_pos;

        while let Some(c) = self.peek() {
            use FstNumState::*;

            state = match (&state, c) {
                (Start, T_MINUS) => {
                    self.advance();
                    Sign
                }
                (Sign | Start, b'0') => {
                    self.advance();
                    Zero
                }
                (Sign | Start, b'1'..=b'9') => {
                    self.advance();
                    Int
                }
                (Int, b'0'..=b'9') => {
                    self.advance();
                    Int
                }
                (Zero | Int, b'.') => {
                    self.advance();
                    Dot
                }
                (Dot | Frac, b'0'..=b'9') => {
                    self.advance();
                    Frac
                }
                (Zero | Int | Frac, b'e' | b'E') => {
                    self.advance();
                    Expon
                }
                (Expon, b'0'..=b'9') => {
                    self.advance();
                    Scient
                }
                (Expon, T_MINUS) => {
                    self.advance();
                    ScientMinus
                }
                (ScientMinus | Scient, b'0'..=b'9') => {
                    self.advance();
                    Scient
                }
                (_, c) if Self::number_is_end(&c) => break,
                _ => self.number_crash_invalid(),
            };
        }

        // Panic if we ended up with invalid state
        match state {
            FstNumState::Zero | FstNumState::Int | FstNumState::Frac | FstNumState::Scient => {}
            _ => self.number_crash_invalid(),
        }

        let tok_end = self.tok_pos;

        let value = from_utf8(&self.source_code[tok_start..tok_end])
            .unwrap_or_else(|_| self.number_crash_invalid());

        AstNode::Number(Primitive {
            value: value.to_string(),
            span: TokSpan {
                start: tok_start,
                end: tok_end,
            },
        })
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

    fn string_is_start(char: &u8) -> bool {
        *char == T_DOUBLE_QT
    }

    fn string_is_end(char: &u8) -> bool {
        *char == T_DOUBLE_QT
    }

    fn string_is_forbidden_char(char: &u8) -> bool {
        *char == b'\n'
    }

    fn string_crash_invalid(&self) -> ! {
        out::crash_at_token_pos(
            messages::M_STRING_INVALID,
            self.source_code,
            self.tok_pos,
            messages::M_PARSING_ERROR,
        );
    }

    fn string_consume(&mut self) -> AstNode {
        let tok_start = self.tok_pos;
        self.advance();

        while let Some(c) = self.peek() {
            if Self::string_is_end(&c) {
                self.advance();
                break;
            }
            if Self::string_is_forbidden_char(&c) {
                self.string_crash_invalid();
            }
            self.advance();
        }

        let value = from_utf8(&self.source_code[tok_start + 1..self.tok_pos - 1])
            .unwrap_or_else(|_| self.string_crash_invalid());
  
        // Taking surrogate pairs and other code points
        // that represent one lexeme into account.
        // We add 2 that represents quote start and end.
        let tok_end = tok_start + value.chars().count() + 2;

        AstNode::String(Primitive {
            value: value.to_string(),
            span: TokSpan {
                start: tok_start,
                end: tok_end,
            },
        })
    }

    // ==========================
    //
    // STRING END
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
        parser::{AstNode, Parser, Primitive, TokSpan},
    };

    // ==========================
    // NUMBER TESTS START
    // ==========================

    #[test]
    fn should_panic_if_number_contains_non_numeric_token() {
        let forbidded_tokens = vec![
            "0a", "-0a", "0.a", "-0.a", "1a", "1.a", "-1a", "-1.a", "12a2", "0.2a",
        ];

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
        let forbidded_tokens = vec!["02", "00"];

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
            ("0", 1),
            ("1", 1),
            ("2", 1),
            ("9", 1),
            ("123", 3),
            ("999999", 6),
            ("0.1", 3),
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
                AstNode::Number(Primitive {
                    value: number.to_string(),
                    span: TokSpan { start: 0, end },
                })
            );
        }
    }

    #[test]
    fn should_parse_negative_numbers() {
        let numbers = vec![
            ("-0", 2),
            ("-0.0", 4),
            ("-0.1", 4),
            ("-0.101", 6),
            ("-2", 2),
            ("-2.0", 4),
            ("-2.01", 5),
            ("-2.101", 6),
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
                AstNode::Number(Primitive {
                    value: number.to_string(),
                    span: TokSpan { start: 0, end },
                })
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
                AstNode::Number(Primitive {
                    value: "3".to_string(),
                    span: TokSpan { start: 0, end: 1 },
                }),
                AstNode::Number(Primitive {
                    value: "56".to_string(),
                    span: TokSpan { start: 2, end: 4 },
                }),
                AstNode::Number(Primitive {
                    value: "-9".to_string(),
                    span: TokSpan { start: 6, end: 8 },
                }),
                AstNode::Number(Primitive {
                    value: "3.2".to_string(),
                    span: TokSpan { start: 11, end: 14 },
                }),
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
            ("0e0", 3),
            ("-0e0", 4),
            ("-0e-0", 5),
            ("0e-0", 4),
            ("1e0", 3),
            ("1e-0", 4),
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
                AstNode::Number(Primitive {
                    value: number.to_string(),
                    span: TokSpan { start: 0, end },
                })
            );
        }
    }

    // ==========================
    // NUMBER TESTS FINISH
    // ==========================

    // ==========================
    // STRING TESTS START
    // ==========================

    #[test]
    fn should_panic_if_string_contains_new_line() {
        assert_panic!(
            {
                Parser::new(
                    "\"Hello
                    World\"",
                )
                .parse();
            },
            String,
            messages::M_PARSING_ERROR
        );
    }

    #[test]
    fn should_parse_string_correctly() {
        let strings = vec![
            ("\"\"", 2),
            ("\"Hello\"", 7),
            ("\"Hello World\"", 13),
            ("\"Hello       world!\"", 20),
            ("\"123 2323 😄😄\"", 13),
        ];
        for (string, end) in strings {
            let ast = Parser::new(string).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode::String(Primitive {
                    value: string
                        .split("\"")
                        .into_iter()
                        .collect::<Vec<&str>>()
                        .get(1)
                        .unwrap()
                        .to_string(),
                    span: TokSpan { start: 0, end },
                })
            );
        }
    }

    // ==========================
    // STRING TESTS END
    // ==========================
}

// ==========================
//
//  TESTS END
//
// ==========================
