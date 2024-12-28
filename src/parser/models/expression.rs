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

const INTERNAL_EXPRESSIONS: [ExprKind; 3] = [
    ExprKind::_EndOfFn,
    ExprKind::_EndOfList,
    ExprKind::_Separator,
];

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub children: Vec<Box<Expr>>,
    pub start_at: usize,
}

impl Expr {
    pub fn new(kind: ExprKind, children: Vec<Box<Expr>>, start_at: usize) -> Self {
        Self {
            kind,
            children,
            start_at,
        }
    }
}

pub fn is_expr_internal(expr: &Expr) -> bool {
    INTERNAL_EXPRESSIONS.contains(&expr.kind)
}
