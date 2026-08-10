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
    Eq(usize),
    MoreEq(usize),
    LessEq(usize),
    More(usize),
    Less(usize),
    Range((usize, usize)),
}
impl ArityMismatchKind {
    pub fn as_str(&self) -> String {
        match self {
            ArityMismatchKind::Eq(n) => format!("{}", n),
            ArityMismatchKind::MoreEq(n) => format!(">={n}"),
            ArityMismatchKind::LessEq(n) => format!("<={n}"),
            ArityMismatchKind::More(n) => format!(">{n}"),
            ArityMismatchKind::Less(n) => format!("<{n}"),
            ArityMismatchKind::Range((min, max)) => format!(">={min}, <={max}"),
        }
    }
}
