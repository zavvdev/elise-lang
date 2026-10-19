pub struct FnTypedef;
impl FnTypedef {
    pub const LEXEME: &'static str = "typedef";
    pub const ARGS_LEN: usize = 2;
}

pub struct FnGet;
impl FnGet {
    pub const LEXEME: &'static str = "get";
    pub const ARGS_LEN_MIN: usize = 2;
}

pub struct TypeAnnotLexeme;
impl TypeAnnotLexeme {
    pub const INT: &'static str = "Int";
    pub const LIST: &'static str = "List";
    pub const RECORD: &'static str = "Record";
}

pub struct SlotData;
impl SlotData {
    pub const LEXEME: &'static str = "data";
    pub const TYPEDEF_LEXEME: &'static str = "Data";
}
