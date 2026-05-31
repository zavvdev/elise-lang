/**
 * This file is a boundary between implementation details and
 * a program consumer. It must only expose functions that are
 * necessary for running the program.
 */
pub mod conf;
pub mod fsys;

use conf::{ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};

use elise_csv::{
    parser::{CsvParser, CsvParserRecord},
    schema_resolver::CsvSchemaResolver,
};
use elise_errors::{LangErr, LangErr::*, errors_csv_parser::CsvParserErr};
use elise_parser::Prelude;
use rayon::scope;
use std::time::Instant;

use crate::conf::config::FILE_EXT_CSV;

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

enum DataParseResult {
    Csv(Result<Vec<CsvParserRecord>, CsvParserErr>),
}

pub fn run<'a>(
    source_code: &'a [u8],
    data: &'a str,
    data_schema: &'a [u8],
    config: &'a ModeRunConf,
) -> Result<RunResult<'a>, LangErr> {
    let start = Instant::now();

    let (mut source_code_ast, mut schema_ast, mut parsed_data) = (None, None, None);

    scope(|s| {
        s.spawn(|_| {
            let ast = Prelude::new(source_code).parse().map_err(ParserSource);
            source_code_ast = Some(ast);
        });
        s.spawn(|_| {
            let ast = Prelude::new(data_schema).parse().map_err(ParserSchema);
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
    let parsed_data = parsed_data.unwrap();

    println!("source: {:#?}", source_code_ast);

    match parsed_data {
        DataParseResult::Csv(records) => {
            let rec = records.map_err(CsvParser)?;
            let res = CsvSchemaResolver::new(&schema_ast)
                .resolve()
                .map_err(CsvSchemaResolver)?;
            println!("csv records: {:#?}", rec);
            println!("csv resolved schema: {:#?}", res);
        }
    }

    Ok(RunResult {
        config,
        ms: start.elapsed().as_millis(),
        output: String::from("123"),
        bytecode: String::from("CALL a [1] [0]"),
    })
}

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
