pub mod messages;

use colored::Colorize;
use messages::{
    M_ERROR_CONFIG, M_ERROR_FATAL, M_ERROR_FILE_READER, M_ERROR_FILE_WRITER, M_ERROR_SILENT,
    M_ERROR_UNEXPECTED, M_INFO_BYTECODE_END, M_INFO_BYTECODE_START, M_INFO_EXEC_TIME,
    M_INFO_EXEC_TIME_TYPE, M_INFO_OUTPUT, M_INFO_SAVED_TO, M_INFO_VALID,
};
use std::str::from_utf8;

pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    let info = info.payload_as_str().unwrap_or(M_ERROR_UNEXPECTED);
    let message = format!("{}: {}", M_ERROR_FATAL, info);
    eprintln!("{}", message.red().bold());
}

pub fn crash(message: &str) -> ! {
    panic!("{}", message);
}

fn silent_error(message: &str, label: Option<&str>) {
    let label = if label.is_some() {
        label.unwrap()
    } else {
        M_ERROR_SILENT
    };
    let error = format!("{}: {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

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

pub fn print_validate_result(ms: u128) {
    println!("{}", M_INFO_VALID);
    println!("{}: {} {}", M_INFO_EXEC_TIME, ms, M_INFO_EXEC_TIME_TYPE);
}

pub fn config_error(msg: &str) {
    silent_error(&format!("{}", msg), Some(M_ERROR_CONFIG))
}

pub fn crash_at(message: &str, source_code_slice: Option<String>, row: usize, col: usize) -> ! {
    if let Some(code) = source_code_slice {
        let location = format!("At {}:{}\n", row, col);
        eprintln!("{}", location.bold());
        eprintln!("{}", source_code_slice);
        let arrow = "-".repeat(col) + "^";
        eprintln!("{}\n", arrow.red().bold());
    }
    panic!("{}", message)
}
