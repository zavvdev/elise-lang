use std::collections::HashMap;

use elise_aast::{AAstNode, AAstNodeCall, AAstNodeTypedef};
use elise_bindings::{
    BindingType, TypeBinding, TypeBindingDesc, TypeBindingsMap,
    binding_path::{BindingPath, BindingPathSegment},
};
use elise_shared::shared_types::Span;
use elise_test_utils::test_utils::semanalyze;

mod common;

// ==================================================================
//
// SUCCESS CASES START
//
// ==================================================================

// ==================================================================
// PRIMITIVE ALIASING START
// ==================================================================

#[test]
fn should_analyze_int_aliasing() {
    let mut type_bindings_map = HashMap::new();
    let aast = semanalyze(".typedef(Age :Int)", &mut type_bindings_map, false);

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

// ==================================================================
// PRIMITIVE ALIASING END
// ==================================================================

// ==================================================================
// COMPOUND ALIASING START
// ==================================================================

#[test]
fn should_analyze_list_aliasing() {
    let mut type_bindings_map = HashMap::new();
    let aast = semanalyze(".typedef(Data :List<:Int>)", &mut type_bindings_map, false);

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
fn should_analyze_record_aliasing() {
    let mut type_bindings_map = HashMap::new();
    let aast = semanalyze(
        r##".typedef (User :Record<{ "id" :Int }>)"##,
        &mut type_bindings_map,
        false,
    );

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

// ==================================================================
// COMPOUND ALIASING END
// ==================================================================

// ==================================================================
//
// SUCCESS CASES END
//
// ==================================================================

// ==================================================================
//
// ERROR CASES START
//
// ==================================================================

// TODO

// TODO: Add test cases for emitting TypedefMode error.

// ==================================================================
//
// ERROR CASES END
//
// ==================================================================
