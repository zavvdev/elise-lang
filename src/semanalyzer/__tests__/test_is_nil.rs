#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_is_nil() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnIsNil, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnIsNil), "1", "0")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnIsNil,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                )]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnIsNil), "1", "2")
        );

        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnIsNil,
                vec![Box::new(Expr::new(ExprKind::Nil, vec![]))]
            )]),
            vec![&Expr::new(
                ExprKind::FnIsNil,
                vec![Box::new(Expr::new(ExprKind::Nil, vec![]))]
            )]
        )
    }
}
