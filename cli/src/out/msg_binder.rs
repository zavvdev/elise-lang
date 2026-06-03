use crate::out::utils::{self};
use elise_errors::errors_binder::BinderErr;

pub fn print_err(parser_err: &BinderErr) {
    use BinderErr::*;

    let (msg, info) = match parser_err {
        NoData => ("No data provided", None),
        RowLenMismatch(info) => (
            "Data row length does not match the length of the row inside the schema",
            Some(info),
        ),
        TypeMismatch(info) => ("Invalid data type", Some(info)),
    };

    utils::print_err(msg, Some("Binder error"));
    if let Some(info) = info {
        utils::print_error_source_code_pos(info.row, info.col);
    }
}
