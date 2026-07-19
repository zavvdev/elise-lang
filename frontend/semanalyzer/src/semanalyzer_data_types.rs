#[derive(Debug, PartialEq)]
pub enum LangPrimitiveType {
    Int,
    Float,
}

// Map types to string representation that we can use
// for error reports.
impl LangPrimitiveType {
    pub const INT_STR: &'static str = "Int";
    pub const FLOAT_STR: &'static str = "Float";

    pub fn as_str(&self) -> &'static str {
        match self {
            LangPrimitiveType::Int => Self::INT_STR,
            LangPrimitiveType::Float => Self::FLOAT_STR,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LangType {
    Primitive(LangPrimitiveType),
}

impl LangType {
    pub const PRIMITIVE_STR: &'static str = "Primitive";

    pub fn as_str(&self) -> &'static str {
        match self {
            LangType::Primitive(_) => Self::PRIMITIVE_STR,
        }
    }

    // Extract string representation of the subtype.
    pub fn as_str_exact(&self) -> &'static str {
        match self {
            LangType::Primitive(primitive) => primitive.as_str(),
        }
    }
}
