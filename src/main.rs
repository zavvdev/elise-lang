pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod types;

use interpreter::interpret;
use lexer::tokenize;
use parser::parse;

fn main() {
    let tokens = tokenize(
        "
        @println(123) 
    ",
    );

    let parser_ast = parse(tokens);

    interpret(&parser_ast);
}
