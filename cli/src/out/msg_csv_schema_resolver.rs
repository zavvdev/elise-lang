use elise_errors::errors_csv_schema_resolver::CsvSchemaResolverErr;
use elise_types::Span;

use colored::Colorize;

use crate::out::utils::print_silent_err;

pub fn print_err(schema_err: &CsvSchemaResolverErr) {
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
    };
    print_silent_err(msg, Some("Schema error"));
    let location = format!("At {}:{}\n", span.start, span.end);
    eprintln!("{}", location.bold());
    // TODO: Provide source code.
}
