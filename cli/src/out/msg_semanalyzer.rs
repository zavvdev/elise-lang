use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;

use crate::out::utils::{
    self, get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(sema_err: &SemanalyzerErr, source_code: &[u8]) {
    use SemanalyzerErr::*;

    let (info, span) = match sema_err {
        ArityMismatch {
            fn_name,
            found,
            span,
            kind,
        } => (
            format!(
                "Invalid number of arguments for \"{}\" function. Expected: {}, found: {}",
                fn_name,
                kind.as_string(),
                found,
            ),
            span,
        ),

        UnknFnCall { span, name } => (format!(r##"Cannot find function named "{}""##, name), span),

        UnknSlot { span, name } => (format!(r##"Cannot find slot named "{}""##, name), span),

        UnknType { span, name } => (
            format!(r##"Cannot find type definition named "{}""##, name),
            span,
        ),

        UnexpNode {
            span,
            expected,
            found,
        } => {
            if expected.is_empty() {
                (format!("Unexpected {}", found), span)
            } else if expected.len() == 1 {
                (
                    format!(
                        "Unexpected node. Expected {}, found {}",
                        expected.first().unwrap(),
                        found
                    ),
                    span,
                )
            } else {
                (
                    format!(
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
                (format!("Unexpected {}", found), span)
            } else if expected.len() == 1 {
                (
                    format!(
                        "Unexpected expression. Expected {}, found {}",
                        expected.first().unwrap(),
                        found
                    ),
                    span,
                )
            } else {
                (
                    format!(
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
                (format!("Unexpected type {}", found), span)
            } else if expected.len() == 1 {
                (
                    format!(
                        "Unexpected type. Expected {}, found {}",
                        expected.first().unwrap(),
                        found
                    ),
                    span,
                )
            } else {
                (
                    format!(
                        "Unexpected type. Expected one of [{}], found {}",
                        expected.join(", "),
                        found
                    ),
                    span,
                )
            }
        }

        MissingGeneric { span, type_annot } => {
            (format!("Missing generic for {} type", type_annot), span)
        }

        MissingTypeDefinition { span, for_name } => {
            (format!("Missing type definition for {}", for_name), span)
        }

        InvalGeneric { span, type_annot } => (format!("Invalid generic for {}", type_annot), span),

        UnresolvableTypePath { path, span } => (
            format!("Unable to resolve type definition following {}", path),
            span,
        ),

        UnableToCoalesceType { name, span } => (format!("Unable to coalesce {} type", name), span),

        TypedefContextViolation { span } => (
            "Only type definitions are supported in this context".to_string(),
            span,
        ),

        InconsistentListItems { span } => (
            "List can contain only elements of the same type".to_string(),
            span,
        ),

        CannotInferType { span, name } => (format!("Unable to infer type from {}", name), span),

        UndefinedDataAccess { span } => (
            "Nothing exists by following this data access path".to_string(),
            span,
        ),
    };

    utils::print_err(&info, Some("Semantic Error"));

    if let Some(code) = get_source_code_slice(source_code, span.start) {
        print_err_source_code_pos(code.pos.row, code.pos.col);
        print_err_source_code_slice(&code.slice, code.pos.col);
    }
}
