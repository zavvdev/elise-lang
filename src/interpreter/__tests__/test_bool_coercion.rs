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
    fn test_bool() {
        let env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(
                        ExprKind::String("2".to_string()),
                        vec![]
                    )),],
                ),
                &env
            ),
            EvalResult::Boolean(true)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![])),],
                ),
                &env
            ),
            EvalResult::Boolean(true)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![])),],
                ),
                &env
            ),
            EvalResult::Boolean(true)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![])),],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![])),],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );
    }
}
