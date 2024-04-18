pub mod types;
pub mod messages;
pub mod lexer;
pub mod parser;

use lexer::tokenize;
use parser::parse;

fn main() {
    let tokens = tokenize("@add (@add (2 3), 2)");
    let parser_ast = parse(tokens);

    println!("{:#?}", parser_ast);
}
