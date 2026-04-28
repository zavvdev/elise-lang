use elise::conf::ConfErr;

use crate::out::utils::print_silent_err;

pub fn print_conf_err(err: &ConfErr) {
    let msg = match err {
        ConfErr::ExtInvalid(ext) => format!("Invalid extension: '{}'", ext),
        ConfErr::ArgInvalid(arg) => format!(
            "Invalid argument. Expected '{}', got '{}'",
            arg.provided, arg.arg_name
        ),
        ConfErr::ArgRequired(arg) => format!("Argument required: '{}'", arg),
    };
    print_silent_err(&msg, Some("Config error"));
}
