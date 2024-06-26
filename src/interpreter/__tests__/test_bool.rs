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
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Boolean(true), vec![]);

        assert_eq!(eval(&expr, &mut env), EvalResult::Boolean(true));
    }

    #[test]
    fn test_false() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Boolean(false), vec![]);

        assert_eq!(eval(&expr, &mut env), EvalResult::Boolean(false));
    }
}
