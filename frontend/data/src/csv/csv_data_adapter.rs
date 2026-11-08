use elise_frontend_parser::config::CharCode;
use elise_shared_errors::errors_data_adapter::CsvDataAdapterErr;

use crate::{
    adapter::DataAdapter,
    csv::csv_data_parser::{CsvDataCol, CsvDataParserDataType, CsvDataRow, ParsedCsv},
};

type SourceCode = Vec<u8>;

pub struct CsvDataAdapter;

/// Translates parsed csv data structure into valid Elise source code.
impl DataAdapter<ParsedCsv, CsvDataAdapterErr> for CsvDataAdapter {
    fn translate(parsed_csv: &ParsedCsv) -> Result<SourceCode, CsvDataAdapterErr> {
        let mut source_code: SourceCode = vec![];
        source_code.push(CharCode::LEFT_SQR_BRACKET);
        for row in parsed_csv {
            Self::translate_dict(row, &mut source_code);
        }
        source_code.push(CharCode::RIGHT_SQR_BRACKET);
        Ok(source_code)
    }
}

impl CsvDataAdapter {
    fn translate_dict(csv_row: &CsvDataRow, source_code: &mut SourceCode) {
        source_code.push(CharCode::LEFT_CUR_BRACKET);
        for col in &csv_row.cols {
            Self::translate_dict_entry(col, source_code);
        }
        source_code.push(CharCode::RIGHT_CUR_BRACKET);
        source_code.push(CharCode::SPACE_SEPARATOR);
    }

    fn translate_dict_entry(csv_col: &CsvDataCol, source_code: &mut SourceCode) {
        // Insert key as string value.
        Self::translate_str_value(&csv_col.name, source_code);
        source_code.push(CharCode::SPACE_SEPARATOR);
        Self::translate_value(&csv_col.dtype, &csv_col.value, source_code);
        source_code.push(CharCode::SPACE_SEPARATOR);
    }

    fn translate_value(dtype: &CsvDataParserDataType, value: &str, source_code: &mut SourceCode) {
        match dtype {
            CsvDataParserDataType::Int
            | CsvDataParserDataType::Float
            | CsvDataParserDataType::Bool
            | CsvDataParserDataType::Null => Self::translate_as_is_value(value, source_code),
            CsvDataParserDataType::Str => Self::translate_str_value(value, source_code),
        }
    }

    fn translate_str_value(value: &str, source_code: &mut SourceCode) {
        source_code.push(CharCode::DOUBLE_QT);
        Self::translate_as_is_value(value, source_code);
        source_code.push(CharCode::DOUBLE_QT);
    }

    fn translate_as_is_value(value: &str, source_code: &mut SourceCode) {
        source_code.extend(value.as_bytes());
    }
}
