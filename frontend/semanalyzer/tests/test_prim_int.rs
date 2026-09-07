use elise_semanalyzer::{Harmony, aast::AAstNode};
use elise_shared::shared_types::Span;
use elise_test_utils::test_utils;

mod common;

#[test]
fn test_integers() {
    let ast = test_utils::parse("-3, 56, 9999999");
    let hir = Harmony::new(&ast).analyze().unwrap();

    assert_eq!(hir.symbol_table.symbols.is_empty(), true);
    assert_eq!(
        hir.aast,
        vec![
            AAstNode::Int {
                value: "-3".to_string(),
                span: Span { start: 0, end: 2 }
            },
            AAstNode::Int {
                value: "56".to_string(),
                span: Span { start: 4, end: 6 }
            },
            AAstNode::Int {
                value: "9999999".to_string(),
                span: Span { start: 8, end: 15 }
            }
        ]
    );
}
