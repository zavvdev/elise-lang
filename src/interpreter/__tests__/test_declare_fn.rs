#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            interpret,
            models::env::{Env, EvalResult, FnDeclaration},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    // SUCCESS CASES

    #[test]
    fn test_define_no_arguments() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("test".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(ExprKind::List, vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".fn(test [] 1)"
            ),
            vec![EvalResult::FnDeclaration(FnDeclaration {
                name: "test".to_string(),
                args: vec![],
                body: vec![Expr::new(ExprKind::Number(1.0), vec![], 0)],
            })]
        );
    }

    #[test]
    fn test_define_with_arguments() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("test".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(
                            ExprKind::List,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("a".to_string()),
                                    vec![],
                                    0
                                )),
                                Box::new(Expr::new(
                                    ExprKind::Identifier("b".to_string()),
                                    vec![],
                                    0
                                )),
                            ],
                            0
                        )),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![], 0)),
                    ],
                    0
                )],
                &mut env,
                ".fn(test [a b] 1)"
            ),
            vec![EvalResult::FnDeclaration(FnDeclaration {
                name: "test".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
                body: vec![Expr::new(ExprKind::Number(1.0), vec![], 0)],
            })]
        );
    }

    #[test]
    fn test_define_nested() {
        let mut env = Env::new();

        assert_eq!(
            interpret(
                &vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("test".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(ExprKind::List, vec![], 0)),
                        Box::new(Expr::new(
                            ExprKind::FnDefine,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("test-nested".to_string()),
                                    vec![],
                                    0
                                )),
                                Box::new(Expr::new(ExprKind::List, vec![], 0)),
                                Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                            ],
                            0
                        )),
                    ],
                    0
                )],
                &mut env,
                ".fn(test [] .fn(test-nested [] 2))"
            ),
            vec![EvalResult::FnDeclaration(FnDeclaration {
                name: "test".to_string(),
                args: vec![],
                body: vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("test-nested".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(ExprKind::List, vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![], 0)),
                    ],
                    0
                )],
            })]
        );
    }
}
