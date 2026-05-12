use crate::out::utils::print_silent_err;

pub fn print_file_reader_err(msg: &str, path: &str) {
    print_silent_err(&format!("{} ({})", msg, path), Some("File reader error"));
}

pub fn print_file_writer_err(msg: &str, path: &str) {
    print_silent_err(&format!("{} ({})", msg, path), Some("File writer error"));
}

pub fn print_saved_to(path: &str) {
    println!("Saved to: {}", path);
}
