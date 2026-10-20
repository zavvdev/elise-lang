//! AST (Abstract Syntax Tree)
//!
//! This data structure is the result of our parser (Prelude).
//! It relies purely on grammar that is defined in GRAMMAR.md.

use elise_shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expr(AstNodeExpr),
    TypeAnnot(AstNodeTypeAnnot),
}

// ==================================================================
//
//  EXPRESSIONS START
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

#[derive(Debug, PartialEq, Clone)]
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
