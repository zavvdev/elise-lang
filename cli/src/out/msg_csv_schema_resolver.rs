use elise_errors::errors_csv_schema_resolver::CsvSchemaResolverErr;

use crate::out::utils::print_silent_err;

// TODO: Use span
pub fn print_err(schema_err: &CsvSchemaResolverErr) {
    use CsvSchemaResolverErr::*;

    let info: String = match schema_err {
        RootInval { span: _ } => {
            "Schema definition must start with .schema function call at the top level".to_string()
        }

        RootArgsLen { span: _ } => {
            "Invalid number of arguments for .schema function call. It must have one argument"
                .to_string()
        }

        RowInval { span: _ } => "Row must be defined using .row function".to_string(),

        RowArgsLen { span: _ } => {
            "Invalid number of arguments for .row function call. Number of arguments must be even".to_string()
        }

        ColInvalName { span: _ } => "Invalid column name".to_string(),

        ColInvalType { span: _ } => "Invalid column type definition".to_string(),
        
        ColTypeNoArgs { span: _ } => "Type functions must not have arguments".to_string(),

        OptArgsLen { span: _ } => {
            "Invalid number of arguments for .optional function".to_string()
        }
    };
    print_silent_err(&info, Some("Schema error"));
}
