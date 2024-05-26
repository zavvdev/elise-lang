#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            eval,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
        types,
    };

    #[test]
    fn test_add_int() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
            ],
        );

        assert_eq!(
            eval(&expr, &mut Env::new()),
            EvalResult::Number(3 as types::Number)
        );
    }

    #[test]
    fn test_add_float() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1.1), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &mut Env::new()), EvalResult::Number(3.5));
    }

    #[test]
    fn test_add() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &mut Env::new()), EvalResult::Number(3.4));
    }

    #[test]
    fn test_add_empty() {
        let expr = Expr::new(ExprKind::FnAdd, vec![]);
        assert_eq!(
            eval(&expr, &mut Env::new()),
            EvalResult::Number(0 as types::Number)
        );
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"add\". Expected numbers."
    )]
    fn test_add_invalid() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr, &mut Env::new());
    }
}
