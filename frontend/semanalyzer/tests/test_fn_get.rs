use std::collections::HashMap;

use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_test_utils::test_utils::{semanalyze, type_bindings_map};

mod common;

const TYPEDEF_MODE: bool = false;

// ==================================================================
//
// ERROR CASES START
//
// ==================================================================

#[test]
fn should_fail_with_arity_mismatch() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".get()", &mut type_bindings_map, TYPEDEF_MODE);
    assert!(matches!(result, Err(SemanalyzerErr::ArityMismatch { .. })));
}

#[test]
fn should_fail_for_invalid_arg() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".get(:Int)", &mut type_bindings_map, TYPEDEF_MODE);
    assert!(matches!(result, Err(SemanalyzerErr::UnexpNode { .. })));
}

#[test]
fn should_fail_if_no_typedef_for_slot() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".get(@data)", &mut type_bindings_map, TYPEDEF_MODE);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::NoTypeDefinition { .. })
    ));
}

#[test]
fn should_fail_if_cannot_infer_type() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(
        ".get(.typedef(A :Int))",
        &mut type_bindings_map,
        TYPEDEF_MODE,
    );
    assert!(matches!(
        result,
        Err(SemanalyzerErr::CannotInferType { .. })
    ));
}

#[test]
fn should_fail_for_type_annot_in_path() {
    let mut type_bindings_map = type_bindings_map();
    let result = semanalyze(".get(@data, :Int)", &mut type_bindings_map, TYPEDEF_MODE);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::InvalDataAccessArg { .. })
    ));
}

#[test]
fn should_fail_for_other_than_int_or_str_in_path() {
    let mut type_bindings_map = type_bindings_map();
    let result = semanalyze(
        ".get(@data, [1, 2, 3])",
        &mut type_bindings_map,
        TYPEDEF_MODE,
    );
    assert!(matches!(
        result,
        Err(SemanalyzerErr::InvalDataAccessArg { .. })
    ));
}

#[test]
fn should_fail_if_accessing_not_resolvable() {
    let mut type_bindings_map = type_bindings_map();
    let result = semanalyze(
        ".get(@data, 0, 1, 2, 3)",
        &mut type_bindings_map,
        TYPEDEF_MODE,
    );
    assert!(matches!(
        result,
        Err(SemanalyzerErr::UnresolvablePath { .. })
    ));
}

// ==================================================================
//
// ERROR CASES END
//
// ==================================================================

// ==================================================================
//
// SUCCESS CASES START
//
// ==================================================================

// TODO

// ==================================================================
//
// SUCCESS CASES END
//
// ==================================================================
