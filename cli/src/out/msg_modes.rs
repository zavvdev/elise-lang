pub fn print_run_result(output: &str, ms: u128) {
    println!("Output: {}", output);
    println!("Execution time: {} ms", ms);
}

pub fn print_build_result(path: &str, ms: u128) {
    println!("Saved to: {}", path);
    println!("Execution time: {} ms", ms);
}

pub fn print_validate_result(ms: u128) {
    println!("Valid");
    println!("Execution time: {} ms", ms);
}
