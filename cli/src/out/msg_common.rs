use elise_shared_errors::errors_common::CommonErr;

use crate::out::utils::{self};

pub fn print_err(err: &CommonErr) {
    use CommonErr::*;

    let info = match err {
        MissingParserData => "Missing data",
    };

    utils::print_err(info, Some("Error"));
}
