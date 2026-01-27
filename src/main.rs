use elise::conf::Conf;
use elise::exec;
use elise::fsys::file_reader;
use elise::handle_exec_result;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Conf::from_args(&args);

    match file_reader::read_file(&config.file_path) {
        Ok(file_descriptor) => {
            let exec_res = exec(file_descriptor.content, &config);
            handle_exec_result(&exec_res, &config);
        }
        Err(error) => {
            println!("Error reading file.");
            println!("{}", error.message);
        }
    }
}
