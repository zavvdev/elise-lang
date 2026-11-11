use elise_ir_bindings::data_bindings::DataBinding;
use elise_ir_bindings::type_bindings::TypeBinding;
use elise_shared_errors::errors_bindings_validator::BindingsValidatorErr;

pub fn validate_bindings(
    _data_binding: &DataBinding,
    _type_binding: &TypeBinding,
) -> Result<(), BindingsValidatorErr> {
    Err(BindingsValidatorErr::Todo)
}
