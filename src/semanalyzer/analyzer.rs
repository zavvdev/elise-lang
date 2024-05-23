use crate::{
    parser::models::ast::{Expr, ExprKind},
    semanalyzer::messages,
};

pub fn analyze(expr: &Expr) -> &Expr {
    match expr.kind {
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
        _ => expr,
    }
}

fn non_zero_children_expr(expr: &Expr) -> &Expr {
    if expr.children.len() == 0 {
        panic!("{}", messages::zero_args_fn(&format!("{:?}", expr.kind)));
    }

    for child in expr.children.iter() {
        analyze(child);
    }

    expr
}

fn one_children_expr(expr: &Expr) -> &Expr {
    let result = non_zero_children_expr(expr);

    if expr.children.len() > 1 {
        panic!(
            "{}",
            messages::more_than_one_arg_fn(&format!("{:?}", expr.kind))
        );
    }

    for child in result.children.iter().skip(1) {
        analyze(child);
    }

    result
}

// ==========================

//  Immutable value binding

// ==========================

fn let_binding(expr: &Expr) -> &Expr {
    let result = non_zero_children_expr(expr);
    let first_arg = result.children.first().unwrap();

    if first_arg.kind != ExprKind::List {
        panic!("{}", messages::let_binding_first_arg_list());
    }

    if first_arg.children.len() & 1 != 0 {
        panic!("{}", messages::let_binding_first_arg_even_elements());
    }

    for (i, arg) in first_arg.children.iter().enumerate() {
        if i & 1 == 0 {
            match arg.kind {
                ExprKind::Identifier(_) => {}
                _ => panic!("{}", messages::let_binding_arg_identifiers()),
            }
        }
    }

    for child in result.children.iter().skip(1) {
        analyze(child);
    }

    result
}

// ==========================

//            If

// ==========================

fn fn_if(expr: &Expr) -> &Expr {
    let result = non_zero_children_expr(expr);

    if result.children.len() == 1 {
        panic!("{}", messages::too_few_args_fn(&format!("{:?}", expr.kind)));
    }

    if result.children.len() > 3 {
        panic!("{}", messages::too_many_args_fn(&format!("{:?}", expr.kind)));
    }

    for child in result.children.iter().skip(1) {
        analyze(child);
    }

    result
}
