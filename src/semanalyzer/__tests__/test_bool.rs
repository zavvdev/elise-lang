#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    // SUCCESS CASES

    #[test]
    fn test_valid() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnBool,
                vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))]
            )]),
            ()
        )
    }

    // FAILURE CASES

    #[test]
    fn test_args_invalid_amount() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnBool, vec![])]);
            },
            String,
            messages::args_invalid_amount(to_str!(ExprKind::FnBool), "1", "0")
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
            messages::args_invalid_amount(to_str!(ExprKind::FnBool), "1", "2")
        );
    }
}
