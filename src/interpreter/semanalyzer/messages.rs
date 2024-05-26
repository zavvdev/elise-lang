pub fn invalid_args_amount(fn_name: &str, expected: &str, got: &str) -> String {
    format!(
        "Interpretation error. Invalid amount of arguments for function: {}. Expected: {}, Got: {}",
        fn_name.to_string(),
        expected,
        got
    )
}
