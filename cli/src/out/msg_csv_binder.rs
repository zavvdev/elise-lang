use crate::out::utils::{self};
use elise_shared::shared_errors::errors_csv_binder::CsvBinderErr;

pub fn print_err(parser_err: &CsvBinderErr) {
    use CsvBinderErr::*;
    let label = Some("Binder error");

    match parser_err {
        NoData => {
            utils::print_err("No data provided", label);
        }
    };
}
