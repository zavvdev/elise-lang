use elise_data::{
    binding_path::{BindingPath, BindingPathSegment::*},
    schema_binder::{
        ArgLen, SchemaBinder, SchemaBinderDataType, SchemaBinderTypeDescriptor, SchemaFnLexeme,
    },
};
use elise_shared::{
    shared_errors::errors_schema_binder::SchemaBinderErr,
    shared_types::{ArityMismatchKind, Span},
};

use crate::common::parse;

mod common;

// ==================================================================
//
// ERROR CASES START
//
// ==================================================================

// ==================================================================
// ROOT ERROR CASES START
// ==================================================================

#[test]
fn should_return_error_if_empty() {
    let ast = parse("");
    let bindings = SchemaBinder::new(&ast).bind();
    assert_eq!(bindings, Err(SchemaBinderErr::Empty));
}

#[test]
fn should_return_error_if_root_is_not_call() {
    let ast = parse("test-test");
    let bindings = SchemaBinder::new(&ast).bind();
    assert!(matches!(bindings, Err(SchemaBinderErr::Unexp { .. })));
}

#[test]
fn should_return_error_if_root_not_valid_call() {
    let ast = parse(".test(.string())");
    let bindings = SchemaBinder::new(&ast).bind();
    assert!(matches!(bindings, Err(SchemaBinderErr::Unexp { .. })));
}

#[test]
fn should_return_error_if_root_arg_len_is_0() {
    let ast = parse(".schema()");
    let bindings = SchemaBinder::new(&ast).bind();
    assert!(matches!(
        bindings,
        Err(SchemaBinderErr::ArityMismatch {
            fn_name: SchemaFnLexeme::ROOT,
            kind: ArityMismatchKind::Eq(ArgLen::ROOT),
            found: 0,
            ..
        })
    ));
}

#[test]
fn should_return_error_if_root_arg_len_is_more_than_1() {
    let ast = parse(".schema(.string(), .string())");
    let bindings = SchemaBinder::new(&ast).bind();
    assert!(matches!(
        bindings,
        Err(SchemaBinderErr::ArityMismatch {
            fn_name: SchemaFnLexeme::ROOT,
            kind: ArityMismatchKind::Eq(ArgLen::ROOT),
            found: 2,
            ..
        })
    ));
}

// ==================================================================
// ROOT ERROR CASES END
// ==================================================================

// ==================================================================
// PRIMITIVES ERROR CASES START
// ==================================================================

#[test]
fn should_return_error_if_primitive_has_arguments() {
    let inputs = vec![
        (".int(.int())", SchemaFnLexeme::INT),
        (".float(.float())", SchemaFnLexeme::FLOAT),
        (".string(.string())", SchemaFnLexeme::STRING),
        (".bool(.bool())", SchemaFnLexeme::BOOL),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input.0));
        let bindings = SchemaBinder::new(&ast).bind();
        match bindings {
            Err(SchemaBinderErr::ArityMismatch {
                fn_name,
                kind,
                found,
                ..
            }) => {
                assert_eq!(fn_name, input.1);
                assert_eq!(kind, ArityMismatchKind::Eq(ArgLen::PRIMITIVE));
                assert_eq!(found, 1);
            }
            other => panic!("expected ArityMismatch, got {:?}", other),
        }
    }
}

// ==================================================================
// PRIMITIVES ERROR CASES END
// ==================================================================

// ==================================================================
// COMPOUND ERROR CASES START
// ==================================================================

