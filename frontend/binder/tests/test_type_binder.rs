use std::collections::HashMap;

use elise_aast::AAstNodeTypedef;
use elise_binder::type_binder::TypeBinder;
use elise_bindings::{
    BindingType, TypeBinding, TypeBindingDesc,
    binding_path::{BindingPath, BindingPathSegment},
};
use elise_shared::{shared_errors::errors_type_binder::TypeBinderErr, shared_types::Span};
use elise_test_utils::test_utils;

mod common;

// ==================================================================
//
// INT START
//
// ==================================================================

#[test]
fn should_bind_int() {
    let int_aast = AAstNodeTypedef::Int {
        span: Span { start: 0, end: 0 },
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&int_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    let binding_path = BindingPath::new();
    let binding_desc = TypeBindingDesc {
        dtype: BindingType::Int,
        span: Span { start: 0, end: 0 },
    };

    expected.insert(binding_path, binding_desc);

    assert_eq!(binding, expected);
}

// ==================================================================
//
// INT END
//
// ==================================================================

// ==================================================================
//
// LIST START
//
// ==================================================================

#[test]
fn should_bind_list_with_primitive_generic() {
    let list_aast = AAstNodeTypedef::List {
        span: Span { start: 0, end: 1 },
        item_type: Box::new(AAstNodeTypedef::Int {
            span: Span { start: 2, end: 3 },
        }),
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 2, end: 3 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_list_with_list_generic() {
    let list_aast = AAstNodeTypedef::List {
        span: Span { start: 0, end: 1 },
        item_type: Box::new(AAstNodeTypedef::List {
            span: Span { start: 2, end: 3 },
            item_type: Box::new(AAstNodeTypedef::Int {
                span: Span { start: 4, end: 5 },
            }),
        }),
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 2, end: 3 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::AbstractIndex,
        ]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 4, end: 5 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_list_with_record_generic() {
    let list_aast = AAstNodeTypedef::List {
        span: Span { start: 0, end: 1 },
        item_type: Box::new(AAstNodeTypedef::Record {
            span: Span { start: 2, end: 3 },
            entries: vec![(
                "id".to_string(),
                Box::new(AAstNodeTypedef::Int {
                    span: Span { start: 4, end: 5 },
                }),
            )],
        }),
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 2, end: 3 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 4, end: 5 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_list_with_custom_type_generic() {
    let list_aast = AAstNodeTypedef::List {
        span: Span { start: 0, end: 1 },
        item_type: Box::new(AAstNodeTypedef::Custom {
            span: Span { start: 2, end: 3 },
            alias: "RecordType".to_string(),
        }),
    };

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 0 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    assert_eq!(binding, expected);
}

// ==================================================================
//
// LIST END
//
// ==================================================================

// ==================================================================
//
// RECORD START
//
// ==================================================================

#[test]
fn should_bind_record_with_primitive_values() {
    let record_aast = AAstNodeTypedef::Record {
        span: Span { start: 0, end: 1 },
        entries: vec![(
            "id".to_string(),
            Box::new(AAstNodeTypedef::Int {
                span: Span { start: 2, end: 3 },
            }),
        )],
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 2, end: 3 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_record_with_list_values() {
    let record_aast = AAstNodeTypedef::Record {
        span: Span { start: 0, end: 1 },
        entries: vec![(
            "scores".to_string(),
            Box::new(AAstNodeTypedef::List {
                span: Span { start: 2, end: 3 },
                item_type: Box::new(AAstNodeTypedef::Int {
                    span: Span { start: 4, end: 5 },
                }),
            }),
        )],
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("scores".to_string())]),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 2, end: 3 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("scores".to_string()),
            BindingPathSegment::AbstractIndex,
        ]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 4, end: 5 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_record_with_record_values() {
    let record_aast = AAstNodeTypedef::Record {
        span: Span { start: 0, end: 1 },
        entries: vec![(
            "user".to_string(),
            Box::new(AAstNodeTypedef::Record {
                span: Span { start: 2, end: 3 },
                entries: vec![(
                    "id".to_string(),
                    Box::new(AAstNodeTypedef::Int {
                        span: Span { start: 4, end: 5 },
                    }),
                )],
            }),
        )],
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("user".to_string())]),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 2, end: 3 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("user".to_string()),
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 4, end: 5 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_record_with_custom_type_values() {
    let record_aast = AAstNodeTypedef::Record {
        span: Span { start: 0, end: 1 },
        entries: vec![(
            "test".to_string(),
            Box::new(AAstNodeTypedef::Custom {
                alias: "RecordType".to_string(),
                span: Span { start: 2, end: 3 },
            }),
        )],
    };

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 1 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("test".to_string())]),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 0 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("test".to_string()),
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    assert_eq!(binding, expected);
}

// ==================================================================
//
// RECORD END
//
// ==================================================================

// ==================================================================
//
// CUSTOM START
//
// ==================================================================

#[test]
fn should_bind_custom_primitive() {
    let custom_aast = AAstNodeTypedef::Custom {
        span: Span { start: 0, end: 1 },
        alias: "SingleType".to_string(),
    };

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&custom_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_custom_list() {
    let custom_aast = AAstNodeTypedef::Custom {
        span: Span { start: 0, end: 1 },
        alias: "ListType".to_string(),
    };

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&custom_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 0, end: 0 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_custom_record() {
    let custom_aast = AAstNodeTypedef::Custom {
        span: Span { start: 0, end: 1 },
        alias: "RecordType".to_string(),
    };

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&custom_aast, &globals).bind().unwrap();

    let mut expected: TypeBinding = HashMap::new();

    expected.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 0, end: 0 },
        },
    );
    expected.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 0, end: 0 },
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_return_err_if_custom_is_not_in_globals() {
    let custom_aast = AAstNodeTypedef::Custom {
        span: Span { start: 0, end: 1 },
        alias: "SingleType".to_string(),
    };

    let globals = HashMap::new();
    let binding = TypeBinder::new(&custom_aast, &globals).bind();

    assert_eq!(
        binding,
        Err(TypeBinderErr::UnknownTypedef {
            span: Span { start: 0, end: 1 }
        })
    );
}

// ==================================================================
//
// CUSTOM END
//
// ==================================================================
