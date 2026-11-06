mod common;

use std::collections::HashMap;

use elise_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    type_bindings::{TypeBinding, TypeBindingDataType, TypeBindingDesc},
};
use elise_shared_types::Span;

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
            span: Span { start: 0, end: 0 },
            dtype: TypeBindingDataType::List,
        },
    );
    type_binding.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            span: Span { start: 1, end: 1 },
            dtype: TypeBindingDataType::Record,
        },
    );
    type_binding.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            span: Span { start: 2, end: 2 },
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
            span: Span { start: 1, end: 1 },
            dtype: TypeBindingDataType::Record,
        },
    );
    expected_result.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            span: Span { start: 2, end: 2 },
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
    let result =
        TypeBinding::new_single_binding(TypeBindingDataType::Int, Span { start: 0, end: 0 });

    let mut expected_result_map = HashMap::new();
    expected_result_map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: Span { start: 0, end: 0 },
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
    let result =
        TypeBinding::new_single_binding(TypeBindingDataType::Int, Span { start: 0, end: 0 });
    assert_eq!(result.is(TypeBindingDataType::Int), true);
}

#[test]
fn is_should_return_false_if_dtype_does_not_match() {
    let result =
        TypeBinding::new_single_binding(TypeBindingDataType::Str, Span { start: 0, end: 0 });
    assert_eq!(result.is(TypeBindingDataType::Int), false);
}

// ==================================================================
//
// is END
//
// ==================================================================
