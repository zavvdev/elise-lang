//! # Elise CLI entry point
//!
//! This file is an entry point for the language when it's being run
//! in CLI mode. You can think of it as a consumer of our program's
//! interface that is expose by `lib.rs`.
//!
//! As a consumer, this file is responsible for:
//!     1. Collecting CLI arguments
//!     2. Building a configuration from those arguments
//!        by using provided `Conf` struct
//!     3. Reading source-code filies
//!     4. Calling respective functions exposed by `lib.rs` in order
//!        to run the program in a specific mode
//!     5. Handling errors

pub mod out;

use elise::conf::{Conf, ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};
use elise::fsys::{read_file_bytes, read_file_string, write_file};
use elise_errors::LangErr;

use std::env;

use crate::out::{msg_common, msg_csv_schema_resolver};
use crate::out::msg_fsys;
use crate::out::msg_modes;
use crate::out::msg_parser;
use crate::out::utils::{panic_hook, print_bytecode};
use crate::out::{msg_conf, msg_csv_binder};
use crate::out::{msg_csv_parser, msg_semantic_analyzer};

fn handle_lang_err(lang_err: &LangErr, source_code: &[u8], schema_source_code: &[u8]) -> ! {
    use LangErr::*;

    match lang_err {
        Common(err) => msg_common::print_err(err),
        ParserSource(err) => msg_parser::print_err(err, source_code),
        ParserSchema(err) => msg_parser::print_err(err, schema_source_code),
        SemanticAnalyzer(err) => msg_semantic_analyzer::print_err(err),
        CsvParser(err) => msg_csv_parser::print_err(err),
        CsvSchemaResolver(err) => msg_csv_schema_resolver::print_err(err, schema_source_code),
        CsvBinder(err) => msg_csv_binder::print_err(err),
    }

    std::process::exit(1);
}

fn cli_run(conf: &ModeRunConf) {
    // We need to keep source code globally available in order to
    // be able to pass it to the function that handles errors.
    let source_code = match read_file_bytes(&conf.source_code_path) {
        Ok(desc) => desc.content,
        Err(e) => return msg_fsys::print_file_rw_err(&e.message, &e.path, true),
    };

    // We need to keep schema source code globally available in order to
    // be able to pass it to the function that handles errors.
    let schema_source_code = match read_file_bytes(&conf.data_schema_path) {
        Ok(desc) => desc.content,
        Err(e) => return msg_fsys::print_file_rw_err(&e.message, &e.path, true),
    };

    match read_file_string(&conf.data_path) {
        Ok(data_desc) => {
            let run_res = elise::run(&source_code, &data_desc.content, &schema_source_code, conf)
                .unwrap_or_else(|e| handle_lang_err(&e, &source_code, &schema_source_code));

            msg_modes::print_run_result(&run_res.output, run_res.ms);

            if run_res.config.print_bytecode {
                print_bytecode(&run_res.bytecode);
            }

            if let Some(path) = run_res.config.output_path.as_ref() {
                match write_file(path, &run_res.output) {
                    Ok(_) => msg_fsys::print_saved_to(path),
                    Err(err) => msg_fsys::print_file_rw_err(&err.message, &err.path, false),
                }
            }
        }
        Err(data_read_err) => {
            msg_fsys::print_file_rw_err(&data_read_err.message, &data_read_err.path, true)
        }
    }
}

fn cli_build(conf: &ModeBuildConf) {
    // We need to keep source code globally available in order to
    // be able to pass it to the function that handles errors.
    let source_code = match read_file_bytes(&conf.source_code_path) {
        Ok(desc) => desc.content,
        Err(e) => return msg_fsys::print_file_rw_err(&e.message, &e.path, true),
    };

    // We need to keep schema source code globally available in order to
    // be able to pass it to the function that handles errors.
    let schema_source_code = match read_file_bytes(&conf.data_schema_path) {
        Ok(desc) => desc.content,
        Err(e) => return msg_fsys::print_file_rw_err(&e.message, &e.path, true),
    };

    let build_res = elise::build(&source_code, &schema_source_code, conf)
        .unwrap_or_else(|e| handle_lang_err(&e, &source_code, &schema_source_code));

    let out_path = &build_res.config.executable_output_path;

    match write_file(out_path, &build_res.executale_output) {
        Ok(_) => msg_modes::print_build_result(out_path, build_res.ms),
        Err(err) => msg_fsys::print_file_rw_err(&err.message, &err.path, false),
    }
}

fn cli_exec(conf: &ModeExecConf) {
    match (
        read_file_bytes(&conf.executable_path),
        read_file_string(&conf.data_path),
    ) {
        (Ok(executable_desc), Ok(data_desc)) => {
            let exec_res = elise::exec(&executable_desc.content, &data_desc.content, conf)
                .unwrap_or_else(|e| handle_lang_err(&e, &[], &[]));

            msg_modes::print_run_result(&exec_res.output, exec_res.ms);
        }
        (Err(executable_read_err), _) => msg_fsys::print_file_rw_err(
            &executable_read_err.message,
            &executable_read_err.path,
            true,
        ),
        (_, Err(data_read_err)) => {
            msg_fsys::print_file_rw_err(&data_read_err.message, &data_read_err.path, true)
        }
    };
}

fn cli_validate(conf: &ModeValidateConf) {
    // We need to keep schema source code globally available in order to
    // be able to pass it to the function that handles errors.
    let schema_source_code = match read_file_bytes(&conf.data_schema_path) {
        Ok(desc) => desc.content,
        Err(e) => return msg_fsys::print_file_rw_err(&e.message, &e.path, true),
    };

    match read_file_string(&conf.data_path) {
        Ok(data_desc) => {
            let validate_res = elise::validate(&data_desc.content, &schema_source_code, conf)
                .unwrap_or_else(|e| handle_lang_err(&e, &[], &schema_source_code));

            msg_modes::print_validate_result(validate_res.ms);
        }
        Err(data_read_err) => {
            msg_fsys::print_file_rw_err(&data_read_err.message, &data_read_err.path, true)
        }
    };
}

fn main() {
    // Override default panic message.
    std::panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    // Skip first argument which is the name of the program.
    let args: Vec<String> = env::args().skip(1).collect();

    match Conf::new(&args) {
        Err(conf_err) => msg_conf::print_err(&conf_err),
        Ok(Conf::Run(run_conf)) => cli_run(&run_conf),
        Ok(Conf::Build(build_conf)) => cli_build(&build_conf),
        Ok(Conf::Exec(exec_conf)) => cli_exec(&exec_conf),
        Ok(Conf::Validate(validate_conf)) => cli_validate(&validate_conf),
    }
}
