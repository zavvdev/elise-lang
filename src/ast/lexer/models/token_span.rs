#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}

impl TokenSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TokenSpan { start, end, literal }
    }
}
