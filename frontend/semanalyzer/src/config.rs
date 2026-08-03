pub struct FnDefine;
impl FnDefine {
    pub const LEXEME: &'static str = "define";
    pub const ARGS_LEN: usize = 2;
}

pub struct FnLet;
impl FnLet {
    pub const LEXEME: &'static str = "let";
    pub const MIN_ARGS_LEN: usize = 2;
}
