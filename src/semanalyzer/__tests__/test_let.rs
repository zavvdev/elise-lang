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
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnLetBinding,
                vec![
                    Box::new(Expr::new(
                        ExprKind::List,
                        vec![
                            Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                            Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                        ],
                    )),
                    Box::new(Expr::new(
                        ExprKind::FnPrint,
                        vec![Box::new(Expr::new(
                            ExprKind::Identifier("x".to_string()),
                            vec![]
                        ))]
                    ))
                ],
            )]),
            ()
        );
    }

    // FAILURE CASES

    #[test]
    #[should_panic]
    fn test_invalid_type_first_arg() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnLetBinding,
            vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_2_args() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnLetBinding,
            vec![Box::new(Expr::new(
                ExprKind::List,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                    Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                ],
            ))],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_bindings_not_even() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnLetBinding,
            vec![Box::new(Expr::new(
                ExprKind::List,
                vec![Box::new(Expr::new(ExprKind::Number(3.4), vec![]))],
            ))],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_binding_form() {
        analyze_semantics(&vec![Expr::new(
            ExprKind::FnLetBinding,
            vec![Box::new(Expr::new(
                ExprKind::List,
                vec![
                    Box::new(Expr::new(ExprKind::Identifier("x".to_string()), vec![])),
                    Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                    Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                    Box::new(Expr::new(ExprKind::Number(3.4), vec![])),
                ],
            ))],
        )]);
    }

    #[test]
    #[should_panic]
    fn test_0_args() {
        analyze_semantics(&vec![Expr::new(ExprKind::FnLetBinding, vec![])]);
    }
}
