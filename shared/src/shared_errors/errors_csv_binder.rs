#[derive(Debug, PartialEq)]
pub struct PosInfo {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct TypeMismatchInfo {
    pub pos: PosInfo,
    pub expected: &'static str,
    pub got: &'static str,
}

#[derive(Debug, PartialEq)]
pub enum CsvBinderErr {
    NoData,
    RowLenMismatch(PosInfo),
    TypeMismatch(TypeMismatchInfo),
}
