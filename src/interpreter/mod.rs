pub mod __tests__;
pub mod enums;
pub mod macros;
pub mod messages;
pub mod models;
pub mod semanalyzer;

use enums::PrintEvalResult;

use crate::{
    binary_op,
    lexer::lexemes::{self, fn_lexeme_to_string},
    messages::print_error_message,
    parser::models::expression::{Expr, ExprKind},
    to_str, types,
};

use self::models::binding::Binding;
use self::models::env::{Env, EnvRecord, EvalResult, FnDeclaration};

struct Interpreter<'a> {
    expressions: &'a Vec<Expr>,
    source_code: &'a str,
}

impl<'a> Interpreter<'a> {
    fn new(expressions: &'a Vec<Expr>, source_code: &'a str) -> Self {
        Interpreter {
            expressions,
            source_code,
        }
    }

    fn interpret(&self, env: &mut Env) -> Vec<EvalResult> {
        let mut res = Vec::new();

        for expr in self.expressions.iter() {
            res.push(self.eval(&expr, env));
        }

        res
    }

    fn eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        match &expr.kind {
            ExprKind::Nil => EvalResult::Nil,
            ExprKind::Number(x) => EvalResult::Number(*x),
            ExprKind::String(x) => EvalResult::String(x.to_string()),
            ExprKind::Boolean(x) => EvalResult::Boolean(*x),

            ExprKind::Identifier(x) => self.identifier_eval(x.to_string(), env, expr.start_at),
            ExprKind::FnLetBinding => self.let_binding_eval(expr, env),

            ExprKind::FnPrint => self.print_eval(expr, env, false),
            ExprKind::FnPrintLn => self.print_eval(expr, env, true),

            ExprKind::FnAdd => self.add_eval(expr, env),
            ExprKind::FnSub => self.sub_eval(expr, env),
            ExprKind::FnMul => self.mul_eval(expr, env),
            ExprKind::FnDiv => self.div_eval(expr, env),

            ExprKind::FnGreatr => self.number_comp_eval(expr, env, |x, y| binary_op!(x, >, y)),
            ExprKind::FnGreatrEq => self.number_comp_eval(expr, env, |x, y| binary_op!(x, >=, y)),
            ExprKind::FnLess => self.number_comp_eval(expr, env, |x, y| binary_op!(x, <, y)),
            ExprKind::FnLessEq => self.number_comp_eval(expr, env, |x, y| binary_op!(x, <=, y)),

            ExprKind::FnNot => self.not_eval(expr, env),
            ExprKind::FnEq => self.eq_eval(expr, env),
            ExprKind::FnNotEq => self.not_eq_eval(expr, env),

            ExprKind::FnBool => self.bool_eval(expr, env),

            ExprKind::FnOr => self.or_eval(expr, env),
            ExprKind::FnAnd => self.and_eval(expr, env),

            ExprKind::FnIf => self.if_eval(expr, env),

            ExprKind::FnIsNil => self.is_nil_eval(expr, env),

            ExprKind::FnDefine => self.fn_def_eval(expr, env),
            ExprKind::FnCustom(x) => self.fn_custom_eval(x, expr, env),

            _ => self.error(
                &messages::unknown_expression(to_str!(expr.kind)),
                expr.start_at,
            ),
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
    // NUMBER START
    //
    // ==========================

    fn number_ensure(&self, res: &EvalResult, start_at: usize) -> types::Number {
        match &res {
            EvalResult::Number(x) => *x,
            _ => self.error(&messages::expected_number(to_str!(res)), start_at),
        }
    }

    // ==========================
    //
    // NUMBER END
    //
    // ==========================

    // ==========================
    //
    // IDENTIFIER START
    //
    // ==========================

    /**
     *   
     * allow_rebind - allow identifiers to be redefined in the same environment
     * allow_deep_rebind - allow identifiers to be redefined in the same and in the parent environments
     *
     */
    fn identifiers_bind(
        &self,
        bindings: Vec<Binding>,
        env: &mut Env,
        mutable: bool,
        allow_rebind: bool,
        allow_deep_rebind: bool,
    ) {
        for binding in bindings {
            if !allow_rebind && env.has(&binding.name) {
                self.error(
                    &messages::identifier_exists_same_env(&binding.name),
                    binding.start_at,
                );
            }

            if !allow_deep_rebind && env.has_deep(&binding.name) {
                self.error(
                    &messages::identifier_exists_parent_env(&binding.name),
                    binding.start_at,
                );
            }

            env.set(
                binding.name,
                EnvRecord {
                    value: binding.value,
                    mutable,
                },
            );
        }
    }

    fn identifier_unwrap(&self, expr: &Expr) -> String {
        match &expr.kind {
            ExprKind::Identifier(x) => x.to_string(),
            x => self.error(&messages::non_identifier(to_str!(x)), expr.start_at),
        }
    }

    fn identifier_eval(&self, name: String, env: &Env, start_at: usize) -> EvalResult {
        match env.get(&name) {
            Some(x) => {
                // TODO: Get rid of clone
                return x.value.clone();
            }
            None => self.error(&messages::undefined_identifier(&name), start_at),
        }
    }

    // ==========================
    //
    // IDENTIFIER END
    //
    // ==========================

    // ==========================
    //
    // LET START
    //
    // ==========================

    fn let_collect_bindings(&self, expr: &Box<Expr>, env: &mut Env) -> Vec<Binding> {
        let mut bindings = Vec::new();

        for (i, child) in expr.children.iter().enumerate() {
            if i & 1 == 1 {
                continue;
            }

            let identifier = self.identifier_unwrap(&child);
            let value = self.eval(
                expr.children
                    .get(i + 1)
                    .expect(&messages::bind_value_not_found()),
                env,
            );

            bindings.push(Binding::new(identifier, value, child.start_at));
        }

        bindings
    }

    fn let_binding_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let bindings = self.let_collect_bindings(expr.children.first().unwrap(), env);
        let mut child_env = Env::new();

        child_env.attach_parent(env);
        self.identifiers_bind(bindings, &mut child_env, false, false, false);

        let mut result = EvalResult::Nil;

        for child_expr in expr.children.iter().skip(1) {
            result = self.eval(child_expr, &mut child_env);
        }

        result
    }

