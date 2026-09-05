use std::collections::HashSet;

use elise_shared::shared_errors::errors_data_validator::DataValidatorErr;

use crate::{
    binding_path::BindingPath,
    data_binder::{DataBinderDataType, DataBindings},
    schema_binder::{SchemaBinderDataType, SchemaBindings},
};

fn match_data_type(
    data_binding_data_type: &DataBinderDataType,
    schema_binding_data_type: &SchemaBinderDataType,
) -> bool {
    use DataBinderDataType::*;

    match data_binding_data_type {
        Int => *schema_binding_data_type == SchemaBinderDataType::Int,
        Float => *schema_binding_data_type == SchemaBinderDataType::Float,
        String => *schema_binding_data_type == SchemaBinderDataType::String,
        Bool => *schema_binding_data_type == SchemaBinderDataType::Bool,
        _ => false,
    }
}

/// Go through data bindings first and use type_binding_path in order to resolve
/// type definition from schema bindings. Successful resolution considered when:
///
/// 1. Data type and schema type matches
/// 2. OR data type is Null and schema type allows nullable.
///
/// When type matching succeeds, we write type_binding_path into 'witnessed' HashSet
/// and map over schema bindings. During that mapping, we check if schema disallows
/// a current record to be optional. If it's not optional and witnessed does not have
/// schema binding path, we fail the validation.
pub fn validate_data(
    data_bindings: &DataBindings,
    schema_bindings: &SchemaBindings,
) -> Result<(), DataValidatorErr> {
    let mut witnessed: HashSet<&BindingPath> = HashSet::new();

    // We don't need to read data binding key here since we're just validating types.
    for data in data_bindings.bindings.values() {
        if let Some(type_def) = schema_bindings.bindings.get(&data.type_binding_path) {
            if match_data_type(&data.dtype, &type_def.dtype)
                || (type_def.nullable && data.dtype == DataBinderDataType::Null)
            {
                witnessed.insert(&data.type_binding_path);
                continue;
            }
            return Err(DataValidatorErr::DataTypeMismatch {
                pos: data.pos.clone(),
                expected: type_def.dtype.as_str(),
                found: data.dtype.as_str(),
            });
        } else {
            return Err(DataValidatorErr::DataMissingTypeDef {
                pos: data.pos.clone(),
            });
        }
    }

    // TODO: Skip any type definition binding paths that do not resolve to literal value?
    //       Or update csv_data_binder to include full paths like schema_binder has?
    for (schema_binding_path, type_def) in &schema_bindings.bindings {
        if !type_def.optional && !witnessed.contains(&schema_binding_path) {
            return Err(DataValidatorErr::DataMissing {
                span: type_def.span.clone(),
            });
        }
    }

    Ok(())
}
