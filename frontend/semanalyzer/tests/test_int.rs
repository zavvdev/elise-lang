use std::collections::HashMap;

use elise_aast::{AAstNode, AAstNodeData};
use elise_shared::shared_types::Span;
use elise_test_utils::test_utils::semanalyze;

mod common;

#[test]
fn should_emit_aast_int() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze("42", &mut type_bindings_map, false);
    assert_eq!(
        result,
        Ok(vec![AAstNode::Data(AAstNodeData::Int {
            span: Span { start: 0, end: 2 },
            lexeme: "42".to_string()
        })])
    );
}
