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
    fn test_let() {
        let mut env = Env::new();

        let expr = Expr::new(
            ExprKind::FnLetBinding,
            vec![
                Box::new(Expr::new(
                    ExprKind::List,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0,
                )),
                Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut env, ".let([x 1] x)"),
            vec![EvalResult::Number(1.0)]
        );
    }

    #[test]
    fn test_let_nested() {
        let mut env = Env::new();

        let expr = Expr::new(
            ExprKind::FnLetBinding,
            vec![
                Box::new(Expr::new(
                    ExprKind::List,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0,
                )),
                Box::new(Expr::new(
                    ExprKind::FnLetBinding,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::List,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("y".to_string()),
                                    vec![],
                                    0,
                                )),
                                Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                            ],
                            0,
                        )),
                        Box::new(Expr::new(
                            ExprKind::FnAdd,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("x".to_string()),
                                    vec![],
                                    0,
                                )),
                                Box::new(Expr::new(
                                    ExprKind::Identifier("y".to_string()),
                                    vec![],
                                    0,
                                )),
                            ],
                            0,
                        )),
                    ],
                    0,
                )),
            ],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut env, ".let([x 1] .let([y 2] .add(x y)))"),
            vec![EvalResult::Number(3.0)]
        );
    }

    #[test]
    fn test_let_empty() {
        let mut env = Env::new();

        let expr = Expr::new(
            ExprKind::FnLetBinding,
            vec![Box::new(Expr::new(
                ExprKind::List,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                    Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                ],
                0,
            ))],
            0,
        );

        assert_eq!(
            interpret(&vec![expr], &mut env, ".let([x 1])"),
            vec![EvalResult::Nil]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_let_rebind() {
        let mut env = Env::new();

        let expr = Expr::new(
            ExprKind::FnLetBinding,
            vec![
                Box::new(Expr::new(
                    ExprKind::List,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0,
                )),
                Box::new(Expr::new(
                    ExprKind::FnLetBinding,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::List,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("x".to_string()),
                                    vec![],
                                    0,
                                )),
                                Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                            ],
                            0,
                        )),
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                    ],
                    0,
                )),
            ],
            0,
        );

        interpret(&vec![expr], &mut env, ".let([x 1] .let([x 2] x))");
    }
}
