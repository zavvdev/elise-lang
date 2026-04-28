use colored::Colorize;

pub fn print_silent_err(message: &str, label: Option<&str>) {
    let label = if label.is_some() {
        label.unwrap()
    } else {
        "Error"
    };
    let error = format!("{}. {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

pub fn print_error_at_code(source_code_slice: &str, row: usize, col: usize) {
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
