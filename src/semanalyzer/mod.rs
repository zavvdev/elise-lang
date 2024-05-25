pub mod __tests__;
pub mod messages;

use crate::{
    parser::models::expression::{Expr, ExprKind},
    to_str,
};

fn analyze(expr: &Expr) -> &Expr {
    let result = match expr.kind {
        ExprKind::FnSub => non_zero_children_expr(expr),
        ExprKind::FnDiv => non_zero_children_expr(expr),
        ExprKind::FnLetBinding => let_binding(expr),
        ExprKind::FnGreatr => non_zero_children_expr(expr),
        ExprKind::FnGreatrEq => non_zero_children_expr(expr),
        ExprKind::FnLess => non_zero_children_expr(expr),
        ExprKind::FnLessEq => non_zero_children_expr(expr),
        ExprKind::FnEq => non_zero_children_expr(expr),
        ExprKind::FnNotEq => non_zero_children_expr(expr),
        ExprKind::FnNot => one_children_expr(expr),
        ExprKind::FnBool => one_children_expr(expr),
        ExprKind::FnIf => fn_if(expr),
        ExprKind::FnIsNil => one_children_expr(expr),
        ExprKind::FnCustom => fn_custom(expr),
        _ => expr,
    };

    for child in expr.children.iter() {
        analyze(child);
    }

    result
}

fn non_zero_children_expr(expr: &Expr) -> &Expr {
    if expr.children.len() == 0 {
        panic!(
            "{}",
            messages::invalid_args_amount(to_str!(expr.kind), "> 0", "0")
        );
    }

    expr
}

fn one_children_expr(expr: &Expr) -> &Expr {
    if expr.children.len() != 1 {
        panic!(
            "{}",
            messages::invalid_args_amount(to_str!(expr.kind), "1", to_str!(expr.children.len()))
        );
    }

    expr
}

// ==========================

//           Let

// ==========================

fn let_binding(expr: &Expr) -> &Expr {
    if expr.children.len() < 1 {
        panic!(
            "{}",
            messages::invalid_args_amount(to_str!(expr.kind), ">= 1", "0")
        );
    }

    let first_arg = expr.children.first().unwrap();

    if first_arg.kind != ExprKind::List {
        panic!(
            "{}",
            messages::invalid_arg_type(
                to_str!(expr.kind),
                1,
                to_str!(ExprKind::List),
                to_str!(first_arg.kind),
            )
        );
    }

    if first_arg.children.len() & 1 != 0 {
        panic!(
            "{}",
            messages::invalid_args_amount(
                to_str!(expr.kind),
                "even",
                to_str!(first_arg.children.len())
            )
        );
    }

    for (i, arg) in first_arg.children.iter().enumerate() {
        if i & 1 == 0 {
            match arg.kind {
                ExprKind::Identifier(_) => {}
                _ => panic!("{}", messages::invalid_let_binding_form()),
            }
        }
    }

    expr
}

// ==========================

//            If

// ==========================

fn fn_if(expr: &Expr) -> &Expr {
    if expr.children.len() < 2 || expr.children.len() > 3 {
        panic!(
            "{}",
            messages::invalid_args_amount(
                to_str!(expr.kind),
                "2 or 3",
                to_str!(expr.children.len())
            )
        );
    }

    expr
}

// ==========================
//
//       Custom function
//
//  ==========================

fn fn_custom(expr: &Expr) -> &Expr {
    if expr.children.len() < 2 {
        panic!(
            "{}",
            messages::invalid_args_amount(to_str!(expr.kind), ">= 2", to_str!(expr.children.len()))
        );
    }

    let first_arg = expr.children.first().unwrap();
    let second_arg = expr.children.get(1).unwrap();
    let mut fn_name = String::new();

    if let ExprKind::Identifier(name) = &first_arg.kind {
        fn_name = name.to_string();
    } else {
        panic!(
            "{}{}",
            fn_name,
            messages::invalid_arg_type(
                to_str!(expr.kind),
                1,
                "Identifier",
                to_str!(first_arg.kind),
            )
        );
    }

    if second_arg.kind != ExprKind::List {
        panic!(
            "{}",
            messages::invalid_arg_type(
                to_str!(fn_name),
                2,
                to_str!(ExprKind::List),
                to_str!(second_arg.kind),
            )
        );
    }

    if second_arg.children.len() > 0 {
        for arg in second_arg.children.iter() {
            match arg.kind {
                ExprKind::Identifier(_) => {}
                _ => panic!(
                    "{}",
                    messages::invalid_fn_arg_decl(to_str!(fn_name), to_str!(arg.kind))
                ),
            }
        }
    }

    let mut args: Vec<String> = vec![];

    for arg in second_arg.children.iter() {
        match arg.kind {
            ExprKind::Identifier(ref name) => {
                if args.contains(name) {
                    panic!("{}", messages::duplicate_fn_arg_decl(to_str!(fn_name)));
                }

                args.push(name.to_string());
            }
            _ => {}
        }
    }

    expr
}

// ==============================================

pub fn analyze_semantics(expressions: &Vec<Expr>) -> Vec<&Expr> {
    let mut result: Vec<&Expr> = Vec::new();

    for expr in expressions {
        let next_expr: &Expr = analyze(expr);

        result.push(next_expr);
    }

    result
}