    // ==========================
    //
    // LET END
    //
    // ==========================

    // ==========================
    //
    // PRINT START
    //
    // ==========================

    pub fn print_eval_exec(&self, expr: &Expr, env: &mut Env) -> PrintEvalResult {
        if expr.children.len() == 0 {
            return PrintEvalResult::Empty;
        }

        let mut result: Vec<String> = Vec::new();

        for child in expr.children.iter() {
            let child_res = self.eval(child, env);

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

    fn print_eval(&self, expr: &Expr, env: &mut Env, new_line: bool) -> EvalResult {
        match self.print_eval_exec(expr, env) {
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
    //
    // PRINT END
    //
    // ==========================

    // ==========================
    //
    // ADD START
    //
    // ==========================

    fn add_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        if expr.children.len() == 0 {
            return EvalResult::Number(0 as types::Number);
        }

        let mut result: types::Number = 0.0;

        for child in expr.children.iter() {
            let child_res = self.eval(child, env);

            match child_res {
                EvalResult::Number(x) => {
                    result += x;
                }
                _ => self.error(
                    &messages::fn_expected_num_arg(lexemes::L_FN_ADD.1),
                    expr.start_at,
                ),
            }
        }

        EvalResult::Number(result)
    }

    // ==========================
    //
    // ADD END
    //
    // ==========================

    // ==========================
    //
    // SUB START
    //
    // ==========================

    fn sub_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let mut result: types::Number = 0.0;

        for (i, child) in expr.children.iter().enumerate() {
            let child_res = self.eval(child, env);

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
                _ => self.error(
                    &messages::fn_expected_num_arg(lexemes::L_FN_SUB.1),
                    expr.start_at,
                ),
            }
        }

        EvalResult::Number(result)
    }

    // ==========================
    //
    // SUB END
    //
    // ==========================

    // ==========================
    //
    // MUL START
    //
    // ==========================

    fn mul_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        if expr.children.len() == 0 {
            return EvalResult::Number(1 as types::Number);
        }

        let mut result = 1 as types::Number;

        for child in expr.children.iter() {
            let child_res = self.eval(child, env);

            match child_res {
                EvalResult::Number(x) => {
                    result *= x;
                }
                _ => self.error(
                    &messages::fn_expected_num_arg(lexemes::L_FN_MUL.1),
                    expr.start_at,
                ),
            }
        }

        EvalResult::Number(result)
    }

