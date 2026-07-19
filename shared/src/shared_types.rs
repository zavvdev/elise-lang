/// Byte offsets into the source file.
///
/// Half-open interval: [start, end)
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
