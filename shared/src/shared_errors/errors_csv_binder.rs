#[derive(Debug, PartialEq)]
pub struct PosInfo {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub enum CsvBinderErr {
    NoData,
    RowLenMismatch(PosInfo),
    TypeMismatch {
        pos: PosInfo,
        expected: &'static str,
        got: &'static str,
    },
    MissingTypeDefinition {
        pos: PosInfo,
        col: String,
    },
}
