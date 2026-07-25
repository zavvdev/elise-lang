use elise_semanalyzer::{
    Harmony,
    semanalyzer_config::{FN_LET_LEXEME, FN_LET_MIN_ARGS_LEN},
};
use elise_shared::shared_errors::errors_semanalyzer::{ArityMismatchKind, SemanalyzerErr};

use crate::common::{empty_data_bindings, parse};

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
    let ast = parse(".let()");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FN_LET_LEXEME,
            expected: FN_LET_MIN_ARGS_LEN,
            found: 0,
            kind: ArityMismatchKind::MoreEq,
            ..
        })
    ));
}

#[test]
fn test_returns_arity_mismatch_if_1_arg() {
    let ast = parse(".let([])");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FN_LET_LEXEME,
            expected: FN_LET_MIN_ARGS_LEN,
            found: 1,
            kind: ArityMismatchKind::MoreEq,
            ..
        })
    ));
}

// ==================================================================
//
//  ERROR CASES END
//
// ==================================================================
