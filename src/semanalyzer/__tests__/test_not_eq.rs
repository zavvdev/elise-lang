#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_not_eq() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnNotEq, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnNotEq))
        );
    }
}
