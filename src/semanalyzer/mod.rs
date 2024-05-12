pub mod analyzer;
pub mod messages;

use self::analyzer::analyze;
use crate::parser::models::ast::Expr;

pub fn analyze_semantics(ast: &Vec<Expr>) -> Vec<&Expr> {
    let mut result: Vec<&Expr> = Vec::new();

    for expr in ast {
        let next_expr: &Expr = analyze(expr);

        result.push(next_expr);
    }

    result
}
