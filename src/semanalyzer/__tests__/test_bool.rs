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
                ExprKind::FnBool,
                vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))]
            )]),
            ()
        )
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_args_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnBool, vec![])]);
    }

    #[test]
    #[should_panic]
    fn test_args_more_than_one() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnBool,
            vec![
                Box::new(Expr::new(ExprKind::Nil, vec![])),
                Box::new(Expr::new(ExprKind::Nil, vec![])),
            ],
        )]);
    }
}
