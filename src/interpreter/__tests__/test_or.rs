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
    fn test_or() {
        let mut env = Env::new();

        assert_eq!(
            eval(&Expr::new(ExprKind::FnOr, vec![],), &mut env),
            EvalResult::Nil
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![])),],
                ),
                &mut env
            ),
            EvalResult::Number(2.2)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![])),],
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![]))
                    ],
                ),
                &mut env
            ),
            EvalResult::Nil
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![]))
                    ],
                ),
                &mut env
            ),
            EvalResult::String("123".to_string())
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                ),
                &mut env
            ),
            EvalResult::String("123".to_string())
        );
    }
}
