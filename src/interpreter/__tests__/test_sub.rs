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
    fn test_sub_int() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
            ],
        );

        assert_eq!(
            eval(&expr, &mut Env::new()),
            EvalResult::Number(1 as types::Number)
        );
    }

    #[test]
    fn test_sub_float() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.5), vec![])),
                Box::new(Expr::new(ExprKind::Number(1.1), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &mut Env::new()), EvalResult::Number(1.4));
    }

    #[test]
    fn test_sub() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(-1.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr, &mut Env::new()), EvalResult::Number(2.4));
    }

    #[test]
    fn test_sub_one() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![Box::new(Expr::new(
                ExprKind::Number(1 as types::Number),
                vec![],
            ))],
        );

        assert_eq!(
            eval(&expr, &mut Env::new()),
            EvalResult::Number(-1 as types::Number)
        );
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"sub\". Expected numbers."
    )]
    fn test_sub_invalid() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr, &mut Env::new());
    }
}
