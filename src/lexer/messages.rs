pub fn int_overflow() -> String {
    format!("Lexing error. Integer overflow.")
}

pub fn float_overflow() -> String {
    format!("Lexing error. Float overflow.")
}

pub fn unknown_lexeme(x: &str) -> String {
    format!("Lexing error. Unknown lexeme \"{}\".", x)
}