#[test]
fn should_return_error_if_dict_has_not_even_args() {
    let ast = parse(r#".schema(.dict("name" .string(), "age"))"#);
    let bindings = SchemaBinder::new(&ast).bind();
    assert!(matches!(bindings, Err(SchemaBinderErr::InvalDict { .. })));
}

#[test]
fn should_return_error_if_dict_invalid_keys() {
    let ast = parse(".schema(.dict(name .string(), age .int()))");
    let bindings = SchemaBinder::new(&ast).bind();
    assert!(matches!(bindings, Err(SchemaBinderErr::InvalDict { .. })));
}

// ==================================================================
// COMPOUND ERROR CASES END
// ==================================================================

// ==================================================================
// MODIFIERS ERROR CASES START
// ==================================================================

#[test]
fn should_return_error_if_modifiers_have_invalid_arity() {
    let inputs = vec![
        // Nullable
        (
            ".nullable()",
            SchemaFnLexeme::NULLABLE,
            ArityMismatchKind::Eq(ArgLen::NULLABLE),
            0,
        ),
        (
            ".nullable(.float(), .int())",
            SchemaFnLexeme::NULLABLE,
            ArityMismatchKind::Eq(ArgLen::NULLABLE),
            2,
        ),
        // Optional
        (
            ".optional()",
            SchemaFnLexeme::OPTIONAL,
            ArityMismatchKind::Eq(ArgLen::OPTIONAL),
            0,
        ),
        (
            ".optional(.float(), .int())",
            SchemaFnLexeme::OPTIONAL,
            ArityMismatchKind::Eq(ArgLen::OPTIONAL),
            2,
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input.0));
        let bindings = SchemaBinder::new(&ast).bind();

        match bindings {
            Err(SchemaBinderErr::ArityMismatch {
                fn_name,
                kind,
                found,
                ..
            }) => {
                assert_eq!(fn_name, input.1);
                assert_eq!(kind, input.2);
                assert_eq!(found, input.3);
            }
            other => panic!("expected ArityMismatch, got {:?}", other),
        }
    }
}

// OPTIONAL START

#[test]
fn should_return_error_if_optional_modifier_applied_to_list_item() {
    let inputs = vec![
        ".list(.nullable(.optional(.int())))",
        ".list(.optional(.int()))",
        ".list(.list(.optional(.int())))",
        r#".dict("x" .list(.optional(.int())))"#,
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input));
        let bindings = SchemaBinder::new(&ast).bind();

        assert!(matches!(
            bindings,
            Err(SchemaBinderErr::InvalUseOfModifier { .. })
        ));
    }
}

// OPTIONAL END

// ==================================================================
// MODIFIERS ERROR CASES END
// ==================================================================

// ==================================================================
//
//  ERROR CASES END
//
// ==================================================================

// ==================================================================
//
//  SUCCESS CASES START
//
// ==================================================================

// ==================================================================
// SINGLE PRIMITIVE SUCCESS CASES START
// ==================================================================

#[test]
fn should_resolve_single_primitive() {
    let inputs = vec![
        (
            ".int()",
            SchemaBinderDataType::Int,
            Span { start: 8, end: 14 },
        ),
        (
            ".float()",
            SchemaBinderDataType::Float,
            Span { start: 8, end: 16 },
        ),
        (
            ".string()",
            SchemaBinderDataType::String,
            Span { start: 8, end: 17 },
        ),
        (
            ".bool()",
            SchemaBinderDataType::Bool,
            Span { start: 8, end: 15 },
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input.0));
        let bindings = SchemaBinder::new(&ast).bind().unwrap();

        assert_eq!(
            *bindings.bindings.get(&BindingPath::new()).unwrap(),
            SchemaBinderTypeDescriptor {
                dtype: input.1,
                nullable: false,
                optional: false,
                span: input.2,
            }
        );
    }
}

// ==================================================================
// SINGLE PRIMITIVE SUCCESS CASES END
// ==================================================================

// ==================================================================
// SINGLE COMPOUND SUCCESS CASES START
// ==================================================================

#[test]
fn should_resolve_single_compound() {
    let inputs = vec![
        (
            r#".dict("name" .string())"#,
            SchemaBinderDataType::Dict,
            Span { start: 8, end: 31 },
        ),
        (
            ".list(.int())",
            SchemaBinderDataType::ListAbstract,
            Span { start: 8, end: 21 },
        ),
        (
            ".list(.int(), 2)",
            SchemaBinderDataType::ListFixed(2),
            Span { start: 8, end: 24 },
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input.0));
        let bindings = SchemaBinder::new(&ast).bind().unwrap();

        assert_eq!(
            *bindings.bindings.get(&BindingPath::new()).unwrap(),
            SchemaBinderTypeDescriptor {
                dtype: input.1,
                nullable: false,
                optional: false,
                span: input.2,
            }
        );
    }
}

