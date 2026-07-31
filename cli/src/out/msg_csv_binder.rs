use crate::out::utils::{self};
use elise_shared::shared_errors::errors_csv_binder::CsvBinderErr;

pub fn print_err(parser_err: &CsvBinderErr) {
    use CsvBinderErr::*;
    let label = Some("Binder error");

    match parser_err {
        NoData => {
            utils::print_err("No data provided", label);
        }
        RowLenMismatch(info) => {
            utils::print_err(
                "Row length does not match the length of the schema row",
                label,
            );
            utils::print_err_source_code_pos(info.row, info.col);
        }
        TypeMismatch { pos, expected, got } => {
            utils::print_err("Type mismatch", label);
            utils::print_err_source_code_pos(pos.row, pos.col);
            utils::print_err_type_mismatch(expected, got);
        }
        MissingTypeDefinition { pos, col } => {
            utils::print_err(&format!("Missing type definition for column: {col}"), label);
            utils::print_err_source_code_pos(pos.row, pos.col);
        }
    };
}
