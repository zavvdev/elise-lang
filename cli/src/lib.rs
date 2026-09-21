//! # Elise language public interface
//!
//! This file is a boundary between implementation details and
//! a program consumer. It must only expose functions that are
//! necessary for running the program.

pub mod conf;
pub mod fsys;

use conf::{ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};
//use elise_data::{
//    csv::{csv_data_binder::CsvDataBinder, csv_data_parser::CsvDataParser},
//    data_binder::{DataBinder, DataBindings},
//    schema_binder::{SchemaBinder, SchemaBindings},
//};
use elise_parser::Prelude;
use elise_shared::shared_errors::LangErr;
use rayon::scope;
use std::{sync::Mutex, time::Instant};

use crate::conf::config::FileExt;

/// Representation of the successful execution of the
/// program in 'RUN' mode.
#[derive(Debug)]
pub struct RunResult<'a> {
    pub config: &'a ModeRunConf,
    pub ms: u128,
    pub output: String,
    pub bytecode: String,
}

/// Representation of the successful execution of the
/// program in 'BUILD' mode.
#[derive(Debug)]
pub struct BuildResult<'a> {
    pub config: &'a ModeBuildConf,
    pub ms: u128,
    pub executable_output: String,
}

/// Representation of the successful execution of the
/// program in 'EXEC' mode.
#[derive(Debug)]
pub struct ExecResult<'a> {
    pub config: &'a ModeExecConf,
    pub ms: u128,
    pub output: String,
}

/// Representation of the successful execution of the
/// program in 'VALIDATE' mode.
#[derive(Debug)]
pub struct ValidateResult<'a> {
    pub config: &'a ModeValidateConf,
    pub ms: u128,
}

/// Entry point for running the program in 'RUN' mode.
pub fn run<'a>(
    source_code: &'a [u8],
    _data: &'a str,
    data_schema: &'a [u8],
    config: &'a ModeRunConf,
) -> Result<RunResult<'a>, LangErr> {
    let start = Instant::now();

    let err: Mutex<Option<LangErr>> = Mutex::new(None);

    //let mut _hir: Result<HIR, LangErr> = Err(LangErr::PreExec(PreExecErr::NoHIR));

    //let mut schema_bindings: Result<SchemaBindings, LangErr> =
    //    Err(LangErr::PreExec(PreExecErr::NoResolvedSchema));

    //let mut data_bindings: Result<DataBindings, LangErr> =
    //    Err(LangErr::PreExec(PreExecErr::NoDataBinding));

    // Run in parallel since these processes don't depend on one another.
    scope(|s| {
        // ===================================================================
        // Source code parsing/semanalizing thread.
        // ===================================================================
        s.spawn(|_| {
            match Prelude::new(source_code)
                .parse()
                .map_err(LangErr::ParserSource)
            {
                Ok(ast) => {
                    println!("SC: {:#?}", ast);
                    // TODO: Probably needs a diff approach since we need to
                    //       handle errors for Harmony as well. So maybe collect
                    //       results into a Vec and assign the first one into err?
                    //hir = Harmony::new(&ast)
                    //    .analyze()
                    //    .map_err(LangErr::SemanticAnalyzer);
                }
                Err(ast_err) => {
                    let mut err_guard = err.lock().unwrap();
                    *err_guard = Some(ast_err);
                }
            };
        });
        // ===================================================================
        // Schema parsing/bindings thread.
        // ===================================================================
        s.spawn(|_| {
            match Prelude::new(data_schema)
                .parse()
                .map_err(LangErr::ParserSchema)
            {
                Ok(ast) => {
                    println!("SCHEMA SC: {:#?}", ast);

                    // schema_bindings = SchemaBinder::new(&ast)
                    //     .bind()
                    //     .map_err(LangErr::SchemaBinder);
                }
                Err(ast_err) => {
                    let mut err_guard = err.lock().unwrap();
                    *err_guard = Some(ast_err);
                }
            }
        });

        // ===================================================================
        // Data parsing/bindings thread.
        // ===================================================================
        if config.data_path.to_lowercase().ends_with(FileExt::CSV) {
            s.spawn(|_| {
                //let parsed = CsvDataParser::new(data).parse()?;
                //data_bindings = CsvDataBinder::new(&parsed)
                //    .bind()
                //    .map_err(LangErr::CsvDataBinder);
            });
        }
    });

    if let Some(final_err) = err.into_inner().unwrap() {
        return Err(final_err);
    }

    //let hir = _hir?;
    //let _schema_bindings = schema_bindings?;
    //let _data_bindings = data_bindings?;
    //validate_data(&data_bindings, &schema_bindings).map_err(LangErr::DataValidator)?;
    //println!("HIR: {:#?}", hir);

    Ok(RunResult {
        config,
        ms: start.elapsed().as_millis(),
        output: String::from("123"),
        bytecode: String::from("CALL a [1] [0]"),
    })
}

/// Entry point for running the program in 'BUILD' mode.
pub fn build<'a>(
    _source_code: &'a [u8],
    _data_schema: &'a [u8],
    config: &'a ModeBuildConf,
) -> Result<BuildResult<'a>, LangErr> {
    let start = Instant::now();

    println!("BUILD MODE");

    Ok(BuildResult {
        config,
        ms: start.elapsed().as_millis(),
        executable_output: String::from("CALL a [1] [0]"),
    })
}

/// Entry point for running the program in 'EXEC' mode.
pub fn exec<'a>(
    _executable: &'a [u8],
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

/// Entry point for running the program in 'VALIDATE' mode.
pub fn validate<'a>(
    _data: &'a str,
    _data_schema: &'a [u8],
    config: &'a ModeValidateConf,
) -> Result<ValidateResult<'a>, LangErr> {
    let start = Instant::now();

    println!("VALIDATE MODE");

    Ok(ValidateResult {
        config,
        ms: start.elapsed().as_millis(),
    })
}
