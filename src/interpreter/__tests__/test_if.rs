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
    fn test_if() {
        let env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(true), vec![])),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![])),
                    ]
                ),
                &env
            ),
            EvalResult::Number(1.0)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![])),
                    ]
                ),
                &env
            ),
            EvalResult::Number(2.0)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(true), vec![])),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![])),
                    ]
                ),
                &env
            ),
            EvalResult::Number(1.0)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![])),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![])),
                    ]
                ),
                &env
            ),
            EvalResult::Nil
        );
    }
}
