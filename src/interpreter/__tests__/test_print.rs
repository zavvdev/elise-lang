#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{models::env::Env, Interpreter, PrintEvalResult},
        lexer::lexemes,
        parser::models::expression::{Expr, ExprKind},
        types,
    };

    // SUCCESS CASES

    #[test]
    fn test_print() {
        let expr = vec![Expr::new(
            ExprKind::FnPrint,
            vec![
                Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                Box::new(Expr::new(ExprKind::Number(1.4), vec![], 0)),
            ],
            0,
        )];

        let interpreter = Interpreter::new(&expr, ".print(1 1.4)");

        assert_eq!(
            interpreter.print_eval_exec(
                &Expr::new(
                    ExprKind::FnPrint,
                    vec![
                        Box::new(Expr::new(ExprKind::Number(1 as types::Number), vec![], 0)),
                        Box::new(Expr::new(ExprKind::Number(1.4), vec![], 0)),
                    ],
                    0,
                ),
                &mut Env::new()
            ),
            PrintEvalResult::Success("1 1.4".to_string())
        );
    }

    #[test]
    fn test_print_empty() {
        let expr = vec![Expr::new(ExprKind::FnPrint, vec![], 0)];
        let interpreter = Interpreter::new(&expr, ".print()");

        assert_eq!(
            interpreter.print_eval_exec(&Expr::new(ExprKind::FnPrint, vec![], 0), &mut Env::new()),
            PrintEvalResult::Empty
        );
    }

    #[test]
    fn test_print_nil() {
        let expr = vec![Expr::new(
            ExprKind::FnPrint,
            vec![Box::new(Expr::new(
                ExprKind::FnPrint,
                vec![Box::new(Expr::new(
                    ExprKind::Number(1 as types::Number),
                    vec![],
                    0,
                ))],
                0,
            ))],
            0,
        )];

        let interpreter = Interpreter::new(&expr, ".print(.print(1))");

        assert_eq!(
            interpreter.print_eval_exec(
                &Expr::new(
                    ExprKind::FnPrint,
                    vec![Box::new(Expr::new(
                        ExprKind::FnPrint,
                        vec![Box::new(Expr::new(
                            ExprKind::Number(1 as types::Number),
                            vec![],
                            0,
                        ))],
                        0,
                    ))],
                    0,
                ),
                &mut Env::new()
            ),
            PrintEvalResult::Success(lexemes::L_NIL.to_string())
        );
    }
}
