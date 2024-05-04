use crate::types;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Nil,
    Number(types::Number),
}
