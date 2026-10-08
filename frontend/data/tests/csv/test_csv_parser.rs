mod common;

use elise_shared::{shared_errors::errors_csv_data_parser::CsvDataParserErr::*, shared_types::Pos};
use elise_test_utils::test_utils;

use crate::csv::csv_data_parser::{CsvDataCol, CsvDataParser, CsvDataParserDataType, CsvDataRow};

// ==================================================================
// NUMBER TESTS START
// ==================================================================

#[test]
fn should_parse_int() {
    let row = vec!["42", "-42", "0", "-0", "9999999"];
    let csv = test_utils::csv::build(&vec![&row]);
    let parser = CsvDataParser::new(&csv);

    let result = CsvDataRow {
        cols: row
            .iter()
            .enumerate()
            .map(|(i, n)| CsvDataCol {
                name: test_utils::csv::build_header(i),
                value: n.to_string(),
                dtype: CsvDataParserDataType::Int,
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

    let csv = test_utils::csv::build(&vec![&row]);
    let parser = CsvDataParser::new(&csv);

    let result = CsvDataRow {
        cols: row
            .iter()
            .enumerate()
            .map(|(i, n)| CsvDataCol {
                name: test_utils::csv::build_header(i),
                value: n.to_string(),
                dtype: CsvDataParserDataType::Float,
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
    let csv = test_utils::csv::build(&vec![&row]);
    let parser = CsvDataParser::new(&csv);

    let result = CsvDataRow {
        cols: row
            .iter()
            .enumerate()
            .map(|(i, n)| CsvDataCol {
                name: test_utils::csv::build_header(i),
                value: n.to_string(),
                dtype: CsvDataParserDataType::Bool,
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
    let csv = test_utils::csv::build(&vec![&row]);
    let parser = CsvDataParser::new(&csv);

    assert_eq!(
        parser.parse(),
        Ok(vec![CsvDataRow {
            cols: vec![
                CsvDataCol {
                    name: test_utils::csv::build_header(0),
                    value: "john".to_string(),
                    dtype: CsvDataParserDataType::Str,
                    pos: Pos { row: 0, col: 0 },
                },
                CsvDataCol {
                    name: test_utils::csv::build_header(1),
                    value: "".to_string(),
                    dtype: CsvDataParserDataType::Str,
                    pos: Pos { row: 0, col: 1 },
                },
                CsvDataCol {
                    name: test_utils::csv::build_header(2),
                    value: "".to_string(),
                    dtype: CsvDataParserDataType::Str,
                    pos: Pos { row: 0, col: 2 },
                },
                CsvDataCol {
                    name: test_utils::csv::build_header(3),
                    value: "".to_string(),
                    dtype: CsvDataParserDataType::Str,
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
    let csv = test_utils::csv::build(&vec![&row]);
    let parser = CsvDataParser::new(&csv);

    let result = CsvDataRow {
        cols: row
            .iter()
            .enumerate()
            .map(|(i, n)| CsvDataCol {
                name: test_utils::csv::build_header(i),
                value: n.trim().to_string(),
                dtype: CsvDataParserDataType::Null,
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
        CsvDataParserDataType::Str,
        CsvDataParserDataType::Null,
        CsvDataParserDataType::Str,
    ];

    let csv = test_utils::csv::build(&vec![&row]);
    let parser = CsvDataParser::new(&csv);

    let result = CsvDataRow {
        cols: row
            .iter()
            .enumerate()
            .map(|(i, n)| CsvDataCol {
                name: test_utils::csv::build_header(i),
                value: n.trim().to_string(),
                dtype: types.get(i).unwrap().clone(),
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
