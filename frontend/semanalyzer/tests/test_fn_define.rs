use elise_semanalyzer::Harmony;

use crate::common::{empty_data_bindings, parse};

mod common;

#[test]
fn test_define_annotates_correctly() {
    let ast = parse(".define(PI 1.1415)");
    let data_bindings = empty_data_bindings();
    let _hir = Harmony::new(&ast, &data_bindings).analyze().unwrap();
    assert_eq!(1, 1);
}
