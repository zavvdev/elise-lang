use elise_ast::AstNode;
use std::collections::HashMap;

use elise_shared::errors::{
    LangErr,
    errors_csv_schema_resolver::{CsvSchemaResolverErr, CsvSchemaResolverErrPos},
};

#[derive(Debug)]
enum CsvTypeKind {
    Number,
    String,
    Bool,
    Empty,
}

#[derive(Debug)]
struct CsvTypeDescriptor {
    kind: CsvTypeKind,
}

#[derive(Debug)]
pub struct CsvResolvedSchema {
    row: HashMap<String, CsvTypeDescriptor>,
}

pub struct CsvSchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
}

impl<'a> CsvSchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self { schema_ast }
    }

    pub fn resolve(&self) -> Result<CsvResolvedSchema, LangErr> {
        Err(LangErr::CsvSchemaResolver(CsvSchemaResolverErr::InvalDef {
            pos: CsvSchemaResolverErrPos { start: 0, end: 1 },
        }))
    }
}
