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
//  ERROR CASES START
//
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
            expected: ArgLen::ROOT,
            kind: ArityMismatchKind::Eq,
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
            expected: ArgLen::ROOT,
            kind: ArityMismatchKind::Eq,
            found: 2,
            ..
        })
    ));
}

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
                expected,
                kind,
                found,
                ..
            }) => {
                assert_eq!(fn_name, input.1);
                assert_eq!(expected, ArgLen::PRIMITIVE);
                assert_eq!(kind, ArityMismatchKind::Eq);
                assert_eq!(found, 1);
            }
            other => panic!("expected ArityMismatch, got {:?}", other),
        }
    }
}

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
//
//  ERROR CASES END
//
// ==================================================================

// ==================================================================
//
//  SUCCESS CASES START
//
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
            }
        );
    }
}

#[test]
fn should_resolve_single_compound() {
    let inputs = vec![
        (r#".dict("name" .string())"#, SchemaDataType::Dict),
        (".list(.int())", SchemaDataType::List),
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
            }
        );
    }
}

#[test]
fn should_resolve_single_nullable_primitive() {
    let inputs = vec![
        (".int()", SchemaDataType::Int),
        (".float()", SchemaDataType::Float),
        (".string()", SchemaDataType::String),
        (".bool()", SchemaDataType::Bool),
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
            }
        );
    }
}

#[test]
fn should_resolve_single_nullable_compound() {
    let inputs = vec![
        (r#".dict("name" .string())"#, SchemaDataType::Dict),
        (".list(.int())", SchemaDataType::List),
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
            }
        );
    }
}

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
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("age".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("score".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Float,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![Field("employed".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Bool,
                nullable: false,
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
fn should_resolve_one_level_list() {
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
                    dtype: SchemaDataType::List,
                    nullable: false,
                },
            ),
            (
                ResolutionPath::with_segments(vec![AbstractIndex]),
                SchemaTypeDescriptor {
                    dtype: input.1,
                    nullable: false,
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
fn should_resolve_complex_schema() {
    let s = r##"
        .schema(
            .list(
                .dict(
                    "id"       .int()
                    "name"     .string()
                    "age"      .int()
                    "emails"   .nullable(.list(.string()))
                    
                    "address"  .dict(
                                  "city"   .string()
                                  "street" .string()
                                  "house"  .nullable(.int())
                                  "index"  .nullable(.int()))
                    
                    "scores"   .list(.list(.float()))
                    "employed" .bool()
                    
                    "admin"    .nullable(.dict(
                                            "id"          .int()
                                            "email"       .string()
                                            "permissions" .list(.string())
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
                dtype: SchemaDataType::List,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("id".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("name".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::String,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("age".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Int,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("emails".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::List,
                nullable: true,
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
                nullable: true,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("address".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: false,
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
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("scores".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::List,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![
                AbstractIndex,
                Field("scores".to_string()),
                AbstractIndex,
            ]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::List,
                nullable: false,
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
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("employed".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Bool,
                nullable: false,
            },
        ),
        (
            ResolutionPath::with_segments(vec![AbstractIndex, Field("admin".to_string())]),
            SchemaTypeDescriptor {
                dtype: SchemaDataType::Dict,
                nullable: true,
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
                nullable: true,
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
                nullable: true,
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
                nullable: true,
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
                nullable: true,
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
//
//  SUCCESS CASES END
//
// ==================================================================
