use std::str::from_utf8;

use colored::Colorize;

pub struct SourceCodeSlice {
    pub slice: String,
    pub row: usize,
    pub col: usize,
}

/// Slices the source code in order to preview an error at `char_pos`.
pub fn get_source_code_slice(source_code: &[u8], char_pos: usize) -> Option<SourceCodeSlice> {
    if source_code.is_empty() {
        return None;
    }

    let mut row = 0;
    let mut col = 0;

    let mut previous_row_start = 0;
    let mut preview_row_start = 0;
    let mut preview_row_end = 0;

    let mut found = false;

    for c in source_code {
        if preview_row_end == char_pos {
            found = true;
        }

        preview_row_end += 1;

        if *c == b'\n' {
            if found {
                break;
            }

            previous_row_start = preview_row_start;
            preview_row_start = preview_row_end;

            row += 1;
            col = 0;
        } else if !found {
            col += 1;
        }
    }

    if let Ok(source_code) = from_utf8(source_code) {
        return Some(SourceCodeSlice {
            slice: source_code[previous_row_start..preview_row_end].to_string(),
            row: row + 1,
            col: col + 1,
        });
    }

    None
}

pub fn print_err(message: &str, label: Option<&str>) {
    let label: &str = label.unwrap_or("Error");
    let error = format!("{}. {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

pub fn print_error_source_code_pos(row: usize, col: usize) {
    let location = format!("At {}:{}\n", row, col);
    eprintln!("{}", location.bold());
}

pub fn print_error_source_code_slice(source_code_slice: &str, col: usize) {
    if !source_code_slice.is_empty() {
        eprintln!("{}", source_code_slice);
        let arrow = "-".repeat(col - 1) + "^";
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
