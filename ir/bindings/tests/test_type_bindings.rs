mod common;

use elise_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    common::BindingType,
    type_bindings::{TypeBinding, TypeBindingDesc},
};
use elise_shared_types::Span;

#[test]
fn should_strip_binding_paths_by_prefix() {
    let mut type_binding = TypeBinding::new();

    type_binding.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            span: Span { start: 0, end: 0 },
            dtype: BindingType::List,
        },
    );
    type_binding.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            span: Span { start: 1, end: 1 },
            dtype: BindingType::Record,
        },
    );
    type_binding.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            span: Span { start: 2, end: 2 },
            dtype: BindingType::Int,
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
            dtype: BindingType::Record,
        },
    );
    expected_result.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            span: Span { start: 2, end: 2 },
            dtype: BindingType::Int,
        },
    );

    assert_eq!(result, expected_result);
}
