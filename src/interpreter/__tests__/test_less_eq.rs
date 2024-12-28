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
        let mut env = Env::new();
        let expr = Expr::new(
            ExprKind::FnLessEq,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(eval(&expr, &mut env), EvalResult::Boolean(false));
    }

    #[test]
    fn test_less_eq_multiple() {
        let mut env = Env::new();
        let expr = Expr::new(
            ExprKind::FnLessEq,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(eval(&expr, &mut env), EvalResult::Boolean(true));
    }
}
