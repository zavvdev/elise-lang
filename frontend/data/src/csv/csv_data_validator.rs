use elise_shared::shared_errors::errors_csv_data_validator::CsvDataValidatorErr;

use crate::{csv::csv_parser::CsvColPos, data_binder::DataBindingTable, schema_resolver::ResolvedSchema};

pub fn csv_data_validate(
    _binding_table: &DataBindingTable<CsvColPos>,
    _resolved_schema: &ResolvedSchema,
) -> Result<(), CsvDataValidatorErr> {
    Ok(())
}
