#[derive(Debug)]
pub struct ParserError {
    pub row: usize,
    pub col: usize,
    pub message: &'static str,
    // This field should not store the whole source code.
    // Instead we just keep a slice of it where exactly
    // an error happened.
    pub source_code_slice: Option<String>,
}

#[derive(Debug)]
pub enum LangError {
    Parser(ParserError),
}
