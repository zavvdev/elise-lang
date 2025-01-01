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
    fn test_if_branch() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(true), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".if(true 1 2)"
            ),
            vec![EvalResult::Number(1.0)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(true), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".if(true 1)"
            ),
            vec![EvalResult::Number(1.0)]
        );
    }

    #[test]
    fn test_else_branch() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".if(false 1 2)"
            ),
            vec![EvalResult::Number(2.0)]
        );
    }

    #[test]
    fn test_implicit_else_branch() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".if(false 1)"
            ),
            vec![EvalResult::Nil]
        );
    }
}
