pub mod parser_config;

use elise_ast::{
    AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprDict, AstNodeExprDictKey, AstNodeExprList,
    AstNodeExprPrim, AstNodeTypedef, AstNodeTypedefGeneric, AstNodeTypedefRecordKey,
};
use elise_shared::shared_types::{Literal, Span};
use std::str::from_utf8;

use crate::parser_config::{CharCode, DfaNumState};

use elise_shared::shared_errors::errors_parser::{ParserErr, ParserErrInfo};

// ==================================================================
//
//  PARSER START
//
// ==================================================================

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
        } else if Self::typedef_is_start(c) {
            self.typedef_consume()

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
        matches!(c, b' ' | b'\n' | b'\t' | b'\r') || *c == CharCode::COMMA
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
        Self::number_is_digit(c) || *c == CharCode::MINUS
    }

    fn number_is_end(c: &u8) -> bool {
        Self::is_separator(c) || *c == CharCode::RIGHT_PAREN || *c == CharCode::RIGHT_SQR_BRACKET
    }

    fn number_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let mut state = DfaNumState::Start;
        let tok_start = self.tok_pos;

        while let Some(c) = self.peek() {
            use DfaNumState::*;

            state = match (&state, c) {
                (Start, CharCode::MINUS) => {
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
                (Expon, CharCode::MINUS) => {
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

        let tok_end = self.tok_pos;
        let value = from_utf8(&self.source_code[tok_start..tok_end]);

        if value.is_err() {
            return Err(self.fail(ParserErr::InvalNum));
        }

        let primitive = AstNodeExprPrim {
            lexeme: value.unwrap().to_string(),
            span: Span {
                start: tok_start,
                end: tok_end,
            },
        };

        match state {
            DfaNumState::Zero | DfaNumState::Int => {
                Ok(Some(AstNode::Expr(AstNodeExpr::Int(primitive))))
            }
            DfaNumState::Frac | DfaNumState::Scient => {
                Ok(Some(AstNode::Expr(AstNodeExpr::Float(primitive))))
            }
            _ => Err(self.fail(ParserErr::InvalNum)),
        }
    }

    // ==================================================================
    // NUMBER END
    // ==================================================================

    // ==================================================================
    // STRING START
    // ==================================================================

    fn string_is_start(char: &u8) -> bool {
        *char == CharCode::DOUBLE_QT
    }

    fn string_is_end(char: &u8) -> bool {
        *char == CharCode::DOUBLE_QT
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

        Ok(Some(AstNode::Expr(AstNodeExpr::Str(AstNodeExprPrim {
            lexeme: value.to_owned(),
            span: Span { start, end },
        }))))
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
        Self::is_separator(c)
            || *c == CharCode::RIGHT_PAREN
            || *c == CharCode::RIGHT_SQR_BRACKET
            || *c == CharCode::LESS
            || *c == CharCode::MORE
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

        let lexeme = from_utf8(&self.source_code[start..self.tok_pos])
            .unwrap()
            .to_string();

        let primitive = AstNodeExprPrim {
            lexeme,
            span: Span {
                start,
                end: self.tok_pos,
            },
        };

        match primitive.lexeme.as_str() {
            // Identify known keywords.
            Literal::TRUE | Literal::FALSE => Ok(Some(AstNode::Expr(AstNodeExpr::Bool(primitive)))),
            Literal::NULL => Ok(Some(AstNode::Expr(AstNodeExpr::Null(primitive)))),
            _ => {
                if Self::identifier_is_valid(&primitive.lexeme) {
                    Ok(Some(AstNode::Expr(AstNodeExpr::Ident(primitive))))
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
        if *c == CharCode::LEFT_SQR_BRACKET {
            self.depth_stack.push(CharCode::LEFT_SQR_BRACKET);
            return true;
        }
        false
    }

    fn list_check_end(&mut self, c: &u8) -> Result<bool, ()> {
        if *c == CharCode::RIGHT_SQR_BRACKET {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != CharCode::LEFT_SQR_BRACKET {
                return Err(());
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn list_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let start = self.tok_pos;
        self.advance();
        let mut items: Vec<Box<AstNodeExpr>> = vec![];

        while let Some(c) = self.peek() {
            let Ok(is_end) = self.list_check_end(&c) else {
                return Err(self.fail(ParserErr::UnexpEoList));
            };
            if is_end {
                self.advance();
                break;
            }
            if let Some(node) = self.get_node_from_char(&c)? {
                match node {
                    // List items must contain only expression nodes.
                    AstNode::Expr(expr) => items.push(Box::new(expr)),
                    _ => return Err(self.fail(ParserErr::UnexpListItem)),
                }
            }
        }

        Ok(Some(AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
            span: Span {
                start,
                end: self.tok_pos,
            },
            items,
        }))))
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // DICT START
    // ==================================================================

    fn dict_is_start(&mut self, c: &u8) -> bool {
        if *c == CharCode::LEFT_CUR_BRACKET {
            self.depth_stack.push(CharCode::LEFT_CUR_BRACKET);
            return true;
        }
        false
    }

    fn dict_check_end(&mut self, c: &u8) -> Result<bool, ()> {
        if *c == CharCode::RIGHT_CUR_BRACKET {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != CharCode::LEFT_CUR_BRACKET {
                return Err(());
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn dict_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let start = self.tok_pos;
        self.advance();

        let mut entries: Vec<(AstNodeExprDictKey, Box<AstNodeExpr>)> = vec![];
        let mut key: Option<AstNodeExprDictKey> = None;

        while let Some(c) = self.peek() {
            let is_end = self
                .dict_check_end(&c)
                .map_err(|_| self.fail(ParserErr::UnexpEoDict))?;

            if is_end {
                // If we ended up in the end of the dict but we still
                // have a dangling key to match, return error since
                // dict must have an even number of children expressions.
                if key.is_some() {
                    return Err(self.fail(ParserErr::InvalDictPair));
                }
                self.advance();
                break;
            }

            let Some(node) = self.get_node_from_char(&c)? else {
                continue;
            };

            // Dictionaries must contain only expression nodes.
            let AstNode::Expr(expr) = node else {
                return Err(self.fail(ParserErr::UnexpDictKey));
            };

            match key.take() {
                None => {
                    // Key must always be a string expression.
                    let AstNodeExpr::Str(prim) = expr else {
                        return Err(self.fail(ParserErr::UnexpDictKey));
                    };
                    key = Some(AstNodeExprDictKey {
                        lexeme: prim.lexeme,
                        span: Span {
                            start: prim.span.start,
                            end: prim.span.end,
                        },
                    });
                }
                Some(k) => entries.push((k, Box::new(expr))),
            }
        }

        Ok(Some(AstNode::Expr(AstNodeExpr::Dict(AstNodeExprDict {
            span: Span {
                start,
                end: self.tok_pos,
            },
            entries,
        }))))
    }

    // ==================================================================
    // DICT END
    // ==================================================================

    // ==================================================================
    // CALL START
    // ==================================================================

    fn call_is_start(&self, char: &u8) -> bool {
        if let Some(next_char) = self.peek() {
            return *char == CharCode::CALL_PREFIX && !Self::is_separator(&next_char);
        }
        false
    }

    fn call_is_end(&self, char: &u8) -> bool {
        *char == CharCode::RIGHT_PAREN
    }

    fn call_validate_name(&self, name: &str) -> Result<String, ParserErr> {
        if !name.is_empty() && Self::identifier_is_valid(name) {
            Ok(name.to_string())
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
            if c == CharCode::LEFT_PAREN {
                self.depth_stack.push(CharCode::LEFT_PAREN);
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

        let mut body: Vec<Box<AstNode>> = vec![];

        // Consume function arguments.
        while let Some(c) = self.peek() {
            if self.call_is_end(&c) {
                let last_entry = self.depth_stack.pop();
                if last_entry.is_none() || last_entry.unwrap() != CharCode::LEFT_PAREN {
                    return Err(self.fail(ParserErr::UnexpEoFn));
                }
                self.advance();
                break;
            }

            if let Some(node) = self.get_node_from_char(&c)? {
                body.push(Box::new(node));
            }
        }

        let call_end = self.tok_pos;

        Ok(Some(AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
            lexeme: call_name,
            span: Span {
                start: call_start,
                end: call_end,
            },
            body,
        }))))
    }

    // ==================================================================
    // CALL END
    // ==================================================================

    // ==================================================================
    // SLOT START
    // ==================================================================

    fn slot_is_start(&self, char: &u8) -> bool {
        if let Some(next_char) = self.peek() {
            return *char == CharCode::SLOT_PREFIX && !Self::is_separator(&next_char);
        }
        false
    }

    fn slot_is_end(c: &u8) -> bool {
        Self::is_separator(c) || *c == CharCode::RIGHT_PAREN || *c == CharCode::RIGHT_SQR_BRACKET
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

        let lexeme = from_utf8(&self.source_code[slot_name_start..self.tok_pos])
            .unwrap()
            .to_string();

        if Self::identifier_is_valid(&lexeme) {
            Ok(Some(AstNode::Expr(AstNodeExpr::Slot(AstNodeExprPrim {
                lexeme,
                span: Span {
                    start,
                    end: self.tok_pos,
                },
            }))))
        } else {
            Err(self.fail(ParserErr::UnexpTok))
        }
    }

    // ==================================================================
    // SLOT END
    // ==================================================================

    // ==================================================================
    // TYPEDEF START
    // ==================================================================

    fn typedef_is_start(char: &u8) -> bool {
        *char == CharCode::COLON
    }

    fn typedef_generic_is_start(&mut self) -> bool {
        if let Some(ch) = self.peek()
            && ch == CharCode::LESS
        {
            self.depth_stack.push(CharCode::LESS);
            return true;
        }
        false
    }

    fn typedef_generic_check_end(&mut self, c: &u8) -> Result<bool, ()> {
        if *c == CharCode::MORE {
            let last_entry = self.depth_stack.pop();
            if last_entry.is_none() || last_entry.unwrap() != CharCode::LESS {
                return Err(());
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn typedef_record_consume(
        &mut self,
    ) -> Result<Vec<(AstNodeTypedefRecordKey, Box<AstNodeTypedef>)>, ParserErr> {
        self.advance();

        let mut entries: Vec<(AstNodeTypedefRecordKey, Box<AstNodeTypedef>)> = vec![];
        let mut key: Option<AstNodeTypedefRecordKey> = None;

        while let Some(c) = self.peek() {
            let is_end = self
                .dict_check_end(&c)
                .map_err(|_| self.fail(ParserErr::UnexpEoDict))?;

            if is_end {
                // If we ended up in the end of the record but we still
                // have a dangling key to match, return error since
                // record must have an even number of children expressions.
                if key.is_some() {
                    return Err(self.fail(ParserErr::InvalDictPair));
                }
                self.advance();
                break;
            }

            let Some(node) = self.get_node_from_char(&c)? else {
                continue;
            };

            match key.take() {
                None => {
                    // Key must always be a string expression.
                    let AstNode::Expr(AstNodeExpr::Str(prim)) = node else {
                        return Err(self.fail(ParserErr::UnexpDictKey));
                    };
                    key = Some(AstNodeTypedefRecordKey {
                        lexeme: prim.lexeme,
                        span: Span {
                            start: prim.span.start,
                            end: prim.span.end,
                        },
                    });
                }
                Some(k) => {
                    let AstNode::Typedef(typedef) = node else {
                        return Err(self.fail(ParserErr::UnexpDictKey));
                    };
                    entries.push((k, Box::new(typedef)));
                }
            }
        }

        Ok(entries)
    }

    fn typedef_consume_lexeme(&mut self) -> Result<String, ParserErr> {
        let ident_node = self.identifier_consume()?;
        match ident_node {
            Some(AstNode::Expr(AstNodeExpr::Ident(prim))) => Ok(prim.lexeme),
            _ => Err(self.fail(ParserErr::UnexpTok)),
        }
    }

    fn typedef_consume_generic(&mut self) -> Result<Option<AstNodeTypedefGeneric>, ParserErr> {
        let mut generic: Option<AstNodeTypedefGeneric> = None;

        if !self.typedef_generic_is_start() {
            return Ok(generic);
        }

        self.advance();

        while let Some(c) = self.peek() {
            let Ok(is_end) = self.typedef_generic_check_end(&c) else {
                return Err(self.fail(ParserErr::UnexpEoTypedefGeneric));
            };
            if is_end {
                if generic.is_none() {
                    return Err(self.fail(ParserErr::EmptyTypedefGeneric));
                }
                self.advance();
                break;
            }

            if self.dict_is_start(&c) {
                let record = self.typedef_record_consume()?;
                generic = Some(AstNodeTypedefGeneric::Record(record));
            }

            if let Some(node) = self.get_node_from_char(&c)? {
                match node {
                    AstNode::Typedef(typedef) => {
                        generic = Some(AstNodeTypedefGeneric::Single(Box::new(typedef)));
                    }
                    _ => return Err(self.fail(ParserErr::UnexpListItem)),
                }
            }
        }

        Ok(generic)
    }

    fn typedef_consume(&mut self) -> Result<Option<AstNode>, ParserErr> {
        let start = self.tok_pos;
        self.advance();

        if let Some(c) = self.peek()
            && Self::identifier_is_start(&c)
        {
            let lexeme = self.typedef_consume_lexeme()?;
            let generic = self.typedef_consume_generic()?;

            Ok(Some(AstNode::Typedef(AstNodeTypedef {
                span: Span {
                    start,
                    end: self.tok_pos,
                },
                lexeme,
                generic,
            })))
        } else {
            return Err(self.fail(ParserErr::UnexpTok));
        }
    }

    // ==================================================================
    // TYPEDEF END
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
    use elise_ast::{
        AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprDict, AstNodeExprDictKey,
        AstNodeExprList, AstNodeExprPrim,
    };
    use elise_shared::{
        shared_errors::errors_parser::{ParserErr, ParserErrInfo},
        shared_types::Span,
    };

    use crate::Prelude;

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
    fn number_should_not_allow_start_with_zero_if_not_float() {
        let forbidded_tokens = vec![("02", 1), ("00", 1), ("00.3", 1), ("02.4", 1)];

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
    fn number_should_not_allow_separator_after_minus() {
        let forbidded_tokens = vec![("- 2", 1), ("-\n2", 1)];

        for (token, pos) in forbidded_tokens {
            assert_eq!(
                Prelude::new(token.as_bytes()).parse(),
                Err(ParserErr::InvalNum(ParserErrInfo { pos }))
            );
        }
    }

    #[test]
    fn number_should_parse_integers() {
        let numbers = vec![
            ("-0", 2),
            ("0", 1),
            ("-1", 2),
            ("2", 1),
            ("-9", 2),
            ("123", 3),
            ("-999999", 7),
            ("101", 3),
        ];
        for (number, end) in numbers {
            let ast = Prelude::new(number.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                    lexeme: number.to_string(),
                    span: Span { start: 0, end },
                }))])
            );
        }
    }

    #[test]
    fn number_should_parse_floats() {
        let numbers = vec![
            ("-0.0", 4),
            ("0.2", 3),
            ("-1.34", 5),
            ("22.4456", 7),
            ("-999.3234", 9),
            ("99999.900", 9),
        ];
        for (number, end) in numbers {
            let ast = Prelude::new(number.as_bytes()).parse();
            assert_eq!(
                ast,
                Ok(vec![AstNode::Expr(AstNodeExpr::Float(AstNodeExprPrim {
                    lexeme: number.to_string(),
                    span: Span { start: 0, end },
                }))])
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
                AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                    lexeme: "3".to_string(),
                    span: Span { start: 0, end: 1 },
                })),
                AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                    lexeme: "56".to_string(),
                    span: Span { start: 2, end: 4 },
                })),
                AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                    lexeme: "-9".to_string(),
                    span: Span { start: 6, end: 8 },
                })),
                AstNode::Expr(AstNodeExpr::Float(AstNodeExprPrim {
                    lexeme: "3.2".to_string(),
                    span: Span { start: 11, end: 14 },
                })),
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
    fn number_should_parse_scientific_numbers_as_floats() {
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
                Ok(vec![AstNode::Expr(AstNodeExpr::Float(AstNodeExprPrim {
                    lexeme: number.to_string(),
                    span: Span { start: 0, end },
                }))])
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
                Ok(vec![AstNode::Expr(AstNodeExpr::Str(AstNodeExprPrim {
                    lexeme: string
                        .split("\"")
                        .into_iter()
                        .collect::<Vec<&str>>()
                        .get(1)
                        .unwrap()
                        .to_string(),
                    span: Span { start: 0, end },
                }))])
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
                Ok(vec![AstNode::Expr(AstNodeExpr::Str(AstNodeExprPrim {
                    lexeme: expected.to_string(),
                    span: Span { start: 0, end },
                }))])
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
            Ok(vec![AstNode::Expr(AstNodeExpr::Bool(AstNodeExprPrim {
                lexeme: "true".to_string(),
                span: Span { start: 0, end: 4 }
            }))])
        )
    }

    #[test]
    fn bool_should_parse_false() {
        let ast = Prelude::new("false".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Bool(AstNodeExprPrim {
                lexeme: "false".to_string(),
                span: Span { start: 0, end: 5 }
            }))])
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
            Ok(vec![AstNode::Expr(AstNodeExpr::Null(AstNodeExprPrim {
                lexeme: "null".to_string(),
                span: Span { start: 0, end: 4 }
            }))])
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
            ("asd<", 3, ParserErr::UnexpTok),
            ("asd>", 3, ParserErr::UnexpTok),
            ("asd/", 4, ParserErr::UnexpTok),
            ("asd+", 4, ParserErr::UnexpTok),
            ("asd%", 4, ParserErr::UnexpTok),
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
                Ok(vec![AstNode::Expr(AstNodeExpr::Ident(AstNodeExprPrim {
                    lexeme: identifier.to_string(),
                    span: Span { start: 0, end },
                }))])
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
            Ok(vec![AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
                span: Span { start: 0, end: 2 },
                items: vec![],
            }))])
        );
    }

    #[test]
    fn list_should_parse_nested_empty() {
        let ast = Prelude::new("[[]]".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
                span: Span { start: 0, end: 4 },
                items: vec![Box::new(AstNodeExpr::List(AstNodeExprList {
                    span: Span { start: 1, end: 3 },
                    items: vec![],
                }))],
            }))])
        );
    }

    #[test]
    fn list_should_parse_non_empty() {
        let ast = Prelude::new("[1, \"hello\", null, false]".as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::List(AstNodeExprList {
                span: Span { start: 0, end: 25 },
                items: vec![
                    Box::new(AstNodeExpr::Int(AstNodeExprPrim {
                        lexeme: "1".to_string(),
                        span: Span { start: 1, end: 2 },
                    })),
                    Box::new(AstNodeExpr::Str(AstNodeExprPrim {
                        lexeme: "hello".to_string(),
                        span: Span { start: 4, end: 11 },
                    })),
                    Box::new(AstNodeExpr::Null(AstNodeExprPrim {
                        lexeme: "null".to_string(),
                        span: Span { start: 13, end: 17 },
                    })),
                    Box::new(AstNodeExpr::Bool(AstNodeExprPrim {
                        lexeme: "false".to_string(),
                        span: Span { start: 19, end: 24 },
                    }))
                ],
            }))])
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
            Ok(vec![AstNode::Expr(AstNodeExpr::Dict(AstNodeExprDict {
                span: Span { start: 0, end: 2 },
                entries: vec![],
            }))])
        );
    }

    #[test]
    fn dict_should_parse_non_empty() {
        let ast = Prelude::new(
            r##"{
                    "a" 1,
                    "b" 2.3,
                    "c" false,
                    "d" null,
                    "e" [1],
                    "f" { "a2"  some-value }
                }"##
            .as_bytes(),
        )
        .parse();

        let pair_1 = (
            AstNodeExprDictKey {
                span: Span { start: 22, end: 25 },
                lexeme: "a".to_string(),
            },
            Box::new(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "1".to_string(),
                span: Span { start: 26, end: 27 },
            })),
        );

        let pair_2 = (
            AstNodeExprDictKey {
                span: Span { start: 49, end: 52 },
                lexeme: "b".to_string(),
            },
            Box::new(AstNodeExpr::Float(AstNodeExprPrim {
                lexeme: "2.3".to_string(),
                span: Span { start: 53, end: 56 },
            })),
        );

        let pair_3 = (
            AstNodeExprDictKey {
                span: Span { start: 78, end: 81 },
                lexeme: "c".to_string(),
            },
            Box::new(AstNodeExpr::Bool(AstNodeExprPrim {
                lexeme: "false".to_string(),
                span: Span { start: 82, end: 87 },
            })),
        );

        let pair_4 = (
            AstNodeExprDictKey {
                span: Span {
                    start: 109,
                    end: 112,
                },
                lexeme: "d".to_string(),
            },
            Box::new(AstNodeExpr::Null(AstNodeExprPrim {
                lexeme: "null".to_string(),
                span: Span {
                    start: 113,
                    end: 117,
                },
            })),
        );

        let pair_5 = (
            AstNodeExprDictKey {
                span: Span {
                    start: 139,
                    end: 142,
                },
                lexeme: "e".to_string(),
            },
            Box::new(AstNodeExpr::List(AstNodeExprList {
                span: Span {
                    start: 143,
                    end: 146,
                },
                items: vec![Box::new(AstNodeExpr::Int(AstNodeExprPrim {
                    span: Span {
                        start: 144,
                        end: 145,
                    },
                    lexeme: "1".to_string(),
                }))],
            })),
        );

        let pair_6 = (
            AstNodeExprDictKey {
                span: Span {
                    start: 168,
                    end: 171,
                },
                lexeme: "f".to_string(),
            },
            Box::new(AstNodeExpr::Dict(AstNodeExprDict {
                span: Span {
                    start: 172,
                    end: 192,
                },
                entries: vec![(
                    AstNodeExprDictKey {
                        span: Span {
                            start: 174,
                            end: 178,
                        },
                        lexeme: "a2".to_string(),
                    },
                    Box::new(AstNodeExpr::Ident(AstNodeExprPrim {
                        span: Span {
                            start: 180,
                            end: 190,
                        },
                        lexeme: "some-value".to_string(),
                    })),
                )],
            })),
        );

        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Dict(AstNodeExprDict {
                span: Span { start: 0, end: 210 },
                entries: vec![pair_1, pair_2, pair_3, pair_4, pair_5, pair_6],
            }))])
        );
    }

    #[test]
    fn dict_should_not_allow_invalid_pair() {
        let code = r##"{ "a" 1, "b" }"##;
        assert_eq!(
            Prelude::new(code.as_bytes()).parse(),
            Err(ParserErr::InvalDictPair(ParserErrInfo { pos: 13 }))
        );
    }

    #[test]
    fn dict_should_not_allow_invalid_key() {
        let inputs = vec![
            ("{ a 1 }", 3),
            (r##"{ 1 "2" }"##, 3),
            ("{ null false }", 6),
            ("{ false true }", 7),
            (r##"{ [] "`" }"##, 4),
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
            (r##"{ "a" 1 }}"##, 9, ParserErr::UnexpTok),
            (r##"{{ "1" "2" }"##, 12, ParserErr::UnexpDictKey),
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
            Ok(vec![AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
                lexeme: "some-fn".to_string(),
                span: Span { start: 0, end: 10 },
                body: vec![],
            }))])
        );
    }

    #[test]
    fn call_should_parse_with_arguments() {
        let ast = Prelude::new(".add(2 .div(4 2))".as_bytes()).parse();
        let nested_children = vec![
            Box::new(AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "4".to_string(),
                span: Span { start: 12, end: 13 },
            }))),
            Box::new(AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "2".to_string(),
                span: Span { start: 14, end: 15 },
            }))),
        ];
        let body = vec![
            Box::new(AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "2".to_string(),
                span: Span { start: 5, end: 6 },
            }))),
            Box::new(AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
                lexeme: "div".to_string(),
                span: Span { start: 7, end: 16 },
                body: nested_children,
            }))),
        ];
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
                lexeme: "add".to_string(),
                span: Span { start: 0, end: 17 },
                body,
            }))])
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
                25,
            ),
            (
                ".test
                             ()",
                37,
            ),
        ];
        for (input, end) in inputs {
            assert_eq!(
                Prelude::new(input.as_bytes()).parse(),
                Ok(vec![AstNode::Expr(AstNodeExpr::Call(AstNodeExprCall {
                    lexeme: "test".to_string(),
                    span: Span { start: 0, end },
                    body: vec![],
                }))])
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
            ("asd<", 5),
            ("asd>", 5),
            ("asd%", 5),
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
    fn call_should_not_allow_missing_names() {
        assert_eq!(
            Prelude::new(".()".as_bytes()).parse(),
            Err(ParserErr::InvalFnName(ParserErrInfo { pos: 1 }))
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
                Ok(vec![AstNode::Expr(AstNodeExpr::Slot(AstNodeExprPrim {
                    lexeme: slot[1..].to_string(),
                    span: Span { start: 0, end },
                }))])
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
            ("@asd<", 5),
            ("@asd>", 5),
            ("@asd%", 5),
            ("@asd$", 5),
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
