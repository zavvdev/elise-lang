use elise_semanalyzer::{Harmony, semanalyzer_aast::AAstNode};
use elise_shared::shared_types::Span;

use crate::common::{empty_data_bindings, parse};

mod common;

#[test]
fn test_bool() {
    let ast = parse("true, false");
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();
    assert_eq!(hir.symbol_table.symbols.is_empty(), true);
    assert_eq!(
        hir.aast,
        vec![
            AAstNode::Bool {
                value: true,
                span: Span { start: 0, end: 4 }
            },
            AAstNode::Bool {
                value: false,
                span: Span { start: 6, end: 11 }
            },
        ]
    );
}
