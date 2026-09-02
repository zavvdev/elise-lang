use csv::{ErrorKind, ReaderBuilder};
use elise_shared::{shared_errors::errors_csv_parser::CsvParserErr, shared_types::Keyword};

// ==================================================================
//
// PARSER START
//
// ==================================================================

#[derive(PartialEq, Debug, Clone)]
pub enum CsvParserDataType {
    Int,
    Float,
    String,
    Bool,
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CsvColPos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct CsvCol {
    pub name: String,
    pub ty: CsvParserDataType,
    pub value: String,
    pub pos: CsvColPos,
}

#[derive(Debug, PartialEq)]
pub struct CsvRow {
    pub cols: Vec<CsvCol>,
}

pub struct CsvParser<'a> {
    data: &'a str,
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
                line: pos.as_ref().map(|p| p.line() - 1),
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

    fn is_int(value: &str) -> bool {
        value.trim().parse::<i64>().is_ok()
    }

    fn is_float(value: &str) -> bool {
        value.trim().parse::<f64>().is_ok()
    }

    fn is_null(value: &str) -> bool {
        value.trim().to_lowercase() == Keyword::NULL
    }

    fn is_bool(value: &str) -> bool {
        let value = value.trim().to_lowercase();
        value == Keyword::TRUE || value == Keyword::FALSE
    }

    fn infer_type(value: &str) -> CsvParserDataType {
        match value {
            v if Self::is_null(v) => CsvParserDataType::Null,
            v if Self::is_bool(v) => CsvParserDataType::Bool,
            v if Self::is_int(v) => CsvParserDataType::Int,
            v if Self::is_float(v) => CsvParserDataType::Float,
            _ => CsvParserDataType::String,
        }
    }

