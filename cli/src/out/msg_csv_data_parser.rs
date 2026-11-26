use elise_shared_diagnostics::errors_csv_data_parser::CsvDataParserErr;

use crate::out::utils;

pub fn print_err(csv_parser_err: &CsvDataParserErr) {
    use CsvDataParserErr::*;

    let (category, message): (&str, &str) = match csv_parser_err {
        UneqLen {
            line,
            expected_len,
            actual_len,
        } => {
            if let Some(line) = line {
                (
                    "TODO",
                    &format!(
                        "Unequal length at line {}.\nExpected {}, got {}.",
                        line + 1,
                        expected_len,
                        actual_len
                    ),
                )
            } else {
                (
                    "TODO",
                    &format!(
                        "Unequal length. Expected {}, got {}.",
                        expected_len, actual_len
                    ),
                )
            }
        }
        InvalidUtf8 { line, detail } => {
            if let Some(line) = line {
                (
                    "TODO",
                    &format!("Invalid utf-8 at line {}.\nDetails: {}.", line + 1, detail),
                )
            } else {
                ("TODO", &format!("Invalid utf-8.\nDetails: {}.", detail))
            }
        }
        Io { kind, detail } => ("TODO", &format!("Unable to read.\n{}, {}.", kind, detail)),
        MissingHeader { col } => ("TODO", &format!("Missing header at col: {}", col)),
        Unknown => ("TODO", "Unknown error"),
    };

    utils::print_err(category, csv_parser_err.code(), message);
}
