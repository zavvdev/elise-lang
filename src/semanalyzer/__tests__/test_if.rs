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
                ExprKind::FnIf,
                vec![
                    Box::new(Expr::new(ExprKind::Nil, vec![])),
                    Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
                ]
            )]),
            ()
        );
    }

    #[test]
    fn test_valid_with_else() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnIf,
                vec![
                    Box::new(Expr::new(ExprKind::Nil, vec![])),
                    Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
                    Box::new(Expr::new(ExprKind::Number(-2.2), vec![])),
                ]
            )]),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_empty_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnIf, vec![])]);
    }

    #[test]
    #[should_panic]
    fn test_one_arg() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnIf,
            vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_four_args() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnIf,
            vec![
                Box::new(Expr::new(ExprKind::Nil, vec![])),
                Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.3), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
            ],
        )]);
    }
}
