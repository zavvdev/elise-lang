use elise_semanalyzer::{Harmony, builtins::FnLet};
use elise_shared::{
    shared_errors::errors_semanalyzer::SemanalyzerErr, shared_node_names::NodeName,
    shared_types::ArityMismatchKind,
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
fn test_returns_err_if_no_args() {
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
fn test_returns_err_if_1_arg() {
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

#[test]
fn test_returns_err_if_first_arg_not_list() {
    let ast = test_utils::parse(".let(1, 2)");
    let result = Harmony::new(&ast).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArgKindMismatch {
            fn_name: FnLet::LEXEME,
            found: NodeName::INT,
            expected: NodeName::LIST,
            position: 0,
            ..
        })
    ));
}

#[test]
fn test_returns_err_if_bindings_elements_not_even() {
    let ast = test_utils::parse(".let([x 1 y] 2, 3)");
    let result = Harmony::new(&ast).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::LetInvalidBindingList { .. })
    ));
}

#[test]
fn test_returns_err_if_even_binding_elements_not_identifiers() {
    let ast = test_utils::parse(".let([1 2] 2, 3)");
    let result = Harmony::new(&ast).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::LetInvalidBindingList { .. })
    ));
}

#[test]
fn test_returns_err_if_binding_to_itself() {
    let ast = test_utils::parse(".let([x 1, y y] 2, 3)");
    let result = Harmony::new(&ast).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::IdentSelfBinding { .. })
    ));
}

// ==================================================================
//
//  ERROR CASES END
//
// ==================================================================
