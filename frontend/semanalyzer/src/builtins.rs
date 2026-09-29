pub struct FnTypedef;
impl FnTypedef {
    pub const LEXEME: &'static str = "typedef";
    pub const ARGS_LEN: usize = 2;
}

pub struct TypedefLexeme;
impl TypedefLexeme {
    pub const INT: &'static str = "Int";
    pub const LIST: &'static str = "List";
    pub const RECORD: &'static str = "Record";
}
