//! # Elise language public interface
//!
//! This file is a boundary between implementation details and
//! a program consumer. It must only expose functions that are
//! necessary for running the program.

pub mod conf;
pub mod fsys;

use conf::{ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};
use elise_binder::Binder;

use elise_csv::{
    csv_binder::CsvDataBinder,
    csv_parser::{CsvParser, CsvRow},
    csv_schema_resolver::CsvSchemaResolver,
};
use elise_errors::{LangErr, errors_common::CommonErr, errors_csv_parser::CsvParserErr};
use elise_parser::Prelude;
use elise_semanalyzer::Harmony;
use rayon::scope;
use std::time::Instant;

use crate::conf::config::FILE_EXT_CSV;

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
    pub executale_output: String,
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

/// Result of the data parsing operation.
enum DataParseResult {
    Csv(Result<Vec<CsvRow>, CsvParserErr>),
}

/// Entry point for running the program in 'RUN' mode.
pub fn run<'a>(
    source_code: &'a [u8],
    data: &'a str,
    data_schema: &'a [u8],
    config: &'a ModeRunConf,
) -> Result<RunResult<'a>, LangErr> {
    let start = Instant::now();

    let (mut source_code_ast, mut schema_ast, mut parsed_data) = (None, None, None);

    // Run in parallel since these processes does not depend on one another.
    scope(|s| {
        s.spawn(|_| {
            // Map ParserErr to LangErr::ParserSource in order to differentiate
            // between data being parsed since we can use Prelude for parsing
            // source code or schema source code.
            let ast = Prelude::new(source_code)
                .parse()
                .map_err(LangErr::ParserSource);
            source_code_ast = Some(ast);
        });
        s.spawn(|_| {
            // Map ParserErr to LangErr::ParserSchema since data schema syntax
            // is the same as a source code syntax.
            let ast = Prelude::new(data_schema)
                .parse()
                .map_err(LangErr::ParserSchema);
            schema_ast = Some(ast);
        });

        if config.data_path.ends_with(FILE_EXT_CSV) {
            s.spawn(|_| {
                let parsed = CsvParser::new(data).parse();
                parsed_data = Some(DataParseResult::Csv(parsed));
            });
        }
    });

    let source_code_ast = source_code_ast.unwrap()?;
    let schema_ast = schema_ast.unwrap()?;
    let parsed_data = parsed_data.ok_or(LangErr::Common(CommonErr::MissingParserData))?;

    let data_binding = match parsed_data {
        DataParseResult::Csv(records) => {
            let rec = records.map_err(LangErr::CsvParser)?;

            let res = CsvSchemaResolver::new(&schema_ast)
                .resolve()
                .map_err(LangErr::CsvSchemaResolver)?;

            CsvDataBinder::new(rec, res)
                .bind()
                .map_err(LangErr::CsvBinder)?
        }
    };

    let hir = Harmony::new(&source_code_ast, &data_binding)
        .analyze()
        .map_err(LangErr::SemanticAnalyzer)?;

    println!("DATA BINDING: {:#?}", data_binding);
    println!("HIR: {:#?}", hir);

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
        executale_output: String::from("CALL a [1] [0]"),
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
