#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_sub() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnSub, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnSub))
        );
    }
}
