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

pub fn crash_at_token_pos(
    message: &str,
    source_code: &[u8],
    char_pos: usize,
    panic_message: &str,
) -> ! {
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

    println!("\n{}", message);
    println!("At {}:{}\n", row + 1, col + 1);

    if source_code.is_ok() {
        println!(
            "{}",
            &source_code.unwrap()[previous_row_start..preview_row_end]
        );
        println!("{}\n", "-".repeat(col) + "^");
    }

    panic!("{}", panic_message)
}
