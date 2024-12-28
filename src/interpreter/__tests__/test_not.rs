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
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![], 0))],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(true)
        );
    }

    #[test]
    fn test_not_nil() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![], 0))],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(true)
        );
    }
    #[test]
    fn test_not_other() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(
                        ExprKind::String("".to_string()),
                        vec![],
                        0
                    ))],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Number(0.0), vec![], 0))],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );
    }
}
