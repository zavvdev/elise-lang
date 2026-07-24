use elise_semanalyzer::{Harmony, semanalyzer_aast::AAstNode};
use elise_shared::shared_types::Span;

use crate::common::{empty_data_bindings, parse};

mod common;

#[test]
fn test_integers() {
    let ast = parse("-1e2, -3, 1e-2, 56");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(hir.symbol_table.symbols.is_empty(), true);
    assert_eq!(
        hir.aast,
        vec![
            AAstNode::Int {
                value: "-1e2".to_string(),
                span: Span { start: 0, end: 4 }
            },
            AAstNode::Int {
                value: "-3".to_string(),
                span: Span { start: 6, end: 8 }
            },
            AAstNode::Int {
                value: "1e-2".to_string(),
                span: Span { start: 10, end: 14 }
            },
            AAstNode::Int {
                value: "56".to_string(),
                span: Span { start: 16, end: 18 }
            }
        ]
    );
}

#[test]
fn test_floats() {
    let ast = parse("-1.2e2, -3.34, 1.5e-2, 5.6");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

    assert_eq!(hir.symbol_table.symbols.is_empty(), true);
    assert_eq!(
        hir.aast,
        vec![
            AAstNode::Float {
                value: "-1.2e2".to_string(),
                span: Span { start: 0, end: 6 }
            },
            AAstNode::Float {
                value: "-3.34".to_string(),
                span: Span { start: 8, end: 13 }
            },
            AAstNode::Float {
                value: "1.5e-2".to_string(),
                span: Span { start: 15, end: 21 }
            },
            AAstNode::Float {
                value: "5.6".to_string(),
                span: Span { start: 23, end: 26 }
            }
        ]
    );
}
