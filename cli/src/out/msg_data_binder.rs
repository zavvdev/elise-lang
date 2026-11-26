use elise_shared_diagnostics::errors_data_binder::DataBinderErr;

use crate::out::{
    config::ErrCategory,
    utils::{self},
};

pub fn print_err(err: &DataBinderErr) {
    use DataBinderErr::*;

    let (category, message) = match err {
        UnresolvablePath { path } => (
            ErrCategory::REFERENCE,
            &format!("Unresolvable path: {}", path),
        ),
    };

    utils::print_err(category, err.code(), message);
}
