#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,

    // Data Types
    Int(i64),
    Float(f64),
    
    // Functions
    Add,
}

