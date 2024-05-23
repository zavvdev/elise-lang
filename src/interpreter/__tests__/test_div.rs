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
    fn test_div_int() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(4 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
            ],
        );

        assert_eq!(
            eval(&expr, &Env::new()),
            EvalResult::Number(2 as types::Number)
        );
    }

    #[test]
    fn test_div_float() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(5.5), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &Env::new()), EvalResult::Number(2.5));
    }

    #[test]
    fn test_div() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(-1.6), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &Env::new()), EvalResult::Number(-1.25));
    }

    #[test]
    fn test_div_one() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![Box::new(Expr::new(
                ExprKind::Number(2 as types::Number),
                vec![],
            ))],
        );

        assert_eq!(eval(&expr, &Env::new()), EvalResult::Number(0.5));
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"div\". Expected numbers."
    )]
    fn test_div_invalid() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr, &Env::new());
    }

    #[test]
    #[should_panic(expected = "Interpretation error. Division by zero.")]
    fn test_div_division_by_zero_single_arg() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![Box::new(Expr::new(
                ExprKind::Number(0 as types::Number),
                vec![],
            ))],
        );
        eval(&expr, &Env::new());
    }

    #[test]
    #[should_panic(expected = "Interpretation error. Division by zero.")]
    fn test_div_division_by_zero() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
                Box::new(Expr::new(ExprKind::Number(0 as types::Number), vec![])),
            ],
        );
        eval(&expr, &Env::new());
    }
}
