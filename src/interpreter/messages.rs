pub fn invalid_expression(x: &str) -> String {
    format!("Interpretation error. Invalid use of expression \"{}\".", x)
}

pub fn unknown_expression(x: &str) -> String {
    format!("Interpretation error. Unknown expression \"{}\".", x)
}

pub fn add_fn_invalid_arg() -> String {
    format!("Interpretation error. Invalid arguments for function \"add\". Expected numbers.")
}
