#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EnvRecord, EvalResult, FnDeclaration},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    // SUCCESS CASES

    #[test]
    fn test_no_arguments() {
        let mut env = Env::new();

        env.set(
            "test".to_string(),
            EnvRecord {
                value: EvalResult::FnDeclaration(FnDeclaration {
                    name: "test".to_string(),
                    args: vec![],
                    body: vec![Expr::new(ExprKind::Number(1.0), vec![], 0)],
                }),
                mutable: false,
            },
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(ExprKind::FnCustom("test".to_string()), vec![], 0)],
                &mut env,
                ".test()"
            ),
            vec![EvalResult::Number(1.0)]
        );
    }

    #[test]
    fn test_with_arguments() {
        let mut env = Env::new();

        env.set(
            "test".to_string(),
            EnvRecord {
                value: EvalResult::FnDeclaration(FnDeclaration {
                    name: "test".to_string(),
                    args: vec!["a".to_string()],
                    body: vec![Expr::new(ExprKind::Identifier("a".to_string()), vec![], 0)],
                }),
                mutable: false,
            },
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnCustom("test".to_string()),
                    vec![Box::new(Expr::new(ExprKind::Number(22.0), vec![], 0))],
                    0
                )],
                &mut env,
                ".test(22)"
            ),
            vec![EvalResult::Number(22.0)]
        );
    }

    #[test]
    fn test_recursive() {
        let mut env = Env::new();

        env.set(
            "fact".to_string(),
            EnvRecord {
                value: EvalResult::FnDeclaration(FnDeclaration {
                    name: "fact".to_string(),
                    args: vec!["n".to_string()],
                    body: vec![Expr::new(
                        ExprKind::FnIf,
                        vec![
                            Box::new(Expr::new(
                                ExprKind::FnEq,
                                vec![
                                    Box::new(Expr::new(
                                        ExprKind::Identifier("n".to_string()),
                                        vec![],
                                        0,
                                    )),
                                    Box::new(Expr::new(ExprKind::Number(0.0), vec![], 0)),
                                ],
                                0,
                            )),
                            Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                            Box::new(Expr::new(
                                ExprKind::FnMul,
                                vec![
                                    Box::new(Expr::new(
                                        ExprKind::Identifier("n".to_string()),
                                        vec![],
                                        0,
                                    )),
                                    Box::new(Expr::new(
                                        ExprKind::FnCustom("fact".to_string()),
                                        vec![Box::new(Expr::new(
                                            ExprKind::FnSub,
                                            vec![
                                                Box::new(Expr::new(
                                                    ExprKind::Identifier("n".to_string()),
                                                    vec![],
                                                    0,
                                                )),
                                                Box::new(Expr::new(
                                                    ExprKind::Number(1.0),
                                                    vec![],
                                                    0,
                                                )),
                                            ],
                                            0,
                                        ))],
                                        0,
                                    )),
                                ],
                                0,
                            )),
                        ],
                        0,
                    )],
                }),
                mutable: false,
            },
        );

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnCustom("fact".to_string()),
                    vec![Box::new(Expr::new(ExprKind::Number(3.0), vec![], 0))],
                    0
                )],
                &mut env,
                ".fact(3)"
            ),
            vec![EvalResult::Number(6.0)]
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_invalid_amount_of_arguments() {
        let mut env = Env::new();

        env.set(
            "test".to_string(),
            EnvRecord {
                value: EvalResult::FnDeclaration(FnDeclaration {
                    name: "test".to_string(),
                    args: vec!["a".to_string()],
                    body: vec![Expr::new(ExprKind::Number(1.0), vec![], 0)],
                }),
                mutable: false,
            },
        );

        interpret(
            &vec![Expr::new(ExprKind::FnCustom("test".to_string()), vec![], 0)],
            &mut env,
            ".test()",
        );
    }
}
