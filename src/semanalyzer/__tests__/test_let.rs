#[cfg(test)]
mod tests {
    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::analyze_semantics,
    };

    // SUCCESS CASES

    #[test]
    fn test_correct_form() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::List,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("x".to_string()),
                                    vec![],
                                    0
                                )),
                                Box::new(Expr::new(ExprKind::Number(3.4), vec![], 0)),
                            ],
                            0
                        )),
                        Box::new(Expr::new(
                            ExprKind::FnPrint,
                            vec![Box::new(Expr::new(
                                ExprKind::Identifier("x".to_string()),
                                vec![],
                                0
                            ))],
                            0
                        ))
                    ],
                    0
                )],
                ".let([x 3.4] .print(x))"
            ),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_invalid_type_first_arg() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnLetBinding,
                vec![Box::new(Expr::new(ExprKind::Nil, vec![], 0))],
                0,
            )],
            ".let(nil)",
        );
    }

    #[test]
    #[should_panic]
    fn test_1_arg() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnLetBinding,
                vec![Box::new(Expr::new(
                    ExprKind::List,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(3.4), vec![], 0)),
                    ],
                    0,
                ))],
                0,
            )],
            ".let([x 3.4])",
        );
    }

    #[test]
    #[should_panic]
    fn test_bindings_not_even() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnLetBinding,
                vec![Box::new(Expr::new(
                    ExprKind::List,
                    vec![Box::new(Expr::new(ExprKind::Number(3.4), vec![], 0))],
                    0,
                ))],
                0,
            )],
            ".let([3.4])",
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_binding_form() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnLetBinding,
                vec![Box::new(Expr::new(
                    ExprKind::List,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(3.4), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(3.4), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(3.4), vec![], 0)),
                    ],
                    0,
                ))],
                0,
            )],
            ".let([x 3.4, 3.4, 3.4])",
        );
    }

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(
            &vec![Expr::new(ExprKind::FnLetBinding, vec![], 0)],
            ".let()",
        );
    }
}
