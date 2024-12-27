#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_sub() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnSub, vec![])]);
            },
            String,
            messages::args_invalid_amount(to_str!(ExprKind::FnSub), "> 0", "0")
        );
    }
}
