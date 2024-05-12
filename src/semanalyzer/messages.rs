pub fn let_binding_min_args() -> String {
    "Semantic Analyzer error. Let binding must have at least one argument".to_string()
}

pub fn let_binding_first_arg_list() -> String {
    "Semantic Analyzer error. Let binding first argument must be a list".to_string()
}

pub fn let_binding_first_arg_even_elements() -> String {
    "Semantic Analyzer error. Let binding first argument must have an even number of elements"
        .to_string()
}

pub fn let_binding_arg_identifiers() -> String {
    "Semantic Analyzer error. Let binding first argument must have an even number of elements, with identifiers at even indexes"
        .to_string()
}
