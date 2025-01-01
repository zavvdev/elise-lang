#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EnvRecord, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    // SUCCESS CASES

    #[test]
    fn test_identifier() {
        let mut env = Env::new();

        env.set(
            "x".to_string(),
            EnvRecord {
                value: EvalResult::Number(1.0),
                mutable: false,
            },
        );

        let expr = Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0);

        assert_eq!(
            interpret(&vec![expr], &mut env, "x"),
            vec![EvalResult::Number(1.0)]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_identifier_undefined() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0);
        interpret(&vec![expr], &mut env, "x");
    }
}
