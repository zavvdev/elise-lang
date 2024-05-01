pub fn unexpected_token(x: &str) -> String {
    format!("Parse error. Unexpected token \"{}\".", x)
}

pub fn unexpected_end_of_input() -> String {
    format!("Parse error. Unexpected end of input.")
}
