#[derive(Debug)]
pub struct ParserError<'a> {
    pub char_pos: usize,
    pub message: &'static str,
    pub source_code: &'a [u8],
}

#[derive(Debug)]
pub enum LangError<'a> {
    Parser(ParserError<'a>),
}
