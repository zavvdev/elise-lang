#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_let() {
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
