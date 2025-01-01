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
    fn test_not_bool() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![], 0))],
                    0
                )],
                &mut env,
                ".not(true)"
            ),
            vec![EvalResult::Boolean(false)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
                )],
                &mut env,
                ".not(false)"
            ),
            vec![EvalResult::Boolean(true)]
        );
    }

    #[test]
    fn test_not_nil() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![], 0))],
                    0
                )],
                &mut env,
                ".not(nil)"
            ),
            vec![EvalResult::Boolean(true)]
        );
    }
    #[test]
    fn test_not_other() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(
                        ExprKind::String("".to_string()),
                        vec![],
                        0
                    ))],
                    0
                )],
                &mut env,
                ".not(\"\")"
            ),
            vec![EvalResult::Boolean(false)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnNot,
                    vec![Box::new(Expr::new(ExprKind::Number(0.0), vec![], 0))],
                    0
                )],
                &mut env,
                ".not(0)"
            ),
            vec![EvalResult::Boolean(false)]
        );
    }
}