    // ==========================
    //
    // MUL END
    //
    // ==========================

    // ==========================
    //
    // DIV START
    //
    // ==========================

    fn div_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let mut result = 1 as types::Number;

        for (i, child) in expr.children.iter().enumerate() {
            let child_res = self.eval(child, env);

            match child_res {
                EvalResult::Number(x) => {
                    if (i != 0 || expr.children.len() == 1) && x == 0.0 {
                        self.error(&messages::division_by_zero(), expr.start_at);
                    }

                    if expr.children.len() == 1 {
                        result = 1.0 / x;
                    } else if i == 0 {
                        result = x;
                    } else {
                        result /= x;
                    }
                }
                _ => self.error(
                    &messages::fn_expected_num_arg(lexemes::L_FN_DIV.1),
                    expr.start_at,
                ),
            }
        }

        EvalResult::Number(result)
    }

    // ==========================
    //
    // DIV END
    //
    // ==========================

    // ==========================
    //
    // NUMBER COMPARISON START
    //
    // ==========================

    fn number_comp_eval<P>(&self, expr: &Expr, env: &mut Env, predicate: P) -> EvalResult
    where
        P: Fn(types::Number, types::Number) -> bool,
    {
        if expr.children.len() == 1 {
            return EvalResult::Boolean(true);
        }

        let mut result = true;

        let mut current: types::Number = self.number_ensure(
            &self.eval(expr.children.first().unwrap(), env),
            expr.start_at,
        );

        for child in expr.children.iter().skip(1) {
            let child_res = self.number_ensure(&self.eval(child, env), child.start_at);

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
    //
    // NUMBER COMPARISON END
    //
    // ==========================

    // ==========================
    //
    // NOT START
    //
    // ==========================

    fn not_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let child_res = self.eval(expr.children.first().unwrap(), env);

        match child_res {
            EvalResult::Boolean(x) => EvalResult::Boolean(!x),
            EvalResult::Nil => EvalResult::Boolean(true),
            _ => EvalResult::Boolean(false),
        }
    }

    // ==========================
    //
    // NOT END
    //
    // ==========================

    // ==========================
    //
    // EQ START
    //
    // ==========================

    fn eq_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let mut result = true;

        for (i, child) in expr.children.iter().enumerate() {
            if i < expr.children.len() - 1 {
                let child_res = self.eval(child, env);
                let next_child_res = self.eval(expr.children.get(i + 1).unwrap(), env);

                if child_res != next_child_res {
                    result = false;
                    break;
                }
            }
        }

        EvalResult::Boolean(result)
    }

    // ==========================
    //
    // EQ END
    //
    // ==========================

    // ==========================
    //
    // NOT EQ START
    //
    // ==========================

    fn not_eq_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let res = self.eq_eval(expr, env);

        match res {
            EvalResult::Boolean(x) => EvalResult::Boolean(!x),
            _ => self.error(&messages::expected_boolean(to_str!(res)), expr.start_at),
        }
    }

    // ==========================
    //
    // NOT EQ END
    //
    // ==========================

    // ==========================
    //
    // BOOL START
    //
    // ==========================

    fn bool_coerce(res: EvalResult) -> EvalResult {
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

    fn bool_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let child_res = self.eval(expr.children.first().unwrap(), env);
        Self::bool_coerce(child_res)
    }

    // ==========================
    //
    // BOOL END
    //
    // ==========================

    // ==========================
    //
    // OR START
    //
    // ==========================

    fn or_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        if expr.children.len() == 0 {
            return EvalResult::Nil;
        }

        let mut result_index = 0;

        for (i, child) in expr.children.iter().enumerate() {
            let child_res = Self::bool_coerce(self.eval(child, env));

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

        self.eval(expr.children.get(result_index).unwrap(), env)
    }

    // ==========================
    //
    // OR END
    //
    // ==========================

    // ==========================
    //
    // AND START
    //
    // ==========================

    fn and_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        if expr.children.len() == 0 {
            return EvalResult::Boolean(true);
        }

        let mut result_index = 0;

        for (i, child) in expr.children.iter().enumerate() {
            let child_res = Self::bool_coerce(self.eval(child, env));

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

        self.eval(expr.children.get(result_index).unwrap(), env)
    }

    // ==========================
    //
    // AND END
    //
    // ==========================

    // ==========================
    //
    // IF START
    //
    // ==========================

    fn if_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let condition = expr.children.first().unwrap();
        let then_branch = expr.children.get(1).unwrap();
        let else_branch = expr.children.get(2);

        let condition_res = Self::bool_coerce(self.eval(condition, env));

        match condition_res {
            EvalResult::Boolean(x) => {
                if x {
                    return self.eval(then_branch, env);
                }

                if let Some(else_branch) = else_branch {
                    return self.eval(else_branch, env);
                }

                return EvalResult::Nil;
            }
            _ => self.error(
                &messages::expected_boolean(to_str!(condition_res)),
                expr.start_at,
            ),
        }
    }

    // ==========================
    //
    // IF END
    //
    // ==========================

    // ==========================
    //
    // NIL? START
    //
    // ==========================

    fn is_nil_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let child_res = self.eval(expr.children.first().unwrap(), env);

        match child_res {
            EvalResult::Nil => EvalResult::Boolean(true),
            _ => EvalResult::Boolean(false),
        }
    }

    // ==========================
    //
    // NIL? END
    //
    // ==========================

    // ==========================
    //
    // FN DEF START
    //
    // ==========================

    fn fn_def_eval(&self, expr: &Expr, env: &mut Env) -> EvalResult {
        let name = self.identifier_unwrap(&expr.children.first().unwrap());

        let args: Vec<String> = expr
            .children
            .get(1)
            .unwrap()
            .children
            .iter()
            .map(|x| self.identifier_unwrap(&x))
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

    // ==========================
    //
    // FN DEF END
    //
    // ==========================

    // ==========================
    //
    // FN CUSTOM START
    //
    // ==========================

    fn fn_custom_eval(&self, name: &str, expr: &Expr, env: &mut Env) -> EvalResult {
        // TODO: Get rid of clone
        if let Some(env_record) = env.clone().get(name) {
            match &env_record.value {
                EvalResult::FnDeclaration(fn_decl) => {
                    let expr = semanalyzer::analyze_fn_call_semantics(
                        expr,
                        &fn_decl.name,
                        fn_decl.args.len(),
                    );

                    let argument_bindings = fn_decl
                        .args
                        .iter()
                        .zip(expr.children.iter())
                        .map(|(x, y)| Binding::new(x.clone(), self.eval(y, env), y.start_at))
                        .collect();

                    let mut child_env = Env::new();
                    child_env.attach_parent(&env);
                    self.identifiers_bind(argument_bindings, &mut child_env, false, false, true);

                    let mut result = EvalResult::Nil;

                    for expr in &fn_decl.body {
                        result = self.eval(expr, &mut child_env);
                    }

                    return result;
                }
                _ => self.error(&messages::not_callable(name), expr.start_at),
            }
        } else {
            self.error(&messages::undefined_identifier(name), expr.start_at);
        }
    }

    // ==========================
    //
    // FN CUSTOM END
    //
    // ==========================
}

pub fn interpret(exprs: &Vec<Expr>, env: &mut Env, source_code: &str) -> Vec<EvalResult> {
    let interpreter = Interpreter::new(&exprs, source_code);
    interpreter.interpret(env)
}
