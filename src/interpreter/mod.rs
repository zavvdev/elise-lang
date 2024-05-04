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
    };

    // ==========================

    //         Print Fn

    // ==========================

    #[test]
    fn test_eval_for_fn_print() {
        let expr = Expr::new(
            ExprKind::FnPrint,
            vec![
                Box::new(Expr::new(ExprKind::Int(1), vec![])),
                Box::new(Expr::new(ExprKind::Float(1.4), vec![])),
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
                vec![Box::new(Expr::new(ExprKind::Int(1), vec![]))],
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
                Box::new(Expr::new(ExprKind::Int(1), vec![])),
                Box::new(Expr::new(ExprKind::Int(2), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Int(3));
    }

    #[test]
    fn test_eval_fn_add_float() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Float(1.1), vec![])),
                Box::new(Expr::new(ExprKind::Float(2.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Float(3.5));
    }

    #[test]
    fn test_eval_fn_add() {
        let expr = Expr::new(
            ExprKind::FnAdd,
            vec![
                Box::new(Expr::new(ExprKind::Int(1), vec![])),
                Box::new(Expr::new(ExprKind::Float(2.4), vec![])),
            ],
        );

        assert_eq!(eval(&expr), EvalResult::Float(3.4));
    }
}
