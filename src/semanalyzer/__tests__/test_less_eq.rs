#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_less_eq() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLessEq, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnLessEq))
        );
    }
}
