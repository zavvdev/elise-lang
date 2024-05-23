#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            eval,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    #[test]
    fn test_true() {
        let env = Env::new();
        let expr = Expr::new(ExprKind::Boolean(true), vec![]);

        assert_eq!(eval(&expr, &env), EvalResult::Boolean(true));
    }

    #[test]
    fn test_false() {
        let env = Env::new();
        let expr = Expr::new(ExprKind::Boolean(false), vec![]);

        assert_eq!(eval(&expr, &env), EvalResult::Boolean(false));
    }
}
