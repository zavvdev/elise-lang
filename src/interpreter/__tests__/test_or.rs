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
    fn test_empty() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(ExprKind::FnOr, vec![], 0)],
                &mut env,
                ".or()"
            ),
            vec![EvalResult::Nil]
        );
    }

    #[test]
    fn test_one_arg() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnOr,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![], 0))],
                    0
                )],
                &mut env,
                ".or(2.2)"
            ),
            vec![EvalResult::Number(2.2)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnOr,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
                )],
                &mut env,
                ".or(false)"
            ),
            vec![EvalResult::Boolean(false)]
        );
    }

    #[test]
    fn test_many_args() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnOr,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0))
                    ],
                    0
                )],
                &mut env,
                ".or(false nil)"
            ),
            vec![EvalResult::Nil]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnOr,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0))
                    ],
                    0
                )],
                &mut env,
                ".or(false nil \"123\")"
            ),
            vec![EvalResult::String("123".to_string())]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnOr,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".or(false \"123\" nil)"
            ),
            vec![EvalResult::String("123".to_string())]
        );
    }
}
