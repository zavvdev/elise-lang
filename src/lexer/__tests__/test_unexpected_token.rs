#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    #[test]
    #[should_panic(expected = "Lexing error. Unknown lexeme \"@klk\"")]
    fn test_unknown() {
        tokenize("@klk");
    }
}
