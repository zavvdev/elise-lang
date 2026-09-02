#[derive(Debug, PartialEq)]
pub enum CsvDataValidatorErr {
    TypeMismatch {
        row: usize,
        col: usize,
        expected: &'static str,
        found: &'static str,
    },
    UnknownDataPath {
        row: usize,
        col: usize,
    }
}
