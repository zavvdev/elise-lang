#[derive(Debug, PartialEq)]
pub enum PreExecErr {
    NoResolvedSchema,
    NoDataBinding,
}
