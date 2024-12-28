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
            eval(&Expr::new(ExprKind::FnOr, vec![], 0), &mut env),
            EvalResult::Nil
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![], 0))],
                    0
                ),
                &mut env
            ),
            EvalResult::Number(2.2)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnOr,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
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
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0))
                    ],
                    0
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
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0))
                    ],
                    0
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
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                    ],
                    0
                ),
                &mut env
            ),
            EvalResult::String("123".to_string())
        );
    }
}
