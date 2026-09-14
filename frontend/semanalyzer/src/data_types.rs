use elise_shared::shared_node_names::NodeName;

#[derive(Debug, PartialEq)]
pub enum LangPrimitiveType {
    Int,
    Float,
    Str,
    Bool,
    Null,
}

// Map types to string representation that we can use
// for error reports.
impl LangPrimitiveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LangPrimitiveType::Int => NodeName::INT,
            LangPrimitiveType::Float => NodeName::FLOAT,
            LangPrimitiveType::Str => NodeName::STR,
            LangPrimitiveType::Bool => NodeName::BOOL,
            LangPrimitiveType::Null => NodeName::NULL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LangType {
    Primitive(LangPrimitiveType),
}

impl LangType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LangType::Primitive(_) => NodeName::PRIMITIVE,
        }
    }
}
