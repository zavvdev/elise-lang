#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            eval,
            models::env::{Env, EvalResult},
        },
        parser::models::expression::{Expr, ExprKind},
    };

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

        assert_eq!(eval(&expr, &mut env), EvalResult::Number(1.0));
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

        assert_eq!(eval(&expr, &mut env), EvalResult::Number(3.0));
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

        assert_eq!(eval(&expr, &mut env), EvalResult::Nil);
    }

    #[test]
    #[should_panic(expected = "Interpretation error. Identifier \"x\" already exists")]
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

        eval(&expr, &mut env);
    }
}
