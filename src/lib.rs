pub mod config;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod types;

use interpreter::interpret;
use lexer::tokenize;
use parser::parse;

pub fn execute(content: String) {
    let tokens = tokenize(&content);
    let ast = parse(tokens);
    interpret(&ast);
}
