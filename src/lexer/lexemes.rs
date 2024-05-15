pub const L_MINUS: char = '-';

pub const L_LEFT_PAREN: char = '(';
pub const L_RIGHT_PAREN: char = ')';

pub const L_LEFT_SQR_BR: char = '[';
pub const L_RIGHT_SQR_BR: char = ']';

pub const L_COMMA: char = ',';
pub const L_WHITESPACE: char = ' ';

pub const L_FN: char = '@';
pub const L_FN_ADD: (char, &str) = (L_FN, "add");
pub const L_FN_SUB: (char, &str) = (L_FN, "sub");
pub const L_FN_MUL: (char, &str) = (L_FN, "mul");
pub const L_FN_DIV: (char, &str) = (L_FN, "div");
pub const L_FN_PRINT: (char, &str) = (L_FN, "print");
pub const L_FN_PRINTLN: (char, &str) = (L_FN, "println");
pub const L_FN_LET_BINDING: (char, &str) = (L_FN, "let");

pub const L_NIL: &str = "nil";
pub const L_TRUE: &str = "true";
pub const L_FALSE: &str = "false";

pub const L_STRING_LITERAL: char = '"';
pub const L_STRING_LITERAL_ESCAPE: char = '\\';

pub fn fn_lexeme_to_string(lexeme: (char, &str)) -> String {
    format!("{}{}", lexeme.0, lexeme.1)
}
