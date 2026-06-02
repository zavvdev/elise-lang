use crate::out::utils::{self};
use elise_errors::errors_binder::BinderErr;

// TODO
pub fn print_err(parser_err: &BinderErr) {
    use BinderErr::*;

    let info = match parser_err {
        Todo => "Todo",
    };

    utils::print_err(info, Some("Binder error"));
}
