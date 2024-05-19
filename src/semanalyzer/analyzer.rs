use crate::{
    parser::models::ast::{Expr, ExprKind},
    semanalyzer::messages,
};

pub fn analyze(expr: &Expr) -> &Expr {
    match expr.kind {
        ExprKind::FnLetBinding => analyze_fn_let_binding(expr),
        ExprKind::FnGreatr => analyze_non_zero_args_fn(expr),
        ExprKind::FnGreatrEq => analyze_non_zero_args_fn(expr),
        ExprKind::FnLess => analyze_non_zero_args_fn(expr),
        ExprKind::FnLessEq => analyze_non_zero_args_fn(expr),
        ExprKind::FnEq => analyze_non_zero_args_fn(expr),
        ExprKind::FnNotEq => analyze_non_zero_args_fn(expr),
        ExprKind::FnNot => analyze_non_zero_args_fn(expr),
        _ => expr,
    }
}

// ==========================

//      Non Zero Args Fn

// ==========================

fn analyze_non_zero_args_fn(expr: &Expr) -> &Expr {
    if expr.children.len() == 0 {
        panic!("{}", messages::zero_args_fn(&format!("{:?}", expr.kind)));
    }

    for child in expr.children.iter() {
        analyze(child);
    }

    expr
}

// ==========================

//      Fn Let Binding

// ==========================

fn analyze_fn_let_binding(expr: &Expr) -> &Expr {
    let result = analyze_non_zero_args_fn(expr);
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
