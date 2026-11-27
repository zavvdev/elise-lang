use elise_shared_diagnostics::errors_bindings_validator::BindingsValidatorErr;

use crate::out::{config::ErrCategory, utils};

pub fn print_err(err: &BindingsValidatorErr) {
    use BindingsValidatorErr::*;

    let (category, message) = match err {
        TypeMismatch {
            data_path_info,
            expected,
            found,
        } => (
            ErrCategory::TYPE,
            &format!(
                "Expected to have {} type, got {} on {}",
                expected, found, data_path_info,
            ),
        ),

        MissingTypeDeclaration {
            data_path_info,
            data_type_info,
        } => (
            ErrCategory::VALIDATION,
            &format!(
                "Missing type declaration for data defined on {} that has type {}",
                data_path_info, data_type_info,
            ),
        ),
    };

    utils::print_err(category, err.code(), message);
}
