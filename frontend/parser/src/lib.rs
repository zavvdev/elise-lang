pub mod parser_config;

use elise_shared_types::Span;
use std::str::from_utf8;

use crate::parser_config::{
    L_CALL_PREFIX, L_COMMA, L_DOUBLE_QT, L_FALSE, L_LEFT_CUR_BRACKET, L_LEFT_PAREN,
    L_LEFT_SQR_BRACKET, L_MINUS, L_NULL, L_RIGHT_CUR_BRACKET, L_RIGHT_PAREN, L_RIGHT_SQR_BRACKET,
    L_SLOT_PREFIX, L_TRUE,
};

use elise_ast::{AstCallKind, AstCompound, AstKeyValuePair, AstNode, AstPrimitive};
use elise_shared_errors::errors_parser::{ParserErr, ParserErrInfo};

// ==================================================================
//
//  PARSER START
//
// ==================================================================

/// Deterministic Finite Automata states for parsing numbers.
#[derive(Debug)]
enum DfaNumState {
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

pub struct Prelude<'a> {
    // We expect to have a single char per byte (ASCII) for the
    // language itself. String literals should support UTF-8 and
    // they must be converted using `from_utf8` in place.
    source_code: &'a [u8],
    tok_pos: usize,
    // Track open and closed brackets via stack.
    depth_stack: Vec<u8>,
}

// Top-Down, Leftmost, Non-backtracking, Recursive
impl<'a> Prelude<'a> {
    pub fn new(source_code: &'a [u8]) -> Self {
        Self {
            source_code,
            tok_pos: 0,
            // For tracking open/closed parens.
            depth_stack: vec![],
        }
    }

    /// We do not use this method for recursive parsing. It should only
    /// be used by the end user that wants to get the whole parsing result.
    pub fn parse(&mut self) -> Result<Vec<AstNode>, ParserErr> {
        let mut ast: Vec<AstNode> = vec![];

        while let Some(c) = self.peek() {
            if let Some(node) = self.get_node_from_char(&c)? {
                ast.push(node);
            }
        }

        if !self.depth_stack.is_empty() {
            return Err(self.fail(ParserErr::UnexpEoFile));
        }

        Ok(ast)
    }

    fn fail(&self, variant: fn(ParserErrInfo) -> ParserErr) -> ParserErr {
        variant(ParserErrInfo { pos: self.tok_pos })
    }

    /// This function was decomposed from parse function in order
    /// to be able to handle AstNode differently in some cases.
    fn get_node_from_char(&mut self, c: &u8) -> Result<Option<AstNode>, ParserErr> {
        if Self::is_separator(c) {
            self.advance();
            Ok(None)
        } else if self.call_is_start(c) {
            self.call_consume()
        } else if self.slot_is_start(c) {
            self.slot_consume()
        } else if Self::number_is_start(c) {
            self.number_consume()
        } else if Self::string_is_start(c) {
            self.string_consume()
        } else if self.list_is_start(c) {
            self.list_consume()
        } else if self.dict_is_start(c) {
            self.dict_consume()

        // Matching identifier should be at the very end
        // since it matches any character.
        } else if Self::identifier_is_start(c) {
            self.identifier_consume()
        } else {
            Err(self.fail(ParserErr::UnexpTok))
        }
    }

    // ==================================================================
    // TOKEN UTILITIES START
    // ==================================================================

    fn peek(&self) -> Option<u8> {
        if self.tok_pos >= self.source_code.len() {
            return None;
        }
        self.source_code.get(self.tok_pos).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let tok = self.peek();
        self.tok_pos += 1;
        tok
    }

    fn is_separator(c: &u8) -> bool {
        matches!(c, b' ' | b'\n' | b'\t' | b'\r') || *c == L_COMMA
    }

    // ==================================================================
    // TOKEN UTILITIES END
    // ==================================================================

    // ==================================================================
    // NUMBER START
    // ==================================================================

    fn number_is_digit(c: &u8) -> bool {
        c.is_ascii_digit()
    }

    fn number_is_start(c: &u8) -> bool {
        Self::number_is_digit(c) || *c == L_MINUS
    }

    fn number_is_end(c: &u8) -> bool {
        Self::is_separator(c) || *c == L_RIGHT_PAREN || *c == L_RIGHT_SQR_BRACKET
    }

    fn number_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let mut state = DfaNumState::Start;
        let tok_start = self.tok_pos;

