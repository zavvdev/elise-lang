#[cfg(test)]
mod tests {
    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::analyze_semantics,
    };

    // SUCCESS CASES

    #[test]
    fn test_valid() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnIsNil,
                vec![Box::new(Expr::new(ExprKind::Nil, vec![]))]
            )]),
            ()
        )
    }

    // FAULURE CASES

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnIsNil, vec![])]);
    }

    #[test]
    #[should_panic]
    fn test_2_args() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnIsNil,
            vec![
                Box::new(Expr::new(ExprKind::Nil, vec![])),
                Box::new(Expr::new(ExprKind::Nil, vec![])),
            ],
        )]);
    }
}
