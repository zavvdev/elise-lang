use elise_semanalyzer::{Harmony, semanalyzer_aast::AAstNode};
use elise_shared::shared_types::Span;

use crate::common::{empty_data_bindings, parse};

mod common;

#[test]
fn test_string() {
    let ast = parse(r#""Hello, World!""#);
    let data_bindings = empty_data_bindings();
    let hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();
    assert_eq!(hir.symbol_table.symbols.is_empty(), true);
    assert_eq!(
        hir.aast,
        vec![AAstNode::String {
            value: "Hello, World!".to_string(),
            span: Span { start: 0, end: 15 }
        },]
    );
}
