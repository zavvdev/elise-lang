pub fn m_int_overflow() -> String {
    format!("Integer overflow")
}

pub fn m_float_overflow() -> String {
    format!("Float overflow")
}

pub fn m_unexpected_token(x: &str) -> String {
    format!("Unexpected token \"{}\"", x)
}

pub fn m_unexpected_end_of_input() -> String {
    format!("Unexpected end of input")
}
