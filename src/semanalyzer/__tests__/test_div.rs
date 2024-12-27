#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_div() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnDiv, vec![])]);
            },
            String,
            messages::args_invalid_amount(to_str!(ExprKind::FnDiv), "> 0", "0")
        );
    }
}
