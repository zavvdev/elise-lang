use std::collections::HashMap;

use elise_aast::{AAstNode, AAstNodeData, AAstNodeDataPrim};
use elise_shared_types::Span;
use elise_test_utils::test_utils::semanalyze;

mod common;

#[test]
fn should_analyze_int() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze("42", &mut type_bindings_map, false);
    assert_eq!(
        result,
        Ok(vec![AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
            span: Span { start: 0, end: 2 },
            lexeme: "42".to_string()
        }))])
    );
}

#[test]
fn should_analyze_str() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r##""elise""##, &mut type_bindings_map, false);
    assert_eq!(
        result,
        Ok(vec![AAstNode::Data(AAstNodeData::Str(AAstNodeDataPrim {
            span: Span { start: 0, end: 7 },
            lexeme: "elise".to_string()
        }))])
    );
}
