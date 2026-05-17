use elise_errors::errors_csv_parser::CsvParserErr;

use crate::out::utils::print_silent_err;

pub fn print_err(csv_parser_err: &CsvParserErr) {
    use CsvParserErr::*;

    let info: String = match csv_parser_err {
        UneqLen {
            pos,
            expected_len,
            actual_len,
        } => {
            if let Some(pos) = pos {
                format!(
                    "Unequal length at line {}.\nExpected {}, got {}.",
                    pos.line, expected_len, actual_len
                )
            } else {
                format!(
                    "Unequal length. Expected {}, got {}.",
                    expected_len, actual_len
                )
            }
        }
        InvalidUtf8 { pos, detail } => {
            if let Some(pos) = pos {
                format!("Invalid utf-8 at line {}.\nDetails: {}.", pos.line, detail)
            } else {
                format!("Invalid utf-8.\nDetails: {}.", detail)
            }
        }
        Io { kind, detail } => format!("Unable to read.\n{}, {}.", kind, detail),
        Unknown => "Unknown failure.".to_string(),
    };

    print_silent_err(&info, Some("Csv parser error"));
}
