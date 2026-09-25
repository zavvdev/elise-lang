use elise_shared::shared_node_names::NodeName;

#[derive(Debug, PartialEq)]
pub enum AAstPrimDataType {
    Int,
}

// Map types to string representation that we can use
// for error reports.
impl AAstPrimDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AAstPrimDataType::Int => NodeName::INT,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AAstDataType {
    Prim(AAstPrimDataType),
}

impl AAstDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AAstDataType::Prim(_) => NodeName::PRIMITIVE,
        }
    }
}
