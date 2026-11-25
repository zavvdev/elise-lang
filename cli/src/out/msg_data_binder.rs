use elise_shared_diagnostics::errors_data_binder::DataBinderErr;

use crate::out::utils::{self};

pub fn print_err(err: &DataBinderErr) {
    use DataBinderErr::*;

    let info = match err {
        UnresolvablePath { path } => &format!("Unresolvable path: {}", path),
    };

    utils::print_err("TODO", err.code(), info);
}
