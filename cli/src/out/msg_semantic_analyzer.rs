use elise_errors::errors_semantic_analyzer::SemanticAnalyzerErr;

use crate::out::utils::{self};

pub fn print_err(sema_err: &SemanticAnalyzerErr) {
    use SemanticAnalyzerErr::*;

    let info = match sema_err {
        UnknownSymbol => "Unknown symbol",
    };

    utils::print_err(info, Some("Semantic error"));
}
