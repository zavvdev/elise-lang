/// Types for data that is being transformed (csv, json).
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Number,
    String,
    Bool,
    Empty,
}

impl DataType {
    pub const NUMBER_STR: &'static str = "Number";
    pub const STRING_STR: &'static str = "String";
    pub const BOOL_STR: &'static str = "Bool";
    pub const EMPTY_STR: &'static str = "Empty";

    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Number => Self::NUMBER_STR,
            DataType::String => Self::STRING_STR,
            DataType::Bool => Self::BOOL_STR,
            DataType::Empty => Self::EMPTY_STR,
        }
    }
}
