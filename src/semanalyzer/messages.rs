pub fn let_binding_first_arg_list() -> String {
    "Semantic Analyzer error. Let binding first argument must be a list".to_string()
}

pub fn let_binding_first_arg_even_elements() -> String {
    "Semantic Analyzer error. Let binding first argument must have an even number of elements"
        .to_string()
}

pub fn let_binding_arg_identifiers() -> String {
    "Semantic Analyzer error. Let binding first argument must have an even number of elements, with identifiers at non-even positions"
        .to_string()
}

pub fn zero_args_fn(fn_name: &str) -> String {
    format!("Semantic Analyzer error. Invalid amount of arguments (0) for function: {}", fn_name.to_string())
}

pub fn more_than_one_arg_fn(fn_name: &str) -> String {
    format!("Semantic Analyzer error. Invalid amount of arguments (> 1) for function: {}", fn_name.to_string())
}
