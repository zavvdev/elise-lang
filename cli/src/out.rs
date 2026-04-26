/**
 * This file must contain anything that is related to displaying messages
 * for a user in case of errors or other cases which requires output.
 * This is the only file that should be responsible for it.
 */
use colored::Colorize;
use elise_shared::errors::errors_csv_parser::CsvParserErr;
use elise_shared::errors::errors_sc_parser::ScParserErr;

// ==========================
//
// SHARED HANDLERS
//
// ==========================

fn silent_err(message: &str, label: Option<&str>) {
    let label = if label.is_some() {
        label.unwrap()
    } else {
        "Error"
    };
    let error = format!("{}. {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

fn error_at_code(source_code_slice: &str, row: usize, col: usize) {
    if source_code_slice.len() > 0 {
        let location = format!("At {}:{}\n", row, col);
        eprintln!("{}", location.bold());
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
    println!(
        "{}\n{}\n{}",
        "--- Bytecode start ---", bytecode, "--- Bytecode end ---"
    );
}

// ==========================
//
// CONFIG HANDLERS
//
// ==========================

fn conf_err(msg: &str) {
    silent_err(msg, Some("Config error"));
}

pub fn conf_err_ext_invalid(invalid_ext: &str) {
    conf_err(&format!("Invalid extension: '{}'", invalid_ext));
}

pub fn conf_err_arg_invalid(invalid_arg: &str, expected: &str) {
    conf_err(&format!(
        "Invalid argument. Expected '{}', got '{}'",
        invalid_arg, expected
    ));
}

pub fn conf_err_arg_required(required_arg: &str) {
    conf_err(&format!("Argument required: '{}'", required_arg));
}

// ==========================
//
// FILE SYSTEM HANDLERS
//
// ==========================

pub fn fsys_file_reader_err(msg: &str, path: &str) {
    silent_err(&format!("{} ({})", msg, path), Some("File reader error"));
}

pub fn fsys_file_writer_err(msg: &str, path: &str) {
    silent_err(&format!("{} ({})", msg, path), Some("File writer error"));
}

pub fn fsys_saved_to(path: &str) {
    println!("Saved to: {}", path);
}

// ==========================
//
// EXEC HANDLERS
//
// ==========================

pub fn run_result(output: &str, ms: u128) {
    println!("Output: {}", output);
    println!("Execution time: {} ms", ms);
}

pub fn build_result(path: &str, ms: u128) {
    fsys_saved_to(path);
    println!("Execution time: {} ms", ms);
}

pub fn validate_result(ms: u128) {
    println!("Valid");
    println!("Execution time: {} ms", ms);
}

// ==========================
//
// SC PARSER ERROR HANDLER
//
// ==========================

pub fn sc_parser_err(parser_err: &ScParserErr) {
    use ScParserErr::*;

    let info = match parser_err {
        UnexpTok(err_info) => ("Unexpected token", err_info),
        UnexpEoFile(err_info) => ("Unexpected end of file", err_info),
        UnexpEoList(err_info) => ("Unexpected end of list", err_info),
        UnexpEoDict(err_info) => ("Unexpected end of dictionary", err_info),
        UnexpEoFn(err_info) => ("Unexpected end of function", err_info),
        UnexpDictKey(err_info) => ("Unexpected dictionary key", err_info),
        InvalNum(err_info) => ("Invalid number", err_info),
        InvalStr(err_info) => ("Invalid string", err_info),
        InvalDictPair(err_info) => ("Invalid dictionary key value pair", err_info),
        InvalFnName(err_info) => ("Invalid function name", err_info),
    };

    let source_code = match &info.1.source_code_slice {
        Some(code) => code,
        None => "",
    };

    silent_err(info.0, Some("Parser error"));
    error_at_code(source_code, info.1.row, info.1.col);
}

// ==========================
//
// CSV PARSER ERROR HANDLER
//
// ==========================

pub fn csv_parser_err(_parser_err: &CsvParserErr) {
    // TODO: handle info
}
