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
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("hello".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(ExprKind::List, vec![], 0)),
                        Box::new(Expr::new(
                            ExprKind::FnPrintLn,
                            vec![Box::new(Expr::new(
                                ExprKind::String("Hello".to_string()),
                                vec![],
                                0
                            ))],
                            0
                        )),
                    ],
                    0
                )],
                ".fn(hello [] .println(\"Hello\"))"
            ),
            ()
        );
    }

    #[test]
    fn test_second_arg_valid_list() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("hello".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(
                            ExprKind::List,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("x".to_string()),
                                    vec![],
                                    0
                                )),
                                Box::new(Expr::new(
                                    ExprKind::Identifier("y".to_string()),
                                    vec![],
                                    0
                                ))
                            ],
                            0
                        )),
                        Box::new(Expr::new(
                            ExprKind::FnPrintLn,
                            vec![
                                Box::new(Expr::new(
                                    ExprKind::Identifier("x".to_string()),
                                    vec![],
                                    0
                                )),
                                Box::new(Expr::new(
                                    ExprKind::Identifier("y".to_string()),
                                    vec![],
                                    0
                                ))
                            ],
                            0
                        )),
                    ],
                    0
                )],
                ".fn(hello [x y] .println(x y))"
            ),
            ()
        );
    }

    // FAILED CASES

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnDefine, vec![], 0)], ".fn()");
    }

    #[test]
    #[should_panic]
    fn test_1_arg() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnDefine,
                vec![Box::new(Expr::new(
                    ExprKind::Identifier("hello".to_string()),
                    vec![],
                    0,
                ))],
                0,
            )],
            ".fn(hello)",
        );
    }

    #[test]
    #[should_panic]
    fn test_2_args() {
        assert_eq!(
            analyze_semantics(
                &vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(
                            ExprKind::Identifier("hello".to_string()),
                            vec![],
                            0
                        )),
                        Box::new(Expr::new(ExprKind::List, vec![], 0)),
                    ],
                    0
                )],
                ".fn(hello [])"
            ),
            ()
        );
    }

    #[test]
    #[should_panic]
    fn test_fist_arg_type() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                    Box::new(Expr::new(ExprKind::List, vec![], 0)),
                ],
                0,
            )],
            ".fn(nil [])",
        );
    }

    #[test]
    #[should_panic]
    fn test_second_arg_type() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(
                        ExprKind::Identifier("hello".to_string()),
                        vec![],
                        0,
                    )),
                    Box::new(Expr::new(ExprKind::Nil, vec![], 0)),
                ],
                0,
            )],
            ".fn(hello nil)",
        );
    }

    #[test]
    #[should_panic]
    fn test_duplicated_argument() {
        analyze_semantics(
            &vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(
                        ExprKind::Identifier("hello".to_string()),
                        vec![],
                        0,
                    )),
                    Box::new(Expr::new(
                        ExprKind::List,
                        vec![
                            Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                            Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![], 0)),
                        ],
                        0,
                    )),
                ],
                0,
            )],
            ".fn(hello [x x])",
        );
    }
}
