#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
        types,
    };

    // SUCCESS CASES

    #[test]
    fn test_div_int() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(4 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".div(4 2)"),
            vec![EvalResult::Number(2 as types::Number)]
        );
    }

    #[test]
    fn test_div_float() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(5.5), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(2.2), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".div(5.5 2.2)"),
            vec![EvalResult::Number(2.5)]
        );
    }

    #[test]
    fn test_div_mixed() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(-1.6), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".div(2 -1.6)"),
            vec![EvalResult::Number(-1.25)]
        );
    }

    #[test]
    fn test_div_one_arg() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![Box::new(Expr::new(
                ExprKind::Number(2 as types::Number),
                vec![],
                0,
            ))],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".div(2)"),
            vec![EvalResult::Number(0.5)]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_div_invalid() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::FnPrint, vec![], 0)),
            ],
            0,
        );
        interpret(&vec![expr], &mut Env::new(), ".div(1 .print())");
    }

    #[test]
    #[should_panic]
    fn test_div_division_by_zero_single_arg() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![Box::new(Expr::new(
                ExprKind::Number(0 as types::Number),
                vec![],
                0,
            ))],
            0,
        );
        interpret(&vec![expr], &mut Env::new(), ".div(0)");
    }

    #[test]
    #[should_panic]
    fn test_div_division_by_zero() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.4), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(0 as types::Number), vec![], 0)),
            ],
            0,
        );
        interpret(&vec![expr], &mut Env::new(), ".div(2.4 0)");
    }
}