// ==================================================================
// SINGLE COMPOUND SUCCESS CASES END
// ==================================================================

// ==================================================================
// MODIFIERS SUCCESS CASES START
// ==================================================================

// NULLABLE START

#[test]
fn should_resolve_one_nullable_child() {
    let inputs = vec![
        (
            ".int()",
            SchemaBinderDataType::Int,
            Span { start: 18, end: 24 },
        ),
        (
            ".float()",
            SchemaBinderDataType::Float,
            Span { start: 18, end: 26 },
        ),
        (
            ".string()",
            SchemaBinderDataType::String,
            Span { start: 18, end: 27 },
        ),
        (
            ".bool()",
            SchemaBinderDataType::Bool,
            Span { start: 18, end: 25 },
        ),
        (
            r#".dict("name" .string())"#,
            SchemaBinderDataType::Dict,
            Span { start: 18, end: 41 },
        ),
        (
            ".list(.int())",
            SchemaBinderDataType::ListAbstract,
            Span { start: 18, end: 31 },
        ),
        (
            ".list(.int(), 3)",
            SchemaBinderDataType::ListFixed(3),
            Span { start: 18, end: 34 },
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.nullable({}))", input.0));
        let bindings = SchemaBinder::new(&ast).bind().unwrap();

        assert_eq!(
            *bindings.bindings.get(&BindingPath::new()).unwrap(),
            SchemaBinderTypeDescriptor {
                dtype: input.1,
                nullable: true,
                optional: false,
                span: input.2,
            }
        );
    }
}

