use elise_frontend_binder::data_binder::DataBinder;
use elise_ir_aast::aast_node_data::{
    AAstNodeData, AAstNodeDataDict, AAstNodeDataDictKey, AAstNodeDataList, AAstNodeDataPrim,
};
use elise_ir_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    data_bindings::{
        DataBinding, DataBindingDesc, DataBindingDescDict, DataBindingDescList, DataBindingDescPrim,
    },
};
use elise_test_utils::test_utils;

mod common;

// ==================================================================
//
// PRIMITIVE START
//
// ==================================================================

#[test]
fn should_bind_primitive() {
    let int_aast = AAstNodeData::Int(AAstNodeDataPrim {
        lexeme: "1".to_string(),
        span: test_utils::span(),
    });

    let result = DataBinder::new(&int_aast).bind().unwrap();

    let mut expected = DataBinding::new();

    let binding_path = BindingPath::new();
    let binding_desc = DataBindingDesc::Int(DataBindingDescPrim {
        value: "1".to_string(),
        span: test_utils::span(),
    });

    expected.map.insert(binding_path, binding_desc);
    assert_eq!(result, expected);
}

// ==================================================================
//
// PRIMITIVE END
//
// ==================================================================

// ==================================================================
//
// LIST START
//
// ==================================================================

#[test]
fn should_bind_flat_list() {
    let list_aast = AAstNodeData::List(AAstNodeDataList {
        span: test_utils::span(),
        items: vec![
            Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                lexeme: "1".to_string(),
                span: test_utils::span(),
            })),
            Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                lexeme: "2".to_string(),
                span: test_utils::span(),
            })),
        ],
    });

    let result = DataBinder::new(&list_aast).bind().unwrap();

    let mut expected = DataBinding::new();
    expected.map.insert(
        BindingPath::new(),
        DataBindingDesc::List(DataBindingDescList {
            span: test_utils::span(),
            size: Some(2),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(0)]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(1)]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "2".to_string(),
            span: test_utils::span(),
        }),
    );

    assert_eq!(result, expected);
}

#[test]
fn should_bind_nested_list() {
    let list_aast = AAstNodeData::List(AAstNodeDataList {
        span: test_utils::span(),
        items: vec![Box::new(AAstNodeData::List(AAstNodeDataList {
            span: test_utils::span(),
            items: vec![
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "1".to_string(),
                    span: test_utils::span(),
                })),
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "2".to_string(),
                    span: test_utils::span(),
                })),
            ],
        }))],
    });

    let result = DataBinder::new(&list_aast).bind().unwrap();

    let mut expected = DataBinding::new();
    expected.map.insert(
        BindingPath::new(),
        DataBindingDesc::List(DataBindingDescList {
            span: test_utils::span(),
            size: Some(1),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(0)]),
        DataBindingDesc::List(DataBindingDescList {
            span: test_utils::span(),
            size: Some(2),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Index(0),
        ]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Index(1),
        ]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "2".to_string(),
            span: test_utils::span(),
        }),
    );

    assert_eq!(result, expected);
}

// ==================================================================
//
// LIST END
//
// ==================================================================

// ==================================================================
//
// DICT START
//
// ==================================================================

#[test]
fn should_bind_flat_dict() {
    let dict_aast = AAstNodeData::Dict(AAstNodeDataDict {
        span: test_utils::span(),
        entries: vec![
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "id".to_string(),
                },
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "1".to_string(),
                    span: test_utils::span(),
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "name".to_string(),
                },
                Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                    lexeme: "John".to_string(),
                    span: test_utils::span(),
                })),
            ),
        ],
    });

    let result = DataBinder::new(&dict_aast).bind().unwrap();

    let mut expected = DataBinding::new();
    expected.map.insert(
        BindingPath::new(),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("name".to_string())]),
        DataBindingDesc::Str(DataBindingDescPrim {
            value: "John".to_string(),
            span: test_utils::span(),
        }),
    );

    assert_eq!(result, expected);
}

