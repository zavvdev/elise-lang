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
    fn test_greatr() {
        let mut env = Env::new();
        let expr = Expr::new(
            ExprKind::FnGreatr,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &mut env), EvalResult::Boolean(true));
    }

    #[test]
    fn test_greatr_multiple() {
        let mut env = Env::new();
        let expr = Expr::new(
            ExprKind::FnGreatr,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &mut env), EvalResult::Boolean(false));
    }
}
