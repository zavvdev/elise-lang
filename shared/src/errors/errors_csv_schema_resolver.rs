#[derive(Debug, PartialEq)]
pub struct CsvSchemaResolverErrPos {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum CsvSchemaResolverErr {
    InvalDef { pos: CsvSchemaResolverErrPos },
    InvalRowDef { pos: CsvSchemaResolverErrPos },
    InvalRowName { pos: CsvSchemaResolverErrPos },
    InvalRowTypeDef { pos: CsvSchemaResolverErrPos },
    Unknown,
}
