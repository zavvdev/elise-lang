//! AST (Abstract Syntax Tree)
//!
//! This data structure is the result of our parser (Prelude).
//! It relies purely on grammar that is defined in GRAMMAR.md.

pub mod ast_type_annot;

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::{CloneGetters, CopyGetters};

use crate::ast_type_annot::AstNodeTypeAnnot;

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
