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

pub fn expected_number(x: &str) -> String {
    format!("Interpretation error. Expected number, found \"{}\".", x)
}

pub fn division_by_zero() -> String {
    format!("Interpretation error. Division by zero.")
}

pub fn undefined_identifier(name: &str) -> String {
    format!("Interpretation error. Undefined identifier \"{}\".", name)
}

pub fn bind_value_not_found() -> String {
    format!("Interpretation error. Value for binding not found.")
}

pub fn non_identifier(x: &str) -> String {
    format!("Interpretation error. \"{}\" is not an identifier.", x)
}

pub fn identifier_exists(x: &str) -> String {
    format!("Interpretation error. Identifier \"{}\" already exists.", x)
}

pub fn expected_boolean(x: &str) -> String {
    format!("Interpretation error. Expected boolean, found \"{}\".", x)
}
