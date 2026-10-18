//! AST (Abstract Syntax Tree)
//!
//! This data structure is the result of our parser (Prelude).
//! It relies purely on grammar that is defined in GRAMMAR.md
//! which is in the root of the project.
//!
//! Each node is a result of differentiation between grammar
//! entities such as functions, slots, type annotations etc.
//! It doesn't know about semantics or behavior, the only
//! purpose is to represent grammatical structure.

use elise_shared::shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expr(AstNodeExpr),
    Typedef(AstNodeTypedef),
}

impl AstNode {
    /// For using in error reports in order to locate the node
    /// in the source code.
    pub fn span(&self) -> Span {
        match self {
            AstNode::Expr(expr) => expr.span().clone(),
            AstNode::Typedef(typedef) => typedef.span().clone(),
        }
    }
}

// ==================================================================
//
//  EXPR START
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub struct AstNodeExprCall {
    pub lexeme: String,
    pub span: Span,
    pub body: Vec<Box<AstNode>>,
}

// Common descriptor for primitives:
// Int, Float, Str, Bool, Null, Identifier, Slot.
#[derive(Debug, PartialEq)]
pub struct AstNodeExprPrim {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprList {
    pub span: Span,
    pub items: Vec<Box<AstNodeExpr>>,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprDictKey {
    pub span: Span,
    pub lexeme: String,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprDict {
    pub span: Span,
    pub entries: Vec<(AstNodeExprDictKey, Box<AstNodeExpr>)>,
}

#[derive(Debug, PartialEq)]
pub enum AstNodeExpr {
    Call(AstNodeExprCall),
    Int(AstNodeExprPrim),
    Float(AstNodeExprPrim),
    Str(AstNodeExprPrim),
    Bool(AstNodeExprPrim),
    Null(AstNodeExprPrim),
    List(AstNodeExprList),
    Dict(AstNodeExprDict),
    Ident(AstNodeExprPrim),
    Slot(AstNodeExprPrim),
}

impl AstNodeExpr {
    pub fn span(&self) -> Span {
        match self {
            AstNodeExpr::Int(p)
            | AstNodeExpr::Float(p)
            | AstNodeExpr::Str(p)
            | AstNodeExpr::Bool(p)
            | AstNodeExpr::Ident(p)
            | AstNodeExpr::Null(p)
            | AstNodeExpr::Slot(p) => p.span.clone(),
            AstNodeExpr::List(l) => l.span.clone(),
            AstNodeExpr::Dict(d) => d.span.clone(),
            AstNodeExpr::Call(c) => c.span.clone(),
        }
    }
}

// ==================================================================
//
//  EXPR END
//
// ==================================================================

// ==================================================================
//
//  TYPEDEF START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone)]
pub struct AstNodeTypedefRecordKey {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNodeTypedefGeneric {
    Single(Box<AstNodeTypedef>),
    Record(Vec<(AstNodeTypedefRecordKey, Box<AstNodeTypedef>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AstNodeTypedef {
    pub span: Span,
    pub lexeme: String,
    // From the perspective of AST, any type can have a generic.
    // Since AST is only focused on code structure/grammar,
    // we don't care here what kind of type we're currently in,
    // although semantically :Int cannot have a generic type.
    pub generic: Option<AstNodeTypedefGeneric>,
}

impl AstNodeTypedef {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

// ==================================================================
//
//  TYPEDEF END
//
// ==================================================================
