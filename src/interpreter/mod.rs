pub mod __tests__;
pub mod macros;
pub mod messages;
pub mod models;
pub mod semanalyzer;

use crate::{
    binary_op,
    lexer::lexemes::{self, fn_lexeme_to_string},
    parser::models::expression::{Expr, ExprKind},
    to_str, types,
};

use self::models::env::{Env, EnvRecord, EvalResult, FnDeclaration};

fn eval(expr: &Expr, env: &mut Env) -> EvalResult {
    match &expr.kind {
        ExprKind::Nil => EvalResult::Nil,
        ExprKind::Number(x) => EvalResult::Number(*x),
        ExprKind::Boolean(x) => EvalResult::Boolean(*x),
        ExprKind::String(x) => EvalResult::String(x.to_string()),

        ExprKind::Identifier(x) => eval_identifier(x.to_string(), env),
        ExprKind::FnLetBinding => eval_fn_let_binding(expr, env),

        ExprKind::FnPrint => eval_fn_print(expr, false, env),
        ExprKind::FnPrintLn => eval_fn_print(expr, true, env),

        ExprKind::FnAdd => eval_fn_add(expr, env),
        ExprKind::FnSub => eval_fn_sub(expr, env),
        ExprKind::FnMul => eval_fn_mul(expr, env),
        ExprKind::FnDiv => eval_fn_div(expr, env),

        ExprKind::FnGreatr => eval_number_comparison(expr, env, |x, y| binary_op!(x, >, y)),
        ExprKind::FnGreatrEq => eval_number_comparison(expr, env, |x, y| binary_op!(x, >=, y)),
        ExprKind::FnLess => eval_number_comparison(expr, env, |x, y| binary_op!(x, <, y)),
        ExprKind::FnLessEq => eval_number_comparison(expr, env, |x, y| binary_op!(x, <=, y)),
        ExprKind::FnNot => eval_fn_not(expr, env),

        ExprKind::FnEq => eval_fn_eq(expr, env),
        ExprKind::FnNotEq => eval_fn_not_eq(expr, env),

        ExprKind::FnBool => eval_fn_bool(expr, env),

        ExprKind::FnOr => eval_fn_or(expr, env),
        ExprKind::FnAnd => eval_fn_and(expr, env),

        ExprKind::FnIf => eval_fn_if(expr, env),

        ExprKind::FnIsNil => eval_fn_is_nil(expr, env),

        ExprKind::FnDefine => eval_fn_definition(expr, env),
        ExprKind::FnCustom(x) => eval_fn_custom(x, expr, env),

        _ => panic!("{}", messages::unknown_expression(to_str!(expr.kind))),
    }
}

fn ensure_number(res: &EvalResult) -> types::Number {
    match &res {
        EvalResult::Number(x) => *x,
        _ => panic!("{}", messages::expected_number(to_str!(res))),
    }
}

// ==========================

//          Print

// ==========================

#[derive(Debug, PartialEq)]
pub enum PrintEvalResult {
    Empty,
    Success(String),
}

pub fn eval_for_fn_print(expr: &Expr, env: &mut Env) -> PrintEvalResult {
    if expr.children.len() == 0 {
        return PrintEvalResult::Empty;
    }

    let mut result: Vec<String> = Vec::new();

    for child in expr.children.iter() {
        let child_res = eval(child, env);

        match child_res {
            EvalResult::Number(x) => {
                result.push(x.to_string());
            }
            EvalResult::Nil => result.push(lexemes::L_NIL.to_string()),
            EvalResult::Boolean(x) => result.push(x.to_string()),
            EvalResult::String(x) => result.push(x),
            EvalResult::FnDeclaration(x) => {
                result.push(format!(
                    "{} {} [{}]",
                    fn_lexeme_to_string(lexemes::L_FN_DEFINE),
                    x.name,
                    x.args.join(", ")
                ));
            }
        }
    }

    return PrintEvalResult::Success(result.join(" "));
}

