pub fn invalid_expression(x: &str) -> String {
    format!("Interpretation error. Invalid use of expression \"{}\".", x)
}

pub fn unknown_expression(x: &str) -> String {
    format!("Interpretation error. Unknown expression \"{}\".", x)
}

pub fn fn_expected_num_arg(fn_name: &str) -> String {
    format!(
        "Interpretation error. Invalid arguments for function \"{}\". Expected numbers.",
        fn_name
    )
}

pub fn fn_no_args(fn_name: &str) -> String {
    format!(
        "Interpretation error. Invalid number of arguments (0) for function \"{}\".",
        fn_name
    )
}

pub fn division_by_zero() -> String {
    format!("Interpretation error. Division by zero.")
}
