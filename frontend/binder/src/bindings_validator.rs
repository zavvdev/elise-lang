use elise_ir_bindings::binding_path::BindingPath;
use elise_ir_bindings::data_bindings::{DataBinding, DataBindingDesc};
use elise_ir_bindings::type_bindings::{TypeBinding, TypeBindingDataType};
use elise_shared_errors::errors_bindings_validator::BindingsValidatorErr;

fn match_type(
    binding_path: &BindingPath,
    type_binding: &TypeBinding,
    data_desc: &DataBindingDesc,
) -> Result<(), BindingsValidatorErr> {
    let type_desc = type_binding
        .map
        .get(binding_path)
        .ok_or(BindingsValidatorErr::MissingTypeDeclaration)?;

    let expected = match data_desc {
        DataBindingDesc::Int(_) => TypeBindingDataType::Int,
        DataBindingDesc::Str(_) => TypeBindingDataType::Str,
        DataBindingDesc::List(_) => TypeBindingDataType::List,
        DataBindingDesc::Dict(_) => TypeBindingDataType::Record,
    };

    if type_desc.dtype == expected {
        Ok(())
    } else {
        Err(BindingsValidatorErr::TypeMismatch)
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
    if let Some((data_binding_path, data_descriptor)) = data_binding.map.iter().next() {
        // Check if data_binding_path has Index segments, and if it does,
        // replace them with AbstractIndex segments in order to be able to
        // get type information from type_binding.
        match data_binding_path.indexes_into_abstract() {
            // If data_binding_path has indexes, a new path is returned.
            Some(path) => {
                return match_type(&path, type_binding, data_descriptor);
            }
            None => {
                return match_type(data_binding_path, type_binding, data_descriptor);
            }
        }
    }

    Ok(())
}
