#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_bool() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnBool, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnBool), "1", "0")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnBool,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                )]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnBool), "1", "2")
        );

        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnBool,
                vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))]
            )]),
            vec![&Expr::new(
                ExprKind::FnBool,
                vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))]
            )]
        )
    }
}
