use elise_shared::shared_node_names::NodeName;

/// Types for data that is being transformed (csv, json).
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Null,
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Int => NodeName::INT,
            DataType::Float => NodeName::FLOAT,
            DataType::String => NodeName::STRING,
            DataType::Bool => NodeName::BOOL,
            DataType::Null => NodeName::NULL,
        }
    }
}

pub struct SchemaFnLexeme;
impl SchemaFnLexeme {
    pub const ROOT: &'static str = "schema";
    pub const ROW: &'static str = "row";
    pub const INT: &'static str = "int";
    pub const FLOAT: &'static str = "float";
    pub const STRING: &'static str = "string";
    pub const BOOL: &'static str = "bool";
    pub const OPT: &'static str = "optional";
}
