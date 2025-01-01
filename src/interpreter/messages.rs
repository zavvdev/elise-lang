pub fn get_panic_message() -> String {
    format!("Interpretation error.")
}

pub fn invalid_expression(x: &str) -> String {
    format!("Invalid use of expression \"{}\"", x)
}

pub fn unknown_expression(x: &str) -> String {
    format!("Unknown expression \"{}\"", x)
}

pub fn fn_expected_num_arg(fn_name: &str) -> String {
    format!(
        "Invalid arguments for function \"{}\". Expected numbers",
        fn_name
    )
}

pub fn expected_number(x: &str) -> String {
    format!("Expected number, found \"{}\"", x)
}

pub fn division_by_zero() -> String {
    format!("Division by zero")
}

pub fn undefined_identifier(name: &str) -> String {
    format!("Undefined identifier \"{}\"", name)
}

pub fn bind_value_not_found() -> String {
    format!("Value for binding not found")
}

pub fn non_identifier(x: &str) -> String {
    format!("\"{}\" is not an identifier", x)
}

pub fn identifier_exists_same_env(x: &str) -> String {
    format!("Identifier \"{}\" already exists in the same scope", x)
}

pub fn identifier_exists_parent_env(x: &str) -> String {
    format!("Identifier \"{}\" already exists in the parent scope", x)
}

pub fn expected_boolean(x: &str) -> String {
    format!("Expected boolean, found \"{}\"", x)
}

pub fn not_callable(x: &str) -> String {
    format!("\"{}\" is not callable", x)
}

pub fn invalid_args_amount(fn_name: &str, expected: &str, got: &str) -> String {
    format!(
        "Invalid amount of arguments for function: {}. Expected: {}, Got: {}",
        fn_name.to_string(),
        expected,
        got
    )
}
