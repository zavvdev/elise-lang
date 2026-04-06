// This file is an entry point for all modules in this project.
// If main.ts needs to import something, it needs to import it
// from this file by using use elise::something;

pub mod conf;
pub mod fsys;

use conf::Conf;

use elise_parser::parser::Prelude;
use elise_shared::out;
use std::time::Instant;

pub enum Status {
    Success,
    Error(String),
}

pub struct BaseResult<'a> {
    pub config: &'a Conf,
    pub ms: u128,
    pub status: Status,
}

pub struct RunResult<'a> {
    pub base: BaseResult<'a>,
    pub output: String,
    pub bytecode: Option<String>,
}

pub struct ExecResult<'a> {
    pub base: BaseResult<'a>,
    pub output: String,
}

pub struct BuildResult<'a> {
    pub base: BaseResult<'a>,
    pub executale_output: String,
}

pub struct ValidateResult<'a> {
    pub base: BaseResult<'a>,
    pub is_valid: bool,
}

#[derive(PartialEq, Debug)]
pub enum HandleResultStatus {
    Success,
    Error,
}

pub fn run<'a>(
    source_code: &'a str,
    _data: &'a str,
    _data_schema: &'a str,
    config: &'a Conf,
) -> RunResult<'a> {
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    let start = Instant::now();
    let ast = Prelude::new(&source_code).parse();

    println!("ast: {:#?}", ast);

    RunResult {
        base: BaseResult {
            config: config,
            ms: start.elapsed().as_millis(),
            status: Status::Success,
        },
        output: String::from("123"),
        bytecode: Some(String::from("CALL a [1] [0]")),
    }
}

pub fn build<'a>(
    _source_code: &'a str,
    _data_schema: &'a str,
    config: &'a Conf,
) -> BuildResult<'a> {
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    let start = Instant::now();

    println!("BUILD MODE");

    BuildResult {
        base: BaseResult {
            config: config,
            ms: start.elapsed().as_millis(),
            status: Status::Success,
        },
        executale_output: String::from("CALL a [1] [0]"),
    }
}

pub fn exec<'a>(_executable: &'a str, _data: &'a str, config: &'a Conf) -> ExecResult<'a> {
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    let start = Instant::now();

    println!("EXEC MODE");

    ExecResult {
        base: BaseResult {
            config: config,
            ms: start.elapsed().as_millis(),
            status: Status::Success,
        },
        output: String::from("Exec Result Output"),
    }
}

pub fn validate<'a>(_data: &'a str, _data_schema: &'a str, config: &'a Conf) -> ValidateResult<'a> {
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    let start = Instant::now();

    println!("VALIDATE MODE");

    ValidateResult {
        base: BaseResult {
            config: config,
            ms: start.elapsed().as_millis(),
            status: Status::Success,
        },
        is_valid: true,
    }
}

//pub fn handle_exec_result(res: &ExecResult, config: &Conf) -> HandleExecResultOperationStatus {
//    match &res.status {
//        ExecStatus::Success => {
//            out::print_exec_result(&res.output, res.ms);
//            if let Some(bytecode) = &res.bytecode {
//                if config.print_bytecode {
//                    out::print_bytecode(bytecode);
//                }
//            }
//            HandleExecResultOperationStatus::Success
//        }
//        ExecStatus::Error(reason) => {
//            out::silent_error(reason, None);
//            HandleExecResultOperationStatus::Error
//        }
//    }
//}

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
