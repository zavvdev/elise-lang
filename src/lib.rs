pub mod config;
pub mod interpreter;
pub mod lexer;
pub mod macros;
pub mod parser;
pub mod semanalyzer;
pub mod types;
pub mod messages;

use interpreter::{interpret, models::env::Env};
use lexer::tokenize;
use parser::parse;
use semanalyzer::analyze_semantics;

pub fn execute(content: String, env: &mut Env) {
    let tokens = tokenize(&content);
        
    let expressions = parse(tokens, &content);
    let expressions = analyze_semantics(&expressions);

    interpret(expressions, env);
}
