use std::env;
use std::fs;

use elise::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match fs::read_to_string(&config.filename) {
        Ok(content) => elise::execute(content),
        Err(e) => {
            println!("Cannot read {} file: {}", config.filename, e);
        }
    }
}
