use std::collections::HashMap;

use elise_aast::{AAstNode, AAstNodeData, AAstNodeDataDictKey, AAstNodeDataPrim};
use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_shared_types::Span;
use elise_test_utils::test_utils::semanalyze;

mod common;

// ==================================================================
//
// ERROR CASES START
//
// ==================================================================

#[test]
fn should_fail_for_unsupported_items() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(
        r##"{ "a" .typedef(Some :Int) }"##,
        &mut type_bindings_map,
        false,
    );
    assert_eq!(
        result,
        Err(SemanalyzerErr::UnsupportedDictItem {
            span: Span { start: 6, end: 25 }
        })
    );
}

// ==================================================================
//
// ERROR CASES END
//
// ==================================================================

// ==================================================================
//
// SUCCESS CASES START
//
// ==================================================================

#[test]
fn should_analyze_dict() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(
        r##"{ "a" 42, "b" "hello" }"##,
        &mut type_bindings_map,
        false,
    );

    let expected = AAstNode::Data(AAstNodeData::Dict {
        span: Span { start: 0, end: 23 },
        entries: vec![
            (
                AAstNodeDataDictKey {
                    span: Span { start: 2, end: 5 },
                    lexeme: "a".to_string(),
                },
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "42".to_string(),
                    span: Span { start: 6, end: 8 },
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: Span { start: 10, end: 13 },
                    lexeme: "b".to_string(),
                },
                Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                    lexeme: "hello".to_string(),
                    span: Span { start: 14, end: 21 },
                })),
            ),
        ],
    });

    assert_eq!(result, Ok(vec![expected]));
}

#[test]
fn should_analyze_nested_dict() {
    let mut type_bindings_map = HashMap::new();
    let result = semanalyze(
        r##"{ "a" 42, "b" { "c" "hello" } }"##,
        &mut type_bindings_map,
        false,
    );

    let expected = AAstNode::Data(AAstNodeData::Dict {
        span: Span { start: 0, end: 31 },
        entries: vec![
            (
                AAstNodeDataDictKey {
                    span: Span { start: 2, end: 5 },
                    lexeme: "a".to_string(),
                },
                Box::new(AAstNodeData::Int(AAstNodeDataPrim {
                    lexeme: "42".to_string(),
                    span: Span { start: 6, end: 8 },
                })),
            ),
            (
                AAstNodeDataDictKey {
                    span: Span { start: 10, end: 13 },
                    lexeme: "b".to_string(),
                },
                Box::new(AAstNodeData::Dict {
                    span: Span { start: 14, end: 29 },
                    entries: vec![(
                        AAstNodeDataDictKey {
                            span: Span { start: 16, end: 19 },
                            lexeme: "c".to_string(),
                        },
                        Box::new(AAstNodeData::Str(AAstNodeDataPrim {
                            lexeme: "hello".to_string(),
                            span: Span { start: 20, end: 27 },
                        })),
                    )],
                }),
            ),
        ],
    });

    assert_eq!(result, Ok(vec![expected]));
}

// ==================================================================
//
// SUCCESS CASES END
//
// ==================================================================
