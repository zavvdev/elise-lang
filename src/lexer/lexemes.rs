pub const L_MINUS: char = '-';

pub const L_LEFT_PAREN: char = '(';
pub const L_RIGHT_PAREN: char = ')';

pub const L_LEFT_SQR_BR: char = '[';
pub const L_RIGHT_SQR_BR: char = ']';

pub const L_COLON: char = ':';
pub const L_COMMA: char = ',';
pub const L_WHITESPACE: char = ' ';

pub const L_RETURN_TYPE: (char, char) = (L_MINUS, '>');

pub const L_FN: char = '@';
pub const L_FN_ADD: (char, &str) = (L_FN, "add");
pub const L_FN_SUB: (char, &str) = (L_FN, "sub");
pub const L_FN_MUL: (char, &str) = (L_FN, "mul");
pub const L_FN_DIV: (char, &str) = (L_FN, "div");

pub fn fn_lexeme_to_string(lexeme: (char, &str)) -> String {
    format!("{}{}", lexeme.0, lexeme.1)
}
