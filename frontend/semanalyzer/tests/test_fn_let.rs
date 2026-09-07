use elise_semanalyzer::{Harmony, config::FnLet};
use elise_shared::{
    shared_errors::errors_semanalyzer::SemanalyzerErr, shared_types::ArityMismatchKind,
};
use elise_test_utils::test_utils;

mod common;

// ==================================================================
//
//  SUCCESS CASES START
//
// ==================================================================

// TODO

// ==================================================================
//
//  SUCCESS CASES END
//
// ==================================================================

// ==================================================================
//
//  ERROR CASES START
//
// ==================================================================

#[test]
fn test_returns_arity_mismatch_if_no_args() {
    let ast = test_utils::parse(".let()");
    let result = Harmony::new(&ast).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FnLet::LEXEME,
            found: 0,
            kind: ArityMismatchKind::MoreEq(FnLet::MIN_ARGS_LEN),
            ..
        })
    ));
}

#[test]
fn test_returns_arity_mismatch_if_1_arg() {
    let ast = test_utils::parse(".let([])");
    let result = Harmony::new(&ast).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FnLet::LEXEME,
            found: 1,
            kind: ArityMismatchKind::MoreEq(FnLet::MIN_ARGS_LEN),
            ..
        })
    ));
}

// ==================================================================
//
//  ERROR CASES END
//
// ==================================================================
