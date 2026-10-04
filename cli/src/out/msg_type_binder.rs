use elise_shared::shared_errors::errors_type_binder::TypeBinderErr;
use elise_shared::shared_types::Span;

use crate::out::utils;

use crate::out::utils::{
    get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

// TODO: Maybe we don't need this at all. Type binder is used inside
// semanalyzer and its errors are mapped there.
pub fn print_err(schema_err: &TypeBinderErr, schema_source_code: &[u8]) {
    use TypeBinderErr::*;

    let (msg, span): (&str, Option<&Span>) = match schema_err {
        UnknownTypedef { span } => ("Unknown type definition", Some(span)),
        UnresolvablePath { path } => (&format!("Unresolvable path: {}", path), None),
        UnableToMerge { span } => ("Unable to merge type definitions", Some(span)),
    };

    utils::print_err(msg, Some("Type binder error"));

    if let Some(span) = span
        && let Some(code) = get_source_code_slice(schema_source_code, span.start)
    {
        print_err_source_code_pos(code.pos.row, code.pos.col);
        print_err_source_code_slice(&code.slice, code.pos.col);
    }
}
