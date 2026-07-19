use std::str::from_utf8;

use colored::Colorize;
use elise_shared_types::DataSourceFieldType;

pub struct SourceCodeSlice {
    pub slice: String,
    pub row: usize,
    pub col: usize,
}

/// Slices the source code in order to preview an error at `byte_pos`.
pub fn get_source_code_slice(source_code: &[u8], byte_pos: usize) -> Option<SourceCodeSlice> {
    // The idea is to show a source code slice for the user with the
    // exact position that caused error. You can see an example below:
    //
    // At 4:14
    //  .fn(allowance-predicate [row]
    //    .eq(.get(1row, "can-smoke")), true))
    //
    // -------------^
    //
    // Instead of showing a slice from start of the row to the `byte_pos`,
    // we also preview one row above + current row until the end.
    if source_code.is_empty() {
        return None;
    }

    // Early from_utf8 conversion done once upfront so we can iterate chars.
    // This also lets us return None early if the input isn't valid UTF-8.
    let source_str = from_utf8(source_code).ok()?;

    // Row number at which we have an error.
    let mut error_row: usize = 0;
    // Character column, not byte offset.
    let mut error_col: usize = 0;

    // Byte offset where the line preceding the error line starts.
    let mut preceding_line_start: usize = 0;
    // Byte offset where the error line starts.
    let mut error_line_start: usize = 0;
    // Byte offset where the error line ends (advances as we scan).
    let mut error_line_end: usize = 0;

    let mut error_pos_found = false;

    // Iterate over chars in order to preserve Unicode characters
    // even if they consist of more than one byte.
    // So `ch` here is one visible character.
    for ch in source_str.chars() {
        if error_line_end == byte_pos {
            error_pos_found = true;
        }
        // len_utf8 returns the number of bytes this `ch` would need if encoded in UTF-8.
        // That number of bytes is always between 1 and 4, inclusive.
        let ch_byte_len = ch.len_utf8();
        error_line_end += ch_byte_len;

        if ch == '\n' {
            if error_pos_found {
                break;
            }
            preceding_line_start = error_line_start;
            error_line_start = error_line_end;
            error_row += 1;
            error_col = 0;
        } else if !error_pos_found {
            // Count characters, not bytes.
            error_col += 1;
        }
    }

    Some(SourceCodeSlice {
        slice: source_str[preceding_line_start..error_line_end].to_string(),
        row: error_row,
        col: error_col,
    })
}

fn match_data_type(ty: &DataSourceFieldType) -> String {
    match ty {
        DataSourceFieldType::Number => "Number".to_string(),
        DataSourceFieldType::String => "String".to_string(),
        DataSourceFieldType::Bool => "Boolean".to_string(),
        DataSourceFieldType::Empty => "Empty".to_string(),
    }
}

pub fn print_err(message: &str, label: Option<&str>) {
    let label: &str = label.unwrap_or("Error");
    let error = format!("{}. {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

pub fn print_err_source_code_pos(row: usize, col: usize) {
    let location = format!("At {}:{}\n", row + 1, col + 1);
    eprintln!("{}", location.bold());
}

pub fn print_err_type_mismatch(expected: &DataSourceFieldType, got: &DataSourceFieldType) {
    let exp_type = match_data_type(expected);
    let got_type = match_data_type(got);
    let msg = format!("Expected: {}, got: {}\n", exp_type, got_type);
    eprintln!("{}", msg.bold());
}

pub fn print_err_source_code_slice(source_code_slice: &str, col: usize) {
    if !source_code_slice.is_empty() {
        eprintln!("{}", source_code_slice);
        let arrow = "-".repeat(col) + "^";
        eprintln!("{}\n", arrow.red().bold());
    }
}

pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    let info = info.payload_as_str().unwrap_or("Unexpected error");
    let message = format!("{}: {}", "Fatal error", info);
    eprintln!("{}", message.red().bold());
}

pub fn print_bytecode(bytecode: &str) {
    println!("--- Bytecode start ---\n{}\n--- Bytecode end ---", bytecode);
}
