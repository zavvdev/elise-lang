use colored::Colorize;
use std::str::from_utf8;

/**
 * This function will be executed whenever we use panic! macro.
 */
pub fn panic_hook(info: &std::panic::PanicHookInfo) {
    let info = info.payload_as_str().unwrap_or("Unexpected");
    let message = format!("Fatal error: {}", info);
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
        "Error"
    };
    let error = format!("{}: {}", label.red().bold(), message);
    eprintln!("{}", error.red().bold());
}

/**
 * Print bytecode to std out.
 */
pub fn print_bytecode(bytecode: &str) {
    println!("--- Bytecode start ---\n{}\n--- Bytecode end ---", bytecode);
}

/**
 * Successful program output.
 */
pub fn print_exec_result(output: &str, ms: u128) {
    println!("Output: {}", output);
    println!("Execution time: {} ms", ms);
}

/**
 * Terminate program on specific code line:col.
 */
pub fn crash_at(message: &str, source_code: &[u8], char_pos: usize, panic_message: &str) -> ! {
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

    eprintln!("\n{}", message.red().bold());
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

    panic!("{}", panic_message)
}
