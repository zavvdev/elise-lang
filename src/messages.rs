fn get_arrow(len: usize) -> String {
    "-".repeat(len) + "^"
}

pub fn print_error_message(message: &str, source_code: &str, char_pos: usize) {
    let mut row = 0;
    let mut col = 0;

    let mut previous_row_start = 0;
    let mut preview_row_start = 0;
    let mut preview_row_end = 0;

    let mut found = false;

    for char in source_code.chars() {
        if preview_row_end == char_pos {
            found = true;
        }

        preview_row_end += 1;

        if char == '\n' {
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

    println!("\n{}", message);
    println!("At {}:{}\n", row + 1, col + 1);
    println!("{}", &source_code[previous_row_start..preview_row_end]);
    println!("{}\n", get_arrow(col));
}
