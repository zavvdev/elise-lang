pub struct CharCode;
impl CharCode {
    pub const CALL_PREFIX: u8 = b'.';
    pub const SLOT_PREFIX: u8 = b'@';
    pub const LEFT_PAREN: u8 = b'(';
    pub const RIGHT_PAREN: u8 = b')';
    pub const LEFT_SQR_BRACKET: u8 = b'[';
    pub const RIGHT_SQR_BRACKET: u8 = b']';
    pub const LEFT_CUR_BRACKET: u8 = b'{';
    pub const RIGHT_CUR_BRACKET: u8 = b'}';
    pub const MINUS: u8 = b'-';
    pub const COMMA: u8 = b',';
    pub const DOUBLE_QT: u8 = b'"';
}

pub struct Keyword;
impl Keyword {
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const NULL: &str = "null";
}
