use elise_shared::shared_errors::errors_preexec::PreExecErr;

use crate::out::utils::{self};

pub fn print_err(err: &PreExecErr) {
    use PreExecErr::*;

    let info = match err {
        NoResolvedSchema => "Missing schema resolution",
    };

    utils::print_err(info, Some("Pre-execution Error"));
}
