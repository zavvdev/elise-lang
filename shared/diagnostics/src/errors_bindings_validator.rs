use crate::codes::{DiagnosticCode, DiagnosticCodeType};

#[derive(Debug, PartialEq)]
pub enum BindingsValidatorErr {
    TypeMismatch {
        data_path_info: String,
        expected: String,
        found: String,
    },

    MissingTypeDeclaration {
        data_path_info: String,
        data_type_info: String,
    },
}

impl BindingsValidatorErr {
    pub fn code(&self) -> DiagnosticCodeType {
        match self {
            Self::TypeMismatch { .. } => DiagnosticCode::BV401.code(),
            Self::MissingTypeDeclaration { .. } => DiagnosticCode::BV402.code(),
        }
    }
}
