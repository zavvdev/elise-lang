use elise_shared::shared_errors::errors_csv_data_validator::CsvDataValidatorErr;

use crate::out::utils::{self};

pub fn print_err(parser_err: &CsvDataValidatorErr) {
    use CsvDataValidatorErr::*;
    let label = Some("CSV data validation error");

    match parser_err {
        TypeMismatch {
            row,
            col,
            expected,
            found,
        } => {
            utils::print_err("Type mismatch", label);
            utils::print_err_source_code_pos(*row, *col);
            format!("Expected: {}, found: {}", expected, found);
        }
        UnknownDataPath { row, col } => {
            utils::print_err("Type definition cannot be found for this record", label);
            utils::print_err_source_code_pos(*row, *col);
        }
    };
}
