pub fn get_panic_message() -> String {
    "Lexing error!".to_string()
}

pub fn number_overflow() -> String {
    "Number overflow".to_string()
}

pub fn invalid_identifier_name(x: &str) -> String {
    format!("Invalid identifier name \"{}\"", x)
}

pub fn invalid_number() -> String {
    "Invalid number".to_string()
}

pub fn unexpected_end_of_string() -> String {
    "Unexpected end of string".to_string()
}

pub fn invalid_fn_name(x: &str) -> String {
    format!("Invalid function name \"{}\"", x)
}
