use elise_semanalyzer::{
    Harmony,
    aast::AAstNode,
    config::FnDefine,
    data_types::{LangPrimitiveType, LangType},
    symbol_table::{SymbolDescriptor, SymbolId},
};
use elise_shared::{
    shared_errors::errors_semanalyzer::SemanalyzerErr,
    shared_node_names::NodeName,
    shared_types::{ArityMismatchKind, Span},
};
use elise_test_utils::test_utils;

mod common;

// ==================================================================
//
// SUCCESS CASES START
//
// ==================================================================

#[test]
fn test_defines_int() {
    let ast = test_utils::parse(".define(AD 3)");
    let hir = Harmony::new(&ast).analyze().unwrap();

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
        vec![AAstNode::CallDefine {
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
    let ast = test_utils::parse(".define(PI 3.1415)");
    let hir = Harmony::new(&ast).analyze().unwrap();

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
        vec![AAstNode::CallDefine {
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
    let ast = test_utils::parse(r#".define(NAME "Carl")"#);
    let hir = Harmony::new(&ast).analyze().unwrap();

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
        vec![AAstNode::CallDefine {
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
    let ast = test_utils::parse(".define(OPEN true)");
    let hir = Harmony::new(&ast).analyze().unwrap();

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
        vec![AAstNode::CallDefine {
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
    let ast = test_utils::parse(".define(OPEN false)");
    let hir = Harmony::new(&ast).analyze().unwrap();

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
        vec![AAstNode::CallDefine {
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
// SUCCESS CASES END
//
// ==================================================================

// ==================================================================
//
// ERROR CASES START
//
// ==================================================================

#[test]
fn test_returns_arity_mismatch_if_no_args() {
    let ast = test_utils::parse(".define()");
    let result = Harmony::new(&ast).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FnDefine::LEXEME,
            found: 0,
            kind: ArityMismatchKind::Eq(FnDefine::ARGS_LEN),
            ..
        })
    ));
}

#[test]
fn test_returns_arity_mismatch_if_more_than_2_args() {
    let ast = test_utils::parse(".define(PI, 2, 3)");
    let result = Harmony::new(&ast).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArityMismatch {
            fn_name: FnDefine::LEXEME,
            found: 3,
            kind: ArityMismatchKind::Eq(FnDefine::ARGS_LEN),
            ..
        })
    ));
}

#[test]
fn test_returns_arg_type_mismatch_if_defines_non_primitive() {
    let ast = test_utils::parse(".define(PI [1, 2])");
    let result = Harmony::new(&ast).analyze();

    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArgTypeMismatch {
            fn_name: FnDefine::LEXEME,
            position: 1,
            expected: NodeName::PRIMITIVE,
            found: NodeName::LIST,
            ..
        })
    ));
}

#[test]
fn test_returns_arg_kind_mismatch_if_first_arg_is_not_identifier() {
    let ast = test_utils::parse(".define(false 2)");
    let result = Harmony::new(&ast).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::ArgKindMismatch {
            fn_name: FnDefine::LEXEME,
            position: 0,
            expected: NodeName::IDENTIFIER,
            found: NodeName::BOOL,
            ..
        })
    ));
}

#[test]
fn test_returns_symbol_duplicate_if_already_defined() {
    let ast = test_utils::parse(".define(PI 3.14) .define(PI 3.1415)");
    let result = Harmony::new(&ast).analyze();
    assert!(matches!(
        result,
        Err(SemanalyzerErr::SymbolDuplicate { .. })
    ));
}

// ==================================================================
//
// ERROR CASES END
//
// ==================================================================
