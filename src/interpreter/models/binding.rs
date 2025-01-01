use crate::interpreter::models::env::EvalResult;

pub struct Binding {
    pub name: String,
    pub value: EvalResult,
    pub start_at: usize,
}

impl Binding {
    pub fn new(name: String, value: EvalResult, start_at: usize) -> Self {
        Self {
            name,
            value,
            start_at,
        }
    }
}
