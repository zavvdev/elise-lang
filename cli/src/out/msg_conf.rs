use elise::conf::ConfErr;

use crate::out::utils;

pub fn print_err(err: &ConfErr) {
    let msg = match err {
        ConfErr::ExtInvalid(path) => format!("Invalid extension: '{}'.", path),

        ConfErr::ArgInvalid(arg) => {
            if arg.expected.is_empty() {
                format!(
                    "Invalid value '{}' for argument '{}'.",
                    arg.provided, arg.arg_name
                )
            } else if arg.expected.len() == 1 {
                format!(
                    "Invalid value for argument '{}'. Expected '{}', got '{}'.",
                    arg.arg_name,
                    arg.expected.first().unwrap(),
                    arg.provided,
                )
            } else {
                format!(
                    "Invalid value for argument '{}'. Expected one of: [{}], got '{}'.",
                    arg.arg_name,
                    arg.expected.join(", "),
                    arg.provided,
                )
            }
        }

        ConfErr::ArgRequired(arg) => format!("Argument required: '{}'.", arg),
    };

    utils::print_err(&msg, Some("Configuration Error"));
}
