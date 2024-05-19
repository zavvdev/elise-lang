use crate::types;

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    // For internal use
    // TODO: Remove if possible
    _EndOfFn,
    _EndOfList,
    _ArgumentSeparator,

    // Public
    Number(types::Number),
    Identifier(String),
    Nil,
    List,
    Boolean(bool),
    String(String),
    FnAdd,
    FnSub,
    FnMul,
    FnDiv,
    FnPrint,
    FnPrintLn,
    FnLetBinding,
    FnGreatr,
    FnLess,
    FnGreatrEq,
    FnLessEq,
    FnEq,
    FnNotEq,
    FnNot,
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub children: Vec<Box<Expr>>,
}

impl Expr {
    pub fn new(kind: ExprKind, children: Vec<Box<Expr>>) -> Self {
        Self { kind, children }
    }
}
