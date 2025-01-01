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
    fn test_sub_int() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".sub(2 1)"),
            vec![EvalResult::Number(1 as types::Number)]
        );
    }

    #[test]
    fn test_sub_float() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.5), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(1.1), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".sub(2.5 1.1)"),
            vec![EvalResult::Number(1.4)]
        );
    }

    #[test]
    fn test_sub_mixed() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(-1.4), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".sub(1 -1.4)"),
            vec![EvalResult::Number(2.4)]
        );
    }

    #[test]
    fn test_sub_one() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![Box::new(Expr::new(
                ExprKind::Number(1 as types::Number),
                vec![],
                0,
            ))],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut Env::new(), ".sub(1)"),
            vec![EvalResult::Number(-1 as types::Number)]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_sub_invalid() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::FnPrint, vec![], 0)),
            ],
            0,
        );
        interpret(&vec![expr], &mut Env::new(), ".sub(1 .print())");
    }
}
