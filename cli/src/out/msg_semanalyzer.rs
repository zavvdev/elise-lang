use elise_errors::errors_semanalyzer::SemanalyzerErr;

use crate::out::utils::{
    self, get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(sema_err: &SemanalyzerErr, source_code: &[u8]) {
    use SemanalyzerErr::*;

    let (info, span) = match sema_err {
        SymbolUndefined { span } => ("Undefined Symbol", Some(span)),
        SymbolDuplicate { span } => ("Duplicated Symbol", Some(span)),
        Unknown => ("Unexpected error", None),
    };

    utils::print_err(info, Some("Semantic error"));

    if span.is_some()
        && let Some(code) = &get_source_code_slice(source_code, span.unwrap().start)
    {
        print_err_source_code_pos(code.row, code.col);
        print_err_source_code_slice(&code.slice, code.col);
    };
}
