use elise_semanalyzer::{
    Harmony,
    semanalyzer_aast::AAstNode,
    semanalyzer_data_types::{LangPrimitiveType, LangType},
    semanalyzer_symbol_table::{SymbolDescriptor, SymbolId},
};
use elise_shared::shared_types::Span;

use crate::common::{empty_data_bindings, parse};

mod common;

#[test]
fn test_define_annotates_correctly() {
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
            value: "3.1415".to_string(),
            span: Span { start: 0, end: 18 }
        }]
    );
}

// TODO: Add tests for failed cases
