// Tokens

pub const IDENTIFIER_REGEX: &str = r"^[A-Za-z][A-Za-z0-9\-?!_]*$";
pub const T_CALL_PREFIX: u8 = b'.';
pub const T_TRUE: &str = "true";
pub const T_FALSE: &str = "false";
pub const T_NULL: &str = "null";
pub const T_LEFT_PAREN: u8 = b'(';
pub const T_RIGHT_PAREN: u8 = b')';
pub const T_LEFT_SQR_BRACKET: u8 = b'[';
pub const T_RIGHT_SQR_BRACKET: u8 = b']';
pub const T_LEFT_CUR_BRACKET: u8 = b'{';
pub const T_RIGHT_CUR_BRACKET: u8 = b'}';
pub const T_MINUS: u8 = b'-';
pub const T_COMMA: u8 = b',';
pub const T_DOUBLE_QT: u8 = b'"';

// Messages

pub const M_NUMBER_INVALID: &str = "Invalid number";
pub const M_STRING_INVALID: &str = "Invalid string";
pub const M_CALL_NAME_INVALID: &str = "Invalid function name";
pub const M_CALL_UNEXPECTED_END: &str = "Unexpected end of function";
pub const M_TOKEN_UNEXPECTED: &str = "Unexpected token";
pub const M_LIST_UNEXPECTED_END: &str = "Unexpected end of list";
pub const M_DICT_UNEXPECTED_END: &str = "Unexpected end of dictionary";
pub const M_DICT_UNEXPECTED_KEY: &str = "Unexpected dictionary key type. Keys should be strings";
pub const M_DICT_INVALID_PAIR: &str = "Invalid dictionary key-value pair";
pub const M_UNDEXPECTED_EOF: &str = "Unexpected end of file";
