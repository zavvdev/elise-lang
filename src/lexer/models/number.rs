pub const FLOAT_SEPARATOR: char = '.';

pub type BaseNumber = i64;

#[derive(Debug)]
pub struct ConsumedNumber {
    pub int: BaseNumber,
    pub precision: BaseNumber,
    pub is_int: bool,
    pub is_negative: bool,
}

impl ConsumedNumber {
    pub fn new(int: BaseNumber, precision: BaseNumber, is_int: bool, is_negative: bool) -> Self {
        Self {
            int,
            precision,
            is_int,
            is_negative,
        }
    }
}
