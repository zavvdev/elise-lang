use elise_shared_diagnostics::errors_config::ConfErr;

use crate::out::{config::ErrCategory, utils};

pub fn print_err(err: &ConfErr) {
    let (category, message) = match err {
        ConfErr::ExtInvalid(path) => (
            ErrCategory::ARGUMENT,
            &format!("Invalid extension: '{}'.", path),
        ),

        ConfErr::ArgInvalid(arg) => {
            if arg.expected.is_empty() {
                (
                    ErrCategory::ARGUMENT,
                    &format!(
                        "Invalid value '{}' for argument '{}'.",
                        arg.provided, arg.arg_name
                    ),
                )
            } else if arg.expected.len() == 1 {
                (
                    ErrCategory::ARGUMENT,
                    &format!(
                        "Invalid value for argument '{}'. Expected '{}', got '{}'.",
                        arg.arg_name,
                        arg.expected.first().unwrap(),
                        arg.provided,
                    ),
                )
            } else {
                (
                    ErrCategory::ARGUMENT,
                    &format!(
                        "Invalid value for argument '{}'. Expected one of: [{}], got '{}'.",
                        arg.arg_name,
                        arg.expected.join(", "),
                        arg.provided,
                    ),
                )
            }
        }

        ConfErr::ArgRequired(arg) => (
            ErrCategory::ARGUMENT,
            &format!("Argument required: '{}'.", arg),
        ),
    };

    utils::print_err(category, err.code(), message);
}
