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
    pub const TRUE: &'static str = "true";
    pub const FALSE: &'static str = "false";
    pub const NULL: &'static str = "null";
}

pub struct Name;
impl Name {
    pub const TYPE_ANNOT__INT: &'static str = "Int";
    pub const TYPE_ANNOT__STR: &'static str = "Str";
    pub const TYPE_ANNOT__LIST: &'static str = "List";
    pub const TYPE_ANNOT__RECORD: &'static str = "Record";
    pub const TYPE_ANNOT__SLOT_DATA: &'static str = "Data";

    pub const FN__TYPEDEF: &'static str = "typedef";
    pub const FN__GET: &'static str = "get";
    pub const FN__ADD: &'static str = "add";

    pub const SLOT__DATA: &'static str = "data";
}
