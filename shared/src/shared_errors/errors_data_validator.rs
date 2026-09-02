use crate::shared_types::Pos;

#[derive(Debug, PartialEq)]
pub enum DataValidatorErr {
    TypeMismatch {
        pos: Pos,
        expected: &'static str,
        found: &'static str,
    },
    UnknownDataPath {
        pos: Pos,
    },
}