#[test]
fn should_bind_nested_dict() {
    let nested_dict = AAstNodeData::Dict(AAstNodeDataDict {
        span: test_utils::span(),
        entries: vec![
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "id".to_string(),
                },
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "1".to_string(),
                    span: test_utils::span(),
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "name".to_string(),
                },
                Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                    lexeme: "John".to_string(),
                    span: test_utils::span(),
                })),
            ),
        ],
    });
    let dict_aast = AAstNodeData::Dict(AAstNodeDataDict {
        span: test_utils::span(),
        entries: vec![(
            AAstNodeDataDictKey {
                span: test_utils::span(),
                lexeme: "user".to_string(),
            },
            Box::new(nested_dict),
        )],
    });

    let result = DataBinder::new(&dict_aast).bind().unwrap();

    let mut expected = DataBinding::new();
    expected.map.insert(
        BindingPath::new(),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("user".to_string())]),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("user".to_string()),
            BindingPathSegment::Field("id".to_string()),
        ]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Field("user".to_string()),
            BindingPathSegment::Field("name".to_string()),
        ]),
        DataBindingDesc::Str(DataBindingDescPrim {
            value: "John".to_string(),
            span: test_utils::span(),
        }),
    );

    assert_eq!(result, expected);
}

// ==================================================================
//
// DICT END
//
// ==================================================================

// ==================================================================
//
// MIXED START
//
// ==================================================================

#[test]
fn should_bind_mixed_data() {
    let dict1 = AAstNodeData::Dict(AAstNodeDataDict {
        span: test_utils::span(),
        entries: vec![
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "id".to_string(),
                },
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "1".to_string(),
                    span: test_utils::span(),
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "name".to_string(),
                },
                Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                    lexeme: "John".to_string(),
                    span: test_utils::span(),
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "address".to_string(),
                },
                Box::new(AAstNodeData::Dict(AAstNodeDataDict {
                    span: test_utils::span(),
                    entries: vec![(
                        AAstNodeDataDictKey {
                            span: test_utils::span(),
                            lexeme: "street".to_string(),
                        },
                        Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                            span: test_utils::span(),
                            lexeme: "Washington St.".to_string(),
                        })),
                    )],
                })),
            ),
        ],
    });
    let dict2 = AAstNodeData::Dict(AAstNodeDataDict {
        span: test_utils::span(),
        entries: vec![
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "id".to_string(),
                },
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "2".to_string(),
                    span: test_utils::span(),
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "name".to_string(),
                },
                Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                    lexeme: "Jane".to_string(),
                    span: test_utils::span(),
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: test_utils::span(),
                    lexeme: "address".to_string(),
                },
                Box::new(AAstNodeData::Dict(AAstNodeDataDict {
                    span: test_utils::span(),
                    entries: vec![(
                        AAstNodeDataDictKey {
                            span: test_utils::span(),
                            lexeme: "street".to_string(),
                        },
                        Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                            span: test_utils::span(),
                            lexeme: "Wall St.".to_string(),
                        })),
                    )],
                })),
            ),
        ],
    });
    let data_aast = AAstNodeData::List(AAstNodeDataList {
        span: test_utils::span(),
        items: vec![Box::new(dict1), Box::new(dict2)],
    });

    let result = DataBinder::new(&data_aast).bind().unwrap();

    let mut expected = DataBinding::new();

    expected.map.insert(
        BindingPath::new(),
        DataBindingDesc::List(DataBindingDescList {
            span: test_utils::span(),
            size: Some(2),
        }),
    );

    // =================== Dict 1 ===================

    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(0)]),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Field("id".to_string()),
        ]),
        DataBindingDesc::Int(DataBindingDescPrim {
            span: test_utils::span(),
            value: "1".to_string(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Field("name".to_string()),
        ]),
        DataBindingDesc::Str(DataBindingDescPrim {
            span: test_utils::span(),
            value: "John".to_string(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Field("address".to_string()),
        ]),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Field("address".to_string()),
            BindingPathSegment::Field("street".to_string()),
        ]),
        DataBindingDesc::Str(DataBindingDescPrim {
            span: test_utils::span(),
            value: "Washington St.".to_string(),
        }),
    );

    // =================== Dict 2 ===================

    expected.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(1)]),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(1),
            BindingPathSegment::Field("id".to_string()),
        ]),
        DataBindingDesc::Int(DataBindingDescPrim {
            span: test_utils::span(),
            value: "2".to_string(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(1),
            BindingPathSegment::Field("name".to_string()),
        ]),
        DataBindingDesc::Str(DataBindingDescPrim {
            span: test_utils::span(),
            value: "Jane".to_string(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(1),
            BindingPathSegment::Field("address".to_string()),
        ]),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );
    expected.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(1),
            BindingPathSegment::Field("address".to_string()),
            BindingPathSegment::Field("street".to_string()),
        ]),
        DataBindingDesc::Str(DataBindingDescPrim {
            span: test_utils::span(),
            value: "Wall St.".to_string(),
        }),
    );

    assert_eq!(result, expected);
}

// ==================================================================
//
// MIXED END
//
// ==================================================================
