// Messages

use crate::out;

pub const M_INVALID_NUMBER: &str = "Invalid Number";

// Message formatter

pub fn error_at_char_pos(message: &str, source_code: &[u8], char_pos: usize) -> ! {
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

    out::print_error_source_code(
        message,
        row + 1,
        col + 1,
        &source_code[previous_row_start..preview_row_end],
    );

    panic!()
}
