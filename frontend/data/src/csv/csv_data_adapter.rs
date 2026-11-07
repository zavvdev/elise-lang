use elise_shared_errors::errors_data_adapter::CsvDataAdapterErr;

use crate::{adapter::DataAdapter, csv::csv_data_parser::ParsedCsv};

pub struct CsvDataAdapter;

impl DataAdapter<ParsedCsv, CsvDataAdapterErr> for CsvDataAdapter {
    fn translate(_parsed_csv: &ParsedCsv) -> Result<Vec<u8>, CsvDataAdapterErr> {
        Ok(vec![])
    }
}
