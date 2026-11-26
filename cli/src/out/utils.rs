use std::str::from_utf8;

use colored::Colorize;
use elise_shared_diagnostics::codes::DiagnosticCodeType;
use elise_shared_types::TablePos;

struct SourceCodeSlice {
    pub slice: String,
    pub pos: TablePos,
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
        pos: TablePos {
            row: error_row,
            col: error_col,
        },
    })
}

pub fn print_err_source_code_pos(table_pos: TablePos, file_name: &str) {
    let location = format!(
        "@ {}:{}:{}\n",
        file_name,
        table_pos.row + 1,
        table_pos.col + 1
    );
    eprintln!("{}", location.red());
}

pub fn print_source_code_slice(source_code_slice: &str, col: usize) {
    if !source_code_slice.is_empty() {
        eprintln!("{}", source_code_slice);
        let arrow = "-".repeat(col) + "^";
        eprintln!("{}\n", arrow.red());
    }
}

pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    let info = info.payload_as_str().unwrap_or("Unexpected");
    let message = format!("{}: {}", "Fatal Error", info);
    eprintln!("{}", message.red());
}

pub fn print_bytecode(bytecode: &str) {
    println!("--- Bytecode start ---\n{}\n--- Bytecode end ---", bytecode);
}

pub fn print_err(category: &str, diag_code: DiagnosticCodeType, message: &str) {
    // ex: Syntax Error E101: Something went wrong
    let error = format!("{} E{}: {}", category, diag_code, message);
    eprintln!("{}", error.red());
}

pub fn print_saved_to(path: &str) {
    println!("Saved to: {}", path);
}
