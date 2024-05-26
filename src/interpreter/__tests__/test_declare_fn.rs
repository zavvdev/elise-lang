#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            eval,
            models::env::{Env, EvalResult, FnDeclaration},
        },
        parser::models::expression::{Expr, ExprKind},
    };

    #[test]
    fn test_define_no_arguments() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("test".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::List, vec![])),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![])),
                    ]
                ),
                &mut env
            ),
            EvalResult::FnDeclaration(FnDeclaration {
                name: "test".to_string(),
                args: vec![],
                body: vec![Expr::new(ExprKind::Number(1.0), vec![])],
            })
        );
    }

    #[test]
    fn test_define_with_arguments() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("test".to_string()), vec![])),
                        Box::new(Expr::new(
                            ExprKind::List,
                            vec![
                                Box::new(Expr::new(ExprKind::Identifier("a".to_string()), vec![])),
                                Box::new(Expr::new(ExprKind::Identifier("b".to_string()), vec![])),
                            ]
                        )),
                        Box::new(Expr::new(ExprKind::Number(1.0), vec![])),
                    ]
                ),
                &mut env
            ),
            EvalResult::FnDeclaration(FnDeclaration {
                name: "test".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
                body: vec![Expr::new(ExprKind::Number(1.0), vec![])],
            })
        );
    }

    #[test]
    fn test_define_nested() {
        let mut env = Env::new();

        assert_eq!(
            eval(
                &Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("test".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::List, vec![])),
                        Box::new(Expr::new(
                            ExprKind::FnDefine,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("test-nested".to_string()),
                                    vec![]
                                )),
                                Box::new(Expr::new(ExprKind::List, vec![])),
                                Box::new(Expr::new(ExprKind::Number(2.0), vec![])),
                            ]
                        )),
                    ]
                ),
                &mut env
            ),
            EvalResult::FnDeclaration(FnDeclaration {
                name: "test".to_string(),
                args: vec![],
                body: vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("test-nested".to_string()),
                            vec![]
                        )),
                        Box::new(Expr::new(ExprKind::List, vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.0), vec![])),
                    ]
                )],
            })
        );
    }
}
