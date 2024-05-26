#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{eval_for_fn_print, models::env::Env, PrintEvalResult},
        lexer::lexemes,
        parser::models::expression::{Expr, ExprKind},
        types,
    };

    #[test]
    fn test_print() {
        let expr = Expr::new(
            ExprKind::FnPrint,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![])),
                Box::new(Expr::new(ExprKind::Number(1.4), vec![])),
            ],
        );

        assert_eq!(
            eval_for_fn_print(&expr, &mut Env::new()),
            PrintEvalResult::Success("1 1.4".to_string())
        );
    }

    #[test]
    fn test_print_empty() {
        let expr = Expr::new(ExprKind::FnPrint, vec![]);
        assert_eq!(
            eval_for_fn_print(&expr, &mut Env::new()),
            PrintEvalResult::Empty
        );
    }

    #[test]
    fn test_print_nil() {
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
        assert_eq!(
            eval_for_fn_print(&expr, &mut Env::new()),
            PrintEvalResult::Success(lexemes::L_NIL.to_string())
        );
    }
}
