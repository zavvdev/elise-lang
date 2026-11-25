use crate::codes::{DiagnosticCode, DiagnosticCodeType};

#[derive(Debug, PartialEq)]
pub enum DataBinderErr {
    UnresolvablePath { path: String },
}

impl DataBinderErr {
    pub fn code(&self) -> DiagnosticCodeType {
        match self {
            Self::UnresolvablePath { .. } => DiagnosticCode::DB501.code(),
        }
    }
}
