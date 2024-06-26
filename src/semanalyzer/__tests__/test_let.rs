#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_let() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLetBinding, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnLetBinding), ">= 1", "0")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                )]);
            },
            String,
            messages::invalid_arg_type(
                to_str!(ExprKind::FnLetBinding),
                1,
                to_str!(ExprKind::List),
                to_str!(ExprKind::Nil)
            )
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
            messages::invalid_args_amount(to_str!(ExprKind::FnLetBinding), "even", "1")
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
            messages::invalid_let_binding_form()
        );
    }
}
