#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_bool() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnBool, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnBool))
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
            messages::more_than_one_arg_fn(&format!("{:?}", ExprKind::FnBool))
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
