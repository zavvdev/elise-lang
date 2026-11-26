use elise_shared_diagnostics::errors_fsys::FSysErr;

use crate::out::utils;

pub fn print_err(err: FSysErr) {
    let message = match &err {
        FSysErr::R { msg, path } => &format!("Unable to read '{}' ({})", path, msg),
        FSysErr::W { msg, path } => &format!("Unable to write '{}' ({})", path, msg),
    };
    utils::print_err(
        "File System Error",
        err.code(),
        message,
    );
}
