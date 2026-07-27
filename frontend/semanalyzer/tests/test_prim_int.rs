use elise_semanalyzer::{Harmony, semanalyzer_aast::AAstNode};
use elise_shared::shared_types::Span;

use crate::common::{empty_data_bindings, parse};

mod common;

#[test]
fn test_integers() {
    let ast = parse("-3, 56, 9999999");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();

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
