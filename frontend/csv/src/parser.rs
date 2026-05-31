use csv::{ErrorKind, ReaderBuilder};
use elise_errors::errors_csv_parser::CsvParserErr;
use elise_types::DataSourceFieldType;

use crate::config::CSV_BOOL_TOKENS_LOWER;

pub struct CsvParser<'a> {
    data: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct CsvCol {
    ty: DataSourceFieldType,
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

    fn map_lib_error(kind: &ErrorKind) -> CsvParserErr {
        match kind {
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
        }
    }

    fn is_bool(value: &str) -> bool {
        let lower_value = value.to_lowercase();
        CSV_BOOL_TOKENS_LOWER.contains(&lower_value.as_str())
    }

    fn is_number(value: &str) -> bool {
        value.parse::<i64>().is_ok() || value.parse::<f64>().is_ok()
    }

    fn annotate_col(
        value: &str,
        row_index: usize,
        col_index: usize,
    ) -> Result<CsvCol, CsvParserErr> {
        let mut result = CsvCol {
            ty: DataSourceFieldType::String,
            value: value.to_string(),
            row: row_index + 1,
            col: col_index + 1,
        };

        if Self::is_bool(value) {
            result.ty = DataSourceFieldType::Bool;
        }

        if Self::is_number(value) {
            result.ty = DataSourceFieldType::Number;
        }

        Ok(result)
    }

    pub fn parse(&self) -> Result<Vec<CsvParserRecord>, CsvParserErr> {
        let mut records: Vec<CsvParserRecord> = vec![];

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(self.data.as_bytes());

        for (row_index, result) in reader.records().enumerate() {
            let str_record = result.map_err(|err| Self::map_lib_error(err.kind()))?;
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
    use crate::parser::{CsvCol, CsvParser, CsvParserRecord};
    use elise_errors::errors_csv_parser::CsvParserErr::*;
    use elise_types::DataSourceFieldType;

    #[test]
    fn parse_should_parse_number() {
        let row = vec![
            "42",
            "4.2",
            "-42",
            "-4.2",
            "1e3",
            "1E-3",
            "1.5e10",
            "1.504E101",
            "-1e3",
            "-1E-3",
            "-1.5e10",
            "-1.504E101",
        ];

        let head: Vec<String> = (0..row.len()).map(|i| format!("n{}", i)).collect();

        let csv = format!("{}\n{}", head.join(","), row.join(","));
        let parser = CsvParser::new(&csv);

        let result = CsvParserRecord {
            row: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    value: n.to_string(),
                    ty: DataSourceFieldType::Number,
                    row: 1,
                    col: i + 1,
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    #[test]
    fn parse_should_parse_bool() {
        let row = vec![
            "true", "True", "TRUE", "false", "False", "FALSE", "yes", "Yes", "YES", "no", "No",
            "NO", "on", "On", "ON", "off", "Off", "OFF", "y", "Y", "n", "N",
        ];

        let head: Vec<String> = (0..row.len()).map(|i| format!("n{}", i)).collect();

        let csv = format!("{}\n{}", head.join(","), row.join(","));
        let parser = CsvParser::new(&csv);

        let result = CsvParserRecord {
            row: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    value: n.to_string(),
                    ty: DataSourceFieldType::Bool,
                    row: 1,
                    col: i + 1,
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    #[test]
    fn parse_should_parse_string() {
        let data = "name\n\"John\"";
        let parser = CsvParser::new(&data);

        assert_eq!(
            parser.parse(),
            Ok(vec![CsvParserRecord {
                row: vec![CsvCol {
                    value: "John".to_string(),
                    ty: DataSourceFieldType::String,
                    row: 1,
                    col: 1,
                }],
            }])
        );
    }

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
            Err(UneqLen {
                line: Some(2),
                expected_len: 2,
                actual_len: 1
            })
        );
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
