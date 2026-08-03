use elise_shared::{shared_errors::errors_semanalyzer::SemanalyzerErr, shared_types::ArityMismatchKind};

use crate::out::utils::{
    self, get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(sema_err: &SemanalyzerErr, source_code: &[u8]) {
    use SemanalyzerErr::*;

    let (info, span) = match sema_err {
        SymbolUndefined { span } => ("Undefined symbol".to_string(), span),
        SymbolDuplicate { span } => ("Symbol already defined in this scope".to_string(), span),

        ArityMismatch {
            fn_name,
            expected,
            found,
            span,
            kind,
        } => {
            let kind_symb = match kind {
                ArityMismatchKind::Eq => "",
                ArityMismatchKind::MoreEq => ">=",
                ArityMismatchKind::LessEq => "<=",
                ArityMismatchKind::More => ">",
                ArityMismatchKind::Less => "<",
            };
            (
                format!(
                    "Invalid number of arguments for \"{fn_name}\" function. Expected: {kind_symb}{expected}, found: {found}"
                ),
                span,
            )
        }
        ArgKindMismatch {
            fn_name,
            position,
            expected,
            found,
            span,
        } => (
            format!(
                "Expected \"{expected}\" to be an argument {} of the \"{fn_name}\" function, found \"{found}\"",
                position + 1,
            ),
            span,
        ),
        ArgTypeMismatch {
            fn_name,
            position,
            expected,
            found,
            span,
        } => (
            format!(
                "Expected \"{expected}\" type to be an argument {} of the \"{fn_name}\" function,
                found \"{found}\"",
                position + 1
            ),
            span,
        ),

        UnknownFunction { span } => ("Unknown function".to_string(), span),

        UnsupportedNode { span } => ("Unsupported expression".to_string(), span),
    };

    utils::print_err(&info, Some("Semantic error"));

    if let Some(code) = get_source_code_slice(source_code, span.start) {
        print_err_source_code_pos(code.row, code.col);
        print_err_source_code_slice(&code.slice, code.col);
    }
}
