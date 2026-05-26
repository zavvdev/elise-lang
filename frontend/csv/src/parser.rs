use csv::{ErrorKind, ReaderBuilder};
use elise_errors::{LangErr, errors_csv_parser::CsvParserErr};

use crate::{config::CSV_BOOL_TOKENS_LOWER, types::CsvColType};

pub struct CsvParser<'a> {
    data: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct CsvCol {
    ty: CsvColType,
    value: String,
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
pub struct CsvParserRecord {
    pub row: Vec<CsvCol>,
}

impl<'a> CsvParser<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }

    fn map_lib_error(kind: &ErrorKind) -> LangErr {
        LangErr::CsvParser(match kind {
            csv::ErrorKind::UnequalLengths {
                pos,
                expected_len,
                len,
            } => CsvParserErr::UneqLen {
                line: pos.as_ref().map(|p| p.line()),
                expected_len: *expected_len,
                actual_len: *len,
            },
            csv::ErrorKind::Utf8 { pos, err } => CsvParserErr::InvalidUtf8 {
                line: pos.as_ref().map(|p| p.line()),
                detail: err.to_string(),
            },
            csv::ErrorKind::Io(io_err) => CsvParserErr::Io {
                kind: io_err.kind().to_string(),
                detail: io_err.to_string(),
            },
            _ => CsvParserErr::Unknown,
        })
    }

    fn is_bool(value: &str) -> bool {
        let lower_value = value.to_lowercase();
        CSV_BOOL_TOKENS_LOWER.contains(&lower_value.as_str())
    }

    fn is_number(value: &str) -> bool {
        value.parse::<i64>().is_ok() || value.parse::<f64>().is_ok()
    }

    fn annotate_col(value: &str, row_index: usize, col_index: usize) -> Result<CsvCol, LangErr> {
        let mut result = CsvCol {
            ty: CsvColType::String,
            value: value.to_string(),
            row: row_index + 1,
            col: col_index + 1,
        };

        if Self::is_bool(value) {
            result.ty = CsvColType::Bool;
        }

        if Self::is_number(value) {
            result.ty = CsvColType::Number;
        }

        return Ok(result);
    }

    pub fn parse(&self) -> Result<Vec<CsvParserRecord>, LangErr> {
        let mut records: Vec<CsvParserRecord> = vec![];

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(self.data.as_bytes());

        for (row_index, result) in reader.records().enumerate() {
            let str_record = result.or_else(|err| Err(Self::map_lib_error(err.kind())))?;
            let mut row_record = CsvParserRecord { row: vec![] };
            for (col_index, col) in str_record.iter().enumerate() {
                let annotated_col = Self::annotate_col(col, row_index, col_index)?;
                row_record.row.push(annotated_col);
            }
            records.push(row_record);
        }

        Ok(records)
    }
}

// ==================================================================
//
//  TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use crate::{
        parser::{CsvCol, CsvParser, CsvParserRecord},
        types::CsvColType,
    };
    use elise_errors::{LangErr, errors_csv_parser::CsvParserErr::*};

    #[test]
    fn parse_should_return_parsed_records() {
        let data = "name,age\n\"John\",\"25\"\n\"Jane\",\"26\"";
        let parser = CsvParser::new(&data);

        let row1 = CsvParserRecord {
            row: vec![
                CsvCol {
                    value: "John".to_string(),
                    ty: CsvColType::String,
                    row: 1,
                    col: 1,
                },
                CsvCol {
                    value: "25".to_string(),
                    ty: CsvColType::Number,
                    row: 1,
                    col: 2,
                },
            ],
        };

        let row2 = CsvParserRecord {
            row: vec![
                CsvCol {
                    value: "Jane".to_string(),
                    ty: CsvColType::String,
                    row: 2,
                    col: 1,
                },
                CsvCol {
                    value: "26".to_string(),
                    ty: CsvColType::Number,
                    row: 2,
                    col: 2,
                },
            ],
        };

        assert_eq!(parser.parse(), Ok(vec![row1, row2]));
    }

    // TODO: Add tests for parsing numbers (including scientific)
    // booleans (all variants) and strings;

    #[test]
    fn parse_should_parse_empty() {
        let data = "name,age";
        let parser = CsvParser::new(&data);
        assert_eq!(parser.parse(), Ok(vec![]));
    }

    #[test]
    fn parse_should_return_uneq_len_error() {
        let data = "name,age\n\"John\"\n\"Jane\",\"26\"";
        let parser = CsvParser::new(&data);

        assert_eq!(
            parser.parse(),
            Err(LangErr::CsvParser(UneqLen {
                line: Some(2),
                expected_len: 2,
                actual_len: 1
            }))
        );
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
