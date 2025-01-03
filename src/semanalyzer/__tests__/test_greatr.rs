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
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnGreatr,
                    vec![Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0))],
                    0
                )],
                ".greatr(4)"
            ),
            ()
        );
    }

    #[test]
    fn test_2_args() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnGreatr,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0))
                    ],
                    0
                )],
                ".greatr(4 4)"
            ),
            ()
        );
    }

    #[test]
    fn test_many_args() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnGreatr,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0))
                    ],
                    0
                )],
                ".greatr(4 4 4 4)"
            ),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_empty_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnGreatr, vec![], 0)], ".greatr()");
    }
}
