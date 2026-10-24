use std::collections::HashMap;

use elise_aast::{
    AAstNode,
    aast_node_data::{AAstNodeData, AAstNodeDataList, AAstNodeDataPrim},
};
use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_shared_types::Span;
use elise_test_utils::test_utils::semanalyze;

mod common;

// ==================================================================
//
// ERROR CASES START
//
// ==================================================================

#[test]
fn should_fail_if_item_types_differ() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r##"[1, "2"]"##, &mut type_bindings_map, false);
    assert_eq!(
        result,
        Err(SemanalyzerErr::ListInconsistentItemTypes {
            span: Span { start: 4, end: 7 }
        })
    );
}

#[test]
fn should_fail_for_unsupported_items() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r##"[.typedef(Some :Int)]"##, &mut type_bindings_map, false);
    assert_eq!(
        result,
        Err(SemanalyzerErr::UnsupportedListItem {
            span: Span { start: 1, end: 20 }
        })
    );
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

#[test]
fn should_analyze_list() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r##"[42, 43]"##, &mut type_bindings_map, false);
    assert_eq!(
        result,
        Ok(vec![AAstNode::Data(AAstNodeData::List(AAstNodeDataList {
            span: Span { start: 0, end: 8 },
            items: vec![
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "42".to_string(),
                    span: Span { start: 1, end: 3 },
                })),
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "43".to_string(),
                    span: Span { start: 5, end: 7 },
                }))
            ]
        }))])
    );
}

#[test]
fn should_analyze_nested_list() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r##"[[1, 2]]"##, &mut type_bindings_map, false);

    let expected_result = Ok(vec![AAstNode::Data(AAstNodeData::List(AAstNodeDataList {
        span: Span { start: 0, end: 8 },
        items: vec![Box::new(AAstNodeData::List(AAstNodeDataList {
            span: Span { start: 1, end: 7 },
            items: vec![
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "1".to_string(),
                    span: Span { start: 2, end: 3 },
                })),
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "2".to_string(),
                    span: Span { start: 5, end: 6 },
                })),
            ],
        }))],
    }))]);

    assert_eq!(result, expected_result);
}

#[test]
fn should_analyze_nested_list_where_each_has_diff_item_types() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r##"[[1], ["2"]]"##, &mut type_bindings_map, false);

    let expected_result = Ok(vec![AAstNode::Data(AAstNodeData::List(AAstNodeDataList {
        span: Span { start: 0, end: 12 },
        items: vec![
            Box::new(AAstNodeData::List(AAstNodeDataList {
                span: Span { start: 1, end: 4 },
                items: vec![Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "1".to_string(),
                    span: Span { start: 2, end: 3 },
                }))],
            })),
            Box::new(AAstNodeData::List(AAstNodeDataList {
                span: Span { start: 6, end: 11 },
                items: vec![Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                    lexeme: "2".to_string(),
                    span: Span { start: 7, end: 10 },
                }))],
            })),
        ],
    }))]);

    assert_eq!(result, expected_result);
}

// ==================================================================
//
// SUCCESS CASES END
//
// ==================================================================
