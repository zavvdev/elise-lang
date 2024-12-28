pub mod __tests__;
pub mod messages;

use crate::{
    parser::models::expression::{Expr, ExprKind},
    to_str,
};

struct Semanalyzer<'expressions> {
    pub expressions: &'expressions Vec<Expr>,
}

impl<'expressions> Semanalyzer<'expressions> {
    fn new(expressions: &'expressions Vec<Expr>) -> Self {
        Self { expressions }
    }

    fn analyze(&self) {
        for expr in self.expressions {
            Self::analyze_expr(&expr)
        }
    }

    fn analyze_expr(expr: &Expr) {
        match expr.kind {
            ExprKind::FnSub => Self::args_non_zero(expr),
            ExprKind::FnDiv => Self::args_non_zero(expr),
            ExprKind::FnLetBinding => Self::let_binding(expr),
            ExprKind::FnGreatr => Self::args_non_zero(expr),
            ExprKind::FnGreatrEq => Self::args_non_zero(expr),
            ExprKind::FnLess => Self::args_non_zero(expr),
            ExprKind::FnLessEq => Self::args_non_zero(expr),
            ExprKind::FnEq => Self::args_non_zero(expr),
            ExprKind::FnNotEq => Self::args_non_zero(expr),
            ExprKind::FnNot => Self::args_n(expr, 1),
            ExprKind::FnBool => Self::args_n(expr, 1),
            ExprKind::FnIf => Self::fn_if(expr),
            ExprKind::FnIsNil => Self::args_n(expr, 1),
            ExprKind::FnDefine => Self::fn_define(expr),
            _ => (),
        };

        for child in expr.children.iter() {
            Self::analyze_expr(child);
        }
    }

    // ==========================
    //
    // ERROR START
    //
    // ==========================

    fn error(message: &str) -> ! {
        println!("{}", message);
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

    fn args_non_zero(expr: &Expr) {
        if expr.children.len() == 0 {
            Self::error(&messages::args_invalid_amount(
                to_str!(expr.kind),
                "> 0",
                "0",
            ))
        }
    }

    fn args_n(expr: &Expr, n: usize) {
        if expr.children.len() != n {
            Self::error(&messages::args_invalid_amount(
                to_str!(expr.kind),
                to_str!(n),
                to_str!(expr.children.len()),
            ))
        }
    }

    fn args_n_or_more(expr: &Expr, n: usize) {
        if expr.children.len() < n {
            Self::error(&messages::args_invalid_amount(
                to_str!(expr.kind),
                &format!(">= {}", n),
                to_str!(expr.children.len()),
            ));
        }
    }

    fn args_even(expr: &Expr) {
        if expr.children.len() & 1 != 0 {
            Self::error(&messages::args_invalid_amount(
                to_str!(expr.kind),
                "even",
                to_str!(expr.children.len()),
            ));
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

    fn type_expr(expr: &Expr, expr_type: ExprKind) {
        if expr.kind != expr_type {
            Self::error(&messages::type_expr_invalid(
                to_str!(expr_type),
                to_str!(expr.kind),
            ));
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

    fn let_binding(expr: &Expr) {
        Self::args_n_or_more(&expr, 2);

        let first_arg = expr.children.first().unwrap();

        Self::type_expr(&first_arg, ExprKind::List);
        Self::args_even(&first_arg);

        for (i, arg) in first_arg.children.iter().enumerate() {
            if i & 1 == 0 {
                match arg.kind {
                    ExprKind::Identifier(_) => (),
                    _ => Self::error(&messages::let_invalid_binding_form()),
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

    fn fn_if(expr: &Expr) {
        if expr.children.len() < 2 || expr.children.len() > 3 {
            Self::error(&messages::args_invalid_amount(
                to_str!(expr.kind),
                "2 or 3",
                to_str!(expr.children.len()),
            ));
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

    fn fn_define(expr: &Expr) {
        Self::args_n_or_more(&expr, 3);

        let first_arg = expr.children.first().unwrap();
        let second_arg = expr.children.get(1).unwrap();

        if let ExprKind::Identifier(fn_name) = &first_arg.kind {
            Self::type_expr(second_arg, ExprKind::List);

            if second_arg.children.len() > 0 {
                for arg in second_arg.children.iter() {
                    match arg.kind {
                        ExprKind::Identifier(_) => (),
                        _ => Self::error(&messages::fn_def_invalid_args_decl(
                            to_str!(fn_name),
                            to_str!(arg.kind),
                        )),
                    }
                }
            }

            let mut args: Vec<String> = vec![];

            for arg in second_arg.children.iter() {
                if let ExprKind::Identifier(name) = &arg.kind {
                    if args.contains(&name) {
                        Self::error(&messages::fn_def_duplicate_arg_decl(to_str!(fn_name)));
                    }

                    args.push(name.to_string());
                }
            }
        } else {
            Self::error(&messages::type_expr_invalid(
                "Identifier",
                to_str!(first_arg.kind),
            ));
        }
    }

    // ==========================
    //
    // FN DEFINE END
    //
    // ==========================
}

pub fn analyze_semantics(expressions: &Vec<Expr>) {
    let analyzer = Semanalyzer::new(expressions);
    analyzer.analyze();
}
