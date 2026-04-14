use colored::Colorize;

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

fn crash_at(message: &str, source_code_slice: Option<String>, row: usize, col: usize) -> ! {
    if let Some(code) = source_code_slice {
        let location = format!("At {}:{}\n", row, col);
        eprintln!("{}", location.bold());
        eprintln!("{}", code);
        let arrow = "-".repeat(col) + "^";
        eprintln!("{}\n", arrow.red().bold());
    }
    panic!("{}", message)
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
    println!("Saved to: {}", path);
    println!("Execution time: {} ms", ms);
}

pub fn validate_result(ms: u128) {
    println!("Valid");
    println!("Execution time: {} ms", ms);
}
