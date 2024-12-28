#[cfg(test)]
mod tests {
    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::analyze_semantics,
    };

    // SUCCESS CASES

    #[test]
    fn test_1_arg() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDiv,
                vec![Box::new(Expr::new(ExprKind::Number(1.0), vec![]))]
            )]),
            ()
        );
    }

    #[test]
    fn test_2_args() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDiv,
                vec![
                    Box::new(Expr::new(ExprKind::Number(4.0), vec![])),
                    Box::new(Expr::new(ExprKind::Number(2.0), vec![]))
                ]
            )]),
            ()
        );
    }

    #[test]
    fn test_many_args() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDiv,
                vec![
                    Box::new(Expr::new(ExprKind::Number(6.0), vec![])),
                    Box::new(Expr::new(ExprKind::Number(4.0), vec![])),
                    Box::new(Expr::new(ExprKind::Number(2.0), vec![])),
                    Box::new(Expr::new(ExprKind::Number(1.0), vec![]))
                ]
            )]),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_empty_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnDiv, vec![])]);
    }
}
