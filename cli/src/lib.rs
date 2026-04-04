// This file is an entry point for all modules in this project.
// If main.ts needs to import something, it needs to import it
// from this file by using use elise::something;

pub mod conf;
pub mod fsys;

use conf::Conf;

use elise_parser::parser::Prelude;
use elise_shared::out;
use std::time::Instant;

pub enum ExecStatus {
    Success,
    Error(String),
}

pub struct ExecResult<'a> {
    pub status: ExecStatus,
    pub output: String,
    pub bytecode: Option<String>,
    pub config: &'a Conf,
    pub ms: u128,
}

#[derive(PartialEq, Debug)]
pub enum HandleExecResultOperationStatus {
    Success,
    Error,
}

pub fn exec<'a>(source_code: &'a str, config: &'a Conf) -> ExecResult<'a> {
    // Customize panic message.
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    let start = Instant::now();
    let ast = Prelude::new(&source_code).parse();

    println!("ast: {:#?}", ast);

    ExecResult {
        status: ExecStatus::Success,
        output: String::from("123"),
        bytecode: Some(String::from("CALL a [1] [0]")),
        config: config,
        ms: start.elapsed().as_millis(),
    }
}

pub fn handle_exec_result(res: &ExecResult, config: &Conf) -> HandleExecResultOperationStatus {
    match &res.status {
        ExecStatus::Success => {
            out::print_exec_result(&res.output, res.ms);
            if let Some(bytecode) = &res.bytecode {
                if config.print_bytecode {
                    out::print_bytecode(bytecode);
                }
            }
            HandleExecResultOperationStatus::Success
        }
        ExecStatus::Error(reason) => {
            out::silent_error(reason, None);
            HandleExecResultOperationStatus::Error
        }
    }
}

// ==========================
//
// TESTS START
//
// ==========================

#[cfg(test)]
mod tests {
    use crate::{ExecStatus, HandleExecResultOperationStatus, conf::Conf, handle_exec_result};

    // Handle exec result

    #[test]
    fn should_handle_error_exec_result() {
        let config = Conf {
            file_path: "test.eli".to_string(),
            print_bytecode: true,
        };
        let result = handle_exec_result(
            &crate::ExecResult {
                status: ExecStatus::Error("Something went wrong".to_string()),
                output: "hello".to_string(),
                bytecode: Some("SOME [1]".to_string()),
                config: &config,
                ms: 1,
            },
            &config,
        );
        assert_eq!(result, HandleExecResultOperationStatus::Error);
    }

    #[test]
    fn should_handle_success_exec_result() {
        let config = Conf {
            file_path: "test.eli".to_string(),
            print_bytecode: true,
        };
        let result = handle_exec_result(
            &crate::ExecResult {
                status: ExecStatus::Success,
                output: "hello".to_string(),
                bytecode: Some("SOME [1]".to_string()),
                config: &config,
                ms: 1,
            },
            &config,
        );
        assert_eq!(result, HandleExecResultOperationStatus::Success);
    }
}

// ==========================
//
// TESTS END
//
// ==========================
