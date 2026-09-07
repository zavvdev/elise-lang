use elise_semanalyzer::{Harmony, aast::AAstNode};
use elise_shared::shared_types::Span;
use elise_test_utils::test_utils;

mod common;

#[test]
fn test_floats() {
    let ast = test_utils::parse("-1.2e2, -3.34, 1.5e-2, 5.6, 3e5, 94E2");
    let hir = Harmony::new(&ast).analyze().unwrap();

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
            },
            AAstNode::Float {
                value: "3e5".to_string(),
                span: Span { start: 28, end: 31 }
            },
            AAstNode::Float {
                value: "94E2".to_string(),
                span: Span { start: 33, end: 37 }
            }
        ]
    );
}
