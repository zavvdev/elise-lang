use crate::out::utils;

pub fn print_file_rw_err(msg: &str, path: &str, read: bool) {
    let label = if read {
        Some("File reader error")
    } else {
        Some("File writer error")
    };
    utils::print_err(&format!("{} ({})", msg, path), label);
}

pub fn print_saved_to(path: &str) {
    println!("Saved to: {}", path);
}
