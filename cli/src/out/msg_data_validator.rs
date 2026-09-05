use elise_shared::shared_errors::errors_data_validator::DataValidatorErr;

use crate::out::utils::{
    self, get_source_code_slice, print_err_source_code_pos, print_err_source_code_slice,
};

pub fn print_err(parser_err: &DataValidatorErr, source_code: &[u8]) {
    use DataValidatorErr::*;
    let label = Some("Data validation error");

    match parser_err {
        DataTypeMismatch {
            pos,
            expected,
            found,
        } => {
            utils::print_err("Type mismatch", label);
            utils::print_err_source_code_pos(pos.row, pos.col);
            utils::print_err(&format!("Expected: {}, found: {}", expected, found), None);
        }
        DataMissingTypeDef { pos } => {
            utils::print_err("Type definition cannot be found for this record", label);
            utils::print_err_source_code_pos(pos.row, pos.col);
        }
        DataMissing { span } => {
            utils::print_err("Cannot find data record for defined data type", label);
            if let Some(code) = &get_source_code_slice(source_code, span.start) {
                print_err_source_code_pos(code.pos.row, code.pos.col);
                print_err_source_code_slice(&code.slice, code.pos.col);
            };
        }
    };
}
