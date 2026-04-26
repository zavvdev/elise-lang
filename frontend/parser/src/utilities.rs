use std::str::from_utf8;

pub struct SourceCodeSlice {
    pub slice: String,
    pub row: usize,
    pub col: usize,
}

// Get source code substring for error report.
pub fn get_source_code_slice(source_code: &[u8], char_pos: usize) -> Result<SourceCodeSlice, ()> {
    let mut row = 0;
    let mut col = 0;

    let mut previous_row_start = 0;
    let mut preview_row_start = 0;
    let mut preview_row_end = 0;

    let mut found = false;

    for c in source_code {
        if preview_row_end == char_pos {
            found = true;
        }

        preview_row_end += 1;

        if *c == b'\n' {
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

    if source_code.is_ok() {
        return Ok(SourceCodeSlice {
            slice: source_code.unwrap()[previous_row_start..preview_row_end].to_string(),
            row: row + 1,
            col: col + 1,
        });
    }

    Err(())
}
