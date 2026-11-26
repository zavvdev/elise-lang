use elise_shared_diagnostics::errors_config::ConfErr;

use crate::out::utils;

pub fn print_err(err: &ConfErr) {
    let (category, message) = match err {
        ConfErr::ExtInvalid(path) => ("TODO", &format!("Invalid extension: '{}'.", path)),

        ConfErr::ArgInvalid(arg) => {
            if arg.expected.is_empty() {
                (
                    "TODO",
                    &format!(
                        "Invalid value '{}' for argument '{}'.",
                        arg.provided, arg.arg_name
                    ),
                )
            } else if arg.expected.len() == 1 {
                (
                    "TODO",
                    &format!(
                        "Invalid value for argument '{}'. Expected '{}', got '{}'.",
                        arg.arg_name,
                        arg.expected.first().unwrap(),
                        arg.provided,
                    ),
                )
            } else {
                (
                    "TODO",
                    &format!(
                        "Invalid value for argument '{}'. Expected one of: [{}], got '{}'.",
                        arg.arg_name,
                        arg.expected.join(", "),
                        arg.provided,
                    ),
                )
            }
        }

        ConfErr::ArgRequired(arg) => ("TODO", &format!("Argument required: '{}'.", arg)),
    };

    utils::print_err(category, err.code(), message);
}
