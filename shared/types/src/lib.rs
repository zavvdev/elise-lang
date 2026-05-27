#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

// Types for data that is being transformed (csv, json)
#[derive(Debug, PartialEq)]
pub enum DataSourceFieldType {
    Number,
    String,
    Bool,
}
