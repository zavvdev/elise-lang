pub enum ParsedNumber {
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Number {
    pub int: i64,
    pub precision: u64,
}

impl Number {
    pub fn new(int: i64, precision: u64) -> Self {
        Self { int, precision }
    }
}
