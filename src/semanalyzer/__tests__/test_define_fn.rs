#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;

    use crate::{
        parser::models::expression::{Expr, ExprKind},
        semanalyzer::{analyze_semantics, messages},
        to_str,
    };

    #[test]
    fn test_args_amount() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnDefine, vec![])]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnDefine), ">= 2", "0")
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![Box::new(Expr::new(
                        ExprKind::Identifier("hello".to_string()),
                        vec![],
                    ))],
                )]);
            },
            String,
            messages::invalid_args_amount(to_str!(ExprKind::FnDefine), ">= 2", "1")
        );
    }

    #[test]
    fn test_fist_arg_type() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::List, vec![])),
                    ],
                )]);
            },
            String,
            messages::invalid_arg_type(
                to_str!(ExprKind::FnDefine),
                1,
                "Identifier",
                to_str!(ExprKind::Nil)
            )
        );
    }

    #[test]
    fn test_second_arg_type() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnDefine,
                    vec![
                        Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                )]);
            },
            String,
            messages::invalid_arg_type(
                "\"hello\"",
                2,
                to_str!(ExprKind::List),
                to_str!(ExprKind::Nil)
            )
        );
    }

    #[test]
    fn test_second_arg_empty_list() {
        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                    Box::new(Expr::new(ExprKind::List, vec![])),
                ],
            )]),
            vec![&Expr::new(
                ExprKind::FnDefine,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("hello".to_string()), vec![])),
                    Box::new(Expr::new(ExprKind::List, vec![])),
                ],
            )]
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
                ],
            )]),
            vec![&Expr::new(
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
                ],
            )]
        );
    }

    #[test]
    fn test_second_arg_invalid_list() {
        assert_panic!(
            {
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
            },
            String,
            messages::duplicate_fn_arg_decl("\"hello\"")
        );
    }
}
