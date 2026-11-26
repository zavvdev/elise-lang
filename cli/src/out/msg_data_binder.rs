use elise_shared_diagnostics::errors_data_binder::DataBinderErr;

use crate::out::utils::{self};

pub fn print_err(err: &DataBinderErr) {
    use DataBinderErr::*;

    let (category, message) = match err {
        UnresolvablePath { path } => ("TODO", &format!("Unresolvable path: {}", path)),
    };

    utils::print_err(category, err.code(), message);
}
