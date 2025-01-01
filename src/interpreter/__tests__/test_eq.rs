#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    // SUCCESS CASES

    #[test]
    fn test_eq() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnEq,
                    vec![
                        Box::new(Expr::new(ExprKind::String("2".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("2".to_string()), vec![], 0))
                    ],
                    0
                )],
                &mut env,
                ".eq(\"2\" \"2\")"
            ),
            vec![EvalResult::Boolean(true)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnEq,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(2.2), vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("2".to_string()), vec![], 0))
                    ],
                    0
                )],
                &mut env,
                ".eq(2.2 \"2\")"
            ),
            vec![EvalResult::Boolean(false)]
        );
    }
}
