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
    fn test_not_bool() {
        let env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![]))],
                ),
                &env
            ),
            EvalResult::Boolean(true)
        );
    }

    #[test]
    fn test_not_nil() {
        let env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                ),
                &env
            ),
            EvalResult::Boolean(true)
        );
    }
    #[test]
    fn test_not_other() {
        let env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(
                        ExprKind::String("".to_string()),
                        vec![]
                    ))],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Number(0.0), vec![]))],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );
    }
}
