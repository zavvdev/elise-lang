pub fn print_error_message(message: &str, source_code: &str, char_pos: usize) {
    let lines = source_code.split("\n").collect::<Vec<&str>>();

    let mut row = 0;
    let mut col = 0;

    let mut preview_row_start = 0;
    let mut preview_row_end = 0;

    for (i, line) in lines.iter().enumerate() {
        let next_col = col + line.len();

        if next_col >= char_pos {
            row = i;
            col = line.len() - (next_col - char_pos);
            preview_row_end = next_col;

            break;
        }

        preview_row_start = col;
        col = next_col;
        continue;
    }

    let arrow = "-".repeat(col) + "^";

    println!("\n{}", message);
    println!("At {}:{}\n", row + 1, col + 1);
    println!("{}", &source_code[preview_row_start..preview_row_end]);
    println!("{}\n", arrow);
}
