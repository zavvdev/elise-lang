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
    fn test_and() {
        let env = Env::new();

        assert_eq!(
            eval(&Expr::new(ExprKind::FnAnd, vec![],), &env),
            EvalResult::Boolean(true)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![])),],
                ),
                &env
            ),
            EvalResult::Number(2.2)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![])),],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![]))
                    ],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![]))
                    ],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                ),
                &env
            ),
            EvalResult::Nil
        );
    }
}
