// https://docs.rs/csv/1.4.0/csv/enum.ErrorKind.html
#[derive(Debug, PartialEq)]
pub enum CsvParserErr {
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
    Unknown,
}
