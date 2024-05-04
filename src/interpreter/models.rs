use crate::types;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Nil,
    Int(types::Integer),
    Float(types::Float),
}
