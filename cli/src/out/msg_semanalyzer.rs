use elise_shared::shared_errors::errors_semanalyzer::SemanalyzerErr;

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
                kind.as_str(),
                found,
            ),
            span,
        ),

        UnknownFunction { span } => ("Unknown function".to_string(), span),

        UnsupportedNode { span } => ("Unsupported expression".to_string(), span),

        ExpectedTypedef { span } => ("Expected type definition".to_string(), span),

        UnknownTypedef { span } => ("Unknown type definition".to_string(), span),

        ExpectedIdentifier { span } => ("Expected identifier".to_string(), span),

        UnexpectedGeneric { span } => ("Unexpected generic".to_string(), span),

        ExpectedGeneric { span } => ("Expected generic".to_string(), span),

        InvalidGeneric { span } => ("Invalid generic".to_string(), span),

        UnresolvableTypedef { span } => ("Unresolvable type definition".to_string(), span),

        TypedefNoReferenceItself { span } => (
            "Referencing itself in type definition is not supported".to_string(),
            span,
        ),

        TypedefMode { span } => (
            "Only type definitions are supported in this context".to_string(),
            span,
        ),
    };

    utils::print_err(&info, Some("Semantic error"));

    if let Some(code) = get_source_code_slice(source_code, span.start) {
        print_err_source_code_pos(code.pos.row, code.pos.col);
        print_err_source_code_slice(&code.slice, code.pos.col);
    }
}
