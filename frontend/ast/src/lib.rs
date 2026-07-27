//! # AST type definitions module
//!
//! This module consists of AST related type definitions
//! and implementations.

use elise_shared::{shared_node_names::NodeName, shared_types::Span};

/// Represents value for a function call.
#[derive(Debug, PartialEq)]
pub struct AstCall {
    pub lexeme: String,
    pub span: Span,
    pub children: Vec<Box<AstNode>>,
}

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
    Call(AstCall),
    Int(AstPrimitive),
    Float(AstPrimitive),
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
            AstNode::Call(f) => &f.span,
            AstNode::Int(p)
            | AstNode::Float(p)
            | AstNode::String(p)
            | AstNode::Bool(p)
            | AstNode::Null(p)
            | AstNode::Identifier(p)
            | AstNode::Slot(p) => &p.span,
            AstNode::List(c) | AstNode::Dict(c) => &c.span,
            AstNode::DictPair(p) => &p.span,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AstNode::Call(_) => NodeName::CALL,
            AstNode::Int(_) => NodeName::INT,
            AstNode::Float(_) => NodeName::FLOAT,
            AstNode::String(_) => NodeName::STRING,
            AstNode::Bool(_) => NodeName::BOOL,
            AstNode::Null(_) => NodeName::NULL,
            AstNode::Dict(_) => NodeName::DICT,
            AstNode::List(_) => NodeName::LIST,
            AstNode::DictPair(_) => NodeName::DICT_PAIR,
            AstNode::Identifier(_) => NodeName::IDENTIFIER,
            AstNode::Slot(_) => NodeName::SLOT,
        }
    }
}
