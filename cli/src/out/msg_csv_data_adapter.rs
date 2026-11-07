use crate::out::utils::{self};
use elise_shared_errors::errors_data_adapter::CsvDataAdapterErr;

pub fn print_err(parser_err: &CsvDataAdapterErr) {
    use CsvDataAdapterErr::*;
    let label = Some("CSV data adapter error");

    match parser_err {
        Todo => {
            utils::print_err("Todo", label);
        }
    };
}
