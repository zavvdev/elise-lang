// Imports from lib.rs by referencing custom name that
// has been specified in Cargo.toml

use elise::conf::Conf;
use elise::exec;
use elise::fsys::file_reader;
//use elise::handle_exec_result;

// Rust ecosystem imports

use std::env;

// This function is the entry point for our program
// that is executed with binary from CLI. Treat it as
// a consumer (end user) of out program.
// Since it's called from CLI, we can use standard
// functions provided by Elise lang library
// to prepare the configuration for the execution
// and then execute.
//
// Main things that are needed in order to run the program (host agnostic):
// 1. Configuration (Conf struct) - main configuration for the runtime.
// If yout want to run the program with custom configuration outside the CLI,
// you can just construct Conf struct manually.
// 2. Function for execution (exec) that takes the source code and configuration.
// If you want to run the program with custom configuration outside the CLI, you can
// use your own file reader or any other approach for deriving source code.
//
// handle_exec_result is used for handling the result of execution in CLI. So if you
// want to run the program with custom configuration outside the CLI, you can
// handle the result of execution in any way you want by just using the ExecResult struct.
fn main() {
    // Accept user input into Vec<Strings> for centralized ownership
    // which starts here.
    let args: Vec<String> = env::args().skip(1).collect();

    // Pass the reference to the args so we can re-use our owned data
    // without copying.
    // Check from_cli for more details.
    let config = Conf::from_cli(&args);

    println!("config: {:#?}", config);

    //match file_reader::read_file(&config.file_path) {
    //    Ok(file_descriptor) => {
    //        let exec_res = exec(&file_descriptor.content, &config);
    //        handle_exec_result(&exec_res, &config);
    //    }
    //    Err(error) => {
    //        // This main.rs is an end user of the program,
    //        // we can't use 'out' module here since it exists only
    //        // inside our program itself and not exposed outside.
    //        // Think of it like we installed lib.rs into other program (main.rs).
    //        panic!("{}", &error.message);
    //    }
    //}
}