fn eval_fn_print(expr: &Expr, new_line: bool, env: &mut Env) -> EvalResult {
    match eval_for_fn_print(expr, env) {
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

//         Addition

// ==========================

fn eval_fn_add(expr: &Expr, env: &mut Env) -> EvalResult {
    if expr.children.len() == 0 {
        return EvalResult::Number(0 as types::Number);
    }

    let mut result: types::Number = 0.0;

    for child in expr.children.iter() {
        let child_res = eval(child, env);

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

//        Subtraction

// ==========================

fn eval_fn_sub(expr: &Expr, env: &mut Env) -> EvalResult {
    let mut result: types::Number = 0.0;

    for (i, child) in expr.children.iter().enumerate() {
        let child_res = eval(child, env);

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

// ==========================

//      Multiplication

// ==========================

fn eval_fn_mul(expr: &Expr, env: &mut Env) -> EvalResult {
    if expr.children.len() == 0 {
        return EvalResult::Number(1 as types::Number);
    }

    let mut result = 1 as types::Number;

    for child in expr.children.iter() {
        let child_res = eval(child, env);

        match child_res {
            EvalResult::Number(x) => {
                result *= x;
            }
            _ => panic!("{}", messages::fn_expected_num_arg(lexemes::L_FN_MUL.1)),
        }
    }

    EvalResult::Number(result)
}

// ==========================

//         Division

// ==========================

fn eval_fn_div(expr: &Expr, env: &mut Env) -> EvalResult {
    let mut result = 1 as types::Number;

    for (i, child) in expr.children.iter().enumerate() {
        let child_res = eval(child, env);

        match child_res {
            EvalResult::Number(x) => {
                if (i != 0 || expr.children.len() == 1) && x == 0.0 {
                    panic!("{}", messages::division_by_zero());
                }

                if expr.children.len() == 1 {
                    result = 1.0 / x;
                } else if i == 0 {
                    result = x;
                } else {
                    result /= x;
                }
            }
            _ => panic!("{}", messages::fn_expected_num_arg(lexemes::L_FN_DIV.1)),
        }
    }

    EvalResult::Number(result)
}

// ==========================

//        Identifier

// ==========================

fn eval_identifier(name: String, env: &Env) -> EvalResult {
    match env.get(&name) {
        Some(x) => {
            // TODO: Get rid of clone
            return x.value.clone();
        }
        None => panic!("{}", messages::undefined_identifier(&name)),
    }
}

// ==========================

//       Value Binding

// ==========================

/**
*   
* allow_rebind - allow identifiers to be redefined in the same environment
* allow_deep_rebind - allow identifiers to be redefined in the same and in the parent environments
*
*/
fn bind(
    bindings: Vec<(String, EvalResult)>,
    env: &mut Env,
    mutable: bool,
    allow_rebind: bool,
    allow_deep_rebind: bool,
) {
    for (identifier, value) in bindings {
        if !allow_rebind && env.has(&identifier) {
            panic!("{}", messages::identifier_exists_same_env(&identifier));
        }

        if !allow_deep_rebind && env.has_deep(&identifier) {
            panic!("{}", messages::identifier_exists_parent_env(&identifier));
        }

        env.set(identifier, EnvRecord { value, mutable });
    }
}

fn unwrap_identifier(expr_kind: &ExprKind) -> String {
    match expr_kind {
        ExprKind::Identifier(x) => x.to_string(),
        x => panic!("{}", messages::non_identifier(to_str!(x))),
    }
}

// ==========================

//  Immutable value binding

// ==========================

fn collect_bindings(expr: &Box<Expr>, env: &mut Env) -> Vec<(String, EvalResult)> {
    let mut bindings = Vec::new();

    for (i, child) in expr.children.iter().enumerate() {
        if i & 1 == 1 {
            continue;
        }

        let identifier = unwrap_identifier(&child.kind);
        let value = eval(
            expr.children
                .get(i + 1)
                .expect(&messages::bind_value_not_found()),
            env,
        );

        bindings.push((identifier, value));
    }

    bindings
}

fn eval_fn_let_binding(expr: &Expr, env: &mut Env) -> EvalResult {
    if expr.children.len() == 1 {
        return EvalResult::Nil;
    }

    let bindings = collect_bindings(expr.children.first().unwrap(), env);
    let mut child_env = Env::new();

    child_env.attach_parent(env);
    bind(bindings, &mut child_env, false, false, false);

    let mut result = EvalResult::Nil;

    for child_expr in expr.children.iter().skip(1) {
        result = eval(child_expr, &mut child_env);
    }

    result
}

// ==========================

//     Number Comparison

// ==========================

fn eval_number_comparison<P>(expr: &Expr, env: &mut Env, predicate: P) -> EvalResult
where
    P: Fn(types::Number, types::Number) -> bool,
{
    if expr.children.len() == 1 {
        return EvalResult::Boolean(true);
    }

    let mut result = true;
    let mut current: types::Number = ensure_number(&eval(expr.children.first().unwrap(), env));

    for child in expr.children.iter().skip(1) {
        let child_res = ensure_number(&eval(child, env));

        if predicate(current, child_res) {
            current = child_res;
            continue;
        }

        result = false;
        break;
    }

    EvalResult::Boolean(result)
}

// ==========================

//            Not

// ==========================

fn eval_fn_not(expr: &Expr, env: &mut Env) -> EvalResult {
    let child_res = eval(expr.children.first().unwrap(), env);

    match child_res {
        EvalResult::Boolean(x) => EvalResult::Boolean(!x),
        EvalResult::Nil => EvalResult::Boolean(true),
        _ => EvalResult::Boolean(false),
    }
}

// ==========================

//           Equal

// ==========================

fn eval_fn_eq(expr: &Expr, env: &mut Env) -> EvalResult {
    let mut result = true;

    for (i, child) in expr.children.iter().enumerate() {
        if i < expr.children.len() - 1 {
            let child_res = eval(child, env);
            let next_child_res = eval(expr.children.get(i + 1).unwrap(), env);

            if child_res != next_child_res {
                result = false;
                break;
            }
        }
    }

    EvalResult::Boolean(result)
}

// ==========================

//         Not Equal

// ==========================

fn eval_fn_not_eq(expr: &Expr, env: &mut Env) -> EvalResult {
    let res = eval_fn_eq(expr, env);

    match res {
        EvalResult::Boolean(x) => EvalResult::Boolean(!x),
        _ => panic!("{}", messages::expected_boolean(to_str!(res))),
    }
}

// ==========================

//      Boolean coercion

// ==========================

fn coerce_to_boolean(res: EvalResult) -> EvalResult {
    match res {
        EvalResult::Boolean(x) => {
            if x {
                EvalResult::Boolean(true)
            } else {
                EvalResult::Boolean(false)
            }
        }
        EvalResult::Nil => EvalResult::Boolean(false),
        _ => EvalResult::Boolean(true),
    }
}

fn eval_fn_bool(expr: &Expr, env: &mut Env) -> EvalResult {
    let child_res = eval(expr.children.first().unwrap(), env);
    coerce_to_boolean(child_res)
}

// ==========================

//            Or

// ==========================

fn eval_fn_or(expr: &Expr, env: &mut Env) -> EvalResult {
    if expr.children.len() == 0 {
        return EvalResult::Nil;
    }

    let mut result_index = 0;

    for (i, child) in expr.children.iter().enumerate() {
        let child_res = coerce_to_boolean(eval(child, env));

        match child_res {
            EvalResult::Boolean(x) => {
                result_index = i;

                if x {
                    break;
                }

                continue;
            }
            _ => continue,
        }
    }

    eval(expr.children.get(result_index).unwrap(), env)
}

// ==========================

//            And

// ==========================

fn eval_fn_and(expr: &Expr, env: &mut Env) -> EvalResult {
    if expr.children.len() == 0 {
        return EvalResult::Boolean(true);
    }

    let mut result_index = 0;

    for (i, child) in expr.children.iter().enumerate() {
        let child_res = coerce_to_boolean(eval(child, env));

        match child_res {
            EvalResult::Boolean(x) => {
                result_index = i;

                if !x {
                    break;
                }

                continue;
            }
            _ => continue,
        }
    }

    eval(expr.children.get(result_index).unwrap(), env)
}

// ==========================

//            If

// ==========================

fn eval_fn_if(expr: &Expr, env: &mut Env) -> EvalResult {
    let condition = expr.children.first().unwrap();
    let then_branch = expr.children.get(1).unwrap();
    let else_branch = expr.children.get(2);

    let condition_res = coerce_to_boolean(eval(condition, env));

    match condition_res {
        EvalResult::Boolean(x) => {
            if x {
                return eval(then_branch, env);
            }

            if let Some(else_branch) = else_branch {
                return eval(else_branch, env);
            }

            return EvalResult::Nil;
        }
        _ => panic!("{}", messages::expected_boolean(to_str!(condition_res))),
    }
}

// ==========================

//          Is Nil

// ==========================

fn eval_fn_is_nil(expr: &Expr, env: &mut Env) -> EvalResult {
    let child_res = eval(expr.children.first().unwrap(), env);

    match child_res {
        EvalResult::Nil => EvalResult::Boolean(true),
        _ => EvalResult::Boolean(false),
    }
}

// ==========================
//
//       Custom function
//
//  ==========================

fn eval_fn_definition(expr: &Expr, env: &mut Env) -> EvalResult {
    let name = unwrap_identifier(&expr.children.first().unwrap().kind);

    let args: Vec<String> = expr
        .children
        .get(1)
        .unwrap()
        .children
        .iter()
        .map(|x| unwrap_identifier(&x.kind))
        .collect();

    let body: Vec<Expr> = expr.children.iter().skip(2).map(|x| *x.clone()).collect();

    let declaration = EvalResult::FnDeclaration(FnDeclaration {
        name: name.clone(),
        args,
        body,
    });

    env.set(
        name,
        EnvRecord {
            value: declaration.clone(),
            mutable: false,
        },
    );

    declaration
}

fn eval_fn_custom(name: &str, expr: &Expr, env: &mut Env) -> EvalResult {
    // TODO: Get rid of clone
    if let Some(env_record) = env.clone().get(name) {
        match &env_record.value {
            EvalResult::FnDeclaration(fn_decl) => {
                let expr =
                    semanalyzer::analyze_fn_call_semantics(expr, &fn_decl.name, fn_decl.args.len());

                let argument_bindings = fn_decl
                    .args
                    .iter()
                    .zip(expr.children.iter())
                    .map(|(x, y)| (x.clone(), eval(y, env)))
                    .collect();

                let mut child_env = Env::new();
                child_env.attach_parent(&env);
                bind(argument_bindings, &mut child_env, false, false, true);

                let mut result = EvalResult::Nil;

                for expr in &fn_decl.body {
                    result = eval(expr, &mut child_env);
                }

                return result;
            }
            _ => panic!("{}", messages::not_callable(name)),
        }
    } else {
        panic!("{}", messages::undefined_identifier(name));
    }
}

// ==============================================

pub fn interpret(exprs: Vec<Expr>, env: &mut Env) {
    for expr in exprs {
        eval(&expr, env);
    }
}
