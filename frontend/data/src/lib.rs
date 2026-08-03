use elise_shared::shared_errors::errors_csv_parser::CsvParserErr;

use crate::csv::csv_parser::CsvRow;

//pub mod binder;
pub mod csv;
pub mod resolution_path;
pub mod schema_resolver;

/// Result of the data parsing operation.
pub enum DataParseResult {
    Csv(Result<Vec<CsvRow>, CsvParserErr>),
}
