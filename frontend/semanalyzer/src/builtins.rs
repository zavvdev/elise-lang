pub struct FnLet;
impl FnLet {
    pub const LEXEME: &'static str = "let";
    pub const ARGS_LEN: usize = 3;
}

pub struct TypedefLexeme;
impl TypedefLexeme {
    pub const INT: &'static str = "Int";
}
