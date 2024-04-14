pub fn m_int_overflow() -> String {
    format!("Integer overflow")
}

pub fn m_float_overflow() -> String {
    format!("Float overflow")
}

pub fn m_unexpected_token(x: &str) -> String {
    format!("Unexpected token \"{}\"", x)
}

pub fn m_parse_error_unexpected_token(x: &str) -> String {
    format!("Parse error. Unexpected token \"{}\"", x)
}
