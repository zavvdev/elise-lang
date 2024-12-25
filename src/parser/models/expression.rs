use crate::types;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    // Internal
    _EndOfFn,
    _EndOfList,
    _Separator,

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
    FnAnd,
    FnOr,
    FnBool,
    FnIf,
    FnIsNil,
    FnDefine,
    FnCustom(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub children: Vec<Box<Expr>>,
}

impl Expr {
    pub fn new(kind: ExprKind, children: Vec<Box<Expr>>) -> Self {
        Self { kind, children }
    }
}

pub fn is_expr_internal(expr: &Expr) -> bool {
    vec![
        ExprKind::_EndOfFn,
        ExprKind::_EndOfList,
        ExprKind::_Separator,
    ]
    .contains(&expr.kind)
}
