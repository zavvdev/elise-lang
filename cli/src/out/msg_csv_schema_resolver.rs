use elise_shared_errors::errors_csv_schema_resolver::CsvSchemaResolverErr;
use elise_shared_types::Span;

use crate::out::utils;

use crate::out::utils::{
    get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(schema_err: &CsvSchemaResolverErr, schema_source_code: &[u8]) {
    use CsvSchemaResolverErr::*;

    let (msg, span): (&str, &Span) = match schema_err {
        RootInval { span } => (
            "Schema definition must start with .schema function call at the top level",
            span,
        ),

        RootArgsLen { span } => (
            "Invalid number of arguments for .schema function call. It must have one argument",
            span,
        ),

        RowInval { span } => ("Row must be defined using .row function", span),

        RowArgsLen { span } => (
            "Invalid number of arguments for .row function call. Number of arguments must be even",
            span,
        ),

        ColInvalName { span } => ("Invalid column name", span),

        ColInvalType { span } => ("Invalid column type definition", span),

        ColTypeNoArgs { span } => ("Type functions must not have arguments", span),

        OptArgsLen { span } => ("Invalid number of arguments for .optional function", span),

        OptEmpty { span } => (
            "Empty type cannot be optional since it's already represents an empty value that cannot have an alternative",
            span,
        ),
    };

    utils::print_err(msg, Some("Schema error"));

    if let Some(code) = &get_source_code_slice(schema_source_code, span.start) {
        print_err_source_code_pos(code.row, code.col);
        print_err_source_code_slice(&code.slice, code.col);
    };
}
