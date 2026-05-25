#[derive(Debug, PartialEq)]
pub struct CsvParserErrPos {
    pub line: u64,
}

// https://docs.rs/csv/1.4.0/csv/enum.ErrorKind.html
#[derive(Debug, PartialEq)]
pub enum CsvParserErr {
    UneqLen {
        pos: Option<CsvParserErrPos>,
        expected_len: u64,
        actual_len: u64,
    },
    InvalidUtf8 {
        pos: Option<CsvParserErrPos>,
        detail: String,
    },
    Io {
        kind: String,
        detail: String,
    },
    Unknown,
}
