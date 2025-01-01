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
    fn test_false() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIsNil,
                    vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![], 0))],
                    0
                )],
                &mut env,
                ".nil?(true)"
            ),
            vec![EvalResult::Boolean(false)]
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIsNil,
                    vec![Box::new(Expr::new(ExprKind::Boolean(false), vec![], 0))],
                    0
                )],
                &mut env,
                ".nil?(false)"
            ),
            vec![EvalResult::Boolean(false)]
        );
    }

    #[test]
    fn test_is_nil_true() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnIsNil,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![], 0))],
                    0
                )],
                &mut env,
                ".nil?(nil)"
            ),
            vec![EvalResult::Boolean(true)]
        );
    }
}
