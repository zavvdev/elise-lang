use csv::{ErrorKind, ReaderBuilder};
use elise_shared::{
    shared_errors::errors_csv_data_parser::CsvDataParserErr,
    shared_types::{Keyword, Pos},
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
    String,
    Bool,
    Null,
}

#[derive(Debug, PartialEq)]
pub struct CsvDataCol {
    pub name: String,
    pub ty: CsvDataParserDataType,
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
        value.trim().to_lowercase() == Keyword::NULL
    }

    fn is_bool(value: &str) -> bool {
        let value = value.trim().to_lowercase();
        value == Keyword::TRUE || value == Keyword::FALSE
    }

    fn infer_type(value: &str) -> CsvDataParserDataType {
        match value {
            v if Self::is_null(v) => CsvDataParserDataType::Null,
            v if Self::is_bool(v) => CsvDataParserDataType::Bool,
            v if Self::is_int(v) => CsvDataParserDataType::Int,
            v if Self::is_float(v) => CsvDataParserDataType::Float,
            _ => CsvDataParserDataType::String,
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
                    ty: Self::infer_type(col),
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

// ==================================================================
//
// TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use elise_shared::{
        shared_errors::errors_csv_data_parser::CsvDataParserErr::*, shared_types::Pos,
    };

    use crate::csv::csv_data_parser::{
        CsvDataCol, CsvDataParser, CsvDataParserDataType, CsvDataRow,
    };

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
        let parser = CsvDataParser::new(&csv);

        let result = CsvDataRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvDataCol {
                    name: build_csv_header(i),
                    value: n.to_string(),
                    ty: CsvDataParserDataType::Int,
                    pos: Pos { row: 0, col: i },
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
        let parser = CsvDataParser::new(&csv);

        let result = CsvDataRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvDataCol {
                    name: build_csv_header(i),
                    value: n.to_string(),
                    ty: CsvDataParserDataType::Float,
                    pos: Pos { row: 0, col: i },
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
        let parser = CsvDataParser::new(&csv);

        let result = CsvDataRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvDataCol {
                    name: build_csv_header(i),
                    value: n.to_string(),
                    ty: CsvDataParserDataType::Bool,
                    pos: Pos { row: 0, col: i },
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
        let parser = CsvDataParser::new(&csv);

        assert_eq!(
            parser.parse(),
            Ok(vec![CsvDataRow {
                cols: vec![
                    CsvDataCol {
                        name: build_csv_header(0),
                        value: "john".to_string(),
                        ty: CsvDataParserDataType::String,
                        pos: Pos { row: 0, col: 0 },
                    },
                    CsvDataCol {
                        name: build_csv_header(1),
                        value: "".to_string(),
                        ty: CsvDataParserDataType::String,
                        pos: Pos { row: 0, col: 1 },
                    },
                    CsvDataCol {
                        name: build_csv_header(2),
                        value: "".to_string(),
                        ty: CsvDataParserDataType::String,
                        pos: Pos { row: 0, col: 2 },
                    },
                    CsvDataCol {
                        name: build_csv_header(3),
                        value: "".to_string(),
                        ty: CsvDataParserDataType::String,
                        pos: Pos { row: 0, col: 3 },
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
        let parser = CsvDataParser::new(&csv);

        let result = CsvDataRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvDataCol {
                    name: build_csv_header(i),
                    value: n.trim().to_string(),
                    ty: CsvDataParserDataType::Null,
                    pos: Pos { row: 0, col: i },
                })
                .collect(),
        };

        assert_eq!(parser.parse(), Ok(vec![result]));
    }

    #[test]
    fn should_parse_empty_csv() {
        let data = "name,age";
        let parser = CsvDataParser::new(&data);
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
            CsvDataParserDataType::Float,
            CsvDataParserDataType::Int,
            CsvDataParserDataType::String,
            CsvDataParserDataType::Null,
            CsvDataParserDataType::String,
        ];

        let csv = build_csv(&row);
        let parser = CsvDataParser::new(&csv);

        let result = CsvDataRow {
            cols: row
                .iter()
                .enumerate()
                .map(|(i, n)| CsvDataCol {
                    name: build_csv_header(i),
                    value: n.trim().to_string(),
                    ty: types.get(i).unwrap().clone(),
                    pos: Pos { row: 0, col: i },
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
        let parser = CsvDataParser::new(&data);

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
