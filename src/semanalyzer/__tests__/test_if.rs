#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_if() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnIf, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnIf))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnIf,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                )]);
            },
            String,
            messages::too_few_args_fn(&format!("{:?}", ExprKind::FnIf))
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
            messages::too_many_args_fn(&format!("{:?}", ExprKind::FnIf))
        );
    }
}
