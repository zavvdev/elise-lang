// Punctuations

pub const L_MINUS: char = '-';
pub const L_LEFT_PAREN: char = '(';
pub const L_RIGHT_PAREN: char = ')';
pub const L_LEFT_SQR_BR: char = '[';
pub const L_RIGHT_SQR_BR: char = ']';
pub const L_COMMA: char = ',';
pub const L_WHITESPACE: char = ' ';
pub const L_NEWLINE: char = '\n';

// Funcitions

pub const L_FN: char = '.';
pub const L_FN_ADD: (char, &str) = (L_FN, "add");
pub const L_FN_SUB: (char, &str) = (L_FN, "sub");
pub const L_FN_MUL: (char, &str) = (L_FN, "mul");
pub const L_FN_DIV: (char, &str) = (L_FN, "div");
pub const L_FN_PRINT: (char, &str) = (L_FN, "print");
pub const L_FN_PRINTLN: (char, &str) = (L_FN, "println");
pub const L_FN_LET_BINDING: (char, &str) = (L_FN, "let");
pub const L_FN_GREATR: (char, &str) = (L_FN, "greatr");
pub const L_FN_GREATR_EQ: (char, &str) = (L_FN, "greatr-eq");
pub const L_FN_LESS: (char, &str) = (L_FN, "less");
pub const L_FN_LESS_EQ: (char, &str) = (L_FN, "less-eq");
pub const L_FN_EQ: (char, &str) = (L_FN, "eq");
pub const L_FN_NOT_EQ: (char, &str) = (L_FN, "not-eq");
pub const L_FN_NOT: (char, &str) = (L_FN, "not");
pub const L_FN_AND: (char, &str) = (L_FN, "and");
pub const L_FN_OR: (char, &str) = (L_FN, "or");
pub const L_FN_BOOL: (char, &str) = (L_FN, "bool");
pub const L_FN_IF: (char, &str) = (L_FN, "if");
pub const L_FN_IS_NIL: (char, &str) = (L_FN, "nil?");
pub const L_FN_DEFINE: (char, &str) = (L_FN, "fn");

// Literals

pub const L_NIL: &str = "nil";
pub const L_TRUE: &str = "true";
pub const L_FALSE: &str = "false";
pub const L_STRING_LITERAL: char = '"';
pub const L_STRING_LITERAL_ESCAPE: char = '\\';

pub const FORBIDDEN_IDENTIFIER_NAMES: [&str; 20] = [
    L_FN_ADD.1,
    L_FN_SUB.1,
    L_FN_MUL.1,
    L_FN_DIV.1,
    L_FN_PRINT.1,
    L_FN_PRINTLN.1,
    L_FN_LET_BINDING.1,
    L_FN_GREATR.1,
    L_FN_GREATR_EQ.1,
    L_FN_LESS.1,
    L_FN_LESS_EQ.1,
    L_FN_EQ.1,
    L_FN_NOT_EQ.1,
    L_FN_NOT.1,
    L_FN_AND.1,
    L_FN_OR.1,
    L_FN_BOOL.1,
    L_FN_IF.1,
    L_FN_IS_NIL.1,
    L_FN_DEFINE.1,
];

// ================

pub fn fn_lexeme_to_string(lexeme: (char, &str)) -> String {
    format!("{}{}", lexeme.0, lexeme.1)
}

pub fn to_fn_string(lexeme: &str) -> String {
    format!("{}{}", L_FN, lexeme)
}

pub fn is_forbidden_identifier_name(name: &str) -> bool {
    FORBIDDEN_IDENTIFIER_NAMES.contains(&name)
}
