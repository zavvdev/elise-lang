use crate::shared_types::{Pos, Span};

#[derive(Debug, PartialEq)]
pub enum DataValidatorErr {
    DataTypeMismatch {
        pos: Pos,
        expected: &'static str,
        found: &'static str,
    },
    DataMissingTypeDef {
        pos: Pos,
    },
    DataMissing {
        span: Span,
    },
}
