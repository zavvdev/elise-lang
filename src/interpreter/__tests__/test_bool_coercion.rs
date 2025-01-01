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
    fn test_true() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(
                        ExprKind::String("2".to_string()),
                        vec![],
                        0
                    ))],
                    0
                )],
                &mut env,
                ".bool(\"2\")"
            ),
            vec![EvalResult::Boolean(true)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![], 0))],
                    0
                )],
                &mut env,
                ".bool(2.2)"
            ),
            vec![EvalResult::Boolean(true)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![], 0))],
                    0
                )],
                &mut env,
                ".bool(true)"
            ),
            vec![EvalResult::Boolean(true)]
        );
    }

    #[test]
    fn test_false() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![], 0))],
                    0
                )],
                &mut env,
                ".bool(nil)"
            ),
            vec![EvalResult::Boolean(false)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnBool,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
                )],
                &mut env,
                ".bool(false)"
            ),
            vec![EvalResult::Boolean(false)]
        );
    }
}
