use csv::{ErrorKind, ReaderBuilder};
use elise_shared::{
    shared_errors::errors_csv_data_parser::CsvDataParserErr,
    shared_types::{Literal, Pos},
};

// ==================================================================
//
// PARSER START
//
// ==================================================================

#[derive(PartialEq, Debug, Clone)]
pub enum CsvDataParserDataType {
    Int,
    Float,
    Str,
    Bool,
    Null,
}

#[derive(Debug, PartialEq)]
pub struct CsvDataCol {
    pub name: String,
    pub dtype: CsvDataParserDataType,
    pub value: String,
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
pub struct CsvDataRow {
    pub cols: Vec<CsvDataCol>,
}

pub struct CsvDataParser<'a> {
    data: &'a str,
}

impl<'a> CsvDataParser<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }

    fn map_lib_error(kind: &ErrorKind) -> CsvDataParserErr {
        match kind {
            csv::ErrorKind::UnequalLengths {
                pos,
                expected_len,
                len,
            } => CsvDataParserErr::UneqLen {
                line: pos.as_ref().map(|p| p.line() - 1),
                expected_len: *expected_len,
                actual_len: *len,
            },
            csv::ErrorKind::Utf8 { pos, err } => CsvDataParserErr::InvalidUtf8 {
                line: pos.as_ref().map(|p| p.line()),
                detail: err.to_string(),
            },
            csv::ErrorKind::Io(io_err) => CsvDataParserErr::Io {
                kind: io_err.kind().to_string(),
                detail: io_err.to_string(),
            },
            _ => CsvDataParserErr::Unknown,
        }
    }

    fn is_int(value: &str) -> bool {
        value.trim().parse::<i64>().is_ok()
    }

    fn is_float(value: &str) -> bool {
        value.trim().parse::<f64>().is_ok()
    }

    fn is_null(value: &str) -> bool {
        value.trim().to_lowercase() == Literal::NULL
    }

    fn is_bool(value: &str) -> bool {
        let value = value.trim().to_lowercase();
        value == Literal::TRUE || value == Literal::FALSE
    }

    fn infer_type(value: &str) -> CsvDataParserDataType {
        match value {
            v if Self::is_null(v) => CsvDataParserDataType::Null,
            v if Self::is_bool(v) => CsvDataParserDataType::Bool,
            v if Self::is_int(v) => CsvDataParserDataType::Int,
            v if Self::is_float(v) => CsvDataParserDataType::Float,
            _ => CsvDataParserDataType::Str,
        }
    }

    pub fn parse(&self) -> Result<Vec<CsvDataRow>, CsvDataParserErr> {
        // TODO: Check if it's possible to pre-allocate a capacity for records.
        // At this moment I know that Reader doesn't know the length of records
        // until it walks down to the last one. Maybe we can rely on some
        // rough guess but this must be investigated further.
        let mut records: Vec<CsvDataRow> = vec![];

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(self.data.as_bytes());

        let headers = reader
            .headers()
            .map_err(|err| Self::map_lib_error(err.kind()))?
            .clone();

        for (row_index, result) in reader.records().enumerate() {
            let str_record = result.map_err(|err| Self::map_lib_error(err.kind()))?;
            let mut row_record = CsvDataRow {
                cols: Vec::with_capacity(headers.len()),
            };
            for (col_index, col) in str_record.iter().enumerate() {
                let col_name = headers
                    .get(col_index)
                    .ok_or(CsvDataParserErr::MissingHeader { col: col_index })?
                    .to_string();

                row_record.cols.push(CsvDataCol {
                    name: col_name,
                    dtype: Self::infer_type(col),
                    value: col.trim().to_string(),
                    pos: Pos {
                        row: row_index,
                        col: col_index,
                    },
                });
            }
            records.push(row_record);
        }

        Ok(records)
    }
}

// ==================================================================
//
// PARSER END
//
// ==================================================================
