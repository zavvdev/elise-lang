use elise_ast::AstNode;

use elise_shared::errors::{
    LangErr,
    errors_csv_schema_resolver::{CsvSchemaResolverErr, CsvSchemaResolverErrPos},
};

#[derive(Debug, PartialEq)]
enum CsvColType {
    Number,
    String,
    Bool,
}

#[derive(Debug, PartialEq)]
struct CsvColDescriptor {
    name: String,
    ty: CsvColType,
    opt: bool,
}

#[derive(Debug, PartialEq)]
pub struct CsvResolvedSchema {
    row: Vec<CsvColDescriptor>,
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

// ==========================
//
//  TESTS START
//
// ==========================

//.schema(
//    .row(
//        name     .string()
//        age      .number()
//        salary   .optional(.number())
//        employed .bool()
//    ))

// 1. [] Check if schema ast is empty. Otherwise return EmptySchema error
// 2. [] Check if we have .schema function call at the top level. Otherwise return InvalDef error
// 3. [] Check if we have a single child which is a .row function call. Otherwise return InvaliRowDef error
// 4. [] Get the number of columns from .row call. If empty then return EmptyRow error
// 5. [] Allocate a vector with length that equals to number of culumns
// 6. [] Walk through .row function call children and check:
//      - [] Each odd argument should be an identifier. Otherwise return InvalColName  error
//      - [] Each even argument should be a known schema function call (SCH_FN_*). Otherwise return
//           InvalColTypeDef error
//      - [] If two prev steps pass then create CsvColDescriptor and insert it into previously
//           created vector

#[cfg(test)]
mod tests {
    use crate::schema_resolver::CsvSchemaResolver;
    use elise_shared::errors::{LangErr, errors_csv_schema_resolver::CsvSchemaResolverErr};

    #[test]
    fn should_return_error_if_top_level_function_is_invalid() {
        let ast = vec![];
        let result = CsvSchemaResolver::new(&ast).resolve();
        assert_eq!(
            result,
            Err(LangErr::CsvSchemaResolver(
                CsvSchemaResolverErr::EmptySchema
            ))
        );
    }
}

// ==========================
//
//  TESTS END
//
// ==========================
