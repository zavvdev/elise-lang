const PREFIX: &str = "Lexing error. ";

pub fn number_overflow() -> String {
    format!("{}Number overflow.", PREFIX)
}

pub fn invalid_identifier_name(x: &str) -> String {
    format!("{}Invalid identifier name \"{}\".", PREFIX, x)
}

pub fn invalid_number() -> String {
    format!("{}Invalid number.", PREFIX)
}
