use std::collections::HashMap;

use elise_aast::{
    AAstNode, AAstNodeCall, AAstNodeCallGet,
    aast_node_data::{AAstNodeData, AAstNodeDataPrim},
    aast_node_slot::AAstNodeSlot,
};
use elise_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    type_bindings::{AliasedTypeBindings, TypeBinding, TypeBindingDataType, TypeBindingDesc},
};
use elise_semanalyzer::builtins::SlotData;
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

#[test]
fn should_accept_a_single_data_arg() {
    let mut type_bindings_map: AliasedTypeBindings = HashMap::new();

    let mut data_slot_bindings = HashMap::new();
    data_slot_bindings.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    type_bindings_map.insert(
        SlotData::TYPEDEF_LEXEME.to_string(),
        TypeBinding::with(&data_slot_bindings),
    );

    let result = semanalyze(".get(@data)", &mut type_bindings_map, TYPEDEF_MODE);

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
        span: Span { start: 0, end: 11 },
        data_node: Box::new(AAstNode::SlotData(AAstNodeSlot {
            span: Span { start: 5, end: 10 },
        })),
        data_access_nodes: vec![],
        type_binding_path: BindingPath::new(),
        data_binding_path: BindingPath::new(),
        return_type_binding: TypeBinding::with(&data_slot_bindings),
    }))];

    assert_eq!(result, Ok(expected_result));
}

#[test]
fn should_access_data_by_int() {
    let mut type_bindings_map = type_bindings_map();
    let result = semanalyze(".get(@data, 0)", &mut type_bindings_map, TYPEDEF_MODE);

    let mut return_type_binding = HashMap::new();
    return_type_binding.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: Span { start: 0, end: 0 },
        },
    );
    return_type_binding.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: Span { start: 0, end: 0 },
        },
    );
    return_type_binding.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("name".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Str,
            span: Span { start: 0, end: 0 },
        },
    );

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
        span: Span { start: 0, end: 14 },
        data_node: Box::new(AAstNode::SlotData(AAstNodeSlot {
            span: Span { start: 5, end: 10 },
        })),
        data_access_nodes: vec![Box::new(AAstNode::Data(AAstNodeData::Int(
            AAstNodeDataPrim {
                lexeme: "0".to_string(),
                span: Span { start: 12, end: 13 },
            },
        )))],
        type_binding_path: BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        data_binding_path: BindingPath::with_segments(vec![BindingPathSegment::Index(0)]),
        return_type_binding: TypeBinding::with(&return_type_binding),
    }))];

    assert_eq!(result, Ok(expected_result));
}

#[test]
fn should_access_data_by_str() {
    let mut type_bindings_map: AliasedTypeBindings = HashMap::new();
    let mut data_slot_bindings = HashMap::new();
    data_slot_bindings.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: Span { start: 0, end: 0 },
        },
    );
    data_slot_bindings.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("name".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Str,
            span: Span { start: 0, end: 0 },
        },
    );
    type_bindings_map.insert(
        SlotData::TYPEDEF_LEXEME.to_string(),
        TypeBinding::with(&data_slot_bindings),
    );
    let result = semanalyze(
        r#".get(@data, "name")"#,
        &mut type_bindings_map,
        TYPEDEF_MODE,
    );

    let mut return_type_binding = HashMap::new();
    return_type_binding.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Str,
            span: Span { start: 0, end: 0 },
        },
    );

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
        span: Span { start: 0, end: 19 },
        data_node: Box::new(AAstNode::SlotData(AAstNodeSlot {
            span: Span { start: 5, end: 10 },
        })),
        data_access_nodes: vec![Box::new(AAstNode::Data(AAstNodeData::Str(
            AAstNodeDataPrim {
                lexeme: "name".to_string(),
                span: Span { start: 12, end: 18 },
            },
        )))],
        type_binding_path: BindingPath::with_segments(vec![BindingPathSegment::Field(
            "name".to_string(),
        )]),
        data_binding_path: BindingPath::with_segments(vec![BindingPathSegment::Field(
            "name".to_string(),
        )]),
        return_type_binding: TypeBinding::with(&return_type_binding),
    }))];

    assert_eq!(result, Ok(expected_result));
}

#[test]
fn should_access_data_by_int_and_str() {
    let mut type_bindings_map = type_bindings_map();
    let result = semanalyze(
        r#".get(@data, 0, "id")"#,
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

    let expected_result = vec![AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
        span: Span { start: 0, end: 20 },
        data_node: Box::new(AAstNode::SlotData(AAstNodeSlot {
            span: Span { start: 5, end: 10 },
        })),
        data_access_nodes: vec![
            Box::new(AAstNode::Data(AAstNodeData::Int(AAstNodeDataPrim {
                lexeme: "0".to_string(),
                span: Span { start: 12, end: 13 },
            }))),
            Box::new(AAstNode::Data(AAstNodeData::Str(AAstNodeDataPrim {
                lexeme: "id".to_string(),
                span: Span { start: 15, end: 19 },
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
    }))];

    assert_eq!(result, Ok(expected_result));
}

// ==================================================================
//
// SUCCESS CASES END
//
// ==================================================================
