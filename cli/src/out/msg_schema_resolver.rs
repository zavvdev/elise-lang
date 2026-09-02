use elise_shared::shared_errors::errors_schema_resolver::SchemaResolverErr;
use elise_shared::shared_types::Span;

use crate::out::utils;

use crate::out::utils::{
    get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(schema_err: &SchemaResolverErr, schema_source_code: &[u8]) {
    use SchemaResolverErr::*;

    let (msg, span): (&str, Option<&Span>) = match schema_err {
        Empty => ("Schema file must not be empty", None),

        Unexp { span } => ("Unexpected expression", Some(span)),

        ArityMismatch {
            fn_name,
            kind,
            found,
            span,
        } => (
            &format!(
                "Invalid number of arguments for \"{}\" function. Expected: {}, found: {}",
                fn_name,
                kind.as_str(),
                found,
            ),
            Some(span),
        ),
        UnresolvablePath { path } => (&format!("Unresolvable path: {}", path), None),
        InvalTypeDef { span } => ("Invalid type definition", Some(span)),
        InvalDict { span } => ("Invalid dictionary", Some(span)),
        InvalUseOfModifier { span } => ("Invalid usage of modifier", Some(span)),
        UndexpType {
            expected,
            found,
            span,
        } => (
            &format!(
                "Unexpected type. Expected \"{}\", found \"{}\"",
                expected, found
            ),
            Some(span),
        ),
        NoUnionOfUnion { span } => (
            "Union cannot be used as a direct child of another union",
            Some(span),
        ),
    };

    utils::print_err(msg, Some("Schema error"));

    if let Some(span) = span
        && let Some(code) = get_source_code_slice(schema_source_code, span.start)
    {
        print_err_source_code_pos(code.pos.row, code.pos.col);
        print_err_source_code_slice(&code.slice, code.pos.col);
    }
}
