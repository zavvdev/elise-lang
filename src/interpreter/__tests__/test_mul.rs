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
    fn test_mul_int() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(3 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".mul(2 3)"),
            vec![EvalResult::Number(6 as types::Number)]
        );
    }

    #[test]
    fn test_mul_float() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.5), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(1.1), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".mul(2.5 1.1)"),
            vec![EvalResult::Number(2.75)]
        );
    }

    #[test]
    fn test_mul_mixed() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(-1.4), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".mul(2 -1.4)"),
            vec![EvalResult::Number(-2.8)]
        );
    }

    #[test]
    fn test_mul_one() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![Box::new(Expr::new(
                ExprKind::Number(3 as types::Number),
                vec![],
                0,
            ))],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".mul(3)"),
            vec![EvalResult::Number(3 as types::Number)]
        );
    }

    #[test]
    fn test_mul_empty() {
        assert_eq!(
            interpret(
                &vec![Expr::new(ExprKind::FnMul, vec![], 0)],
                &mut Env::new(),
                ".mul()"
            ),
            vec![EvalResult::Number(1 as types::Number)]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_mul_invalid() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::FnPrint, vec![], 0)),
            ],
            0,
        );
        interpret(&vec![expr], &mut Env::new(), ".mul(1 .print())");
    }
}
