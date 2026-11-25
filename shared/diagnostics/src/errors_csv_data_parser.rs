use crate::codes::{DiagnosticCode, DiagnosticCodeType};

// https://docs.rs/csv/1.4.0/csv/enum.ErrorKind.html
#[derive(Debug, PartialEq)]
pub enum CsvDataParserErr {
    // =================================
    // Lib types start.
    // =================================
    UneqLen {
        line: Option<u64>,
        expected_len: u64,
        actual_len: u64,
    },
    InvalidUtf8 {
        line: Option<u64>,
        detail: String,
    },
    Io {
        kind: String,
        detail: String,
    },

    // =================================
    // Lib types end.
    // =================================
    MissingHeader {
        col: usize,
    },
    Unknown,
}

impl CsvDataParserErr {
    pub fn code(&self) -> DiagnosticCodeType {
        match self {
            Self::UneqLen { .. } => DiagnosticCode::CDP301.code(),
            Self::InvalidUtf8 { .. } => DiagnosticCode::CDP302.code(),
            Self::Io { .. } => DiagnosticCode::CDP303.code(),
            Self::MissingHeader { .. } => DiagnosticCode::CDP304.code(),
            Self::Unknown { .. } => DiagnosticCode::CDP305.code(),
        }
    }
}
