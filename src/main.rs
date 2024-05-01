pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod types;

use std::fs;

use interpreter::interpret;
use lexer::tokenize;
use parser::parse;

fn main() {
    match fs::read_to_string("sample.txt") {
        Ok(content) => {
            let tokens = tokenize(&content);
            let parser_ast = parse(tokens);
            interpret(&parser_ast);
        }
        Err(e) => {
            println!("Cannot read sample.txt: {}", e);
        }
    }
}
