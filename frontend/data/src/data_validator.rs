use elise_shared::shared_errors::errors_data_validator::DataValidatorErr;

use crate::{
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

// TODO:
// If data is not there and it's optional we must not fail.
// If data is there and has Null and it's nullable: true we must not fail.
// ISSUE:
// If we go only through data binding table, but at the same time we have
// some fields in schema declared as optional, we can't validate them properly
// because we never encounter them since we don't walk through resolved schema.
//
// WE NEED SOME SEPARATE STEP FOR RECONCILIATION/NORMALIZATION OF THE
// BINDING TABLE AND RESOLVED SCHEMA. SOMETHING THAT REDUCES RESOLUTION PATHS
// TO A STABLE INDEXES THAT CAN BE REFERENCED BY BYTECODE AND VM.
pub fn validate_data(
    data_bindings: &DataBindings,
    schema_bindings: &SchemaBindings,
) -> Result<(), DataValidatorErr> {
    if let Some((_, data_binding_descriptor)) = data_bindings.bindings.iter().next() {
        if let Some(resolved_type) = schema_bindings
            .bindings
            .get(&data_binding_descriptor.type_resolution_path)
        {
            if match_data_type(&data_binding_descriptor.ty, &resolved_type.dtype) {
                return Ok(());
            } else {
                return Ok(());
            }
        } else {
            return Err(DataValidatorErr::UnknownDataPath {
                pos: data_binding_descriptor.pos.clone(),
            });
        }
    }

    Ok(())
}
