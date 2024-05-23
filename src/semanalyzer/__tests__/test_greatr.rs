#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
    };

    #[test]
    fn test_greatr() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnGreatr, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnGreatr))
        );
    }
}
