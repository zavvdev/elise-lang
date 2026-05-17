use elise_errors::errors_csv_schema_resolver::CsvSchemaResolverErr;

use crate::out::utils::print_silent_err;

pub fn print_err(schema_err: &CsvSchemaResolverErr) {
    use CsvSchemaResolverErr::*;

    let info: String = match schema_err {
        EmptySchema => "Schema definition file cannot be empty".to_string(),
        InvalDef { pos: _ } => "Invalid schema definition function".to_string(),
        InvalRowDef { pos: _ } => "Invalid row definition".to_string(),
        TooManySchemaDefArgs { pos: _ } => {
            "Schema definition function should have only one argument".to_string()
        }
        EmptyRow { pos: _ } => "Row definition does not have any column descriptors".to_string(),
        RowInvalArgsLen { pos: _ } => {
            "Arguments length for the row definition function must be even".to_string()
        }
        InvalColName { pos: _ } => "Invalid column name".to_string(),
        InvalColTypeDef { pos: _ } => "Invalid column type definition".to_string(),
        Unknown => "Unexpected error".to_string(),
    };
    print_silent_err(&info, Some("Schema error"));
}
