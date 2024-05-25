pub fn invalid_let_binding_form() -> String {
    "Semantic Analyzer error. Let binding first argument must have an even number of elements, with identifiers at non-even positions"
        .to_string()
}

pub fn invalid_args_amount(fn_name: &str, expected: &str, got: &str) -> String {
    format!(
        "Semantic Analyzer error. Invalid amount of arguments for function: {}. Expected: {}, Got: {}",
        fn_name.to_string(),
        expected,
        got
    )
}

pub fn invalid_arg_type(fn_name: &str, position: usize, expected: &str, got: &str) -> String {
    format!(
        "Semantic Analyzer error. Invalid argument type at position ({}) for function: {}. Expected: {}, Got: {}",
        position,
        fn_name.to_string(),
        expected,
        got
    )
}

pub fn invalid_fn_arg_decl(fn_name: &str, got: &str) -> String {
    format!(
        "Semantic Analyzer error. Invalid function argument declaration for function: {}. Expected Identifier, got: {}",
        fn_name.to_string(),
        got,
    )
}

pub fn duplicate_fn_arg_decl(fn_name: &str) -> String {
    format!(
        "Semantic Analyzer error. Duplicate function argument declaration for function: {}",
        fn_name.to_string(),
    )
}
