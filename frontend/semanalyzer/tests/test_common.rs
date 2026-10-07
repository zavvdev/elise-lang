use std::collections::HashMap;

use elise_shared::shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_test_utils::test_utils::semanalyze;

mod common;

#[test]
fn should_fail_for_unknown_fn_call() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".unknown()", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::UnknownFunction { .. })
    ));
}
