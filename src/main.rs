pub mod types;
pub mod messages;
pub mod lexer;
pub mod parser;

use lexer::tokenize;
use parser::parse;

fn main() {
    let tokens = tokenize("@add(2 @mul(3 @div(10 @sub(2 1))))");
    let parser_ast = parse(tokens);

    println!("{:#?}", parser_ast);
}
