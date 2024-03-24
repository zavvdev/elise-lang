use ast::lexer::tokenize;

pub mod ast;

fn main() {
    let tokens = tokenize("33.3213");

    println!("{:#?}", tokens);
}
