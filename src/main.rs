pub mod lexer;
pub mod messages;
pub mod parser;
pub mod types;

use lexer::tokenize;
use parser::parse;

fn main() {
    let tokens = tokenize(
        "
        @add(
            @sub(4 2)
            @mul(2 2)
            @div(4 2))        
    ",
    );
    let parser_ast = parse(tokens);

    println!("{:#?}", parser_ast);
}
