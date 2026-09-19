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
    pub const COLON: u8 = b':';
    pub const LESS_EQ: u8 = b'<';
    pub const MORE_EQ: u8 = b'>';
}

/// Deterministic Finite Automata states for parsing numbers.
#[derive(Debug)]
pub enum DfaNumState {
    Start,
    Sign,
    Zero,
    Int,
    Frac,
    Dot,
    Scient,
    ScientMinus,
    Expon,
}
