use elise_ast::AstNode;
use elise_semanalyzer::{
    Harmony,
    semanalyzer_aast::AAstNode,
    semanalyzer_config::{FN_DEFINE_ARGS_LEN, FN_DEFINE_LEXEME},
    semanalyzer_data_types::{LangPrimitiveType, LangType},
    semanalyzer_symbol_table::{SymbolDescriptor, SymbolId},
};
use elise_shared::{shared_errors::errors_semanalyzer::SemanalyzerErr, shared_types::Span};

use crate::common::{empty_data_bindings, parse};

mod common;

// ==================================================================
//
//  SUCCESS CASES START
//
// ==================================================================

#[test]
fn test_defines_int() {
    let ast = parse(".define(AD 3)");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(
        *hir.symbol_table.symbols.get(&SymbolId(0)).unwrap(),
        SymbolDescriptor {
            name: "AD".to_string(),
            ty: LangType::Primitive(LangPrimitiveType::Int),
            is_captured: false,
        }
    );

    assert_eq!(
        hir.aast,
        vec![AAstNode::FDefine {
            symbol_id: SymbolId(0),
            value: Box::new(AAstNode::Int {
                value: "3".to_string(),
                span: Span { start: 11, end: 12 }
            }),
            span: Span { start: 0, end: 13 }
        }]
    );
}

#[test]
fn test_defines_float() {
    let ast = parse(".define(PI 3.1415)");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(
        *hir.symbol_table.symbols.get(&SymbolId(0)).unwrap(),
        SymbolDescriptor {
            name: "PI".to_string(),
            ty: LangType::Primitive(LangPrimitiveType::Float),
            is_captured: false,
        }
    );

    assert_eq!(
        hir.aast,
        vec![AAstNode::FDefine {
            symbol_id: SymbolId(0),
            value: Box::new(AAstNode::Float {
                value: "3.1415".to_string(),
                span: Span { start: 11, end: 17 }
            }),
            span: Span { start: 0, end: 18 }
        }]
    );
}

#[test]
fn test_defines_string() {
    let ast = parse(r#".define(NAME "Carl")"#);
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(
        *hir.symbol_table.symbols.get(&SymbolId(0)).unwrap(),
        SymbolDescriptor {
            name: "NAME".to_string(),
            ty: LangType::Primitive(LangPrimitiveType::String),
            is_captured: false,
        }
    );

    assert_eq!(
        hir.aast,
        vec![AAstNode::FDefine {
            symbol_id: SymbolId(0),
            value: Box::new(AAstNode::String {
                value: "Carl".to_string(),
                span: Span { start: 13, end: 19 }
            }),
            span: Span { start: 0, end: 20 }
        }]
    );
}

#[test]
fn test_defines_bool_true() {
    let ast = parse(".define(OPEN true)");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(
        *hir.symbol_table.symbols.get(&SymbolId(0)).unwrap(),
        SymbolDescriptor {
            name: "OPEN".to_string(),
            ty: LangType::Primitive(LangPrimitiveType::Bool),
            is_captured: false,
        }
    );

    assert_eq!(
        hir.aast,
        vec![AAstNode::FDefine {
            symbol_id: SymbolId(0),
            value: Box::new(AAstNode::Bool {
                value: true,
                span: Span { start: 13, end: 17 }
            }),

            span: Span { start: 0, end: 18 }
        }]
    );
}

#[test]
fn test_defines_bool_false() {
    let ast = parse(".define(OPEN false)");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(
        *hir.symbol_table.symbols.get(&SymbolId(0)).unwrap(),
        SymbolDescriptor {
            name: "OPEN".to_string(),
            ty: LangType::Primitive(LangPrimitiveType::Bool),
            is_captured: false,
        }
    );

    assert_eq!(
        hir.aast,
        vec![AAstNode::FDefine {
            symbol_id: SymbolId(0),
            value: Box::new(AAstNode::Bool {
                value: false,
                span: Span { start: 13, end: 18 }
            }),

            span: Span { start: 0, end: 19 }
        }]
    );
}

// ==================================================================
//
//  SUCCESS CASES END
//
// ==================================================================

// ==================================================================
//
//  ERROR CASES START
//
// ==================================================================

#[test]
fn test_returns_arity_mismatch_if_no_args() {
    let ast = parse(".define()");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FN_DEFINE_LEXEME,
            expected: FN_DEFINE_ARGS_LEN,
            found: 0,
            ..
        })
    ));
}

#[test]
fn test_returns_arity_mismatch_if_more_than_2_args() {
    let ast = parse(".define(PI, 2, 3)");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FN_DEFINE_LEXEME,
            expected: FN_DEFINE_ARGS_LEN,
            found: 3,
            ..
        })
    ));
}

#[test]
fn test_returns_arg_type_mismatch_if_defines_non_primitive() {
    let ast = parse(".define(PI [1, 2])");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArgTypeMismatch {
            fn_name: FN_DEFINE_LEXEME,
            position: 1,
            expected: LangType::PRIMITIVE_STR,
            found: AstNode::LIST_STR,
            ..
        })
    ));
}

#[test]
fn test_returns_arg_kind_mismatch_if_first_arg_is_not_identifier() {
    let ast = parse(".define(false 2)");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArgKindMismatch {
            fn_name: FN_DEFINE_LEXEME,
            position: 0,
            expected: AstNode::IDENTIFIER_STR,
            found: AstNode::BOOL_STR,
            ..
        })
    ));
}

#[test]
fn test_returns_symbol_duplicate_if_already_defined() {
    let ast = parse(".define(PI 3.14) .define(PI 3.1415)");
    let data_bindings = empty_data_bindings();
    let result = Harmony::new(&ast, &data_bindings).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::SymbolDuplicate { .. })
    ));
}

// ==================================================================
//
//  ERROR CASES END
//
// ==================================================================
