use elise_shared::shared_errors::errors_data_validator::DataValidatorErr;

use crate::out::utils::{self};

pub fn print_err(parser_err: &DataValidatorErr) {
    use DataValidatorErr::*;
    let label = Some("Data validation error");

    match parser_err {
        TypeMismatch {
            pos,
            expected,
            found,
        } => {
            utils::print_err("Type mismatch", label);
            utils::print_err_source_code_pos(pos.row, pos.col);
            utils::print_err(&format!("Expected: {}, found: {}", expected, found), None);
        }
        UnknownDataPath { pos } => {
            utils::print_err("Type definition cannot be found for this record", label);
            utils::print_err_source_code_pos(pos.row, pos.col);
        }
    };
}
