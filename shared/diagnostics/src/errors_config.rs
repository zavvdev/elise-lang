use crate::codes::{DiagnosticCode, DiagnosticCodeType};

#[derive(Debug, PartialEq)]
pub struct InvalidArg {
    pub arg_name: String,
    pub provided: String,
    pub expected: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum ConfErr {
    ExtInvalid(String),
    ArgInvalid(InvalidArg),
    ArgRequired(String),
}

impl ConfErr {
    pub fn code(&self) -> DiagnosticCodeType {
        match self {
            Self::ExtInvalid(_) => DiagnosticCode::CO601.code(),
            Self::ArgInvalid(_) => DiagnosticCode::CO602.code(),
            Self::ArgRequired(_) => DiagnosticCode::CO603.code(),
        }
    }
}
