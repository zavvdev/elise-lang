pub mod analyzer;
pub mod messages;

use self::analyzer::analyze;
use crate::parser::models::ast::Expr;

pub fn analyze_semantics(ast: &Vec<Expr>) -> Vec<&Expr> {
    let mut result: Vec<&Expr> = Vec::new();

    for expr in ast {
        let next_expr: &Expr = analyze(expr);

        result.push(next_expr);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::ast::ExprKind;
    use assert_panic::assert_panic;

    // ==========================
    //
    //        Subtraction
    //
    // ==========================

    #[test]
    fn test_subtraction() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnSub, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnSub))
        );
    }

    // ==========================
    //
    //         Division
    //
    // ==========================

    #[test]
    fn test_division() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnDiv, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnDiv))
        );
    }

    // ==========================
    //
    //  Immutable value binding
    //
    // ==========================

    #[test]
    fn test_let_binding() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLetBinding, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnLetBinding))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                )]);
            },
            String,
            messages::let_binding_first_arg_list()
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnLetBinding,
                    vec![Box::new(Expr::new(
                        ExprKind::List,
                        vec![Box::new(Expr::new(ExprKind::Number(3.4), vec![]))],
                    ))],
                )]);
            },
            String,
            messages::let_binding_first_arg_even_elements()
        );

        assert_panic!(
            {
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
            },
            String,
            messages::let_binding_arg_identifiers()
        );
    }

    // ==========================
    //
    //         Negation
    //
    // ==========================

    #[test]
    fn test_not() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnNot, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnNot))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnNot,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                )]);
            },
            String,
            messages::more_than_one_arg_fn(&format!("{:?}", ExprKind::FnNot))
        );
    }

    // ==========================
    //
    //       Greater than
    //
    // ==========================

    #[test]
    fn test_greatr_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnGreatr, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnGreatr))
        );
    }

    // ==========================
    //
    //         greatr-eq
    //
    // ==========================

    #[test]
    fn test_greater_eq_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnGreatrEq, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnGreatrEq))
        );
    }

    // ==========================
    //
    //           less
    //
    // ==========================

    #[test]
    fn test_less_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLess, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnLess))
        );
    }

    // ==========================
    //
    //          less-eq
    //
    // ==========================

    #[test]
    fn test_less_eq_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnLessEq, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnLessEq))
        );
    }

    // ==========================
    //
    //            eq
    //
    // ==========================

    #[test]
    fn test_eq_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnEq, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnEq))
        );
    }

    // ==========================
    //
    //          not-eq
    //
    // ==========================

    #[test]
    fn test_not_eq_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnNotEq, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnNotEq))
        );
    }

    // ==========================
    //
    //      boolean coercion
    //
    // ==========================

    #[test]
    fn test_bool_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnBool, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnBool))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnBool,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                    ],
                )]);
            },
            String,
            messages::more_than_one_arg_fn(&format!("{:?}", ExprKind::FnBool))
        );

        assert_eq!(
            analyze_semantics(&vec![Expr::new(
                ExprKind::FnBool,
                vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))]
            )]),
            vec![&Expr::new(
                ExprKind::FnBool,
                vec![Box::new(Expr::new(ExprKind::Boolean(true), vec![]))]
            )]
        )
    }

    // ==========================
    //
    //            if
    //
    // ==========================

    #[test]
    fn test_if_semantics() {
        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(ExprKind::FnIf, vec![])]);
            },
            String,
            messages::zero_args_fn(&format!("{:?}", ExprKind::FnIf))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnIf,
                    vec![Box::new(Expr::new(ExprKind::Nil, vec![]))],
                )]);
            },
            String,
            messages::too_few_args_fn(&format!("{:?}", ExprKind::FnIf))
        );

        assert_panic!(
            {
                analyze_semantics(&vec![Expr::new(
                    ExprKind::FnIf,
                    vec![
                        Box::new(Expr::new(ExprKind::Nil, vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.3), vec![])),
                        Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
                    ],
                )]);
            },
            String,
            messages::too_many_args_fn(&format!("{:?}", ExprKind::FnIf))
        );
    }
}
