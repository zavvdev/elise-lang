use elise_errors::errors_csv_parser::CsvParserErr;

use crate::out::utils;

pub fn print_err(csv_parser_err: &CsvParserErr) {
    use CsvParserErr::*;

    let info: String = match csv_parser_err {
        UneqLen {
            line,
            expected_len,
            actual_len,
        } => {
            if let Some(line) = line {
                format!(
                    "Unequal length at line {}.\nExpected {}, got {}.",
                    line + 1, expected_len, actual_len
                )
            } else {
                format!(
                    "Unequal length. Expected {}, got {}.",
                    expected_len, actual_len
                )
            }
        }
        InvalidUtf8 { line, detail } => {
            if let Some(line) = line {
                format!("Invalid utf-8 at line {}.\nDetails: {}.", line + 1, detail)
            } else {
                format!("Invalid utf-8.\nDetails: {}.", detail)
            }
        }
        Io { kind, detail } => format!("Unable to read.\n{}, {}.", kind, detail),
        Unknown => "Unknown failure.".to_string(),
    };

    utils::print_err(&info, Some("Csv parser error"));
}
