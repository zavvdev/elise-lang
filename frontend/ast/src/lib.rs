//! # AST type definitions module
//!
//! This module consists of AST related type definitions
//! and implementations.

use elise_shared::{shared_node_names::NodeName, shared_types::Span};

/// Represents a primitive value that does not
/// have any nested values inside.
#[derive(Debug, PartialEq)]
pub struct AstPrim {
    pub span: Span,
}

/// Represents a value that consists of other values.
/// Ex: lists, dictionaries or functions.
#[derive(Debug, PartialEq)]
pub struct AstComp {
    pub span: Span,
    pub children: Vec<Box<AstNode>>,
}

#[derive(Debug, PartialEq)]
pub struct AstTypeDesc {
    nullable: bool,
    optional: bool,
    span: Span,
}

#[derive(Debug, PartialEq)]
pub enum AstTypeDef {
    TInt(AstTypeDesc),
    TFloat(AstTypeDesc),
    TStr(AstTypeDesc),
    TBool(AstTypeDesc),
    // TODO: We must keep path segments here.
    TList { desc: AstTypeDesc },
    TDict { desc: AstTypeDesc },
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Int(AstPrim),
    Float(AstPrim),
    Str(AstPrim),
    Bool(AstPrim),
    Null(AstPrim),
    Identifier(AstPrim),
    Slot(AstPrim),
    List(AstComp),
    Dict(AstComp),
    FnCall(AstComp),
    TypeDef(AstTypeDef),
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
