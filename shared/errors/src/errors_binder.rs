#[derive(Debug, PartialEq)]
pub struct BinderErrInfo {
    pub row: usize,
    pub col: usize,
}

// TODO: We need to have an error descriptor that includes info about row and col.
#[derive(Debug, PartialEq)]
pub enum BinderErr {
    NoData,
    RowLenMismatch(BinderErrInfo),
}
