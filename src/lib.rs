pub mod config;
pub mod interpreter;
pub mod lexer;
pub mod macros;
pub mod messages;
pub mod parser;
pub mod semanalyzer;
pub mod types;

use interpreter::{interpret, models::env::Env};
use lexer::tokenize;
use parser::parse;
use semanalyzer::analyze_semantics;

pub fn execute(content: String, env: &mut Env) {
    let content = content.trim();

    let tokens = tokenize(&content);
    let expressions = parse(tokens, &content);

    analyze_semantics(&expressions, &content);

    interpret(&expressions, env, &content);
}
