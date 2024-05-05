pub mod evaluator;
pub mod messages;
pub mod models;

use self::evaluator::eval;
use crate::parser::models::ast::Expr;

pub fn interpret(exprs: &Vec<Expr>) {
    for expr in exprs {
        eval(expr);
    }
}

// ======== Tests ========

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            evaluator::{eval, eval_for_fn_print, PrintEvalResult},
            models::EvalResult,
        },
        parser::models::ast::{Expr, ExprKind},
        types,
    };

    // ==========================

    //         Print Fn

    // ==========================

    #[test]
    fn test_eval_for_fn_print() {
        let expr = Expr::new(
            ExprKind::FnPrint,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1.4), vec![])),
            ],
        );

        assert_eq!(
            eval_for_fn_print(&expr),
            PrintEvalResult::Success("1 1.4".to_string())
        );
    }

    #[test]
    fn test_eval_for_fn_print_empty() {
        let expr = Expr::new(ExprKind::FnPrint, vec![]);
        assert_eq!(eval_for_fn_print(&expr), PrintEvalResult::Empty);
    }

    #[test]
    #[should_panic]
    fn test_eval_for_fn_print_invalid() {
        let expr = Expr::new(
            ExprKind::FnPrint,
            vec![Box::new(Expr::new(
                ExprKind::FnPrint,
                vec![Box::new(Expr::new(
                    ExprKind::Number(1 as types::Number),
                    vec![],
                ))],
            ))],
        );
        eval_for_fn_print(&expr);
    }

    // ==========================

    //          Add Fn

    // ==========================

    #[test]
    fn test_eval_fn_add_int() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(3 as types::Number));
    }

    #[test]
    fn test_eval_fn_add_float() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1.1), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(3.5));
    }

    #[test]
    fn test_eval_fn_add() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(3.4));
    }

    #[test]
    fn test_eval_fn_add_empty() {
        let expr = Expr::new(ExprKind::FnAdd, vec![]);
        assert_eq!(eval(&expr), EvalResult::Number(0 as types::Number));
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"add\". Expected numbers."
    )]
    fn test_eval_fn_add_invalid() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr);
    }

    // ==========================

    //          Sub Fn

    // ==========================

    #[test]
    fn test_eval_fn_sub_int() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(1 as types::Number));
    }

    #[test]
    fn test_eval_fn_sub_float() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.5), vec![])),
                Box::new(Expr::new(ExprKind::Number(1.1), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(1.4));
    }

    #[test]
    fn test_eval_fn_sub() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(-1.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(2.4));
    }

    #[test]
    fn test_eval_fn_sub_one() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![Box::new(Expr::new(
                ExprKind::Number(1 as types::Number),
                vec![],
            ))],
        );

        assert_eq!(eval(&expr), EvalResult::Number(-1 as types::Number));
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid number of arguments (0) for function \"sub\"."
    )]
    fn test_eval_fn_sub_empty() {
        eval(&Expr::new(ExprKind::FnSub, vec![]));
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"sub\". Expected numbers."
    )]
    fn test_eval_fn_sub_invalid() {
        let expr = Expr::new(
            ExprKind::FnSub,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr);
    }

    // ==========================

    //          Mul Fn

    // ==========================

    #[test]
    fn test_eval_fn_mul_int() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(3 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(6 as types::Number));
    }

    #[test]
    fn test_eval_fn_mul_float() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.5), vec![])),
                Box::new(Expr::new(ExprKind::Number(1.1), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(2.75));
    }

    #[test]
    fn test_eval_fn_mul() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(-1.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(-2.8));
    }

    #[test]
    fn test_eval_fn_mul_one() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![Box::new(Expr::new(
                ExprKind::Number(3 as types::Number),
                vec![],
            ))],
        );

        assert_eq!(eval(&expr), EvalResult::Number(3 as types::Number));
    }

    #[test]
    fn test_eval_fn_mul_empty() {
        assert_eq!(
            eval(&Expr::new(ExprKind::FnMul, vec![],)),
            EvalResult::Number(1 as types::Number)
        );
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"mul\". Expected numbers."
    )]
    fn test_eval_fn_mul_invalid() {
        let expr = Expr::new(
            ExprKind::FnMul,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr);
    }

    // ==========================

    //          Div Fn

    // ==========================

    #[test]
    fn test_eval_fn_div_int() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(4 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(2 as types::Number));
    }

    #[test]
    fn test_eval_fn_div_float() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(5.5), vec![])),
                Box::new(Expr::new(ExprKind::Number(2.2), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(2.5));
    }

    #[test]
    fn test_eval_fn_div() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(2 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(-1.6), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Number(-1.25));
    }

    #[test]
    fn test_eval_fn_div_one() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![Box::new(Expr::new(
                ExprKind::Number(2 as types::Number),
                vec![],
            ))],
        );

        assert_eq!(eval(&expr), EvalResult::Number(0.5));
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid number of arguments (0) for function \"div\"."
    )]
    fn test_eval_fn_div_empty() {
        eval(&Expr::new(ExprKind::FnDiv, vec![]));
    }

    #[test]
    #[should_panic(
        expected = "Interpretation error. Invalid arguments for function \"div\". Expected numbers."
    )]
    fn test_eval_fn_div_invalid() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::FnPrint, vec![])),
            ],
        );
        eval(&expr);
    }

    #[test]
    #[should_panic(expected = "Interpretation error. Division by zero.")]
    fn test_eval_fn_div_division_by_zero_single_arg() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![Box::new(Expr::new(
                ExprKind::Number(0 as types::Number),
                vec![],
            ))],
        );
        eval(&expr);
    }

    #[test]
    #[should_panic(expected = "Interpretation error. Division by zero.")]
    fn test_eval_fn_div_division_by_zero() {
        let expr = Expr::new(
            ExprKind::FnDiv,
            vec![
                Box::new(Expr::new(ExprKind::Number(2.4), vec![])),
                Box::new(Expr::new(ExprKind::Number(0 as types::Number), vec![])),
            ],
        );
        eval(&expr);
    }
}
