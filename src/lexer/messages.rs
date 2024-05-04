pub fn number_overflow() -> String {
    format!("Lexing error. Number overflow.")
}

pub fn unknown_lexeme(x: &str) -> String {
    format!("Lexing error. Unknown lexeme \"{}\".", x)
}
