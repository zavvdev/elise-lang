use elise_shared::shared_errors::errors_csv_parser::CsvParserErr;

use crate::data_csv::data_csv_parser::CsvRow;

pub mod data_binder;
pub mod data_csv;
pub mod data_types;
pub mod data_schema_resolver;
pub mod data_config;

/// Result of the data parsing operation.
pub enum DataParseResult {
    Csv(Result<Vec<CsvRow>, CsvParserErr>),
}
