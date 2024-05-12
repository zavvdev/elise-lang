pub mod config;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod semanalyzer;
pub mod types;

use interpreter::{interpret, models::Env};
use lexer::tokenize;
use parser::parse;
use semanalyzer::analyze_semantics;

pub fn execute(content: String, env: Env) {
    let tokens = tokenize(&content);
    let ast = parse(tokens);
    let ast = analyze_semantics(&ast);

    interpret(ast, env);
}