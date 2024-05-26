#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            eval,
            models::env::{Env, EnvRecord, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

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

        let expr = Expr::new(ExprKind::Identifier("x".to_string()), vec![]);

        assert_eq!(eval(&expr, &mut env), EvalResult::Number(1.0));
    }

    #[test]
    #[should_panic(expected = "Interpretation error. Undefined identifier \"x\".")]
    fn test_identifier_undefined() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Identifier("x".to_string()), vec![]);
        eval(&expr, &mut env);
    }
}
