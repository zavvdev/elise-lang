pub mod __tests__;
pub mod messages;

use crate::{
    messages::print_error_message,
    parser::models::expression::{Expr, ExprKind},
    to_str,
};

// TODO: Do not pass Expression Name to error message
// Ex: Invalid amount of arguments for function: FnDiv
// Correct: Invalid amount of arguments for function: div

struct Semanalyzer<'a> {
    pub expressions: &'a Vec<Expr>,
    pub source_code: &'a str,
}

impl<'a> Semanalyzer<'a> {
    fn new(expressions: &'a Vec<Expr>, source_code: &'a str) -> Self {
        Self {
            expressions,
            source_code,
        }
    }

    fn analyze(&self) {
        for expr in self.expressions {
            self.analyze_expr(&expr)
        }
    }

    fn analyze_expr(&self, expr: &Expr) {
        match expr.kind {
            ExprKind::FnSub => self.args_non_zero(expr),
            ExprKind::FnDiv => self.args_non_zero(expr),
            ExprKind::FnLetBinding => self.let_binding(expr),
            ExprKind::FnGreatr => self.args_non_zero(expr),
            ExprKind::FnGreatrEq => self.args_non_zero(expr),
            ExprKind::FnLess => self.args_non_zero(expr),
            ExprKind::FnLessEq => self.args_non_zero(expr),
            ExprKind::FnEq => self.args_non_zero(expr),
            ExprKind::FnNotEq => self.args_non_zero(expr),
            ExprKind::FnNot => self.args_n(expr, 1),
            ExprKind::FnBool => self.args_n(expr, 1),
            ExprKind::FnIf => self.fn_if(expr),
            ExprKind::FnIsNil => self.args_n(expr, 1),
            ExprKind::FnDefine => self.fn_define(expr),
            _ => (),
        };

        for child in expr.children.iter() {
            self.analyze_expr(child);
        }
    }

    // ==========================
    //
    // ERROR START
    //
    // ==========================

    fn error(&self, message: &str, char_pos: usize) -> ! {
        print_error_message(message, self.source_code, char_pos);
        panic!("{}", messages::get_panic_message());
    }

    // ==========================
    //
    // ERROR END
    //
    // ==========================

    // ==========================
    //
    // ARGS START
    //
    // ==========================

    fn args_non_zero(&self, expr: &Expr) {
        if expr.children.len() == 0 {
            self.error(
                &messages::args_invalid_amount(to_str!(expr.kind), "> 0", "0"),
                expr.start_at,
            );
        }
    }

    fn args_n(&self, expr: &Expr, n: usize) {
        if expr.children.len() != n {
            self.error(
                &messages::args_invalid_amount(
                    to_str!(expr.kind),
                    to_str!(n),
                    to_str!(expr.children.len()),
                ),
                expr.start_at,
            );
        }
    }

    fn args_n_or_more(&self, expr: &Expr, n: usize) {
        if expr.children.len() < n {
            self.error(
                &messages::args_invalid_amount(
                    to_str!(expr.kind),
                    &format!(">= {}", n),
                    to_str!(expr.children.len()),
                ),
                expr.start_at,
            );
        }
    }

    fn args_even(&self, expr: &Expr) {
        if expr.children.len() & 1 != 0 {
            self.error(
                &messages::args_invalid_amount(
                    to_str!(expr.kind),
                    "even",
                    to_str!(expr.children.len()),
                ),
                expr.start_at,
            );
        }
    }

    // ==========================
    //
    // ARGS END
    //
    // ==========================

    // ==========================
    //
    // TYPE EXPR START
    //
    // ==========================

    fn type_expr(&self, expr: &Expr, expr_type: ExprKind) {
        if expr.kind != expr_type {
            self.error(
                &messages::type_expr_invalid(to_str!(expr_type), to_str!(expr.kind)),
                expr.start_at,
            );
        }
    }

    // ==========================
    //
    // TYPE EXPR END
    //
    // ==========================

    // ==========================
    //
    // LET START
    //
    // - 2 or more arguments
    // - first arguments is a List
    // - first argument has an even amount of items
    // - first argument has Identifiers at non-event positions
    // ==========================

    fn let_binding(&self, expr: &Expr) {
        self.args_n_or_more(&expr, 2);

        let first_arg = expr.children.first().unwrap();

        self.type_expr(&first_arg, ExprKind::List);
        self.args_even(&first_arg);

        for (i, arg) in first_arg.children.iter().enumerate() {
            if i & 1 == 0 {
                match arg.kind {
                    ExprKind::Identifier(_) => (),
                    _ => self.error(&messages::let_invalid_binding_form(), arg.start_at),
                }
            }
        }
    }

    // ==========================
    //
    // LET END
    //
    // ==========================

    // ==========================
    //
    // IF START
    //
    // - has 2 or 3 arguments
    // ==========================

    fn fn_if(&self, expr: &Expr) {
        if expr.children.len() < 2 || expr.children.len() > 3 {
            self.error(
                &messages::args_invalid_amount(
                    to_str!(expr.kind),
                    "2 or 3",
                    to_str!(expr.children.len()),
                ),
                expr.start_at,
            );
        }
    }

    // ==========================
    //
    // IF END
    //
    // ==========================

    // ==========================
    //
    // FN DEFINE START
    //
    // - has 3 or more arguments
    // - first arguments is an Identifier
    // - second arguments is a List of Identifiers with unique names
    // ==========================

    fn fn_define(&self, expr: &Expr) {
        self.args_n_or_more(&expr, 3);

        let first_arg = expr.children.first().unwrap();
        let second_arg = expr.children.get(1).unwrap();

        if let ExprKind::Identifier(fn_name) = &first_arg.kind {
            self.type_expr(second_arg, ExprKind::List);

            if second_arg.children.len() > 0 {
                for arg in second_arg.children.iter() {
                    match arg.kind {
                        ExprKind::Identifier(_) => (),
                        _ => self.error(
                            &messages::fn_def_invalid_args_decl(
                                to_str!(fn_name),
                                to_str!(arg.kind),
                            ),
                            arg.start_at,
                        ),
                    }
                }
            }

            let mut args: Vec<String> = vec![];

            for arg in second_arg.children.iter() {
                if let ExprKind::Identifier(name) = &arg.kind {
                    if args.contains(&name) {
                        self.error(
                            &messages::fn_def_duplicate_arg_decl(to_str!(fn_name)),
                            arg.start_at,
                        );
                    }

                    args.push(name.to_string());
                }
            }
        } else {
            self.error(
                &messages::type_expr_invalid("Identifier", to_str!(first_arg.kind)),
                first_arg.start_at,
            );
        }
    }

    // ==========================
    //
    // FN DEFINE END
    //
    // ==========================
}

pub fn analyze_semantics(expressions: &Vec<Expr>, source_code: &str) {
    let analyzer = Semanalyzer::new(expressions, source_code);
    analyzer.analyze();
}
