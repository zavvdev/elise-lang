use elise_shared::shared_errors::errors_data_validator::DataValidatorErr;

use crate::{data_binder::DataBindingTable, schema_resolver::ResolvedSchema};

pub fn validate_data(
    _binding_table: &DataBindingTable,
    _resolved_schema: &ResolvedSchema,
) -> Result<(), DataValidatorErr> {
    Ok(())
}
