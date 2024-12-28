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
                    ExprKind::FnSub,
                    vec![Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0))],
                    0
                )],
                ".sub(1)"
            ),
            ()
        );
    }

    #[test]
    fn test_2_args() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnSub,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0))
                    ],
                    0
                )],
                ".sub(4 2)"
            ),
            ()
        );
    }

    #[test]
    fn test_many_args() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnSub,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(6.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(4.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0))
                    ],
                    0
                )],
                ".sub(6 4 2 1)"
            ),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnSub, vec![], 0)], ".sub()");
    }
}
