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
    fn test_less_eq() {
        let env = Env::new();
        let expr = Expr::new(
            ExprKind::FnLessEq,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &env), EvalResult::Boolean(false));
    }

    #[test]
    fn test_less_eq_multiple() {
        let env = Env::new();
        let expr = Expr::new(
            ExprKind::FnLessEq,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &env), EvalResult::Boolean(true));
    }
}
