/**
 * This file is a boundary between implementation details and
 * a program consumer. It must only expose functions that are
 * necessary for running the program.
 */
pub mod conf;
pub mod fsys;

use conf::{ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};

use elise_parser::parser::Prelude;
use elise_shared::errors::LangErr;
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
    config: &'a ModeRunConf,
) -> Result<RunResult<'a>, LangErr> {
    let start = Instant::now();
    let ast = Prelude::new(&source_code).parse()?;

    println!("ast: {:#?}", ast);

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
) -> Result<BuildResult<'a>, LangErr> {
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
) -> Result<ExecResult<'a>, LangErr> {
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
) -> Result<ValidateResult<'a>, LangErr> {
    let start = Instant::now();

    println!("VALIDATE MODE");

    Ok(ValidateResult {
        config,
        ms: start.elapsed().as_millis(),
    })
}
