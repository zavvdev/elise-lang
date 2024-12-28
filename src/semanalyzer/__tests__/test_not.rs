#[cfg(test)]
mod tests {
    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::analyze_semantics,
    };

    // SUCCESS CASES

    #[test]
    fn test_correct_form() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnNot,
                vec![Box::new(Expr::new(ExprKind::Number(4.0), vec![])),]
            )]),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnNot, vec![])]);
    }

    #[test]
    #[should_panic]
    fn test_2_args() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnNot,
            vec![
                Box::new(Expr::new(ExprKind::Nil, vec![])),
                Box::new(Expr::new(ExprKind::Nil, vec![])),
            ],
        )]);
    }
}
