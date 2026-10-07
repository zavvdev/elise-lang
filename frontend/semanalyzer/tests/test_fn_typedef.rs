use std::collections::HashMap;

use elise_aast::{AAstNode, AAstNodeCall, AAstNodeTypedef};
use elise_bindings::{
    BindingType, TypeBinding, TypeBindingDesc, TypeBindingsMap,
    binding_path::{BindingPath, BindingPathSegment},
};
use elise_shared::{shared_errors::errors_semanalyzer::SemanalyzerErr, shared_types::Span};
use elise_test_utils::test_utils::semanalyze;

mod common;

// ==================================================================
//
// COMMON START
//
// ==================================================================

// TODO: Add test cases for emitting TypedefMode error.

#[test]
fn should_fail_for_invalid_number_of_arguments() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :Int :Int)", &mut type_bindings_map, false);
    assert!(matches!(result, Err(SemanalyzerErr::ArityMismatch { .. })));
}

#[test]
fn should_fail_if_first_arg_is_not_ident() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(:Int :Int)", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ExpectedIdentifier { .. })
    ));
}

#[test]
fn should_fail_if_second_arg_is_not_typedef() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age Age)", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ExpectedTypedef { .. })
    ));
}

// ==================================================================
//
// COMMON END
//
// ==================================================================

// ==================================================================
//
// INT START
//
// ==================================================================

#[test]
fn should_analyze_int_aliasing() {
    let mut type_bindings_map = HashMap::new();
    let aast = semanalyze(".typedef(Age :Int)", &mut type_bindings_map, false).unwrap();

    let mut expected_type_binding: TypeBinding = HashMap::new();
    let mut expected_type_bindings_map: TypeBindingsMap = HashMap::new();
    let mut expected_aast: Vec<AAstNode> = vec![];

    expected_type_binding.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 13, end: 17 },
        },
    );
    expected_type_bindings_map.insert("Age".to_string(), expected_type_binding);

    expected_aast.push(AAstNode::Call(AAstNodeCall::Typedef {
        alias: "Age".to_string(),
        typedef: AAstNodeTypedef::Int {
            span: Span { start: 13, end: 17 },
        },
    }));

    assert_eq!(type_bindings_map, expected_type_bindings_map);
    assert_eq!(aast, expected_aast);
}

#[test]
fn should_fail_if_int_has_generic() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :Int<:Int>)", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::UnexpectedGeneric { .. })
    ));
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
fn should_analyze_list_aliasing() {
    let mut type_bindings_map = HashMap::new();
    let aast = semanalyze(".typedef(Data :List<:Int>)", &mut type_bindings_map, false).unwrap();

    let mut expected_type_binding: TypeBinding = HashMap::new();
    let mut expected_type_bindings_map: TypeBindingsMap = HashMap::new();
    let mut expected_aast: Vec<AAstNode> = vec![];

    expected_type_binding.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::List,
            span: Span { start: 14, end: 25 },
        },
    );
    expected_type_binding.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 20, end: 24 },
        },
    );

    expected_type_bindings_map.insert("Data".to_string(), expected_type_binding);

    expected_aast.push(AAstNode::Call(AAstNodeCall::Typedef {
        alias: "Data".to_string(),
        typedef: AAstNodeTypedef::List {
            span: Span { start: 14, end: 25 },
            item_type: Box::new(AAstNodeTypedef::Int {
                span: Span { start: 20, end: 24 },
            }),
        },
    }));

    assert_eq!(type_bindings_map, expected_type_bindings_map);
    assert_eq!(aast, expected_aast);
}

#[test]
fn should_fail_if_list_has_no_generic() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :List)", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ExpectedGeneric { .. })
    ));
}

#[test]
fn should_fail_if_list_has_invalid_generic() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(
        r##".typedef(Age :List<{ "id" :Int }>)"##,
        &mut type_bindings_map,
        false,
    );
    assert!(matches!(result, Err(SemanalyzerErr::InvalidGeneric { .. })));
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
fn should_analyze_record_aliasing() {
    let mut type_bindings_map = HashMap::new();
    let aast = semanalyze(
        r##".typedef (User :Record<{ "id" :Int }>)"##,
        &mut type_bindings_map,
        false,
    )
    .unwrap();

    let mut expected_type_binding: TypeBinding = HashMap::new();
    let mut expected_type_bindings_map: TypeBindingsMap = HashMap::new();
    let mut expected_aast: Vec<AAstNode> = vec![];

    expected_type_binding.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: BindingType::Record,
            span: Span { start: 15, end: 37 },
        },
    );
    expected_type_binding.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
        TypeBindingDesc {
            dtype: BindingType::Int,
            span: Span { start: 30, end: 34 },
        },
    );

    expected_type_bindings_map.insert("User".to_string(), expected_type_binding);

    expected_aast.push(AAstNode::Call(AAstNodeCall::Typedef {
        alias: "User".to_string(),
        typedef: AAstNodeTypedef::Record {
            span: Span { start: 15, end: 37 },
            entries: vec![(
                "id".to_string(),
                Box::new(AAstNodeTypedef::Int {
                    span: Span { start: 30, end: 34 },
                }),
            )],
        },
    }));

    assert_eq!(type_bindings_map, expected_type_bindings_map);
    assert_eq!(aast, expected_aast);
}

#[test]
fn should_fail_if_record_has_no_generic() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(User :Record)", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ExpectedGeneric { .. })
    ));
}

#[test]
fn should_fail_if_record_has_invalid_generic() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :Record<:Int>)", &mut type_bindings_map, false);
    assert!(matches!(result, Err(SemanalyzerErr::InvalidGeneric { .. })));
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
fn should_fail_if_custom_alias_has_generic() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :Some<:Int>)", &mut type_bindings_map, false);
    assert!(matches!(
        result,
        Err(SemanalyzerErr::UnexpectedGeneric { .. })
    ));
}

#[test]
fn should_fail_if_custom_is_aliasing_undefined() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :Some)", &mut type_bindings_map, false);
    assert!(matches!(result, Err(SemanalyzerErr::UnknownTypedef { .. })));
}

#[test]
fn should_fail_if_custom_aliasing_itself() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(".typedef(Age :Age)", &mut type_bindings_map, false);
    assert!(matches!(result, Err(SemanalyzerErr::UnknownTypedef { .. })));
}

#[test]
fn should_fail_if_custom_aliasing_itself_deep() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(
        r##".typedef(Age :Record<{ "some" :Age }>)"##,
        &mut type_bindings_map,
        false,
    );
    assert!(matches!(result, Err(SemanalyzerErr::UnknownTypedef { .. })));
}

// ==================================================================
//
// CUSTOM END
//
// ==================================================================
