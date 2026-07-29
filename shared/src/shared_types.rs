/// Byte offsets into the source file.
///
/// Half-open interval: [start, end)
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct Keyword;
impl Keyword {
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const NULL: &str = "null";
}
