// This file is an entry point for all modules in this project.
// If main.ts needs to import something, it needs to import it
// from this file by using use elise::something;

pub mod conf;
pub mod fsys;

use conf::{ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};

//use elise_parser::parser::Prelude;
use elise_shared::errors::LangError;
use std::time::Instant;

#[derive(Debug)]
pub struct RunResult<'a> {
    pub config: &'a ModeRunConf,
    pub ms: u128,
    pub output: String,
    pub bytecode: String,
}

#[derive(Debug)]
pub struct BuildResult<'a> {
    pub config: &'a ModeBuildConf,
    pub ms: u128,
    pub executale_output: String,
}

#[derive(Debug)]
pub struct ExecResult<'a> {
    pub config: &'a ModeExecConf,
    pub ms: u128,
    pub output: String,
}

#[derive(Debug)]
pub struct ValidateResult<'a> {
    pub config: &'a ModeValidateConf,
    pub ms: u128,
    pub is_valid: bool,
}

#[derive(PartialEq, Debug)]
pub enum HandleResultStatus {
    Success,
    Error,
}

pub fn run<'a>(
    _source_code: &'a str,
    _data: &'a str,
    _data_schema: &'a str,
    config: &'a ModeRunConf,
) -> Result<RunResult<'a>, LangError> {
    let start = Instant::now();
    //let ast = Prelude::new(&source_code).parse();

    //println!("ast: {:#?}", ast);

    println!("RUN MODE");

    Ok(RunResult {
        config,
        ms: start.elapsed().as_millis(),
        output: String::from("123"),
        bytecode: String::from("CALL a [1] [0]"),
    })
}

pub fn build<'a>(
    _source_code: &'a str,
    _data_schema: &'a str,
    config: &'a ModeBuildConf,
) -> Result<BuildResult<'a>, LangError> {
    let start = Instant::now();

    println!("BUILD MODE");

    Ok(BuildResult {
        config,
        ms: start.elapsed().as_millis(),
        executale_output: String::from("CALL a [1] [0]"),
    })
}

pub fn exec<'a>(
    _executable: &'a str,
    _data: &'a str,
    config: &'a ModeExecConf,
) -> Result<ExecResult<'a>, LangError> {
    let start = Instant::now();

    println!("EXEC MODE");

    Ok(ExecResult {
        config,
        ms: start.elapsed().as_millis(),
        output: String::from("Exec Result Output"),
    })
}

pub fn validate<'a>(
    _data: &'a str,
    _data_schema: &'a str,
    config: &'a ModeValidateConf,
) -> Result<ValidateResult<'a>, LangError> {
    let start = Instant::now();

    println!("VALIDATE MODE");

    Ok(ValidateResult {
        config,
        ms: start.elapsed().as_millis(),
        is_valid: true,
    })
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
