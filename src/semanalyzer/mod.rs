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

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::ast::ExprKind;
    use assert_panic::assert_panic;

    // ==========================
    //
    //        Let Binding
    //
    // ==========================

    #[test]
    fn test_let_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLetBinding, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnLetBinding))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                )]);
            },
            String,
            messages::let_binding_first_arg_list()
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![Box::new(Expr::new(
                        ExprKind::List,
                        vec![Box::new(Expr::new(ExprKind::Number(3.4), vec![]))],
                    ))],
                )]);
            },
            String,
            messages::let_binding_first_arg_even_elements()
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![Box::new(Expr::new(
                        ExprKind::List,
                        vec![
                            Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                            Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                            Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                            Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                        ],
                    ))],
                )]);
            },
            String,
            messages::let_binding_arg_identifiers()
        );
    }
}
