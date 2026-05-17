#[derive(Debug, PartialEq)]
pub struct Pos {
    pub line: u64,
}

// https://docs.rs/csv/1.4.0/csv/enum.ErrorKind.html
#[derive(Debug, PartialEq)]
pub enum CsvParserErr {
    UneqLen {
        pos: Option<Pos>,
        expected_len: u64,
        actual_len: u64,
    },
    InvalidUtf8 {
        pos: Option<Pos>,
        detail: String,
    },
    Io {
        kind: String,
        detail: String,
    },
    Unknown,
}
