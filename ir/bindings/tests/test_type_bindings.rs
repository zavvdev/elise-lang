mod common;

use std::collections::HashMap;

use elise_ir_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    type_bindings::{TypeBinding, TypeBindingDataType, TypeBindingDesc},
};
use elise_test_utils::test_utils;

// ==================================================================
//
// strip_binding_paths_by_prefix START
//
// ==================================================================

#[test]
fn should_strip_binding_paths_by_prefix() {
    let mut type_binding = TypeBinding::new();

    type_binding.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            span: test_utils::span(),
            dtype: TypeBindingDataType::List,
        },
    );
    type_binding.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            span: test_utils::span(),
            dtype: TypeBindingDataType::Record,
        },
    );
    type_binding.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            span: test_utils::span(),
            dtype: TypeBindingDataType::Int,
        },
    );

    let result = type_binding.strip_binding_paths_by_prefix(&BindingPath::with_segments(vec![
        BindingPathSegment::AbstractIndex,
    ]));

    let mut expected_result = TypeBinding::new();

    expected_result.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            span: test_utils::span(),
            dtype: TypeBindingDataType::Record,
        },
    );
    expected_result.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            span: test_utils::span(),
            dtype: TypeBindingDataType::Int,
        },
    );

    assert_eq!(result, expected_result);
}

// ==================================================================
//
// strip_binding_paths_by_prefix END
//
// ==================================================================

// ==================================================================
//
// new_single_binding START
//
// ==================================================================

#[test]
fn should_create_new_single_binding() {
    let result = TypeBinding::new_single_binding(TypeBindingDataType::Int, test_utils::span());

    let mut expected_result_map = HashMap::new();
    expected_result_map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );
    let expected_result = TypeBinding::with(&expected_result_map);

    assert_eq!(result, expected_result);
}

// ==================================================================
//
// new_single_binding END
//
// ==================================================================

// ==================================================================
//
// is START
//
// ==================================================================

#[test]
fn is_should_return_true_if_dtype_matches() {
    let result = TypeBinding::new_single_binding(TypeBindingDataType::Int, test_utils::span());
    assert_eq!(result.is(TypeBindingDataType::Int), true);
}

#[test]
fn is_should_return_false_if_dtype_does_not_match() {
    let result = TypeBinding::new_single_binding(TypeBindingDataType::Str, test_utils::span());
    assert_eq!(result.is(TypeBindingDataType::Int), false);
}

// ==================================================================
//
// is END
//
// ==================================================================
