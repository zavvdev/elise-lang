pub fn get_panic_message() -> String {
    "Parse error!".to_string()
}

pub fn unexpected_token(x: &str) -> String {
    format!("Unexpected token \"{}\"", x)
}

pub fn unexpected_end_of_input() -> String {
    format!("Unexpected end of input")
}
