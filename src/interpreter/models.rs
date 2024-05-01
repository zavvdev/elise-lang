use crate::types;

#[derive(Debug)]
pub enum EvalResult {
    Nil,
    Int(types::Integer),
    Float(types::Float),
}
