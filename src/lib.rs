// This file is an entry point for all modules in this project.
// If main.ts needs to import something, it needs to import it
// from this file by using use elise::something;

pub mod conf;
pub mod fsys;
pub mod out;
pub mod parser;

use conf::Conf;

use crate::parser::Parser;

pub enum ExecStatus {
    Success,
    Error(String),
}

pub struct ExecResult<'a> {
    pub status: ExecStatus,
    pub output: String,
    pub bytecode: Option<String>,
    pub config: &'a Conf,
}

#[derive(PartialEq, Debug)]
pub enum HandleExecResultOperationStatus {
    Success,
    Error,
}

pub fn exec<'a>(source_code: &'a str, config: &'a Conf) -> ExecResult<'a> {
    let ast = Parser::new(&source_code).parse();

    println!("ast: {:?}", ast);

    ExecResult {
        status: ExecStatus::Success,
        output: String::from("123"),
        bytecode: Some(String::from("CALL a [1] [0]")),
        config: config,
    }
}

pub fn handle_exec_result(res: &ExecResult, config: &Conf) -> HandleExecResultOperationStatus {
    match &res.status {
        ExecStatus::Success => {
            out::print_execution_output(&res.output);
            if let Some(bytecode) = &res.bytecode {
                if config.print_bytecode {
                    out::print_bytecode(bytecode);
                }
            }
            HandleExecResultOperationStatus::Success
        }
        ExecStatus::Error(reason) => {
            out::error(reason, None);
            HandleExecResultOperationStatus::Error
        }
    }
}

// ===============================
// Tests
// ===============================

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
            },
            &config,
        );
        assert_eq!(result, HandleExecResultOperationStatus::Success);
    }
}
