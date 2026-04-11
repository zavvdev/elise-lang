use crate::messages;
use regex::Regex;
use std::str::from_utf8;

use crate::config::{
    IDENTIFIER_REGEX, T_CALL_PREFIX, T_COMMA, T_DOUBLE_QT, T_LEFT_CUR_BRACKET, T_LEFT_PAREN,
    T_LEFT_SQR_BRACKET, T_MINUS, T_RIGHT_CUR_BRACKET, T_RIGHT_PAREN, T_RIGHT_SQR_BRACKET,
};

use elise_ast::{AstNode, Compound, Primitive, TokSpan};
use elise_shared::errors::{LangError, ParserError};

// ==========================
//
//  PARSER START
//
// ==========================

/**
 * Finite automata states for parsing numbers.
 */
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

/**
 * Since strings and chars in Rust are UTF-8 encoded,
 * which means that even if our char fits into 1 byte (ASCII)
 * we still have 4 bytes allocated for it. So we split our
 * source code input into raw bytes which takes less memory
 * space and compare tokens as bytes.
 * For preserving UTF-8 encodings for Elise Strings we just
 * slice string bytes and convert to UTF-8 that particular
 * slice of bytes.
 */
pub struct Prelude<'a> {
    source_code: &'a [u8],
    tok_pos: usize,
    // Track open and closed brackets via stack.
    depth_stack: Vec<u8>,
}

