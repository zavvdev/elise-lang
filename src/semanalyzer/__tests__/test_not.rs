#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_not() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnNot, vec![])]);
            },
            String,
            messages::args_invalid_amount(to_str!(ExprKind::FnNot), "1", "0")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnNot,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                )]);
            },
            String,
            messages::args_invalid_amount(to_str!(ExprKind::FnNot), "1", "2")
        );
    }
}
