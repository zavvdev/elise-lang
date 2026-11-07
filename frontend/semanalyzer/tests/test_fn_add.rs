use std::collections::HashMap;

use elise_aast::{
    AAstNode, AAstNodeCall, AAstNodeCallAdd, AAstNodeCallGet,
    aast_node_data::{AAstNodeData, AAstNodeDataPrim},
    aast_node_slot::AAstNodeSlot,
};
use elise_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    type_bindings::{TypeBinding, TypeBindingDataType, TypeBindingDesc},
};
use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_shared_types::Span;
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
    let result = semanalyze(".add()", &mut type_bindings_map, TYPEDEF_MODE);
    assert!(matches!(result, Err(SemanalyzerErr::ArityMismatch { .. })));
}

#[test]
fn should_fail_if_args_are_not_numbers() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(r#".add("2", "3")"#, &mut type_bindings_map, TYPEDEF_MODE);
    assert!(matches!(result, Err(SemanalyzerErr::UnexpType { .. })));
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
fn should_allow_single_arg() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".add(1)", &mut type_bindings_map, TYPEDEF_MODE);

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Add(AAstNodeCallAdd {
        span: Span { start: 0, end: 7 },
        arguments: vec![Box::new(AAstNode::Data(AAstNodeData::Int(
            AAstNodeDataPrim {
                span: Span { start: 5, end: 6 },
                lexeme: "1".to_string(),
            },
        )))],
    }))];

    assert_eq!(result, Ok(expected_result));
}

#[test]
fn should_allow_more_than_one_arg() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".add(1, 2, 3)", &mut type_bindings_map, TYPEDEF_MODE);

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Add(AAstNodeCallAdd {
        span: Span { start: 0, end: 13 },
        arguments: vec![
            Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                span: Span { start: 5, end: 6 },
                lexeme: "1".to_string(),
            }))),
            Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                span: Span { start: 8, end: 9 },
                lexeme: "2".to_string(),
            }))),
            Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                span: Span { start: 11, end: 12 },
                lexeme: "3".to_string(),
            }))),
        ],
    }))];

    assert_eq!(result, Ok(expected_result));
}

#[test]
fn should_allow_nested_add_calls() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".add(1, .add(2, 3))", &mut type_bindings_map, TYPEDEF_MODE);

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Add(AAstNodeCallAdd {
        span: Span { start: 0, end: 19 },
        arguments: vec![
            Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                span: Span { start: 5, end: 6 },
                lexeme: "1".to_string(),
            }))),
            Box::new(AAstNode::Call(AAstNodeCall::Add(AAstNodeCallAdd {
                span: Span { start: 8, end: 18 },
                arguments: vec![
                    Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                        span: Span { start: 13, end: 14 },
                        lexeme: "2".to_string(),
                    }))),
                    Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                        span: Span { start: 16, end: 17 },
                        lexeme: "3".to_string(),
                    }))),
                ],
            }))),
        ],
    }))];

    assert_eq!(result, Ok(expected_result));
}

#[test]
fn should_allow_get_call_if_accessing_numbers() {
    let mut type_bindings_map = type_bindings_map();
    let result = semanalyze(
        r#".add(1, .get(@data, 0, "id"))"#,
        &mut type_bindings_map,
        TYPEDEF_MODE,
    );

    let mut return_type_binding = HashMap::new();
    return_type_binding.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Add(AAstNodeCallAdd {
        span: Span { start: 0, end: 29 },
        arguments: vec![
            Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                span: Span { start: 5, end: 6 },
                lexeme: "1".to_string(),
            }))),
            Box::new(AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
                span: Span { start: 8, end: 28 },
                data_node: Box::new(AAstNode::SlotData(AAstNodeSlot {
                    span: Span { start: 13, end: 18 },
                })),
                data_access_nodes: vec![
                    Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                        lexeme: "0".to_string(),
                        span: Span { start: 20, end: 21 },
                    }))),
                    Box::new(AAstNode::Data(AAstNodeData::Str(AAstNodeDataPrim {
                        lexeme: "id".to_string(),
                        span: Span { start: 23, end: 27 },
                    }))),
                ],
                type_binding_path: BindingPath::with_segments(vec![
                    BindingPathSegment::AbstractIndex,
                    BindingPathSegment::Field("id".to_string()),
                ]),
                data_binding_path: BindingPath::with_segments(vec![
                    BindingPathSegment::Index(0),
                    BindingPathSegment::Field("id".to_string()),
                ]),
                return_type_binding: TypeBinding::with(&return_type_binding),
            }))),
        ],
    }))];

    assert_eq!(result, Ok(expected_result));
}

// ==================================================================
//
// SUCCESS CASES END
//
// ==================================================================
