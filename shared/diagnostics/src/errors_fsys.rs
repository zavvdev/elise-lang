use crate::codes::{DiagnosticCode, DiagnosticCodeType};

pub enum FSysErr {
    R { msg: String, path: String },
    W { msg: String, path: String },
}

impl FSysErr {
    pub fn code(&self) -> DiagnosticCodeType {
        match self {
            Self::R { .. } => DiagnosticCode::FS701.code(),
            Self::W { .. } => DiagnosticCode::FS702.code(),
        }
    }
}
