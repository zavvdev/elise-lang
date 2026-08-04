use elise_ast::AstNode;
use elise_parser::Prelude;

pub fn parse(source_code: &str) -> Vec<AstNode> {
    Prelude::new(&source_code.as_bytes()).parse().unwrap()
}
