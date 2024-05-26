pub fn number_overflow() -> String {
    format!("Lexing error. Number overflow.")
}

pub fn invalid_identifier_name(x: &str) -> String {
    format!("Lexing error. Invalid identifier name \"{}\".", x)
}
