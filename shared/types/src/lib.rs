/// Byte offsets into the source file.
/// Half-open interval: [start, end)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TablePos {
    pub row: usize,
    pub col: usize,
}

pub struct Literal;
impl Literal {
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const NULL: &str = "null";
}

// TODO: Must be under shared/errors.
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
    pub fn as_string(&self) -> String {
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

// TODO: Prob needs to be just Name and list all possible name aliases for any nodes
//       including function names, slot names, type names etc.
//       Also, each method `as_string` must be renamed to `name` except those that do
//       not return a literal name.
pub struct DataTypeName;
impl DataTypeName {
    pub const INT: &str = "Int";
    pub const STR: &str = "Str";
    pub const LIST: &str = "List";
    pub const RECORD: &str = "Record";
}
