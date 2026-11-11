use elise_shared_errors::errors_bindings_validator::BindingsValidatorErr;

use crate::out::utils::{self};

pub fn print_err(err: &BindingsValidatorErr) {
    use BindingsValidatorErr::*;

    let info = match err {
        Todo => "Todo",
    };

    utils::print_err(info, Some("Bindings Validator Error"));
}
