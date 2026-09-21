pub mod parser_config;

use elise_ast::{
    AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprDict, AstNodeExprDictKey, AstNodeExprList,
    AstNodeExprPrim, AstNodeTypedef, AstNodeTypedefGeneric, AstNodeTypedefRecordKey,
};
use elise_shared::shared_types::{Literal, Span};
use std::str::from_utf8;

use crate::parser_config::{CharCode, DfaNumState};

use elise_shared::shared_errors::errors_parser::{ParserErr, ParserErrInfo};

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

    fn is_tok_end(c: &u8) -> bool {
        Self::is_separator(c)
            || *c == CharCode::RIGHT_PAREN
            || *c == CharCode::RIGHT_SQR_BRACKET
            || *c == CharCode::LESS
            || *c == CharCode::MORE
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
        Self::is_tok_end(c)
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
        Self::is_tok_end(c)
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
        Self::is_tok_end(c)
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

        if entries.is_empty() {
            return Err(self.fail(ParserErr::EmptyRecord));
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
                return Err(self.fail(ParserErr::UnexpEoGeneric));
            };
            if is_end {
                if generic.is_none() {
                    return Err(self.fail(ParserErr::EmptyGeneric));
                }
                self.advance();
                break;
            }

            if generic.is_some() {
                return Err(self.fail(ParserErr::UnexpTok));
            }

            if self.dict_is_start(&c) {
                let record = self.typedef_record_consume()?;
                generic = Some(AstNodeTypedefGeneric::Record(record));
            } else if let Some(node) = self.get_node_from_char(&c)? {
                match node {
                    AstNode::Typedef(typedef) => {
                        generic = Some(AstNodeTypedefGeneric::Single(Box::new(typedef)));
                    }
                    _ => return Err(self.fail(ParserErr::InvalGenericTypedef)),
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
            Err(self.fail(ParserErr::UnexpTok))
        }
    }

    // ==================================================================
    // TYPEDEF END
    // ==================================================================
}
