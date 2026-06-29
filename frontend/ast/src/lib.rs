//! # Ast type definitions module
//!
//! This module consists of AST related type definitions
//! and implementations.

use elise_types::Span;

/// Represents a primitive value that does not
/// have any nested values inside. Numbers, strings, bools etc.
#[derive(Debug, PartialEq)]
pub struct AstPrimitive {
    // Interpreted runtime string. Encoding aware.
    pub value: String,
    // Pointer to the original source of bytes.
    // Does not aware of any encoding.
    pub span: Span,
}

/// Represents a value that consists of other values like
/// lists, dictionaries or functions.
#[derive(Debug, PartialEq)]
pub struct AstCompound {
    // Slice of bytes.
    pub span: Span,
    pub children: Vec<Box<AstNode>>,
}

/// Different kinds of functions.
/// We support named and anonymous for now.
#[derive(Debug, PartialEq)]
pub enum AstCallKind {
    Named(String),
    Anon,
}

/// Dictionary key-value pair representation.
#[derive(Debug, PartialEq)]
pub struct AstKeyValuePair {
    pub key: String,
    // Span for key itself since we don't want
    // to keep the whole ast node as key.
    pub key_span: Span,
    // Value has its own span since it's AstNode.
    pub value: Box<AstNode>,
    // Span from the start of the key and
    // to the end of the value.
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Call((AstCallKind, AstCompound)),
    Number(AstPrimitive),
    String(AstPrimitive),
    Bool(AstPrimitive),
    Null(AstPrimitive),
    List(AstCompound),
    Dict(AstCompound),
    // We treat DictPair as an AstNode in order to be consistent
    // and always provide ast nodes as children for compound values.
    DictPair(AstKeyValuePair),
    Identifier(AstPrimitive),
    Slot(AstPrimitive),
}

impl AstNode {
    pub fn span(&self) -> &Span {
        match self {
            AstNode::Call((_, c)) => &c.span,
            AstNode::Number(p)
            | AstNode::String(p)
            | AstNode::Bool(p)
            | AstNode::Null(p)
            | AstNode::Identifier(p)
            | AstNode::Slot(p) => &p.span,
            AstNode::List(c) | AstNode::Dict(c) => &c.span,
            AstNode::DictPair(p) => &p.span,
        }
    }
}