        while let Some(c) = self.peek() {
            use DfaNumState::*;

            state = match (&state, c) {
                (Start, L_MINUS) => {
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
                (Expon, L_MINUS) => {
                    self.advance();
                    ScientMinus
                }
                (ScientMinus | Scient, b'0'..=b'9') => {
                    self.advance();
                    Scient
                }
                (_, c) if Self::number_is_end(&c) => break,
                _ => {
                    return Err(self.fail(ParserErr::InvalNum));
                }
            };
        }

        // Return an error we ended up with invalid state.
        match state {
            DfaNumState::Zero | DfaNumState::Int | DfaNumState::Frac | DfaNumState::Scient => {}
            _ => {
                return Err(self.fail(ParserErr::InvalNum));
            }
        }

        let tok_end = self.tok_pos;
        let value = from_utf8(&self.source_code[tok_start..tok_end]);

        if value.is_err() {
            return Err(self.fail(ParserErr::InvalNum));
        }

        Ok(Some(AstNode::Number(AstPrimitive {
            value: value.unwrap().to_string(),
            span: Span {
                start: tok_start,
                end: tok_end,
            },
        })))
    }

    // ==================================================================
    // NUMBER END
    // ==================================================================

    // ==================================================================
    // STRING START
    // ==================================================================

    fn string_is_start(char: &u8) -> bool {
        *char == L_DOUBLE_QT
    }

    fn string_is_end(char: &u8) -> bool {
        *char == L_DOUBLE_QT
    }

    fn string_is_forbidden_char(char: &u8) -> bool {
        *char == b'\n' || *char == b'\r'
    }

    fn string_is_escape(char: &u8) -> bool {
        *char == b'\\'
    }

    fn string_decode_escape(char: Option<u8>) -> Option<u8> {
        let c = char?;
        Some(match c {
            b'\\' => b'\\',
            b'r' => b'\r',
            b'n' => b'\n',
            b't' => b'\t',
            b'0' => b'\0',
            b'"' => b'"',
            any => any,
        })
    }

    /// Consumes a string literal preserving UTF-8 encoding.
    /// Regardless of the contents, Span will always point
    /// to the start and end position of bytes instead of
    /// encoded characters.
    fn string_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        // Capture start pos before advancing forward in order to
        // construct valid Span.
        let start = self.tok_pos;
        let mut closed = false;
        let mut slice: Vec<u8> = vec![];
        // Skip open quotes.
        self.advance();

        while let Some(c) = self.peek() {
            let mut next_byte = c;

            if Self::string_is_end(&c) {
                closed = true;
                self.advance();
                break;
            }
            if Self::string_is_forbidden_char(&c) {
                return Err(self.fail(ParserErr::InvalStr));
            }
            if Self::string_is_escape(&c) {
                // Skip escaping back slash.
                self.advance();
                if let Some(esc) = Self::string_decode_escape(self.peek()) {
                    next_byte = esc;
                }
            }
            slice.push(next_byte);
            self.advance();
        }

        if !closed {
            return Err(self.fail(ParserErr::UntermStr));
        }

        let end = self.tok_pos;

        // Preserve UTF-8 encoding for string.
        let value = std::str::from_utf8(&slice).map_err(|_| self.fail(ParserErr::InvalStr))?;

