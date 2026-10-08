use std::collections::HashMap;

use elise_aast::AAstNode;
use elise_shared::shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_shared::shared_types::Span;
use elise_test_utils::test_utils::semanalyze;

mod common;

#[test]
fn should_fail_for_unknown_slot() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze("@unknown", &mut type_bindings_map, false);
    println!("result: {:#?}", result);
    assert!(matches!(result, Err(SemanalyzerErr::UnknownSlot { .. })));
}

#[test]
fn should_emit_data_slot_node() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze("@data", &mut type_bindings_map, false);
    assert_eq!(
        result,
        Ok(vec![AAstNode::SlotData {
            span: Span { start: 0, end: 5 }
        }])
    );
}
