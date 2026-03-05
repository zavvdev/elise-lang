use std::str::from_utf8;

pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    println!("ERR: {}", info.payload_as_str().unwrap_or("UNEXPECTED"));
}

pub fn crash(message: &str) -> ! {
    panic!("{}", message);
}

pub fn error(message: &str, label: Option<&str>) {
    let label = label.unwrap_or("Error");
    println!("{}: {}", label, message);
}

pub fn print_bytecode(bytecode: &str) {
    println!("--- bytecode start ---");
    println!("{}", bytecode);
    println!("--- bytecode end ---");
}

pub fn print_execution_output(output: &str) {
    println!("{}", output);
}

pub fn print_error_source_code(message: &str, row: usize, col: usize, source_code: &[u8]) {
    let source_code = from_utf8(source_code);
    println!("\n{}", message);
    println!("At {}:{}\n", row, col);
    if source_code.is_ok() {
        println!("{}", source_code.unwrap());
    }
    println!("{}\n", "-".repeat(col) + "^");
}
