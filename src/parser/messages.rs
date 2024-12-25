pub fn get_panic_message() -> String {
    "Parse error!".to_string()
}

pub fn unexpected_token(x: &str) -> String {
    format!("Unexpected token \"{}\"", x)
}

pub fn unmatched_parenthesis() -> String {
    format!("Unmatched closing parenthesis")
}

pub fn unmatched_sqr_bracket() -> String {
    format!("Unmatched closing square bracket")
}

pub fn unclosed_opening_symbols() -> String {
    format!("Unclosed opening symbols")
}
