use elise_errors::errors_csv_schema_resolver::CsvSchemaResolverErr;

use crate::out::utils::print_silent_err;

pub fn print_err(schema_err: &CsvSchemaResolverErr) {
    use CsvSchemaResolverErr::*;

    let info: String = match schema_err {
        RootMissing => "Missing root .schema function call".to_string(),
        RootNoArgs { pos: _ } => "Root .schema call arguments cannot be empty".to_string(),
        RootInval { pos: _ } => {
            "Invalid root function. Use .schema function at the top level".to_string()
        }
        RootTooManyArgs { pos: _ } => {
            "Root .schema function should have only one argument".to_string()
        }
        RowInval { pos: _ } => "Invalid row definition".to_string(),
        RowEmpty { pos: _ } => "Row definition cannot be empty".to_string(),
        RowInvalArgsLen { pos: _ } => {
            "Argument length for the row definition function must be even".to_string()
        }
        ColInvalName { pos: _ } => "Invalid column name".to_string(),
        ColInvalType { pos: _ } => "Invalid column type definition".to_string(),
        Unknown => "Unexpected error".to_string(),
    };
    print_silent_err(&info, Some("Schema error"));
}
