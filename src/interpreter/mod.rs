pub mod evaluator;
pub mod models;
pub mod messages;

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
        interpreter::evaluator::{eval_for_fn_print, PrintEvalResult},
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
}
