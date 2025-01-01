#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    // SUCCESS CASES

    #[test]
    fn test_nil() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Nil, vec![], 0);

        assert_eq!(
            interpret(&vec![expr], &mut env, "nil"),
            vec![EvalResult::Nil]
        );
    }
}
