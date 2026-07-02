use elise_errors::errors_semantic_analyzer::SemanticAnalyzerErr;

use crate::out::utils::{
    self, get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(sema_err: &SemanticAnalyzerErr, source_code: &[u8]) {
    use SemanticAnalyzerErr::*;

    let (info, span) = match sema_err {
        SymbolUndefined { span } => ("Undefined Symbol", span),
        SymbolDuplicate { span } => ("Duplicated Symbol", span),
    };

    utils::print_err(info, Some("Semantic error"));

    if let Some(code) = &get_source_code_slice(source_code, span.start) {
        print_err_source_code_pos(code.row, code.col);
        print_err_source_code_slice(&code.slice, code.col);
    };
}
