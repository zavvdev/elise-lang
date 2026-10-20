//! AST (Abstract Syntax Tree)
//!
//! This data structure is the result of our parser (Prelude).
//! It relies purely on grammar that is defined in GRAMMAR.md.

use elise_shared_proc_macros::HasSpan;
use elise_shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expr(AstNodeExpr),
    TypeAnnot(AstNodeTypeAnnot),
}

impl AstNode {
    /// For error reports in order to locate the node
    /// in the source code.
    pub fn span(&self) -> Span {
        match self {
            AstNode::Expr(expr) => expr.span(),
            AstNode::TypeAnnot(annot) => annot.span(),
        }
    }
}

// ==================================================================
//
//  EXPRESSIONS START
//
// ==================================================================

#[derive(Debug, PartialEq, HasSpan)]
pub struct AstNodeExprCall {
    pub lexeme: String,
    pub span: Span,
    pub body: Vec<Box<AstNode>>,
}

// Common descriptor for primitives:
// Int, Float, Str, Bool, Null, Identifier, Slot.
#[derive(Debug, PartialEq, HasSpan)]
pub struct AstNodeExprPrim {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, HasSpan)]
pub struct AstNodeExprList {
    pub span: Span,
    pub items: Vec<Box<AstNodeExpr>>,
}

#[derive(Debug, PartialEq, HasSpan)]
pub struct AstNodeExprDictKey {
    pub span: Span,
    pub lexeme: String,
}

#[derive(Debug, PartialEq, HasSpan)]
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
            | AstNodeExpr::Slot(p) => p.span(),
            AstNodeExpr::List(l) => l.span(),
            AstNodeExpr::Dict(d) => d.span(),
            AstNodeExpr::Call(c) => c.span(),
        }
    }
}

// ==================================================================
//
//  EXPRESSIONS END
//
// ==================================================================

// ==================================================================
//
//  TYPE ANNOTATIONS START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone)]
pub struct AstNodeTypeAnnotRecordKey {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNodeTypeAnnotGeneric {
    Single(Box<AstNodeTypeAnnot>),
    Record(Vec<(AstNodeTypeAnnotRecordKey, Box<AstNodeTypeAnnot>)>),
}

#[derive(Debug, PartialEq, Clone, HasSpan)]
pub struct AstNodeTypeAnnot {
    pub span: Span,
    pub lexeme: String,
    // From the perspective of AST, any type can have a generic.
    // Since AST is only focused on code structure/grammar,
    // we don't care here what kind of type we're currently in,
    // although semantically :Int cannot have a generic type.
    pub generic: Option<AstNodeTypeAnnotGeneric>,
}

// ==================================================================
//
//  TYPE ANNOTATIONS END
//
// ==================================================================
