pub fn number_overflow() -> String {
    format!("Lexing error. Number overflow.")
}

pub fn unknown_lexeme(x: &str) -> String {
    format!("Lexing error. Unknown lexeme \"{}\".", x)
}

pub fn invalid_identifier_name(x: &str) -> String {
    format!("Lexing error. Invalid identifier name \"{}\".", x)
}
