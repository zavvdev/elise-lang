#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_if() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnIf, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnIf), "2 or 3", "0")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnIf,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                )]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnIf), "2 or 3", "1")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.3), vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
                    ],
                )]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnIf), "2 or 3", "4")
        );
    }
}
