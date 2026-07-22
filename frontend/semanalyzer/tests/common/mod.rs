use std::collections::HashMap;

use elise_ast::AstNode;
use elise_data::data_binder::DataBindingTable;
use elise_parser::Prelude;

pub fn parse(source_code: &str) -> Vec<AstNode> {
    Prelude::new(&source_code.as_bytes()).parse().unwrap()
}

pub fn empty_data_bindings() -> DataBindingTable {
    DataBindingTable {
        table: HashMap::new(),
    }
}
