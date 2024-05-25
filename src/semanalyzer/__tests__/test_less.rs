#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_less() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLess, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnLess), "> 0", "0")
        );
    }
}
