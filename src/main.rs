pub mod types;
pub mod messages;
pub mod lexer;
pub mod parser;

use lexer::tokenize;
use parser::parse;

fn main() {
    let tokens = tokenize("@add(@add(1 @add(4, 4.5)), 2.3)");
    let parser_ast = parse(tokens);

    println!("{:#?}", parser_ast);
}
