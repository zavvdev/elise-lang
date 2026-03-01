// Imports from lib.rs by referencing custom name that
// has been specified in Cargo.toml

use elise::conf::Conf;
use elise::exec;
use elise::fsys::file_reader;
use elise::handle_exec_result;
use elise::out;

// Rust ecosystem imports

use std::env;
use std::panic;

// This function is the entry point for our program
// that is executed with binary from CLI.
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
    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(out::panic_hook));
    }

    // env::args() returns back an Iterator:
    //
    // https://doc.rust-lang.org/book/ch13-02-iterators.html
    // https://doc.rust-lang.org/rust-by-example/trait/iter.html
    //
    // iterator::collect() method can be used to produce an instance
    // of any type implementing the FromIterator trait. That trait
    // is implemented for all of the collections in the standard library.
    // Because FromIterator is implemented for many types, you need
    // to let Rust know what sort of collection you desire.
    //
    // For example, if we want a Vec<char> from a string slice we could
    // insert the type into our call to collect like so:
    //
    // let my_chars = "Hello, World!".chars().collect::<Vec<char>>();
    //
    // The double colons and outer angle brackets (i.e. ::<_>) are known
    // as the turbofish. In this case the turbofish is noisier syntax than
    // we need. By specifying the type on the binding itself the collect
    // can infer what collection it should create with less syntactical noise.
    //
    // let my_chars: Vec<char> = "Hello, World!".chars().collect();
    // or
    // let my_chars: Vec<_> = "Hello, World!".chars().collect();
    let args = env::args().skip(1).collect::<Vec<String>>();
    // Moving ownership of args to from_cli since we don't need to
    // reference args variable in this scope anymore.
    let config = Conf::from_cli(args);

    match file_reader::read_file(&config.file_path) {
        Ok(file_descriptor) => {
            let exec_res = exec(file_descriptor.content, &config);
            handle_exec_result(&exec_res, &config);
        }
        Err(error) => {
            out::error(&error.message, Some("Error reading file"));
        }
    }
}

// TODO FOR PRE-EXECUTION STAGE:
// - [x] Add config builder
// - [x] Add cli args parsing in conf/input
// - [x] Add custom panic hook
// - [ ] Improve args parsing in conf/input.
//       Add support for more flexible configuration (continue in conf.rs)
// - [ ] Add tests for conf/input
// - [ ] Add tests for Conf struct
// - [ ] Add tests for file reader
// - [ ] Add tests for handle_exec_result
