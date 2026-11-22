use elise_ir_bindings::binding_path::BindingPath;
use elise_ir_bindings::data_bindings::{DataBinding, DataBindingDesc};
use elise_ir_bindings::type_bindings::{TypeBinding, TypeBindingDataType};
use elise_shared_errors::errors_bindings_validator::BindingsValidatorErr;

fn match_type(
    binding_path: &BindingPath,
    type_binding: &TypeBinding,
    data_desc: &DataBindingDesc,
    error_report_binding_path: &BindingPath,
) -> Result<(), BindingsValidatorErr> {
    let type_desc =
        type_binding
            .map
            .get(binding_path)
            .ok_or(BindingsValidatorErr::MissingTypeDeclaration {
                data_path_info: error_report_binding_path.as_string(),
                data_type_info: data_desc.name(),
            })?;

    let matched_type = match data_desc {
        DataBindingDesc::Int(_) => TypeBindingDataType::Int,
        DataBindingDesc::Str(_) => TypeBindingDataType::Str,
        DataBindingDesc::List(_) => TypeBindingDataType::List,
        DataBindingDesc::Dict(_) => TypeBindingDataType::Record,
    };

    if type_desc.dtype == matched_type {
        Ok(())
    } else {
        Err(BindingsValidatorErr::TypeMismatch {
            data_path_info: error_report_binding_path.as_string(),
            expected: type_desc.dtype.name(),
            found: data_desc.name(),
        })
    }
}

/// Type verification for data bindings.
/// Takes data bindings and type bindings. Maps through
/// data binding entries and uses its binding path to
/// access the respective type definition in order to proof
/// that path points to the same type in data and type bindings.
pub fn validate_bindings(
    data_binding: &DataBinding,
    type_binding: &TypeBinding,
) -> Result<(), BindingsValidatorErr> {
    for (data_binding_path, data_descriptor) in data_binding.map.iter() {
        // Replace Index segments with AbstractIndex segments, if any,
        // so type information can be looked up in type_binding.
        let abstract_path = data_binding_path.indexes_into_abstract();
        let path = abstract_path.as_ref().unwrap_or(data_binding_path);
        match_type(path, type_binding, data_descriptor, data_binding_path)?;
    }
    Ok(())
}
