pub enum LangError {
    ParserError {
        line: usize,
        col: usize,
        message: String,
    },
}
