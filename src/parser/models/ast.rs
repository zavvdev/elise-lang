use crate::types;

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    // For internal use
    // TODO: Remove if possible
    _EndOfFn,
    _Separator,
    _ArgumentSeparator,

    // Public
    Int(types::Integer),
    Float(types::Float),
    FnAdd,
    FnSub,
    FnMul,
    FnDiv,
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
