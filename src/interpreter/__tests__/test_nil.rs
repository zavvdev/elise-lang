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
    fn test_nil() {
        let mut env = Env::new();
        let expr = Expr::new(ExprKind::Nil, vec![]);

        assert_eq!(eval(&expr, &mut env), EvalResult::Nil);
    }
}
