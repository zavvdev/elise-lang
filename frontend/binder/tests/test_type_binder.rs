use std::collections::HashMap;

use elise_frontend_binder::type_binder::TypeBinder;
use elise_ir_aast::aast_type_annot::{
    AAstNodeTypeAnnot, AAstNodeTypeAnnotCustom, AAstNodeTypeAnnotList, AAstNodeTypeAnnotPrim,
    AAstNodeTypeAnnotRecord,
};
use elise_ir_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    type_bindings::{TypeBinding, TypeBindingDataType, TypeBindingDesc},
};
use elise_shared_errors::errors_type_binder::TypeBinderErr;
use elise_test_utils::test_utils;

mod common;

// ==================================================================
//
// INT START
//
// ==================================================================

#[test]
fn should_bind_int() {
    let int_aast = AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
        span: test_utils::span(),
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&int_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    let binding_path = BindingPath::new();
    let binding_desc = TypeBindingDesc {
        dtype: TypeBindingDataType::Int,
        span: test_utils::span(),
    };

    expected.map.insert(binding_path, binding_desc);
    assert_eq!(binding, expected);
}

// ==================================================================
//
// INT END
//
// ==================================================================

// ==================================================================
//
// STR START
//
// ==================================================================

#[test]
fn should_bind_str() {
    let str_aast = AAstNodeTypeAnnot::Str(AAstNodeTypeAnnotPrim {
        span: test_utils::span(),
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&str_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    let binding_path = BindingPath::new();
    let binding_desc = TypeBindingDesc {
        dtype: TypeBindingDataType::Str,
        span: test_utils::span(),
    };

    expected.map.insert(binding_path, binding_desc);
    assert_eq!(binding, expected);
}

// ==================================================================
//
// STR END
//
// ==================================================================

// ==================================================================
//
// LIST START
//
// ==================================================================

#[test]
fn should_bind_list_with_primitive_generic() {
    let list_aast = AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
        span: test_utils::span(),
        item_type: Box::new(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
            span: test_utils::span(),
        })),
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_list_with_list_generic() {
    let list_aast = AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
        span: test_utils::span(),
        item_type: Box::new(AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
            span: test_utils::span(),
            item_type: Box::new(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
                span: test_utils::span(),
            })),
        })),
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::AbstractIndex,
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_list_with_record_generic() {
    let list_aast = AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
        span: test_utils::span(),
        item_type: Box::new(AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
            span: test_utils::span(),
            entries: vec![(
                "id".to_string(),
                Box::new(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
                    span: test_utils::span(),
                })),
            )],
        })),
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_list_with_custom_type_generic() {
    let list_aast = AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
        span: test_utils::span(),
        item_type: Box::new(AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
            span: test_utils::span(),
            alias: "RecordType".to_string(),
        })),
    });

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&list_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
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
    let record_aast = AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
        span: test_utils::span(),
        entries: vec![(
            "id".to_string(),
            Box::new(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
                span: test_utils::span(),
            })),
        )],
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_record_with_list_values() {
    let record_aast = AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
        span: test_utils::span(),
        entries: vec![(
            "scores".to_string(),
            Box::new(AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
                span: test_utils::span(),
                item_type: Box::new(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
                    span: test_utils::span(),
                })),
            })),
        )],
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("scores".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("scores".to_string()),
            BindingPathSegment::AbstractIndex,
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_record_with_record_values() {
    let record_aast = AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
        span: test_utils::span(),
        entries: vec![(
            "user".to_string(),
            Box::new(AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
                span: test_utils::span(),
                entries: vec![(
                    "id".to_string(),
                    Box::new(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
                        span: test_utils::span(),
                    })),
                )],
            })),
        )],
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("user".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("user".to_string()),
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_record_with_custom_type_values() {
    let record_aast = AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
        span: test_utils::span(),
        entries: vec![(
            "test".to_string(),
            Box::new(AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
                alias: "RecordType".to_string(),
                span: test_utils::span(),
            })),
        )],
    });

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&record_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("test".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("test".to_string()),
            BindingPathSegment::Field("id".to_string()),
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
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
    let custom_aast = AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
        span: test_utils::span(),
        alias: "SingleType".to_string(),
    });

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&custom_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_custom_list() {
    let custom_aast = AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
        span: test_utils::span(),
        alias: "ListType".to_string(),
    });

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&custom_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_bind_custom_record() {
    let custom_aast = AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
        span: test_utils::span(),
        alias: "RecordType".to_string(),
    });

    let globals = test_utils::type_bindings_map();
    let binding = TypeBinder::new(&custom_aast, &globals).bind().unwrap();

    let mut expected = TypeBinding::new();

    expected.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(binding, expected);
}

#[test]
fn should_return_err_if_custom_is_not_in_globals() {
    let custom_aast = AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
        span: test_utils::span(),
        alias: "SingleType".to_string(),
    });

    let globals = HashMap::new();
    let binding = TypeBinder::new(&custom_aast, &globals).bind();

    assert_eq!(
        binding,
        Err(TypeBinderErr::Unresolvable {
            span: test_utils::span(),
        })
    );
}

// ==================================================================
//
// CUSTOM END
//
// ==================================================================