    pub fn parse(&self) -> Result<Vec<CsvRow>, CsvParserErr> {
        // TODO: Check if it's possible to pre-allocate a capacity for records.
        // At this moment I know that Reader doesn't know the length of records
        // until it walks down to the last one. Maybe we can rely on some
        // rough guess but this must be investigated further.
        let mut records: Vec<CsvRow> = vec![];

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(self.data.as_bytes());

        let headers = reader
            .headers()
            .map_err(|err| Self::map_lib_error(err.kind()))?
            .clone();

        for (row_index, result) in reader.records().enumerate() {
            let str_record = result.map_err(|err| Self::map_lib_error(err.kind()))?;
            let mut row_record = CsvRow {
                cols: Vec::with_capacity(headers.len()),
            };
            for (col_index, col) in str_record.iter().enumerate() {
                let col_name = headers
                    .get(col_index)
                    .ok_or(CsvParserErr::MissingHeader { col: col_index })?
                    .to_string();

                row_record.cols.push(CsvCol {
                    name: col_name,
                    ty: Self::infer_type(col),
                    value: col.trim().to_string(),
                    pos: CsvColPos {
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

// ==================================================================
//
// TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use elise_shared::shared_errors::errors_csv_parser::CsvParserErr::*;

    use crate::csv::csv_parser::{CsvCol, CsvColPos, CsvParser, CsvParserDataType, CsvRow};

    fn build_csv_header(index: usize) -> String {
        format!("n{}", index)
    }

    fn build_csv(row: &Vec<&str>) -> String {
        let head: Vec<String> = (0..row.len()).map(|i| build_csv_header(i)).collect();
        format!("{}\n{}", head.join(","), row.join(","))
    }

    // ==================================================================
    // NUMBER TESTS START
    // ==================================================================

    #[test]
    fn should_parse_int() {
        let row = vec!["42", "-42", "0", "-0", "9999999"];
        let csv = build_csv(&row);
        let parser = CsvParser::new(&csv);

        let result = CsvRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    name: build_csv_header(i),
                    value: n.to_string(),
                    ty: CsvParserDataType::Int,
                    pos: CsvColPos { row: 0, col: i },
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    #[test]
    fn should_parse_float() {
        let row = vec![
            "0.0",
            "-0.0",
            "0.1",
            "4.2",
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

        let csv = build_csv(&row);
        let parser = CsvParser::new(&csv);

        let result = CsvRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    name: build_csv_header(i),
                    value: n.to_string(),
                    ty: CsvParserDataType::Float,
                    pos: CsvColPos { row: 0, col: i },
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    // ==================================================================
    // NUMBER TESTS END
    // ==================================================================

    // ==================================================================
    // BOOLEAN TESTS START
    // ==================================================================

    #[test]
    fn should_parse_bool() {
        let row = vec!["true", "True", "TRUE", "false", "False", "FALSE"];
        let csv = build_csv(&row);
        let parser = CsvParser::new(&csv);

        let result = CsvRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    name: build_csv_header(i),
                    value: n.to_string(),
                    ty: CsvParserDataType::Bool,
                    pos: CsvColPos { row: 0, col: i },
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    // ==================================================================
    // BOOLEAN TESTS END
    // ==================================================================

    // ==================================================================
    // STRING TESTS START
    // ==================================================================

    #[test]
    fn should_parse_string() {
        let row = vec!["john", " ", "", "     "];
        let csv = build_csv(&row);
        let parser = CsvParser::new(&csv);

        assert_eq!(
            parser.parse(),
            Ok(vec![CsvRow {
                cols: vec![
                    CsvCol {
                        name: build_csv_header(0),
                        value: "john".to_string(),
                        ty: CsvParserDataType::String,
                        pos: CsvColPos { row: 0, col: 0 },
                    },
                    CsvCol {
                        name: build_csv_header(1),
                        value: "".to_string(),
                        ty: CsvParserDataType::String,
                        pos: CsvColPos { row: 0, col: 1 },
                    },
                    CsvCol {
                        name: build_csv_header(2),
                        value: "".to_string(),
                        ty: CsvParserDataType::String,
                        pos: CsvColPos { row: 0, col: 2 },
                    },
                    CsvCol {
                        name: build_csv_header(3),
                        value: "".to_string(),
                        ty: CsvParserDataType::String,
                        pos: CsvColPos { row: 0, col: 3 },
                    }
                ],
            }])
        );
    }

    // ==================================================================
    // STRING TESTS END
    // ==================================================================

    // ==================================================================
    // NULL TESTS START
    // ==================================================================

    #[test]
    fn should_parse_null() {
        let row = vec!["null", "NULL", "Null"];
        let csv = build_csv(&row);
        let parser = CsvParser::new(&csv);

        let result = CsvRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    name: build_csv_header(i),
                    value: n.trim().to_string(),
                    ty: CsvParserDataType::Null,
                    pos: CsvColPos { row: 0, col: i },
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    #[test]
    fn should_parse_empty_csv() {
        let data = "name,age";
        let parser = CsvParser::new(&data);
        assert_eq!(parser.parse(), Ok(vec![]));
    }

    // ==================================================================
    // NULL TESTS START
    // ==================================================================

    // ==================================================================
    // MISC TESTS START
    // ==================================================================

    #[test]
    fn should_trim_values() {
        let row = vec![" 12.3  ", "  12 ", "  S  ", "  Null ", "   "];

        let types = vec![
            CsvParserDataType::Float,
            CsvParserDataType::Int,
            CsvParserDataType::String,
            CsvParserDataType::Null,
            CsvParserDataType::String,
        ];

        let csv = build_csv(&row);
        let parser = CsvParser::new(&csv);

        let result = CsvRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvCol {
                    name: build_csv_header(i),
                    value: n.trim().to_string(),
                    ty: types.get(i).unwrap().clone(),
                    pos: CsvColPos { row: 0, col: i },
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    // ==================================================================
    // MISC TESTS END
    // ==================================================================

    // ==================================================================
    // ERROR TESTS START
    // ==================================================================

    #[test]
    fn should_return_uneq_len_error() {
        let data = "name,age\n\"John\"\n\"Jane\",\"26\"";
        let parser = CsvParser::new(&data);

        assert_eq!(
            parser.parse(),
            Err(UneqLen {
                line: Some(1),
                expected_len: 2,
                actual_len: 1
            })
        );
    }

    // ==================================================================
    // ERROR TESTS END
    // ==================================================================
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
