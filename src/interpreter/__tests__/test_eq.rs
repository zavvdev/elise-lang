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
    fn test_eq() {
        let env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnEq,
                    vec![
                        Box::new(Expr::new(ExprKind::String("2".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::String("2".to_string()), vec![]))
                    ],
                ),
                &env
            ),
            EvalResult::Boolean(true)
        );

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnEq,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
                        Box::new(Expr::new(ExprKind::String("2".to_string()), vec![]))
                    ],
                ),
                &env
            ),
            EvalResult::Boolean(false)
        );
    }
}