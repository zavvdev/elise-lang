#[derive(Debug, PartialEq)]
pub struct CsvParserErrInfo {
    // TODO
}

#[derive(Debug, PartialEq)]
pub enum CsvParserErr {
    Impl,
    // TODO: Add error kind list:
    // https://docs.rs/csv/1.4.0/csv/enum.ErrorKind.html
}
