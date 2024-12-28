pub fn get_panic_message() -> String {
    "Semantic Analysis error!".to_string()
}

// Args

pub fn args_invalid_amount(fn_name: &str, expected: &str, got: &str) -> String {
    format!(
        "Invalid amount of arguments for function: {}. Expected: {}, Got: {}",
        fn_name.to_string(),
        expected,
        got
    )
}

// Type Expr

pub fn type_expr_invalid(expected: &str, got: &str) -> String {
    format!(
        "Invalid type of expression. Expected: {}, Got: {}",
        expected, got
    )
}

// Let

pub fn let_invalid_binding_form() -> String {
    "Let binding first argument must have an even number of elements, with identifiers at non-even positions"
        .to_string()
}

// Fn Define

pub fn fn_def_invalid_args_decl(fn_name: &str, got: &str) -> String {
    format!(
        "Invalid argument declaration for function: {}. Expected Identifier, got: {}",
        fn_name.to_string(),
        got,
    )
}

pub fn fn_def_duplicate_arg_decl(fn_name: &str) -> String {
    format!(
        "Duplicate argument declaration for function: {}",
        fn_name.to_string(),
    )
}
