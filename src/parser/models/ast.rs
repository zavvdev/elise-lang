use crate::types;

#[derive(Debug, PartialEq)]
pub enum AstNodeKind {
    _EndOfFn,
    _Separator,
    _ArgumentSeparator,

    Int(types::Integer),
    Float(types::Float),
    FnAdd,
    FnSub,
    FnMul,
    FnDiv,
}

#[derive(Debug, PartialEq)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub args: Vec<Box<AstNode>>,
}

impl AstNode {
    pub fn new(kind: AstNodeKind, args: Vec<Box<AstNode>>) -> Self {
        Self { kind, args }
    }
}
