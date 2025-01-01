#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
        types,
    };

    // SUCCESS CASES

    #[test]
    fn test_less_eq() {
        let mut env = Env::new();
        let expr = Expr::new(
            ExprKind::FnLessEq,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut env, ".less-eq(2 1)"),
            vec![EvalResult::Boolean(false)]
        );
    }

    #[test]
    fn test_less_eq_multiple() {
        let mut env = Env::new();
        let expr = Expr::new(
            ExprKind::FnLessEq,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut env, ".less-eq(1 2 2)"),
            vec![EvalResult::Boolean(true)]
        );
    }
}
