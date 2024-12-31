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
                &vec![Expr::new(ExprKind::FnAnd, vec![], 0)],
                &mut env,
                ".and()"
            ),
            vec![EvalResult::Boolean(true)]
        );
    }

    #[test]
    fn test_number() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnAnd,
                    vec![Box::new(Expr::new(ExprKind::Number(2.2), vec![], 0))],
                    0
                )],
                &mut env,
                ".and(2.2)"
            ),
            vec![EvalResult::Number(2.2)]
        );
    }

    #[test]
    fn test_false() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnAnd,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
                )],
                &mut env,
                ".and(false)"
            ),
            vec![EvalResult::Boolean(false)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0))
                    ],
                    0
                )],
                &mut env,
                ".and(false, nil)"
            ),
            vec![EvalResult::Boolean(false)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0))
                    ],
                    0
                )],
                &mut env,
                ".and(false, nil, \"123\")"
            ),
            vec![EvalResult::Boolean(false)]
        );
    }

    #[test]
    fn test_last() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnAnd,
                    vec![
                        Box::new(Expr::new(ExprKind::String("123".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".and(\"123\", nil)"
            ),
            vec![EvalResult::Nil]
        );
    }
}
