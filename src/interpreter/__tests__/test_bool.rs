#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    #[test]
    fn test_true() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Boolean(true), vec![], 0);

        assert_eq!(
            interpret(&vec![expr], &mut env, "true"),
            vec![EvalResult::Boolean(true)]
        );
    }

    #[test]
    fn test_false() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Boolean(false), vec![], 0);

        assert_eq!(
            interpret(&vec![expr], &mut env, "false"),
            vec![EvalResult::Boolean(false)]
        );
    }
}
