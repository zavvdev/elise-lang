use std::collections::HashMap;

use elise_shared::shared_node_names::NodeName;

/// Types for data that is being transformed (csv, json).
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Null,
    // Allows any number of values of the same type.
    ListOf(Box<DataType>),
    // Allows to provide a fixed amount of arguments
    // of the different types.
    List(Vec<Box<DataType>>),

    // TODO: ?Maybe we need to use another DS
    // instead of HashMap.
    Dict(HashMap<String, Box<DataType>>),
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::Int => NodeName::INT,
            DataType::Float => NodeName::FLOAT,
            DataType::String => NodeName::STRING,
            DataType::Bool => NodeName::BOOL,
            DataType::Null => NodeName::NULL,
            DataType::ListOf(_) => NodeName::LIST,
            DataType::List(_) => NodeName::LIST,
            DataType::Dict(_) => NodeName::DICT,
        }
    }
}

pub struct SchemaFnLexeme;
impl SchemaFnLexeme {
    pub const ROOT: &'static str = "schema";
    pub const INT: &'static str = "int";
    pub const FLOAT: &'static str = "float";
    pub const STRING: &'static str = "string";
    pub const BOOL: &'static str = "bool";
    pub const LIST: &'static str = "list";
    pub const LIST_OF: &'static str = "of";
    pub const DICT: &'static str = "dict";
    pub const OPT: &'static str = "optional";
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResolutionPathSegment {
    // We can use index for cases when user iterates
    // over some iterable data and we can track indexes
    // and use them for building a Path key.
    Index(usize),
    
    // Means that we can use any index. For example,
    // if list has all items of the same type.
    AbstractIndex,
    // Just a regular string segment such as csv column
    // name or json object property.
    Field(String),
}

pub type ResolutionPath = Vec<ResolutionPathSegment>;
