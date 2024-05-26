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
    fn test_is_nil_false() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIsNil,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![])),]
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIsNil,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![])),]
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );
    }

    #[test]
    fn test_is_nil_true() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIsNil,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![])),]
                ),
                &mut env
            ),
            EvalResult::Boolean(true)
        );
    }
}
