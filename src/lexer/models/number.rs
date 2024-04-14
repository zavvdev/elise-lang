use crate::types;

pub type FloatPrecision = u64;

pub const FLOAT_SEPARATOR: char = '.';

pub enum ParsedNumber {
    Int(types::Integer),
    Float(types::Float),
}

#[derive(Debug)]
pub struct Number {
    pub int: types::Integer,
    pub precision: FloatPrecision,
}

impl Number {
    pub fn new(int: types::Integer, precision: FloatPrecision) -> Self {
        Self { int, precision }
    }
}
