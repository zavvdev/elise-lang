//! AST (Abstract Syntax Tree)
//!
//! This data structure is the result of our parser (Prelude).
//! It relies purely on grammar that is defined in GRAMMAR.md.

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::{CloneGetters, CopyGetters};

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AstNode {
    Expr(AstNodeExpr),
    TypeAnnot(AstNodeTypeAnnot),
}

// ==================================================================
//
//  EXPRESSIONS START
//
// ==================================================================

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
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

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AstNodeExprCall {
    pub lexeme: String,

    #[getset(get_copy = "pub")]
    pub span: Span,

    pub body: Vec<Box<AstNode>>,
}

// Common descriptor for primitives:
// Int, Float, Str, Bool, Null, Identifier, Slot.
#[derive(Debug, PartialEq, CopyGetters, CloneGetters)]
pub struct AstNodeExprPrim {
    #[getset(get_clone = "pub")]
    pub lexeme: String,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AstNodeExprList {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub items: Vec<Box<AstNodeExpr>>,
}

#[derive(Debug, PartialEq, CopyGetters, CloneGetters)]
pub struct AstNodeExprDictKey {
    #[getset(get_copy = "pub")]
    pub span: Span,

    #[getset(get_clone = "pub")]
    pub lexeme: String,
}

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AstNodeExprDict {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub entries: Vec<(AstNodeExprDictKey, Box<AstNodeExpr>)>,
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

// TODO: Check if we need Clone trait here.
#[derive(Debug, PartialEq, Clone, CopyGetters, CloneGetters)]
pub struct AstNodeTypeAnnot {
    #[getset(get_copy = "pub")]
    pub span: Span,

    #[getset(get_clone = "pub")]
    pub lexeme: String,

    // From the perspective of AST, any type can have a generic.
    // Since AST is only focused on code structure/grammar,
    // we don't care here what kind of type we're currently in,
    // although semantically :Int cannot have a generic type.
    pub generic: Option<AstNodeTypeAnnotGeneric>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNodeTypeAnnotGeneric {
    Single(Box<AstNodeTypeAnnot>),
    Record(Vec<(AstNodeTypeAnnotRecordKey, Box<AstNodeTypeAnnot>)>),
}

#[derive(Debug, PartialEq, Clone, CloneGetters)]
pub struct AstNodeTypeAnnotRecordKey {
    #[getset(get_clone = "pub")]
    pub lexeme: String,

    pub span: Span,
}

// ==================================================================
//
//  TYPE ANNOTATIONS END
//
// ==================================================================
