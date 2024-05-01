pub fn invalid_expression(x: &str) -> String {
    format!("Interpretation error. Invalid use of expression \"{}\".", x)
}

pub fn unknown_expression(x: &str) -> String {
    format!("Interpretation error. Unknown expression \"{}\".", x)
}
