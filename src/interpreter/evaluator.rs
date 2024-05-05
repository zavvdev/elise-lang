use crate::{
    interpreter::messages,
    lexer::lexemes,
    parser::models::ast::{Expr, ExprKind},
    types,
};

use super::models::EvalResult;

pub fn eval(expr: &Expr) -> EvalResult {
    match expr.kind {
        ExprKind::Number(x) => EvalResult::Number(x),
        ExprKind::FnPrint => eval_fn_print(expr, false),
        ExprKind::FnPrintLn => eval_fn_print(expr, true),
        ExprKind::FnAdd => eval_fn_add(expr),
        ExprKind::FnSub => eval_fn_sub(expr),
        _ => panic!(
            "{}",
            messages::unknown_expression(&format!("{:?}", expr.kind))
        ),
    }
}

// ==========================

//         Print Fn

// ==========================

#[derive(Debug, PartialEq)]
pub enum PrintEvalResult {
    Empty,
    Success(String),
}

pub fn eval_for_fn_print(expr: &Expr) -> PrintEvalResult {
    if expr.children.len() == 0 {
        return PrintEvalResult::Empty;
    }

    let mut result: Vec<String> = Vec::new();

    for child in expr.children.iter() {
        let child_res = eval(child);

        match child_res {
            EvalResult::Number(x) => {
                result.push(x.to_string());
            }
            x => panic!("{}", messages::invalid_expression(&format!("{:?}", x))),
        }
    }

    return PrintEvalResult::Success(result.join(" "));
}

fn eval_fn_print(expr: &Expr, new_line: bool) -> EvalResult {
    match eval_for_fn_print(expr) {
        PrintEvalResult::Empty => EvalResult::Nil,
        PrintEvalResult::Success(result) => {
            if new_line {
                println!("{}", result);
            } else {
                print!("{}", result);
            }

            return EvalResult::Nil;
        }
    }
}

// ==========================

//          Add Fn

// ==========================

fn eval_fn_add(expr: &Expr) -> EvalResult {
    if expr.children.len() == 0 {
        return EvalResult::Number(0 as types::Number);
    }

    let mut result: types::Number = 0.0;

    for child in expr.children.iter() {
        let child_res = eval(child);

        match child_res {
            EvalResult::Number(x) => {
                result += x;
            }
            _ => panic!("{}", messages::fn_expected_num_arg(lexemes::L_FN_ADD.1)),
        }
    }

    EvalResult::Number(result)
}

// ==========================

//          Sub Fn

// ==========================

fn eval_fn_sub(expr: &Expr) -> EvalResult {
    if expr.children.len() == 0 {
        panic!("{}", messages::fn_no_args(lexemes::L_FN_SUB.1));
    }

    let mut result: types::Number = 0.0;

    for (i, child) in expr.children.iter().enumerate() {
        let child_res = eval(child);

        match child_res {
            EvalResult::Number(x) => {
                if expr.children.len() == 1 {
                    result = -x;
                } else if i == 0 {
                    result = x;
                } else {
                    result -= x;
                }
            }
            _ => panic!("{}", messages::fn_expected_num_arg(lexemes::L_FN_SUB.1)),
        }
    }

    EvalResult::Number(result)
}
