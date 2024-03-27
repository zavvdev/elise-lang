pub type Integer = i64;
pub type Float = f64;
pub type FloatPrecision = u64;

pub enum ParsedNumber {
    Int(Integer),
    Float(Float),
}

#[derive(Debug)]
pub struct Number {
    pub int: Integer,
    pub precision: FloatPrecision,
}

impl Number {
    pub fn new(int: Integer, precision: FloatPrecision) -> Self {
        Self { int, precision }
    }
}
