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

    pub const EXPR__CALL: &'static str = "CallExpression";
    pub const EXPR__INT: &'static str = "IntExpression";
    pub const EXPR__FLOAT: &'static str = "FloatExpression";
    pub const EXPR__STR: &'static str = "StrExpression";
    pub const EXPR__BOOL: &'static str = "BoolExpression";
    pub const EXPR__NULL: &'static str = "NullExpression";
    pub const EXPR__LIST: &'static str = "ListExpression";
    pub const EXPR__DICT: &'static str = "DictExpression";
    pub const EXPR__IDENTIFIER: &'static str = "IdentifierExpression";
    pub const EXPR__SLOT: &'static str = "SlotExpression";
}
