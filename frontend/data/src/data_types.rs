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
