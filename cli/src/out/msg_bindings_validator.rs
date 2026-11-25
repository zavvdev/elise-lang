use elise_shared_diagnostics::errors_bindings_validator::BindingsValidatorErr;

use crate::out::utils;

pub fn print_err(err: &BindingsValidatorErr) {
    use BindingsValidatorErr::*;

    let info = match err {
        TypeMismatch {
            data_path_info,
            expected,
            found,
        } => format!(
            "Expected to have {} type, got {} on {}",
            expected, found, data_path_info,
        ),

        MissingTypeDeclaration {
            data_path_info,
            data_type_info,
        } => format!(
            "Missing type declaration for data defined on {} that has type {}",
            data_path_info, data_type_info,
        ),
    };

    utils::print_err("TODO", err.code(), &info, None);
}
