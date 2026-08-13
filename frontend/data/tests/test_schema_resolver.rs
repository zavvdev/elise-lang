use elise_data::{
    resolution_path::{ResolutionPath, ResolutionPathSegment::*},
    schema_resolver::{
        ArgLen, SchemaDataType, SchemaFnLexeme, SchemaResolver, SchemaTypeDescriptor,
    },
};
use elise_shared::{
    shared_errors::errors_schema_resolver::SchemaResolverErr, shared_types::ArityMismatchKind,
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
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert_eq!(resolved_schema, Err(SchemaResolverErr::Empty));
}

#[test]
fn should_return_error_if_root_is_not_call() {
    let ast = parse("test-test");
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::Unexp { .. })
    ));
}

#[test]
fn should_return_error_if_root_not_valid_call() {
    let ast = parse(".test(.string())");
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::Unexp { .. })
    ));
}

#[test]
fn should_return_error_if_root_arg_len_is_0() {
    let ast = parse(".schema()");
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::ArityMismatch {
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
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::ArityMismatch {
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
        let resolved_schema = SchemaResolver::new(&ast).resolve();
        match resolved_schema {
            Err(SchemaResolverErr::ArityMismatch {
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
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::InvalDict { .. })
    ));
}

#[test]
fn should_return_error_if_dict_invalid_keys() {
    let ast = parse(".schema(.dict(name .string(), age .int()))");
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::InvalDict { .. })
    ));
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
        let resolved_schema = SchemaResolver::new(&ast).resolve();

        match resolved_schema {
            Err(SchemaResolverErr::ArityMismatch {
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
        let resolved_schema = SchemaResolver::new(&ast).resolve();

        assert!(matches!(
            resolved_schema,
            Err(SchemaResolverErr::InvalUseOfModifier { .. })
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
        (".int()", SchemaDataType::Int),
        (".float()", SchemaDataType::Float),
        (".string()", SchemaDataType::String),
        (".bool()", SchemaDataType::Bool),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input.0));
        let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

        assert_eq!(
            *resolved_schema
                .resolved_schema
                .get(&ResolutionPath::new())
                .unwrap(),
            SchemaTypeDescriptor {
                dtype: input.1,
                nullable: false,
                optional: false,
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
        (r#".dict("name" .string())"#, SchemaDataType::Dict),
        (".list(.int())", SchemaDataType::ListAbstract),
        (".list(.int(), 2)", SchemaDataType::ListFixed(2)),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema({})", input.0));
        let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

        assert_eq!(
            *resolved_schema
                .resolved_schema
                .get(&ResolutionPath::new())
                .unwrap(),
            SchemaTypeDescriptor {
                dtype: input.1,
                nullable: false,
                optional: false,
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
        (".int()", SchemaDataType::Int),
        (".float()", SchemaDataType::Float),
        (".string()", SchemaDataType::String),
        (".bool()", SchemaDataType::Bool),
        (r#".dict("name" .string())"#, SchemaDataType::Dict),
        (".list(.int())", SchemaDataType::ListAbstract),
        (".list(.int(), 3)", SchemaDataType::ListFixed(3)),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.nullable({}))", input.0));
        let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

        assert_eq!(
            *resolved_schema
                .resolved_schema
                .get(&ResolutionPath::new())
                .unwrap(),
            SchemaTypeDescriptor {
                dtype: input.1,
                nullable: true,
                optional: false,
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
    let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

    let cases = vec![
        (
            ResolutionPath::new(),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("email".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("address".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("name".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("code".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("score".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Float,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("id".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: false,
            },
        ),
    ];

    for case in cases {
        assert_eq!(
            *resolved_schema.resolved_schema.get(&case.0).unwrap(),
            case.1
        );
    }
}

// NULLABLE END

// OPTIONAL START

#[test]
fn should_resolve_one_optional_child() {
    let inputs = vec![
        (".int()", SchemaDataType::Int),
        (".float()", SchemaDataType::Float),
        (".string()", SchemaDataType::String),
        (".bool()", SchemaDataType::Bool),
        (r#".dict("name" .string())"#, SchemaDataType::Dict),
        (".list(.int())", SchemaDataType::ListAbstract),
        (".list(.int(), 2)", SchemaDataType::ListFixed(2)),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.optional({}))", input.0));
        let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

        assert_eq!(
            *resolved_schema
                .resolved_schema
                .get(&ResolutionPath::new())
                .unwrap(),
            SchemaTypeDescriptor {
                dtype: input.1,
                nullable: false,
                optional: true,
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
    let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

    let cases = vec![
        (
            ResolutionPath::new(),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("email".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("address".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("name".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                Field("address".to_string()),
                Field("state".to_string()),
                Field("code".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("score".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Float,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("id".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: false,
            },
        ),
    ];

    for case in cases {
        assert_eq!(
            *resolved_schema.resolved_schema.get(&case.0).unwrap(),
            case.1
        );
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
    let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

    let cases = vec![
        (
            ResolutionPath::new(),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: true,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("age".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: true,
                optional: true,
            },
        ),
    ];

    for case in cases {
        assert_eq!(
            *resolved_schema.resolved_schema.get(&case.0).unwrap(),
            case.1
        );
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
    let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

    let cases = vec![
        (
            ResolutionPath::new(),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("age".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("score".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Float,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("employed".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Bool,
                nullable: false,
                optional: false,
            },
        ),
    ];

    for case in cases {
        assert_eq!(
            *resolved_schema.resolved_schema.get(&case.0).unwrap(),
            case.1
        );
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
        (SchemaFnLexeme::INT, SchemaDataType::Int),
        (SchemaFnLexeme::FLOAT, SchemaDataType::Float),
        (SchemaFnLexeme::STRING, SchemaDataType::String),
        (SchemaFnLexeme::BOOL, SchemaDataType::Bool),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.list(.{}()))", input.0));
        let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

        let cases = vec![
            (
                ResolutionPath::new(),
                SchemaTypeDescriptor {
                    dtype: SchemaDataType::ListAbstract,
                    nullable: false,
                    optional: false,
                },
            ),
            (
                ResolutionPath::with_segments(vec![AbstractIndex]),
                SchemaTypeDescriptor {
                    dtype: input.1,
                    nullable: false,
                    optional: false,
                },
            ),
        ];

        for case in cases {
            assert_eq!(
                *resolved_schema.resolved_schema.get(&case.0).unwrap(),
                case.1
            );
        }
    }
}

#[test]
fn should_resolve_one_level_fixed_list() {
    let inputs = vec![
        (SchemaFnLexeme::INT, SchemaDataType::Int),
        (SchemaFnLexeme::FLOAT, SchemaDataType::Float),
        (SchemaFnLexeme::STRING, SchemaDataType::String),
        (SchemaFnLexeme::BOOL, SchemaDataType::Bool),
    ];

    for input in inputs {
        let ast = parse(&format!(".schema(.list(.{}(), 2))", input.0));
        let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

        let cases = vec![
            (
                ResolutionPath::new(),
                SchemaTypeDescriptor {
                    dtype: SchemaDataType::ListFixed(2),
                    nullable: false,
                    optional: false,
                },
            ),
            (
                ResolutionPath::with_segments(vec![AbstractIndex]),
                SchemaTypeDescriptor {
                    dtype: input.1,
                    nullable: false,
                    optional: false,
                },
            ),
        ];

        for case in cases {
            assert_eq!(
                *resolved_schema.resolved_schema.get(&case.0).unwrap(),
                case.1
            );
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
    let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

    let cases = vec![
        (
            ResolutionPath::new(),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListAbstract,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("id".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("age".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("emails".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListFixed(3),
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("emails".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("address".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("city".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("index".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("scores".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListAbstract,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListFixed(2),
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Float,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("employed".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Bool,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("admin".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("id".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("email".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListAbstract,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: true,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("city".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: true,
                optional: false,
            },
        ),
    ];

    for case in cases {
        assert_eq!(
            *resolved_schema.resolved_schema.get(&case.0).unwrap(),
            case.1
        );
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
    let resolved_schema = SchemaResolver::new(&ast).resolve().unwrap();

    let cases = vec![
        (
            ResolutionPath::new(),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListAbstract,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("id".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("emails".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListFixed(3),
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("emails".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("address".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("house".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("address".to_string()),
                Field("index".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("scores".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListAbstract,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListFixed(2),
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Float,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("employed".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Bool,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("admin".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("email".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::ListAbstract,
                nullable: false,
                optional: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("permissions".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("city".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("admin".to_string()),
                Field("address".to_string()),
                Field("street".to_string()),
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
                optional: true,
            },
        ),
    ];

    for case in cases {
        assert_eq!(
            *resolved_schema.resolved_schema.get(&case.0).unwrap(),
            case.1
        );
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
