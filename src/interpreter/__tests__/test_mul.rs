#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            eval,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
        types,
    };

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
            eval(&expr, &mut Env::new()),
            EvalResult::Number(6 as types::Number)
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

        assert_eq!(eval(&expr, &mut Env::new()), EvalResult::Number(2.75));
    }

    #[test]
    fn test_mul() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(-1.4), vec![], 0)),
            ],
            0,
        );

        assert_eq!(eval(&expr, &mut Env::new()), EvalResult::Number(-2.8));
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
            eval(&expr, &mut Env::new()),
            EvalResult::Number(3 as types::Number)
        );
    }

    #[test]
    fn test_mul_empty() {
        assert_eq!(
            eval(&Expr::new(ExprKind::FnMul, vec![], 0), &mut Env::new()),
            EvalResult::Number(1 as types::Number)
        );
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"mul\". Expected numbers."
    )]
    fn test_mul_invalid() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::FnPrint, vec![], 0)),
            ],
            0,
        );
        eval(&expr, &mut Env::new());
    }
}
