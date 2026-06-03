/// Byte offsets into the source file.
///
/// Half-open interval: [start, end)
#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Types for data that is being transformed (csv, json)
#[derive(Debug, PartialEq, Clone)]
pub enum DataSourceFieldType {
    Number,
    String,
    Bool,
}
