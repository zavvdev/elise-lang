pub mod messages;

use colored::Colorize;
use messages::{
    M_ERROR_FATAL, M_ERROR_FILE_READER, M_ERROR_FILE_WRITER, M_ERROR_SILENT, M_ERROR_UNEXPECTED,
    M_INFO_BYTECODE_END, M_INFO_BYTECODE_START, M_INFO_EXEC_TIME, M_INFO_EXEC_TIME_TYPE,
    M_INFO_OUTPUT, M_INFO_SAVED_TO,
};
use std::str::from_utf8;

/**
 * This function will be executed whenever we use panic! macro.
 */
pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    let info = info.payload_as_str().unwrap_or(M_ERROR_UNEXPECTED);
    let message = format!("{}: {}", M_ERROR_FATAL, info);
    eprintln!("{}", message.red().bold());
}

/**
 * Use this function when you want to terminate program execution
 * due to some error.
 */
pub fn crash(message: &str) -> ! {
    panic!("{}", message);
}

/**
 * Use this function when you want to show an error message
 * without terminating the program.
 */
pub fn silent_error(message: &str, label: Option<&str>) {
    let label = if label.is_some() {
        label.unwrap()
    } else {
        M_ERROR_SILENT
    };
    let error = format!("{}: {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

/**
 * Print bytecode to std out.
 */
pub fn print_bytecode(bytecode: &str) {
    println!(
        "{}\n{}\n{}",
        M_INFO_BYTECODE_START, bytecode, M_INFO_BYTECODE_END
    );
}

pub fn print_file_reader_error(msg: &str, path: &str) {
    silent_error(&format!("{} ({})", msg, path), Some(M_ERROR_FILE_READER));
}

pub fn print_file_writer_error(msg: &str, path: &str) {
    silent_error(&format!("{} ({})", msg, path), Some(M_ERROR_FILE_WRITER));
}

pub fn print_run_result(output: &str, ms: u128) {
    println!("{}: {}", M_INFO_OUTPUT, output);
    println!("{}: {} {}", M_INFO_EXEC_TIME, ms, M_INFO_EXEC_TIME_TYPE);
}

pub fn print_build_result(path: &str, ms: u128) {
    println!("{}: {}", M_INFO_SAVED_TO, path);
    println!("{}: {} {}", M_INFO_EXEC_TIME, ms, M_INFO_EXEC_TIME_TYPE);
}

/**
 * Terminate program on specific code line:col.
 */
pub fn crash_at(message: &str, source_code: &[u8], char_pos: usize) -> ! {
    let mut row = 0;
    let mut col = 0;

    let mut previous_row_start = 0;
    let mut preview_row_start = 0;
    let mut preview_row_end = 0;

    let mut found = false;

    for char in source_code {
        if preview_row_end == char_pos {
            found = true;
        }

        preview_row_end += 1;

        if *char == b'\n' {
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

    let source_code = from_utf8(source_code);

    let location = format!("At {}:{}\n", row + 1, col + 1);
    eprintln!("{}", location.bold());

    if source_code.is_ok() {
        eprintln!(
            "{}",
            &source_code.unwrap()[previous_row_start..preview_row_end]
        );
        let arrow = "-".repeat(col) + "^";
        eprintln!("{}\n", arrow.red().bold());
    }

    panic!("{}", message)
}
