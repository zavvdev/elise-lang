use elise_semanalyzer::{Harmony, aast::AAstNode};
use elise_shared::shared_types::Span;
use elise_test_utils::test_utils;

mod common;

#[test]
fn test_string() {
    let ast = test_utils::parse(r#""Hello, World!""#);
    let hir = Harmony::new(&ast).analyze().unwrap();
    assert_eq!(hir.symbol_table.symbols.is_empty(), true);
    assert_eq!(
        hir.aast,
        vec![AAstNode::String {
            value: "Hello, World!".to_string(),
            span: Span { start: 0, end: 15 }
        },]
    );
}
