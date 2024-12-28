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
        let mut env = Env::new();

        assert_eq!(
            eval(&Expr::new(ExprKind::FnAnd, vec![], 0), &mut env),
            EvalResult::Boolean(true)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
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
                    ExprKind::FnAnd,
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
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0))
                    ],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0))
                    ],
                    0
                ),
                &mut env
            ),
            EvalResult::Boolean(false)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                    ],
                    0
                ),
                &mut env
            ),
            EvalResult::Nil
        );
    }
}
