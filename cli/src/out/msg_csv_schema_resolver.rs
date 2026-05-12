use elise_shared::errors::errors_csv_schema_resolver::CsvSchemaResolverErr;

use crate::out::utils::print_silent_err;

pub fn print_err(schema_err: &CsvSchemaResolverErr) {
    use CsvSchemaResolverErr::*;

    let info: String = match schema_err {
        InvalDef { pos: _ } => {
            format!("Invalid schema definition function")
        }
        InvalRowDef { pos: _ } => {
            format!("Invalid row definition")
        }
        InvalRowName { pos: _ } => {
            format!("Invalid column name")
        }
        InvalRowTypeDef { pos: _ } => {
            format!("Invalid column type definition")
        }
        Unknown => {
            format!("Unexpected error")
        }
    };
    print_silent_err(&info, Some("Schema error"));
}
