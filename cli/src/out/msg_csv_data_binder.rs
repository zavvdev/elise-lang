use crate::out::utils::{self};
use elise_shared::shared_errors::errors_csv_data_binder::CsvDataBinderErr;

pub fn print_err(parser_err: &CsvDataBinderErr) {
    use CsvDataBinderErr::*;
    let label = Some("CSV data binder error");

    match parser_err {
        NoData => {
            utils::print_err("No data provided", label);
        }
    };
}
