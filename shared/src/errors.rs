// Config

#[derive(Debug, PartialEq)]
pub struct InvalidArg {
    pub arg_name: String,
    pub provided: String,
}

#[derive(Debug, PartialEq)]
pub enum ConfError {
    ExtInvalid(String),
    ArgInvalid(InvalidArg),
    ArgRequired(String),
}

// Language specific

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
