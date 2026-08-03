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

/// Determines what exactly expected arguments number means.
#[derive(Debug, PartialEq)]
pub enum ArityMismatchKind {
    Eq,
    MoreEq,
    LessEq,
    More,
    Less,
}
impl ArityMismatchKind {
    pub fn symbol(&self) -> String {
        let res = match self {
            ArityMismatchKind::Eq => "",
            ArityMismatchKind::MoreEq => ">=",
            ArityMismatchKind::LessEq => "<=",
            ArityMismatchKind::More => ">",
            ArityMismatchKind::Less => "<",
        };
        res.to_string()
    }
}
