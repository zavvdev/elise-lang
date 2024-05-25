#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_less_eq() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLessEq, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnLessEq), "> 0", "0")
        );
    }
}
