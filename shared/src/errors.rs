#[derive(Debug)]
pub struct ParserError {
    pub char_pos: usize,
    pub message: String,
    pub source_code: Vec<u8>,
}

#[derive(Debug)]
pub enum LangError {
    Parser(ParserError),
}