impl<'a> Prelude<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code: &source_code.as_bytes(),
            tok_pos: 0,
            depth_stack: vec![],
        }
    }

    /**
     * We do not use this method for recursive parsing. It should only
     * be used by the end user that wants to get the whole parsing result.
     */
    pub fn parse(&mut self) -> Result<Vec<AstNode>, LangError> {
        let mut ast: Vec<AstNode> = vec![];

        while let Some(c) = self.peek() {
            match self.get_node_from_char(&c) {
                Ok(node_option) => {
                    if let Some(node) = node_option {
                        ast.push(node);
                    }
                }
                Err(lang_error) => {
                    return Err(lang_error);
                }
            }
        }

        if self.depth_stack.len() > 0 {
            return Err(self.fail(messages::M_UNDEXPECTED_EOF));
        }

        Ok(ast)
    }

    fn fail(&self, msg: &'static str) -> LangError {
        LangError::Parser(ParserError {
            char_pos: self.tok_pos,
            source_code: self.source_code,
            message: msg,
        })
    }

    /**
     * We decomposed this function from parse method in order
     * to be able to parse recursively in a more convenient way
     * when we do not want to get Vec<AstNode> but we need
     * Vec<Box<AstNode>>. So we can loop, get node and compose
     * them in a way that we need.
     * For example, if we need to parse list of values, we can't
     * use parse method since it returns an array of nodes but
     * we need an array of pointers to the nodes.
     */
    fn get_node_from_char(&mut self, c: &u8) -> Result<Option<AstNode>, LangError> {
        if Self::is_separator(c) {
            self.advance();
            return Ok(None);
        } else if self.call_is_start(c) {
            return Ok(Some(self.call_consume()));
        } else if Self::number_is_start(c) {
            return Ok(Some(self.number_consume()));
        } else if Self::string_is_start(c) {
            return Ok(Some(self.string_consume()));
        } else if self.list_is_start(c) {
            return Ok(Some(self.list_consume()));
        } else if self.dict_is_start(c) {
            return Ok(Some(self.dict_consume()));

        // Matching identifier should be at the very end
        // since it matches any character.
        } else if Self::identifier_is_start(c) {
            return Ok(Some(self.identifier_consume()));
        } else {
            Err(self.fail(messages::M_TOKEN_UNEXPECTED))
        }
    }

    // ==========================
    //
    // TOKEN UTILITIES START
    //
    // ==========================

    fn peek_at(&self, pos: usize) -> Option<u8> {
        if pos >= self.source_code.len() {
            return None;
        }
        self.source_code.get(pos).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.peek_at(self.tok_pos)
    }

    fn advance(&mut self) -> Option<u8> {
        let tok = self.peek();
        self.tok_pos += 1;
        tok
    }

    fn is_separator(c: &u8) -> bool {
        matches!(c, b' ' | b'\n' | b'\t' | b'\r') || *c == T_COMMA
    }

    // ==========================
    //
    // TOKEN UTILITIES END
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
        Self::is_separator(c) || *c == T_RIGHT_PAREN || *c == T_RIGHT_SQR_BRACKET
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
                _ => self.crash(messages::M_NUMBER_INVALID),
            };
        }

        // Panic if we ended up with invalid state.
        match state {
            FstNumState::Zero | FstNumState::Int | FstNumState::Frac | FstNumState::Scient => {}
            _ => self.crash(messages::M_NUMBER_INVALID),
        }

        let tok_end = self.tok_pos;

        let value = from_utf8(&self.source_code[tok_start..tok_end])
            .unwrap_or_else(|_| self.crash(messages::M_NUMBER_INVALID));

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

    fn string_consume(&mut self) -> AstNode {
        let tok_start = self.tok_pos;
        self.advance();

        while let Some(c) = self.peek() {
            if Self::string_is_end(&c) {
                self.advance();
                break;
            }
            if Self::string_is_forbidden_char(&c) {
                self.crash(messages::M_STRING_INVALID);
            }
            self.advance();
        }

        let value = from_utf8(&self.source_code[tok_start + 1..self.tok_pos - 1])
            .unwrap_or_else(|_| self.crash(messages::M_STRING_INVALID));

        // Taking surrogate pairs and other code points
        // that represent one lexeme into account.
        // We add 2 in order to include quote start and end.
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

    // ==========================
    //
    // IDENTIFIER START
    //
    // ==========================

    fn identifier_is_start(c: &u8) -> bool {
        (*c >= b'a' && *c <= b'z') || (*c >= b'A' && *c <= b'Z')
    }

    fn identifier_is_end(c: &u8) -> bool {
        Self::is_separator(c) || *c == T_RIGHT_PAREN || *c == T_RIGHT_SQR_BRACKET
    }

    fn identifier_consume(&mut self) -> AstNode {
        let start = self.tok_pos;

        while let Some(c) = self.peek() {
            if Self::identifier_is_end(&c) {
                break;
            } else {
                self.advance();
            }
        }

        let value = from_utf8(&self.source_code[start..self.tok_pos])
            .unwrap()
            .to_string();

        let primitive = Primitive {
            value,
            span: TokSpan {
                start,
                end: self.tok_pos,
            },
        };

        match primitive.value.as_str() {
            // Identify known identifiers.
            T_TRUE | T_FALSE => AstNode::Bool(primitive),
            T_NULL => AstNode::Null(primitive),
            _ => {
                let re = Regex::new(IDENTIFIER_REGEX).unwrap();
                if re.is_match(&primitive.value) {
                    return AstNode::Identifier(primitive);
                } else {
                    self.crash(messages::M_TOKEN_UNEXPECTED);
                }
            }
        }
    }

    // ==========================
    //
    // IDENTIFIER END
    //
    // ==========================

    // ==========================
    //
    // LIST START
    //
    // ==========================

    fn list_is_start(&mut self, c: &u8) -> bool {
        if *c == T_LEFT_SQR_BRACKET {
            self.depth_stack.push(T_LEFT_SQR_BRACKET);
            return true;
        }
        false
    }

    fn list_is_end(&mut self, c: &u8) -> bool {
        if *c == T_RIGHT_SQR_BRACKET {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != T_LEFT_SQR_BRACKET {
                self.crash(messages::M_LIST_UNEXPECTED_END);
            }
            return true;
        }
        false
    }

    fn list_consume(&mut self) -> AstNode {
        let start = self.tok_pos;
        self.advance();
        let mut children: Vec<Box<AstNode>> = vec![];

        while let Some(c) = self.peek() {
            if self.list_is_end(&c) {
                self.advance();
                break;
            }
            if let Some(node) = self.get_node_from_char(&c) {
                children.push(Box::new(node));
            }
        }

        AstNode::List(Compound {
            span: TokSpan {
                start,
                end: self.tok_pos,
            },
            children,
        })
    }

    // ==========================
    //
    // LIST END
    //
    // ==========================

    // ==========================
    //
    // DICT START
    //
    // ==========================

    fn dict_is_start(&mut self, c: &u8) -> bool {
        if *c == T_LEFT_CUR_BRACKET {
            self.depth_stack.push(T_LEFT_CUR_BRACKET);
            return true;
        }
        false
    }

    fn dict_is_end(&mut self, c: &u8) -> bool {
        if *c == T_RIGHT_CUR_BRACKET {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != T_LEFT_CUR_BRACKET {
                self.crash(messages::M_DICT_UNEXPECTED_END);
            }
            return true;
        }
        false
    }

    fn dict_consume(&mut self) -> AstNode {
        let start = self.tok_pos;
        self.advance();

        let mut children: Vec<Box<AstNode>> = vec![];
        let mut key: Option<String> = None;

        while let Some(c) = self.peek() {
            if self.dict_is_end(&c) {
                if key.is_some() {
                    self.crash(messages::M_DICT_INVALID_PAIR);
                }
                self.advance();
                break;
            }
            if let Some(node) = self.get_node_from_char(&c) {
                if key.is_none() {
                    match node {
                        AstNode::String(primitive) => {
                            key = Some(primitive.value);
                        }
                        _ => {
                            self.crash(messages::M_DICT_UNEXPECTED_KEY);
                        }
                    }
                } else {
                    children.push(Box::new(AstNode::DictPair((
                        key.clone().unwrap(),
                        Box::new(node),
                    ))));
                    key = None;
                }
            }
        }

        AstNode::Dict(Compound {
            span: TokSpan {
                start,
                end: self.tok_pos,
            },
            children,
        })
    }

    // ==========================
    //
    // DICT END
    //
    // ==========================

    // ==========================
    //
    // CALL START
    //
    // ==========================

    fn call_is_start(&self, char: &u8) -> bool {
        let next_char = self.peek_at(self.tok_pos);
        *char == T_CALL_PREFIX && next_char.is_some() && !Self::is_separator(&next_char.unwrap())
    }

    fn call_is_end(&self, char: &u8) -> bool {
        *char == T_RIGHT_PAREN
    }

    fn call_is_name_valid(name: &str) -> bool {
        let re = Regex::new(IDENTIFIER_REGEX).unwrap();
        !name.is_empty() && re.is_match(name)
    }

    fn call_consume(&mut self) -> AstNode {
        let call_start = self.tok_pos;

        // Go to the start of the function name.
        self.advance();

        let call_name_start = self.tok_pos;

        while let Some(c) = self.peek() {
            if c == T_LEFT_PAREN {
                self.depth_stack.push(T_LEFT_PAREN);
                break;
            } else {
                self.advance();
            }
        }

        let call_name = from_utf8(&self.source_code[call_name_start..self.tok_pos]).unwrap();
        // Allow separators at the end for user preferences.
        let call_name = call_name.trim_end();

        if !Self::call_is_name_valid(call_name) {
            self.crash(messages::M_CALL_NAME_INVALID);
        }

        // Go to the next char after the function name.
        self.advance();

        let mut children = vec![];

        // Consume function arguments.
        while let Some(c) = self.peek() {
            if self.call_is_end(&c) {
                let last_entry = self.depth_stack.pop();
                if last_entry.is_none() || last_entry.unwrap() != T_LEFT_PAREN {
                    self.crash(messages::M_CALL_UNEXPECTED_END);
                }
                self.advance();
                break;
            }
            if let Some(node) = self.get_node_from_char(&c) {
                children.push(Box::new(node));
            }
        }

        let call_end = self.tok_pos;

        AstNode::Call((
            call_name.to_string(),
            Compound {
                span: TokSpan {
                    start: call_start,
                    end: call_end,
                },
                children,
            },
        ))
    }

    // ==========================
    //
    // CALL END
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
        parser::{AstNode, Compound, Prelude, Primitive, TokSpan},
    };

    // ==========================
    // NUMBER TESTS START
    // ==========================

    #[test]
    fn number_test_should_panic_if_contains_non_numeric_token() {
        let forbidded_tokens = vec![
            "0a", "-0a", "0.a", "-0.a", "1a", "1.a", "-1a", "-1.a", "12a2", "0.2a",
        ];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Prelude::new(token).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn number_test_should_panic_if_contains_more_than_one_minus_token() {
        let forbidded_tokens = vec!["--1", "-1-2", "-2-3-"];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Prelude::new(token).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn number_test_should_panic_if_contains_more_than_one_period_token() {
        let forbidded_tokens = vec!["0.2.3", "0.3."];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Prelude::new(token).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn number_test_should_panic_if_starts_with_zero_and_not_float() {
        let forbidded_tokens = vec!["02", "00"];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Prelude::new(token).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn number_test_should_panic_if_we_start_from_minus_and_nothing_follows() {
        assert_panic!(
            {
                Prelude::new("-").parse();
            },
            String,
            messages::M_PARSER_ERROR
        );
    }

    #[test]
    fn number_test_should_parse_positive_numbers() {
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
            let ast = Prelude::new(number).parse();
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
    fn number_test_should_parse_negative_numbers() {
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
            let ast = Prelude::new(number).parse();
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
    fn number_test_should_parse_numbers_correctly_that_are_separated() {
        let ast = Prelude::new(
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
    fn number_test_should_panic_if_scientific_notation_number_is_invalid() {
        let forbidded_tokens = vec!["1e1.2", "1e-", "1e"];

        for token in forbidded_tokens {
            assert_panic!(
                {
                    Prelude::new(token).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn number_test_should_parse_scientific_numbers_correctly() {
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
            let ast = Prelude::new(number).parse();
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
    // NUMBER TESTS END
    // ==========================

    // ==========================
    // STRING TESTS START
    // ==========================

    #[test]
    fn string_test_should_panic_if_contains_new_line() {
        assert_panic!(
            {
                Prelude::new(
                    "\"Hello
                    World\"",
                )
                .parse();
            },
            String,
            messages::M_PARSER_ERROR
        );
    }

    #[test]
    fn string_test_should_parse_correctly() {
        let strings = vec![
            ("\"\"", 2),
            ("\"Hello\"", 7),
            ("\"Hello World\"", 13),
            ("\"Hello       world!\"", 20),
            ("\"123 2323 😄😄\"", 13),
        ];
        for (string, end) in strings {
            let ast = Prelude::new(string).parse();
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

    // ==========================
    // BOOL TESTS START
    // ==========================

    #[test]
    fn bool_test_should_parse_true_correctly() {
        let ast = Prelude::new("true").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Bool(Primitive {
                value: "true".to_string(),
                span: TokSpan { start: 0, end: 4 }
            })
        )
    }

    #[test]
    fn bool_test_should_parse_false_correctly() {
        let ast = Prelude::new("false").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Bool(Primitive {
                value: "false".to_string(),
                span: TokSpan { start: 0, end: 5 }
            })
        )
    }

    // ==========================
    // BOOL TESTS END
    // ==========================

    // ==========================
    // NULL TESTS START
    // ==========================

    #[test]
    fn null_test_should_parse_null_correctly() {
        let ast = Prelude::new("null").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Null(Primitive {
                value: "null".to_string(),
                span: TokSpan { start: 0, end: 4 }
            })
        )
    }

    // ==========================
    // NULL TESTS END
    // ==========================

    // ==========================
    // IDENTIFIER TESTS START
    // ==========================

    #[test]
    fn identifier_test_should_reject_invalid_names() {
        let identifiers = vec![
            "1asd", "!asd", "@asd", "#asd", "$asd", "%asd", "^asd", "&asd", "*asd", "-asd", "_asd",
            "=asd", "+asd", "?asd", "?asd", ">asd", "<asd", "/asd",
        ];
        for identifier in identifiers {
            assert_panic!(
                {
                    Prelude::new(identifier).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn identifier_test_should_parse_correctly() {
        let identifiers = vec![
            ("asd", 3),
            ("asd?", 4),
            ("as?d", 4),
            ("as5?d", 5),
            ("asd-", 4),
            ("as-d", 4),
            ("asd!", 4),
            ("as!d", 4),
            ("asd_", 4),
        ];
        for (identifier, end) in identifiers {
            let ast = Prelude::new(identifier).parse();
            assert_eq!(
                *ast.get(0).unwrap(),
                AstNode::Identifier(Primitive {
                    value: identifier.to_string(),
                    span: TokSpan { start: 0, end },
                })
            );
        }
    }

    // ==========================
    // IDENTIFIER TESTS END
    // ==========================

    // ==========================
    // LIST TESTS START
    // ==========================

    #[test]
    fn list_test_should_parse_empty() {
        let ast = Prelude::new("[]").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::List(Compound {
                span: TokSpan { start: 0, end: 2 },
                children: vec![],
            })
        );
    }

    #[test]
    fn list_test_should_parse_nested_empty() {
        let ast = Prelude::new("[[]]").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::List(Compound {
                span: TokSpan { start: 0, end: 4 },
                children: vec![Box::new(AstNode::List(Compound {
                    span: TokSpan { start: 1, end: 3 },
                    children: vec![],
                }))],
            })
        );
    }

    #[test]
    fn list_test_should_parse_non_empty() {
        let ast = Prelude::new("[1, \"hello\", null, false]").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::List(Compound {
                span: TokSpan { start: 0, end: 25 },
                children: vec![
                    Box::new(AstNode::Number(Primitive {
                        value: "1".to_string(),
                        span: TokSpan { start: 1, end: 2 },
                    })),
                    Box::new(AstNode::String(Primitive {
                        value: "hello".to_string(),
                        span: TokSpan { start: 4, end: 11 },
                    })),
                    Box::new(AstNode::Null(Primitive {
                        value: "null".to_string(),
                        span: TokSpan { start: 13, end: 17 },
                    })),
                    Box::new(AstNode::Bool(Primitive {
                        value: "false".to_string(),
                        span: TokSpan { start: 19, end: 24 },
                    }))
                ],
            })
        );
    }

    #[test]
    fn list_test_should_panic_if_not_closed() {
        assert_panic!(
            {
                Prelude::new("[[1, 3]").parse();
            },
            String,
            messages::M_PARSER_ERROR
        );
    }

    // ==========================
    // LIST TESTS END
    // ==========================

    // ==========================
    // DICT TESTS START
    // ==========================

    #[test]
    fn dict_test_should_parse_empty() {
        let ast = Prelude::new("{}").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Dict(Compound {
                span: TokSpan { start: 0, end: 2 },
                children: vec![],
            })
        );
    }

    #[test]
    fn dict_test_should_parse_non_empty() {
        let ast = Prelude::new(
            "{ \"a\" 1, \"b\" \"2\", \"c\" false, \"d\" null, \"e\" [1, 2, 3], \"f\" { \"a2\" some_value } }",
        )
        .parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Dict(Compound {
                span: TokSpan { start: 0, end: 79 },
                children: vec![
                    Box::new(AstNode::DictPair((
                        "a".to_string(),
                        Box::new(AstNode::Number(Primitive {
                            value: "1".to_string(),
                            span: TokSpan { start: 6, end: 7 }
                        }))
                    ))),
                    Box::new(AstNode::DictPair((
                        "b".to_string(),
                        Box::new(AstNode::String(Primitive {
                            value: "2".to_string(),
                            span: TokSpan { start: 13, end: 16 }
                        }))
                    ))),
                    Box::new(AstNode::DictPair((
                        "c".to_string(),
                        Box::new(AstNode::Bool(Primitive {
                            value: "false".to_string(),
                            span: TokSpan { start: 22, end: 27 }
                        }))
                    ))),
                    Box::new(AstNode::DictPair((
                        "d".to_string(),
                        Box::new(AstNode::Null(Primitive {
                            value: "null".to_string(),
                            span: TokSpan { start: 33, end: 37 }
                        }))
                    ))),
                    Box::new(AstNode::DictPair((
                        "e".to_string(),
                        Box::new(AstNode::List(Compound {
                            span: TokSpan { start: 43, end: 52 },
                            children: vec![
                                Box::new(AstNode::Number(Primitive {
                                    value: "1".to_string(),
                                    span: TokSpan { start: 44, end: 45 }
                                })),
                                Box::new(AstNode::Number(Primitive {
                                    value: "2".to_string(),
                                    span: TokSpan { start: 47, end: 48 }
                                })),
                                Box::new(AstNode::Number(Primitive {
                                    value: "3".to_string(),
                                    span: TokSpan { start: 50, end: 51 }
                                }))
                            ]
                        }))
                    ))),
                    Box::new(AstNode::DictPair((
                        "f".to_string(),
                        Box::new(AstNode::Dict(Compound {
                            span: TokSpan { start: 58, end: 77 },
                            children: vec![Box::new(AstNode::DictPair((
                                "a2".to_string(),
                                Box::new(AstNode::Identifier(Primitive {
                                    value: "some_value".to_string(),
                                    span: TokSpan { start: 65, end: 75 }
                                }))
                            )))]
                        }))
                    )))
                ],
            })
        );
    }

    #[test]
    #[should_panic(expected = "Parser error")]
    fn dict_test_should_panic_if_pair_is_invalid() {
        Prelude::new("{ \"a\" 1, \"b\" }").parse();
    }

    #[test]
    fn dict_test_should_panic_if_key_is_invalid() {
        let inputs = vec![
            "{ a 1 }",
            "{ 1 \"2\" }",
            "{ null false }",
            "{ false true }",
            "{ [] \"`\" }",
            "{ {} a }",
        ];
        for input in inputs {
            assert_panic!(
                {
                    Prelude::new(input).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    fn dict_test_should_panic_if_not_closed_correctly() {
        let inputs = vec!["{ \"a\" 1 }}", "{{ \"1\" \"2\" }"];
        for input in inputs {
            assert_panic!(
                {
                    Prelude::new(input).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    // ==========================
    // DICT TESTS END
    // ==========================

    // ==========================
    // CALL TESTS START
    // ==========================

    #[test]
    fn call_test_should_parse_with_no_arguments() {
        let ast = Prelude::new(".some-fn()").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Call((
                "some-fn".to_string(),
                Compound {
                    span: TokSpan { start: 0, end: 10 },
                    children: vec![],
                }
            ))
        );
    }

    #[test]
    fn call_test_should_parse_with_arguments() {
        let ast = Prelude::new(".add(2 .div(4 2))").parse();
        assert_eq!(
            *ast.get(0).unwrap(),
            AstNode::Call((
                "add".to_string(),
                Compound {
                    span: TokSpan { start: 0, end: 17 },
                    children: vec![
                        Box::new(AstNode::Number(Primitive {
                            value: "2".to_string(),
                            span: TokSpan { start: 5, end: 6 }
                        })),
                        Box::new(AstNode::Call((
                            "div".to_string(),
                            Compound {
                                span: TokSpan { start: 7, end: 16 },
                                children: vec![
                                    Box::new(AstNode::Number(Primitive {
                                        value: "4".to_string(),
                                        span: TokSpan { start: 12, end: 13 }
                                    })),
                                    Box::new(AstNode::Number(Primitive {
                                        value: "2".to_string(),
                                        span: TokSpan { start: 14, end: 15 }
                                    }))
                                ]
                            }
                        )))
                    ],
                }
            ))
        );
    }

    #[test]
    fn call_test_should_parse_with_separators_after_name() {
        let inputs = vec![
            (".test ()", 8),
            (".test  ()", 9),
            (
                ".test
            ()",
                20,
            ),
            (
                ".test
                        ()",
                32,
            ),
        ];
        for (input, end) in inputs {
            assert_eq!(
                Prelude::new(input).parse(),
                vec![AstNode::Call((
                    "test".to_string(),
                    Compound {
                        span: TokSpan { start: 0, end },
                        children: vec![],
                    }
                ))]
            );
        }
    }

    #[test]
    #[should_panic(expected = "Parser error")]
    fn call_test_should_panic_if_not_closed_correctly() {
        Prelude::new(".some-fn(2 2 3))").parse();
    }

    #[test]
    #[should_panic(expected = "Parser error")]
    fn call_test_should_panic_if_separator_after_call_symbol() {
        Prelude::new(". some-fn()").parse();
    }

    #[test]
    fn call_test_should_reject_invalid_names() {
        let identifiers = vec![
            "1asd", "!asd", "@asd", "#asd", "$asd", "%asd", "^asd", "&asd", "*asd", "-asd", "_asd",
            "=asd", "+asd", "?asd", "?asd", ">asd", "<asd", "/asd",
        ];
        for identifier in identifiers {
            assert_panic!(
                {
                    Prelude::new(&format!(".{}()", identifier)).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    #[test]
    #[should_panic(expected = "Parser error")]
    fn call_test_should_panic_if_parens_are_standalone() {
        Prelude::new("()").parse();
    }

    // ==========================
    // CALL TESTS END
    // ==========================

    // ==========================
    // DEPTH TESTS START
    // ==========================

    #[test]
    fn depth_test_should_reject_invalid_depth() {
        let depth_cases = vec![
            ".a())",
            ".a(()",
            ".a().a()))",
            "()()))",
            "())",
            "(()",
            "[]]",
            "[][[][][]]][[",
            "[{}}]",
            "[{{{{}]",
            "[{{}]",
        ];
        for depth_case in depth_cases {
            assert_panic!(
                {
                    Prelude::new(depth_case).parse();
                },
                String,
                messages::M_PARSER_ERROR
            );
        }
    }

    // ==========================
    // DEPTH TESTS END
    // ==========================
}

// ==========================
//
//  TESTS END
//
// ==========================
