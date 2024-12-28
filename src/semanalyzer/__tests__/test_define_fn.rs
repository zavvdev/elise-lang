#[cfg(test)]
mod tests {
    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::analyze_semantics,
    };

    // SUCCESS CASES

    #[test]
    fn test_second_arg_empty_list() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                    Box::new(Expr::new(ExprKind::List, vec![])),
                    Box::new(Expr::new(
                        ExprKind::FnPrintLn,
                        vec![Box::new(Expr::new(
                            ExprKind::String("Hello".to_string()),
                            vec![]
                        ))]
                    )),
                ],
            )]),
            ()
        );
    }

    #[test]
    fn test_second_arg_valid_list() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                    Box::new(Expr::new(
                        ExprKind::List,
                        vec![
                            Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                            Box::new(Expr::new(ExprKind::Identifier("y".to_string()), vec![]))
                        ]
                    )),
                    Box::new(Expr::new(
                        ExprKind::FnPrintLn,
                        vec![
                            Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                            Box::new(Expr::new(ExprKind::Identifier("y".to_string()), vec![]))
                        ]
                    )),
                ],
            )]),
            ()
        );
    }

    // FAILED CASES

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnDefine, vec![])]);
    }

    #[test]
    #[should_panic]
    fn test_1_arg() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnDefine,
            vec![Box::new(Expr::new(
                ExprKind::Identifier("hello".to_string()),
                vec![],
            ))],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_2_args() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                    Box::new(Expr::new(ExprKind::List, vec![])),
                ],
            )]),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn test_fist_arg_type() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnDefine,
            vec![
                Box::new(Expr::new(ExprKind::Nil, vec![])),
                Box::new(Expr::new(ExprKind::List, vec![])),
            ],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_second_arg_type() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnDefine,
            vec![
                Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                Box::new(Expr::new(ExprKind::Nil, vec![])),
            ],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_duplicated_argument() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnDefine,
            vec![
                Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                Box::new(Expr::new(
                    ExprKind::List,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                    ],
                )),
            ],
        )]);
    }
}
