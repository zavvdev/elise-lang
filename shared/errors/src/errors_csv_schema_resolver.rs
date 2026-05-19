#[derive(Debug, PartialEq)]
pub struct CsvSchemaResolverErrPos {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum CsvSchemaResolverErr {
    RootMissing,
    RootNoArgs { pos: CsvSchemaResolverErrPos },
    RootInval { pos: CsvSchemaResolverErrPos },
    RootTooManyArgs { pos: CsvSchemaResolverErrPos },
    RowInval { pos: CsvSchemaResolverErrPos },
    RowEmpty { pos: CsvSchemaResolverErrPos },
    RowInvalArgsLen { pos: CsvSchemaResolverErrPos },
    ColInvalName { pos: CsvSchemaResolverErrPos },
    ColInvalType { pos: CsvSchemaResolverErrPos },
    Unknown,
}
