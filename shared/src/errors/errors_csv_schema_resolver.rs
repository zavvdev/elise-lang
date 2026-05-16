#[derive(Debug, PartialEq)]
pub struct CsvSchemaResolverErrPos {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum CsvSchemaResolverErr {
    EmptySchema,
    InvalDef { pos: CsvSchemaResolverErrPos },
    InvalRowDef { pos: CsvSchemaResolverErrPos },
    EmptyRow { pos: CsvSchemaResolverErrPos },
    InvalColName { pos: CsvSchemaResolverErrPos },
    InvalColTypeDef { pos: CsvSchemaResolverErrPos },
    Unknown,
}
