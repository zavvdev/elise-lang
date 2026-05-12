use elise_shared::errors::errors_csv_schema_resolver::CsvSchemaResolverErr;

use crate::out::utils::print_silent_err;

pub fn print_err(schema_err: &CsvSchemaResolverErr) {
    use CsvSchemaResolverErr::*;

    let info: String = match schema_err {
        InvalDef { pos: _ } => {
            "Invalid schema definition function".to_string()
        }
        InvalRowDef { pos: _ } => {
            "Invalid row definition".to_string()
        }
        InvalRowName { pos: _ } => {
            "Invalid column name".to_string()
        }
        InvalRowTypeDef { pos: _ } => {
            "Invalid column type definition".to_string()
        }
        Unknown => {
            "Unexpected error".to_string()
        }
    };
    print_silent_err(&info, Some("Schema error"));
}
