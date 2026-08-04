use elise_data::{
    resolution_path::ResolutionPath,
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
fn should_return_error_if_nullable_has_nullable_arg() {
    let ast = parse(".schema(.nullable(.nullable(.int())))");
    let resolved_schema = SchemaResolver::new(&ast).resolve();
    assert!(matches!(
        resolved_schema,
        Err(SchemaResolverErr::NullableNullable { .. })
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

// ==================================================================
//
//  SUCCESS CASES END
//
// ==================================================================
