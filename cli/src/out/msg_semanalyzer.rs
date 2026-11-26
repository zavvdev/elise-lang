use elise_shared_diagnostics::errors_semanalyzer::SemanalyzerErr;
use elise_shared_types::Span;

use crate::out::utils::{
    self, get_source_code_slice, print_err_source_code_pos, print_source_code_slice,
};

pub fn print_err(sema_err: &SemanalyzerErr, source_code: &[u8]) {
    use SemanalyzerErr::*;

    let (category, message, span): (&str, &str, &Span) = match sema_err {
        ArityMismatch {
            fn_name,
            found,
            span,
            kind,
        } => (
            "TODO",
            &format!(
                "Invalid number of arguments for \"{}\" function. Expected: {}, found: {}",
                fn_name,
                kind.as_string(),
                found,
            ),
            span,
        ),

        UnknFnCall { span, name } => (
            "TODO",
            &format!(r##"Cannot find function named "{}""##, name),
            span,
        ),

        UnknSlot { span, name } => (
            "TODO",
            &format!(r##"Cannot find slot named "{}""##, name),
            span,
        ),

        UnknType { span, name } => (
            "TODO",
            &format!(r##"Cannot find type definition named "{}""##, name),
            span,
        ),

        UnexpNode {
            span,
            expected,
            found,
        } => {
            if expected.is_empty() {
                ("TODO", &format!("Unexpected {}", found), span)
            } else if expected.len() == 1 {
                (
                    "TODO",
                    &format!(
                        "Unexpected node. Expected {}, found {}",
                        expected.first().unwrap(),
                        found
                    ),
                    span,
                )
            } else {
                (
                    "TODO",
                    &format!(
                        "Unexpected node. Expected one of [{}], found {}",
                        expected.join(", "),
                        found
                    ),
                    span,
                )
            }
        }

        UnexpExpr {
            span,
            found,
            expected,
        } => {
            if expected.is_empty() {
                ("TODO", &format!("Unexpected {}", found), span)
            } else if expected.len() == 1 {
                (
                    "TODO",
                    &format!(
                        "Unexpected expression. Expected {}, found {}",
                        expected.first().unwrap(),
                        found
                    ),
                    span,
                )
            } else {
                (
                    "TODO",
                    &format!(
                        "Unexpected expression. Expected one of [{}], found {}",
                        expected.join(", "),
                        found
                    ),
                    span,
                )
            }
        }

        UnexpType {
            span,
            expected,
            found,
        } => {
            if expected.is_empty() {
                ("TODO", &format!("Unexpected type {}", found), span)
            } else if expected.len() == 1 {
                (
                    "TODO",
                    &format!(
                        "Unexpected type. Expected {}, found {}",
                        expected.first().unwrap(),
                        found
                    ),
                    span,
                )
            } else {
                (
                    "TODO",
                    &format!(
                        "Unexpected type. Expected one of [{}], found {}",
                        expected.join(", "),
                        found
                    ),
                    span,
                )
            }
        }

        MissingGeneric { span, type_annot } => (
            "TODO",
            &format!("Missing generic for {} type", type_annot),
            span,
        ),

        MissingTypeDefinition { span, for_name } => (
            "TODO",
            &format!("Missing type definition for {}", for_name),
            span,
        ),

        InvalGeneric { span, type_annot } => {
            ("TODO", &format!("Invalid generic for {}", type_annot), span)
        }

        UnresolvableTypePath { path, span } => (
            "TODO",
            &format!("Unable to resolve type definition following {}", path),
            span,
        ),

        UnableToCoalesceType { name, span } => {
            ("TODO", &format!("Unable to coalesce {} type", name), span)
        }

        TypedefContextViolation { span } => (
            "TODO",
            "Only type definitions are supported in this context",
            span,
        ),

        InconsistentListItems { span } => (
            "TODO",
            "List can contain only elements of the same type",
            span,
        ),

        CannotInferType { span, name } => {
            ("TODO", &format!("Unable to infer type from {}", name), span)
        }

        UndefinedDataAccess { span } => (
            "TODO",
            "Nothing exists by following this data access path",
            span,
        ),
    };

    utils::print_err(category, sema_err.code(), message);

    if let Some(code) = get_source_code_slice(source_code, span.start) {
        print_err_source_code_pos(code.pos, "TODO");
        print_source_code_slice(&code.slice, code.pos.col);
    }
}