        Ok(Some(AstNode::String(AstPrimitive {
            value: value.to_owned(),
            span: Span { start, end },
        })))
    }

    // ==================================================================
    // STRING END
    // ==================================================================

    // ==================================================================
    // IDENTIFIER START
    // ==================================================================

    fn identifier_is_start(c: &u8) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase()
    }

    fn identifier_is_end(c: &u8) -> bool {
        Self::is_separator(c) || *c == L_RIGHT_PAREN || *c == L_RIGHT_SQR_BRACKET
    }

    fn identifier_is_valid(s: &str) -> bool {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) => {
                c.is_ascii_alphabetic()
                    && chars
                        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '?' | '!' | '_'))
            }
            None => false,
        }
    }

    fn identifier_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
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

        let primitive = AstPrimitive {
            value,
            span: Span {
                start,
                end: self.tok_pos,
            },
        };

        match primitive.value.as_str() {
            // Identify known identifiers.
            L_TRUE | L_FALSE => Ok(Some(AstNode::Bool(primitive))),
            L_NULL => Ok(Some(AstNode::Null(primitive))),
            _ => {
                if Self::identifier_is_valid(&primitive.value) {
                    Ok(Some(AstNode::Identifier(primitive)))
                } else {
                    Err(self.fail(ParserErr::UnexpTok))
                }
            }
        }
    }

    // ==================================================================
    // IDENTIFIER END
    // ==================================================================

    // ==================================================================
    // LIST START
    // ==================================================================

    fn list_is_start(&mut self, c: &u8) -> bool {
        if *c == L_LEFT_SQR_BRACKET {
            self.depth_stack.push(L_LEFT_SQR_BRACKET);
            return true;
        }
        false
    }

    fn list_check_end(&mut self, c: &u8) -> Result<bool, ()> {
        if *c == L_RIGHT_SQR_BRACKET {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != L_LEFT_SQR_BRACKET {
                return Err(());
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn list_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let start = self.tok_pos;
        self.advance();
        let mut children: Vec<Box<AstNode>> = vec![];

        while let Some(c) = self.peek() {
            if let Ok(eo_list) = self.list_check_end(&c) {
                if eo_list {
                    self.advance();
                    break;
                }
                if let Some(node) = self.get_node_from_char(&c)? {
                    children.push(Box::new(node));
                }
            } else {
                return Err(self.fail(ParserErr::UnexpEoList));
            }
        }

        Ok(Some(AstNode::List(AstCompound {
            span: Span {
                start,
                end: self.tok_pos,
            },
            children,
        })))
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // DICT START
    // ==================================================================

    fn dict_is_start(&mut self, c: &u8) -> bool {
        if *c == L_LEFT_CUR_BRACKET {
            self.depth_stack.push(L_LEFT_CUR_BRACKET);
            return true;
        }
        false
    }

    fn dict_check_end(&mut self, c: &u8) -> Result<bool, ()> {
        if *c == L_RIGHT_CUR_BRACKET {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != L_LEFT_CUR_BRACKET {
                return Err(());
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn dict_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let start = self.tok_pos;
        self.advance();

        let mut children: Vec<Box<AstNode>> = vec![];
        let mut key: Option<String> = None;
        let mut key_start = 0;
        let mut key_end = 0;

        while let Some(c) = self.peek() {
            if let Ok(eo_dict) = self.dict_check_end(&c) {
                if eo_dict {
                    if key.is_some() {
                        return Err(self.fail(ParserErr::InvalDictPair));
                    }
                    self.advance();
                    break;
                }
            } else {
                return Err(self.fail(ParserErr::UnexpEoDict));
            }

            if let Some(node) = self.get_node_from_char(&c)? {
                if key.is_none() {
                    match node {
                        AstNode::String(primitive) => {
                            key_start = primitive.span.start;
                            key_end = primitive.span.end;
                            key = Some(primitive.value);
                        }
                        _ => {
                            return Err(self.fail(ParserErr::UnexpDictKey));
                        }
                    }
                } else {
                    let pair_end = node.span().end;
                    children.push(Box::new(AstNode::DictPair(AstKeyValuePair {
                        key: key.clone().unwrap(),
                        key_span: Span {
                            start: key_start,
                            end: key_end,
                        },
                        value: Box::new(node),
                        span: Span {
                            start: key_start,
                            end: pair_end,
                        },
                    })));
                    key = None;
                    key_start = 0;
                    key_end = 0;
                }
            }
        }

        Ok(Some(AstNode::Dict(AstCompound {
            span: Span {
                start,
                end: self.tok_pos,
            },
            children,
        })))
    }

    // ==================================================================
    // DICT END
    // ==================================================================

    // ==================================================================
    // CALL START
    // ==================================================================

    fn call_is_start(&self, char: &u8) -> bool {
        if let Some(next_char) = self.peek() {
            return *char == L_CALL_PREFIX && !Self::is_separator(&next_char);
        }
        false
    }

    fn call_is_end(&self, char: &u8) -> bool {
        *char == L_RIGHT_PAREN
    }

    fn call_validate_name(&self, name: &str) -> Result<AstCallKind, ParserErr> {
        // Anonymous function
        if name.is_empty() {
            return Ok(AstCallKind::Anon);
        }
        if !name.is_empty() && Self::identifier_is_valid(name) {
            Ok(AstCallKind::Named(name.to_string()))
        } else {
            Err(self.fail(ParserErr::InvalFnName))
        }
    }

    fn call_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let call_start = self.tok_pos;

        // Go to the start of the function name.
        self.advance();

        let call_name_start = self.tok_pos;

        while let Some(c) = self.peek() {
            if c == L_LEFT_PAREN {
                self.depth_stack.push(L_LEFT_PAREN);
                break;
            } else {
                self.advance();
            }
        }

        let call_name = from_utf8(&self.source_code[call_name_start..self.tok_pos]).unwrap();
        // Allow separators at the end for user preferences.
        let call_name = self.call_validate_name(call_name.trim_end())?;

        // Go to the next char after the function name.
        self.advance();

        let mut children = vec![];

        // Consume function arguments.
        while let Some(c) = self.peek() {
            if self.call_is_end(&c) {
                let last_entry = self.depth_stack.pop();
                if last_entry.is_none() || last_entry.unwrap() != L_LEFT_PAREN {
                    return Err(self.fail(ParserErr::UnexpEoFn));
                }
                self.advance();
                break;
            }

            if let Some(node) = self.get_node_from_char(&c)? {
                children.push(Box::new(node));
            }
        }

        let call_end = self.tok_pos;

        Ok(Some(AstNode::Call((
            call_name,
            AstCompound {
                span: Span {
                    start: call_start,
                    end: call_end,
                },
                children,
            },
        ))))
    }

    // ==================================================================
    // CALL END
    // ==================================================================

    // ==================================================================
    // SLOT START
    // ==================================================================

    fn slot_is_start(&self, char: &u8) -> bool {
        if let Some(next_char) = self.peek() {
            return *char == L_SLOT_PREFIX && !Self::is_separator(&next_char);
        }
        false
    }

    fn slot_is_end(c: &u8) -> bool {
        Self::is_separator(c) || *c == L_RIGHT_PAREN || *c == L_RIGHT_SQR_BRACKET
    }

    fn slot_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let start = self.tok_pos;

        // Exclude slot prefix.
        self.advance();

        let slot_name_start = self.tok_pos;

        while let Some(c) = self.peek() {
            if Self::slot_is_end(&c) {
                break;
            } else {
                self.advance();
            }
        }

        let value = from_utf8(&self.source_code[slot_name_start..self.tok_pos])
            .unwrap()
            .to_string();

        if Self::identifier_is_valid(&value) {
            Ok(Some(AstNode::Slot(AstPrimitive {
                value,
                span: Span {
                    start,
                    end: self.tok_pos,
                },
            })))
        } else {
            Err(self.fail(ParserErr::UnexpTok))
        }
    }

    // ==================================================================
    // SLOT END
    // ==================================================================
}

// ==================================================================
//
//  PARSER END
//
// ==================================================================

// ==================================================================
//
//  TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use crate::{AstCallKind, AstCompound, AstNode, AstPrimitive, Prelude, Span};
    use elise_ast::AstKeyValuePair;
    use elise_shared_errors::errors_parser::{ParserErr, ParserErrInfo};

    // ==================================================================
    // NUMBER TESTS START
    // ==================================================================

    #[test]
    fn number_should_not_contain_non_numeric_tokens() {
        let forbidded_tokens = vec![
            ("0a", 1),
            ("-0a", 2),
            ("0.a", 2),
            ("-0.a", 3),
            ("1a", 1),
            ("1.a", 2),
            ("-1a", 2),
            ("-1.a", 3),
            ("12a2", 2),
            ("0.2a", 3),
        ];

        for (token, pos) in forbidded_tokens {
            assert_eq!(
                Prelude::new(token.as_bytes()).parse(),
                Err(ParserErr::InvalNum(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn number_should_not_allow_more_than_one_minus_token() {
        let forbidded_tokens = vec![("--1", 1), ("-1-2", 2), ("-2-3-", 2)];

        for (token, pos) in forbidded_tokens {
            assert_eq!(
                Prelude::new(token.as_bytes()).parse(),
                Err(ParserErr::InvalNum(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn number_should_not_allow_more_than_one_period_token() {
        let forbidded_tokens = vec![("0.2.3", 3), ("0.3.", 3)];

        for (token, pos) in forbidded_tokens {
            assert_eq!(
                Prelude::new(token.as_bytes()).parse(),
                Err(ParserErr::InvalNum(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn number_should_not_allow_start_with_zero_which_not_float() {
        let forbidded_tokens = vec![("02", 1), ("00", 1)];

        for (token, pos) in forbidded_tokens {
            assert_eq!(
                Prelude::new(token.as_bytes()).parse(),
                Err(ParserErr::InvalNum(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn number_should_not_allow_start_from_minus_if_nothing_follows() {
        let code = "-".to_string();
        assert_eq!(
            Prelude::new(&code.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos: 1 }))
        );
    }

    #[test]
    fn number_should_parse_positive_numbers() {
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
            let ast = Prelude::new(number.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Number(AstPrimitive {
                    value: number.to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    #[test]
    fn number_should_parse_negative_numbers() {
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
            let ast = Prelude::new(number.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Number(AstPrimitive {
                    value: number.to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    #[test]
    fn number_should_parse_numbers_that_are_separated() {
        let ast = Prelude::new(
            "3
56  -9   3.2"
                .as_bytes(),
        )
        .parse();
        assert_eq!(
            ast,
            Ok(vec![
                AstNode::Number(AstPrimitive {
                    value: "3".to_string(),
                    span: Span { start: 0, end: 1 },
                }),
                AstNode::Number(AstPrimitive {
                    value: "56".to_string(),
                    span: Span { start: 2, end: 4 },
                }),
                AstNode::Number(AstPrimitive {
                    value: "-9".to_string(),
                    span: Span { start: 6, end: 8 },
                }),
                AstNode::Number(AstPrimitive {
                    value: "3.2".to_string(),
                    span: Span { start: 11, end: 14 },
                }),
            ])
        );
    }

    #[test]
    fn number_should_not_allow_invalid_scientific_notation_numbers() {
        let forbidded_tokens = vec![("1e1.2", 3), ("1e-", 3), ("1e", 2)];

        for (token, pos) in forbidded_tokens {
            assert_eq!(
                Prelude::new(token.as_bytes()).parse(),
                Err(ParserErr::InvalNum(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn number_should_parse_scientific_numbers() {
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
            let ast = Prelude::new(number.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Number(AstPrimitive {
                    value: number.to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    // ==================================================================
    // NUMBER TESTS END
    // ==================================================================

    // ==================================================================
    // STRING TESTS START
    // ==================================================================

    #[test]
    fn string_should_not_allow_new_line() {
        assert_eq!(
            Prelude::new(
                r#""Hello
            World""#
                    .as_bytes()
            )
            .parse(),
            Err(ParserErr::InvalStr(ParserErrInfo { pos: 6 }))
        );
    }

    #[test]
    fn string_should_not_allow_unterminated() {
        let strings = vec![(r#""Hello"#, 6), (r#""Hello\""#, 8)];
        for (string, end) in strings {
            assert_eq!(
                Prelude::new(string.as_bytes()).parse(),
                Err(ParserErr::UntermStr(ParserErrInfo { pos: end }))
            );
        }
    }

    #[test]
    fn string_should_parse() {
        let strings = vec![
            (r#""""#, 2),
            (r#""Hello""#, 7),
            (r#""Hello World""#, 13),
            (r#""Hello       world!""#, 20),
            // Span is always bytes aware.
            // Each of these emojis are 4 bytes.
            (r#""123 2323 😄😄""#, 19),
        ];
        for (string, end) in strings {
            let ast = Prelude::new(string.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::String(AstPrimitive {
                    value: string
                        .split("\"")
                        .into_iter()
                        .collect::<Vec<&str>>()
                        .get(1)
                        .unwrap()
                        .to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    #[test]
    fn string_should_parse_escape_chars() {
        let strings = vec![
            (r#""\"""#, "\"", 4),
            (r#""Hello\r""#, "Hello\r", 9),
            (r#""Hello\n""#, "Hello\n", 9),
            (r#""Hello\0""#, "Hello\0", 9),
            (r#""Hello\\""#, "Hello\\", 9),
            (r#""Hello\tworld!""#, "Hello\tworld!", 15),
            (r#""\y""#, "y", 4),
        ];
        for (string, expected, end) in strings {
            let ast = Prelude::new(string.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::String(AstPrimitive {
                    value: expected.to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    // ==================================================================
    // STRING TESTS END
    // ==================================================================

    // ==================================================================
    // BOOL TESTS START
    // ==================================================================

    #[test]
    fn bool_should_parse_true() {
        let ast = Prelude::new("true".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Bool(AstPrimitive {
                value: "true".to_string(),
                span: Span { start: 0, end: 4 }
            })])
        )
    }

    #[test]
    fn bool_should_parse_false() {
        let ast = Prelude::new("false".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Bool(AstPrimitive {
                value: "false".to_string(),
                span: Span { start: 0, end: 5 }
            })])
        )
    }

    // ==================================================================
    // BOOL TESTS END
    // ==================================================================

    // ==================================================================
    // NULL TESTS START
    // ==================================================================

    #[test]
    fn null_should_parse() {
        let ast = Prelude::new("null".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Null(AstPrimitive {
                value: "null".to_string(),
                span: Span { start: 0, end: 4 }
            })])
        )
    }

    // ==================================================================
    // NULL TESTS END
    // ==================================================================

    // ==================================================================
    // IDENTIFIER TESTS START
    // ==================================================================

    #[test]
    fn identifier_should_reject_invalid_names() {
        let identifiers: Vec<(&str, usize, fn(ParserErrInfo) -> ParserErr)> = vec![
            ("1asd", 1, ParserErr::InvalNum),
            ("!asd", 0, ParserErr::UnexpTok),
            ("#asd", 0, ParserErr::UnexpTok),
            ("$asd", 0, ParserErr::UnexpTok),
            ("%asd", 0, ParserErr::UnexpTok),
            ("^asd", 0, ParserErr::UnexpTok),
            ("&asd", 0, ParserErr::UnexpTok),
            ("*asd", 0, ParserErr::UnexpTok),
            ("-asd", 1, ParserErr::InvalNum),
            ("_asd", 0, ParserErr::UnexpTok),
            ("=asd", 0, ParserErr::UnexpTok),
            ("+asd", 0, ParserErr::UnexpTok),
            ("?asd", 0, ParserErr::UnexpTok),
            ("?asd", 0, ParserErr::UnexpTok),
            (">asd", 0, ParserErr::UnexpTok),
            ("<asd", 0, ParserErr::UnexpTok),
            ("/asd", 0, ParserErr::UnexpTok),
        ];
        for (identifier, pos, err) in identifiers {
            assert_eq!(
                Prelude::new(identifier.as_bytes()).parse(),
                Err(err(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn identifier_should_parse() {
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
            let ast = Prelude::new(identifier.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Identifier(AstPrimitive {
                    value: identifier.to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    // ==================================================================
    // IDENTIFIER TESTS END
    // ==================================================================

    // ==================================================================
    // LIST TESTS START
    // ==================================================================

    #[test]
    fn list_should_parse_empty() {
        let ast = Prelude::new("[]".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::List(AstCompound {
                span: Span { start: 0, end: 2 },
                children: vec![],
            })])
        );
    }

    #[test]
    fn list_should_parse_nested_empty() {
        let ast = Prelude::new("[[]]".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::List(AstCompound {
                span: Span { start: 0, end: 4 },
                children: vec![Box::new(AstNode::List(AstCompound {
                    span: Span { start: 1, end: 3 },
                    children: vec![],
                }))],
            })])
        );
    }

    #[test]
    fn list_should_parse_non_empty() {
        let ast = Prelude::new("[1, \"hello\", null, false]".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::List(AstCompound {
                span: Span { start: 0, end: 25 },
                children: vec![
                    Box::new(AstNode::Number(AstPrimitive {
                        value: "1".to_string(),
                        span: Span { start: 1, end: 2 },
                    })),
                    Box::new(AstNode::String(AstPrimitive {
                        value: "hello".to_string(),
                        span: Span { start: 4, end: 11 },
                    })),
                    Box::new(AstNode::Null(AstPrimitive {
                        value: "null".to_string(),
                        span: Span { start: 13, end: 17 },
                    })),
                    Box::new(AstNode::Bool(AstPrimitive {
                        value: "false".to_string(),
                        span: Span { start: 19, end: 24 },
                    }))
                ],
            })])
        );
    }

    #[test]
    fn list_should_not_allow_non_closed() {
        let code = "[[1, 3]";
        assert_eq!(
            Prelude::new(code.as_bytes()).parse(),
            Err(ParserErr::UnexpEoFile(ParserErrInfo { pos: 7 }))
        );
    }

    // ==================================================================
    // LIST TESTS END
    // ==================================================================

    // ==================================================================
    // DICT TESTS START
    // ==================================================================

    #[test]
    fn dict_should_parse_empty() {
        let ast = Prelude::new("{}".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Dict(AstCompound {
                span: Span { start: 0, end: 2 },
                children: vec![],
            })])
        );
    }

    #[test]
    fn dict_should_parse_non_empty() {
        let ast = Prelude::new(
             "{ \"a\" 1, \"b\" \"2\", \"c\" false, \"d\" null, \"e\" [1, 2, 3], \"f\" { \"a2\" some_value } }".as_bytes(),
         )
         .parse();

        let pair_1 = Box::new(AstNode::DictPair(AstKeyValuePair {
            key: "a".to_string(),
            value: Box::new(AstNode::Number(AstPrimitive {
                value: "1".to_string(),
                span: Span { start: 6, end: 7 },
            })),
            key_span: Span { start: 2, end: 5 },
            span: Span { start: 2, end: 7 },
        }));

        let pair_2 = Box::new(AstNode::DictPair(AstKeyValuePair {
            key: "b".to_string(),
            value: Box::new(AstNode::String(AstPrimitive {
                value: "2".to_string(),
                span: Span { start: 13, end: 16 },
            })),
            key_span: Span { start: 9, end: 12 },
            span: Span { start: 9, end: 16 },
        }));

        let pair_3 = Box::new(AstNode::DictPair(AstKeyValuePair {
            key: "c".to_string(),
            value: Box::new(AstNode::Bool(AstPrimitive {
                value: "false".to_string(),
                span: Span { start: 22, end: 27 },
            })),
            key_span: Span { start: 18, end: 21 },
            span: Span { start: 18, end: 27 },
        }));

        let pair_4 = Box::new(AstNode::DictPair(AstKeyValuePair {
            key: "d".to_string(),
            value: Box::new(AstNode::Null(AstPrimitive {
                value: "null".to_string(),
                span: Span { start: 33, end: 37 },
            })),
            key_span: Span { start: 29, end: 32 },
            span: Span { start: 29, end: 37 },
        }));

        let pair_5 = Box::new(AstNode::DictPair(AstKeyValuePair {
            key: "e".to_string(),
            value: Box::new(AstNode::List(AstCompound {
                span: Span { start: 43, end: 52 },
                children: vec![
                    Box::new(AstNode::Number(AstPrimitive {
                        value: "1".to_string(),
                        span: Span { start: 44, end: 45 },
                    })),
                    Box::new(AstNode::Number(AstPrimitive {
                        value: "2".to_string(),
                        span: Span { start: 47, end: 48 },
                    })),
                    Box::new(AstNode::Number(AstPrimitive {
                        value: "3".to_string(),
                        span: Span { start: 50, end: 51 },
                    })),
                ],
            })),
            key_span: Span { start: 39, end: 42 },
            span: Span { start: 39, end: 52 },
        }));

        let pair_6 = Box::new(AstNode::DictPair(AstKeyValuePair {
            key: "f".to_string(),
            value: Box::new(AstNode::Dict(AstCompound {
                span: Span { start: 58, end: 77 },
                children: vec![Box::new(AstNode::DictPair(AstKeyValuePair {
                    key: "a2".to_string(),
                    value: Box::new(AstNode::Identifier(AstPrimitive {
                        value: "some_value".to_string(),
                        span: Span { start: 65, end: 75 },
                    })),
                    key_span: Span { start: 60, end: 64 },
                    span: Span { start: 60, end: 75 },
                }))],
            })),
            key_span: Span { start: 54, end: 57 },
            span: Span { start: 54, end: 77 },
        }));

        assert_eq!(
            ast,
            Ok(vec![AstNode::Dict(AstCompound {
                span: Span { start: 0, end: 79 },
                children: vec![pair_1, pair_2, pair_3, pair_4, pair_5, pair_6,],
            })])
        );
    }

    #[test]
    fn dict_should_not_allow_invalid_pair() {
        let code = "{ \"a\" 1, \"b\" }";
        assert_eq!(
            Prelude::new(code.as_bytes()).parse(),
            Err(ParserErr::InvalDictPair(ParserErrInfo { pos: 13 }))
        );
    }

    #[test]
    fn dict_should_not_allow_invalid_key() {
        let inputs = vec![
            ("{ a 1 }", 3),
            ("{ 1 \"2\" }", 3),
            ("{ null false }", 6),
            ("{ false true }", 7),
            ("{ [] \"`\" }", 4),
            ("{ {} a }", 4),
        ];
        for (input, pos) in inputs {
            assert_eq!(
                Prelude::new(input.as_bytes()).parse(),
                Err(ParserErr::UnexpDictKey(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn dict_should_not_allow_non_closed() {
        let inputs: Vec<(&str, usize, fn(ParserErrInfo) -> ParserErr)> = vec![
            ("{ \"a\" 1 }}", 9, ParserErr::UnexpTok),
            ("{{ \"1\" \"2\" }", 12, ParserErr::UnexpDictKey),
        ];
        for (input, pos, err) in inputs {
            assert_eq!(
                Prelude::new(input.as_bytes()).parse(),
                Err(err(ParserErrInfo { pos }))
            );
        }
    }

    // ==================================================================
    // DICT TESTS END
    // ==================================================================

    // ==================================================================
    // CALL TESTS START
    // ==================================================================

    #[test]
    fn call_should_parse_with_no_arguments() {
        let ast = Prelude::new(".some-fn()".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Call((
                AstCallKind::Named("some-fn".to_string()),
                AstCompound {
                    span: Span { start: 0, end: 10 },
                    children: vec![],
                }
            ))])
        );
    }

    #[test]
    fn call_should_parse_with_arguments() {
        let ast = Prelude::new(".add(2 .div(4 2))".as_bytes()).parse();
        let nested_children = vec![
            Box::new(AstNode::Number(AstPrimitive {
                value: "4".to_string(),
                span: Span { start: 12, end: 13 },
            })),
            Box::new(AstNode::Number(AstPrimitive {
                value: "2".to_string(),
                span: Span { start: 14, end: 15 },
            })),
        ];
        let children = vec![
            Box::new(AstNode::Number(AstPrimitive {
                value: "2".to_string(),
                span: Span { start: 5, end: 6 },
            })),
            Box::new(AstNode::Call((
                AstCallKind::Named("div".to_string()),
                AstCompound {
                    span: Span { start: 7, end: 16 },
                    children: nested_children,
                },
            ))),
        ];
        assert_eq!(
            ast,
            Ok(vec![AstNode::Call((
                AstCallKind::Named("add".to_string()),
                AstCompound {
                    span: Span { start: 0, end: 17 },
                    children,
                }
            ))])
        );
    }

    #[test]
    fn call_should_parse_with_separators_after_name() {
        let inputs = vec![
            (".test ()", 8),
            (".test  ()", 9),
            (
                ".test
             ()",
                21,
            ),
            (
                ".test
                         ()",
                33,
            ),
        ];
        for (input, end) in inputs {
            assert_eq!(
                Prelude::new(input.as_bytes()).parse(),
                Ok(vec![AstNode::Call((
                    AstCallKind::Named("test".to_string()),
                    AstCompound {
                        span: Span { start: 0, end },
                        children: vec![],
                    }
                ))])
            );
        }
    }

    #[test]
    fn call_should_not_allow_non_closed() {
        let code = ".some-fn(2 2 3))";
        assert_eq!(
            Prelude::new(code.as_bytes()).parse(),
            Err(ParserErr::UnexpTok(ParserErrInfo { pos: 15 }))
        );
    }

    #[test]
    fn call_should_not_allow_separator_after_call_symbol() {
        let code = ". some-fn()";
        assert_eq!(
            Prelude::new(code.as_bytes()).parse(),
            Err(ParserErr::InvalFnName(ParserErrInfo { pos: 9 }))
        );
    }

    #[test]
    fn call_should_reject_invalid_names() {
        let identifiers = vec![
            ("1asd", 5),
            ("!asd", 5),
            ("@asd", 5),
            ("#asd", 5),
            ("$asd", 5),
            ("%asd", 5),
            ("^asd", 5),
            ("&asd", 5),
            ("*asd", 5),
            ("-asd", 5),
            ("_asd", 5),
            ("=asd", 5),
            ("+asd", 5),
            ("?asd", 5),
            ("?asd", 5),
            (">asd", 5),
            ("<asd", 5),
            ("/asd", 5),
        ];
        for (identifier, pos) in identifiers {
            assert_eq!(
                Prelude::new(&format!(".{}()", identifier).as_bytes()).parse(),
                Err(ParserErr::InvalFnName(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn call_should_not_allow_standalone_parens() {
        let code = "()";
        assert_eq!(
            Prelude::new(code.as_bytes()).parse(),
            Err(ParserErr::UnexpTok(ParserErrInfo { pos: 0 }))
        );
    }

    #[test]
    fn call_should_parse_anon() {
        assert_eq!(
            Prelude::new(".()".as_bytes()).parse(),
            Ok(vec![AstNode::Call((
                AstCallKind::Anon,
                AstCompound {
                    span: Span { start: 0, end: 3 },
                    children: vec![],
                }
            ))])
        );
    }

    // ==================================================================
    // CALL TESTS END
    // ==================================================================

    // ==================================================================
    // SLOT TESTS START
    // ==================================================================

    #[test]
    fn slot_should_parse() {
        let slots = vec![
            ("@asd", 4),
            ("@asd?", 5),
            ("@as?d", 5),
            ("@as5?d", 6),
            ("@asd-", 5),
            ("@as-d", 5),
            ("@asd!", 5),
            ("@as!d", 5),
            ("@asd_", 5),
        ];
        for (slot, end) in slots {
            let ast = Prelude::new(slot.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Slot(AstPrimitive {
                    value: slot[1..].to_string(),
                    span: Span { start: 0, end },
                })])
            );
        }
    }

    #[test]
    fn slot_should_reject_invalid_names() {
        let slots = vec![
            ("@1asd", 5),
            ("@!asd", 5),
            ("@@asd", 5),
            ("@#asd", 5),
            ("@$asd", 5),
            ("@%asd", 5),
            ("@^asd", 5),
            ("@&asd", 5),
            ("@*asd", 5),
            ("@-asd", 5),
            ("@_asd", 5),
            ("@=asd", 5),
            ("@+asd", 5),
            ("@?asd", 5),
            ("@?asd", 5),
            ("@>asd", 5),
            ("@<asd", 5),
            ("@/asd", 5),
            ("@@asd", 5),
            ("@ asd", 1),
        ];
        for (slot, pos) in slots {
            assert_eq!(
                Prelude::new(slot.as_bytes()).parse(),
                Err(ParserErr::UnexpTok(ParserErrInfo { pos }))
            );
        }
    }

    // ==================================================================
    // SLOT TESTS END
    // ==================================================================

    // ==================================================================
    // DEPTH TESTS START
    // ==================================================================

    #[test]
    fn depth_should_reject_invalid() {
        let depth_cases: Vec<(&str, usize, fn(ParserErrInfo) -> ParserErr)> = vec![
            (".a())", 4, ParserErr::UnexpTok),
            (".a(()", 3, ParserErr::UnexpTok),
            (".a().a()))", 8, ParserErr::UnexpTok),
            ("()()))", 0, ParserErr::UnexpTok),
            ("())", 0, ParserErr::UnexpTok),
            ("(()", 0, ParserErr::UnexpTok),
            ("[]]", 2, ParserErr::UnexpTok),
            ("[][[][][]]][[", 10, ParserErr::UnexpTok),
            ("[{}}]", 3, ParserErr::UnexpTok),
            ("[{{{{}]", 6, ParserErr::UnexpDictKey),
            ("[{{}]", 4, ParserErr::UnexpDictKey),
        ];
        for (depth_case, pos, err) in depth_cases {
            assert_eq!(
                Prelude::new(depth_case.as_bytes()).parse(),
                Err(err(ParserErrInfo { pos }))
            );
        }
    }

    // ==================================================================
    // DEPTH TESTS END
    // ==================================================================
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