#[test]
fn should_resolve_nested_nullables() {
    let s = r##"
        .schema(
            .dict(
                "name"    .string()

                "email"   .nullable(.string())

                "address" .nullable(.dict(
                                        "street" .string()
                                        "house"  .nullable(.int())
                                        "state"  .dict(
                                                    "name" .string()
                                                    "code" .nullable(.string()))))
                
                "score"   .nullable(.float())
                
                "id"      .int()
            )
        )
    "##;

    let ast = parse(s);
    let bindings = SchemaBinder::new(&ast).bind().unwrap();

    let cases = vec![
        (
            BindingPath::new(),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 30,
                    end: 625,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("name".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span { start: 63, end: 72 },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("email".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: true,
                optional: false,
                span: Span {
                    start: 110,
                    end: 119,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("address".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: true,
                optional: false,
                span: Span {
                    start: 158,
                    end: 497,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 214,
                    end: 223,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: true,
                optional: false,
                span: Span {
                    start: 283,
                    end: 289,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 340,
                    end: 496,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("name".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 406,
                    end: 415,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("code".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: true,
                optional: false,
                span: Span {
                    start: 485,
                    end: 494,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("score".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Float,
                nullable: true,
                optional: false,
                span: Span {
                    start: 552,
                    end: 560,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("id".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: false,
                span: Span {
                    start: 605,
                    end: 611,
                },
            },
        ),
    ];

    for case in cases {
        assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
    }
}

// NULLABLE END

// OPTIONAL START

#[test]
fn should_resolve_one_optional_child() {
    let inputs = vec![
        (
            ".int()",
            SchemaBinderDataType::Int,
            Span { start: 18, end: 24 },
        ),
        (
            ".float()",
            SchemaBinderDataType::Float,
            Span { start: 18, end: 26 },
        ),
        (
            ".string()",
            SchemaBinderDataType::String,
            Span { start: 18, end: 27 },
        ),
        (
            ".bool()",
            SchemaBinderDataType::Bool,
            Span { start: 18, end: 25 },
        ),
        (
            r#".dict("name" .string())"#,
            SchemaBinderDataType::Dict,
            Span { start: 18, end: 41 },
        ),
        (
            ".list(.int())",
            SchemaBinderDataType::ListAbstract,
            Span { start: 18, end: 31 },
        ),
        (
            ".list(.int(), 2)",
            SchemaBinderDataType::ListFixed(2),
            Span { start: 18, end: 34 },
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.optional({}))", input.0));
        let bindings = SchemaBinder::new(&ast).bind().unwrap();

        assert_eq!(
            *bindings.bindings.get(&BindingPath::new()).unwrap(),
            SchemaBinderTypeDescriptor {
                dtype: input.1,
                nullable: false,
                optional: true,
                span: input.2,
            }
        );
    }
}

#[test]
fn should_resolve_nested_optionals() {
    let s = r##"
        .schema(
            .dict(
                "name"    .string()

                "email"   .optional(.string())

                "address" .optional(.dict(
                                        "street" .string()
                                        "house"  .optional(.int())
                                        "state"  .dict(
                                                    "name" .string()
                                                    "code" .optional(.string()))))
                
                "score"   .optional(.float())
                
                "id"      .int()
            )
        )
    "##;

    let ast = parse(s);
    let bindings = SchemaBinder::new(&ast).bind().unwrap();

    let cases = vec![
        (
            BindingPath::new(),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 30,
                    end: 625,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("name".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span { start: 63, end: 72 },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("email".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: true,
                span: Span {
                    start: 110,
                    end: 119,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("address".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: true,
                span: Span {
                    start: 158,
                    end: 497,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 214,
                    end: 223,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: true,
                span: Span {
                    start: 283,
                    end: 289,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 340,
                    end: 496,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("name".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 406,
                    end: 415,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("code".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: true,
                span: Span {
                    start: 485,
                    end: 494,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("score".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Float,
                nullable: false,
                optional: true,
                span: Span {
                    start: 552,
                    end: 560,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("id".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: false,
                span: Span {
                    start: 605,
                    end: 611,
                },
            },
        ),
    ];

    for case in cases {
        assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
    }
}

#[test]
fn should_resolve_optional_with_nullable() {
    let s = r##"
        .schema(
            .dict(
                "name"     .optional(.nullable(.string()))
                "age"      .nullable(.optional(.int()))
            )
        )
    "##;

    let ast = parse(s);
    let bindings = SchemaBinder::new(&ast).bind().unwrap();

    let cases = vec![
        (
            BindingPath::new(),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 30,
                    end: 165,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("name".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: true,
                optional: true,
                span: Span { start: 84, end: 93 },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("age".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: true,
                optional: true,
                span: Span {
                    start: 143,
                    end: 149,
                },
            },
        ),
    ];

    for case in cases {
        assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
    }
}

// OPTIONAL END

// ==================================================================
// MODIFIERS SUCCESS CASES END
// ==================================================================

// ==================================================================
// DICT SUCCESS CASES START
// ==================================================================

#[test]
fn should_resolve_one_level_dict() {
    let s = r##"
        .schema(
            .dict(
                "name"     .string()
                "age"      .nullable(.int())
                "score"    .float()
                "employed" .bool()
            )
        )
    "##;

    let ast = parse(s);
    let bindings = SchemaBinder::new(&ast).bind().unwrap();

    let cases = vec![
        (
            BindingPath::new(),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 30,
                    end: 203,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("name".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span { start: 64, end: 73 },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("age".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: true,
                optional: false,
                span: Span {
                    start: 111,
                    end: 117,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("score".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Float,
                nullable: false,
                optional: false,
                span: Span {
                    start: 146,
                    end: 154,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![Field("employed".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Bool,
                nullable: false,
                optional: false,
                span: Span {
                    start: 182,
                    end: 189,
                },
            },
        ),
    ];

    for case in cases {
        assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
    }
}

// ==================================================================
// DICT SUCCESS CASES END
// ==================================================================

// ==================================================================
// LIST SUCCESS CASES START
// ==================================================================

#[test]
fn should_resolve_one_level_abstract_list() {
    let inputs = vec![
        (
            SchemaFnLexeme::INT,
            SchemaBinderDataType::Int,
            Span { start: 8, end: 21 },
            Span { start: 14, end: 20 },
        ),
        (
            SchemaFnLexeme::FLOAT,
            SchemaBinderDataType::Float,
            Span { start: 8, end: 23 },
            Span { start: 14, end: 22 },
        ),
        (
            SchemaFnLexeme::STRING,
            SchemaBinderDataType::String,
            Span { start: 8, end: 24 },
            Span { start: 14, end: 23 },
        ),
        (
            SchemaFnLexeme::BOOL,
            SchemaBinderDataType::Bool,
            Span { start: 8, end: 22 },
            Span { start: 14, end: 21 },
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.list(.{}()))", input.0));
        let bindings = SchemaBinder::new(&ast).bind().unwrap();

        let cases = vec![
            (
                BindingPath::new(),
                SchemaBinderTypeDescriptor {
                    dtype: SchemaBinderDataType::ListAbstract,
                    nullable: false,
                    optional: false,
                    span: input.2,
                },
            ),
            (
                BindingPath::with_segments(vec![AbstractIndex]),
                SchemaBinderTypeDescriptor {
                    dtype: input.1,
                    nullable: false,
                    optional: false,
                    span: input.3,
                },
            ),
        ];

        for case in cases {
            assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
        }
    }
}

#[test]
fn should_resolve_one_level_fixed_list() {
    let inputs = vec![
        (
            SchemaFnLexeme::INT,
            SchemaBinderDataType::Int,
            Span { start: 8, end: 24 },
            Span { start: 14, end: 20 },
        ),
        (
            SchemaFnLexeme::FLOAT,
            SchemaBinderDataType::Float,
            Span { start: 8, end: 26 },
            Span { start: 14, end: 22 },
        ),
        (
            SchemaFnLexeme::STRING,
            SchemaBinderDataType::String,
            Span { start: 8, end: 27 },
            Span { start: 14, end: 23 },
        ),
        (
            SchemaFnLexeme::BOOL,
            SchemaBinderDataType::Bool,
            Span { start: 8, end: 25 },
            Span { start: 14, end: 21 },
        ),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.list(.{}(), 2))", input.0));
        let bindings = SchemaBinder::new(&ast).bind().unwrap();

        let cases = vec![
            (
                BindingPath::new(),
                SchemaBinderTypeDescriptor {
                    dtype: SchemaBinderDataType::ListFixed(2),
                    nullable: false,
                    optional: false,
                    span: input.2,
                },
            ),
            (
                BindingPath::with_segments(vec![AbstractIndex]),
                SchemaBinderTypeDescriptor {
                    dtype: input.1,
                    nullable: false,
                    optional: false,
                    span: input.3,
                },
            ),
        ];

        for case in cases {
            assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
        }
    }
}

// ==================================================================
// LIST SUCCESS CASES END
// ==================================================================

// ==================================================================
// COMPLEX SCHEMAS SUCCESS CASES START
// ==================================================================

#[test]
fn should_resolve_complex_schema_with_nullables() {
    let s = r##"
        .schema(
            .list(
                .dict(
                    "id"       .int()
                    "name"     .string()
                    "age"      .int()
                    "emails"   .nullable(.list(.string(), 3))

                    "address"  .dict(
                                  "city"   .string()
                                  "street" .string()
                                  "house"  .nullable(.int())
                                  "index"  .nullable(.int()))

                    "scores"   .list(.list(.float(), 2))
                    "employed" .bool()

                    "admin"    .nullable(.dict(
                                            "id"          .int()
                                            "email"       .string()
                                            "permissions" .list(.nullable(.string()))
                                            "address"     .dict(
                                                             "city"   .string()
                                                             "street" .nullable(.string())))))))
    "##;

    let ast = parse(s);
    let bindings = SchemaBinder::new(&ast).bind().unwrap();

    let cases = vec![
        (
            BindingPath::new(),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListAbstract,
                nullable: false,
                optional: false,
                span: Span {
                    start: 30,
                    end: 1112,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 53,
                    end: 1111,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("id".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: false,
                span: Span { start: 91, end: 97 },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("name".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 129,
                    end: 138,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("age".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: false,
                span: Span {
                    start: 170,
                    end: 176,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("emails".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListFixed(3),
                nullable: true,
                optional: false,
                span: Span {
                    start: 218,
                    end: 237,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("emails".to_string()),
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 224,
                    end: 233,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("address".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 271,
                    end: 506,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("city".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 321,
                    end: 330,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 374,
                    end: 383,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: true,
                optional: false,
                span: Span {
                    start: 437,
                    end: 443,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("index".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: true,
                optional: false,
                span: Span {
                    start: 498,
                    end: 504,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("scores".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListAbstract,
                nullable: false,
                optional: false,
                span: Span {
                    start: 539,
                    end: 564,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListFixed(2),
                nullable: false,
                optional: false,
                span: Span {
                    start: 545,
                    end: 563,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Float,
                nullable: false,
                optional: false,
                span: Span {
                    start: 551,
                    end: 559,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("employed".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Bool,
                nullable: false,
                optional: false,
                span: Span {
                    start: 596,
                    end: 603,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("admin".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: true,
                optional: false,
                span: Span {
                    start: 646,
                    end: 1109,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("id".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: false,
                span: Span {
                    start: 711,
                    end: 717,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("email".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 776,
                    end: 785,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListAbstract,
                nullable: false,
                optional: false,
                span: Span {
                    start: 844,
                    end: 871,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: true,
                optional: false,
                span: Span {
                    start: 860,
                    end: 869,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 930,
                    end: 1108,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("city".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 1007,
                    end: 1016,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: true,
                optional: false,
                span: Span {
                    start: 1097,
                    end: 1106,
                },
            },
        ),
    ];

    for case in cases {
        assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
    }
}

#[test]
fn should_resolve_complex_schema_with_optionals() {
    let s = r##"
        .schema(
            .list(
                .dict(
                    "id"       .int()
                    "emails"   .optional(.list(.string(), 3))

                    "address"  .dict(
                                  "street" .string()
                                  "house"  .optional(.int())
                                  "index"  .optional(.int()))

                    "scores"   .optional(.list(.list(.float(), 2)))
                    "employed" .optional(.bool())

                    "admin"    .optional(.dict(
                                            "email"       .string()
                                            "permissions" .optional(.list(.string()))
                                            "address"     .dict(
                                                             "city"   .string()
                                                             "street" .optional(.string())))))))
    "##;

    let ast = parse(s);
    let bindings = SchemaBinder::new(&ast).bind().unwrap();

    let cases = vec![
        (
            BindingPath::new(),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListAbstract,
                nullable: false,
                optional: false,
                span: Span {
                    start: 30,
                    end: 937,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 53,
                    end: 936,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("id".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: false,
                span: Span { start: 91, end: 97 },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("emails".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListFixed(3),
                nullable: false,
                optional: true,
                span: Span {
                    start: 139,
                    end: 158,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("emails".to_string()),
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 145,
                    end: 154,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("address".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 192,
                    end: 374,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 242,
                    end: 251,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: true,
                span: Span {
                    start: 305,
                    end: 311,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("index".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Int,
                nullable: false,
                optional: true,
                span: Span {
                    start: 366,
                    end: 372,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("scores".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListAbstract,
                nullable: false,
                optional: true,
                span: Span {
                    start: 417,
                    end: 442,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListFixed(2),
                nullable: false,
                optional: false,
                span: Span {
                    start: 423,
                    end: 441,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Float,
                nullable: false,
                optional: false,
                span: Span {
                    start: 429,
                    end: 437,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("employed".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Bool,
                nullable: false,
                optional: true,
                span: Span {
                    start: 485,
                    end: 492,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![AbstractIndex, Field("admin".to_string())]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: true,
                span: Span {
                    start: 536,
                    end: 934,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("email".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 601,
                    end: 610,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::ListAbstract,
                nullable: false,
                optional: true,
                span: Span {
                    start: 679,
                    end: 695,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
                AbstractIndex,
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 685,
                    end: 694,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::Dict,
                nullable: false,
                optional: false,
                span: Span {
                    start: 755,
                    end: 933,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("city".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: false,
                span: Span {
                    start: 832,
                    end: 841,
                },
            },
        ),
        (
            BindingPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaBinderTypeDescriptor {
                dtype: SchemaBinderDataType::String,
                nullable: false,
                optional: true,
                span: Span {
                    start: 922,
                    end: 931,
                },
            },
        ),
    ];

    for case in cases {
        assert_eq!(*bindings.bindings.get(&case.0).unwrap(), case.1);
    }
}

// ==================================================================
// COMPLEX SCHEMAS SUCCESS CASES END
// ==================================================================

// ==================================================================
//
//  SUCCESS CASES END
//
// ==================================================================
