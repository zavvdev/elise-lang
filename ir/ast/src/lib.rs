//! AST (Abstract Syntax Tree)
//!
//! This data structure is the result of our parser (Prelude).
//! It relies purely on grammar that is defined in GRAMMAR.md.

pub mod ast_type_annot;

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::{Name, Span};
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

impl AstNodeExpr {
    pub fn name(&self) -> String {
        match self {
            Self::Call(_) => Name::EXPR__CALL.to_string(),
            Self::Int(_) => Name::EXPR__INT.to_string(),
            Self::Float(_) => Name::EXPR__FLOAT.to_string(),
            Self::Str(_) => Name::EXPR__STR.to_string(),
            Self::Bool(_) => Name::EXPR__BOOL.to_string(),
            Self::Null(_) => Name::EXPR__NULL.to_string(),
            Self::List(_) => Name::EXPR__LIST.to_string(),
            Self::Dict(_) => Name::EXPR__DICT.to_string(),
            Self::Ident(_) => Name::EXPR__IDENTIFIER.to_string(),
            Self::Slot(_) => Name::EXPR__SLOT.to_string(),
        }
    }
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
