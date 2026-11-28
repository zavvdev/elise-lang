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
use elise_shared_diagnostics::LangErr;
use elise_shared_diagnostics::errors_fsys::FSysErr;

use std::env;

use crate::out::msg_fsys;
use crate::out::msg_modes;
use crate::out::msg_parser;
use crate::out::utils::{panic_hook, print_bytecode};
use crate::out::{msg_bindings_validator, msg_conf, msg_data_binder};
use crate::out::{msg_csv_data_parser, msg_semanalyzer};

// NOTE: Not the best solution since some errors need only a subset
// of these context values, so for now we just pass empty values
// in case we don't have access to them. For example, EXEC mode
// doesn't need source code and schema code, therefore we don't
// have these during running in EXEC mode, so we pass empty
// vectors and empty strings.
struct LangErrHandlerContext<'a> {
    lang_err: &'a LangErr,
    source_code: &'a [u8],
    schema_source_code: &'a [u8],
    source_filename: &'a str,
    schema_filename: &'a str,
}

fn handle_lang_err(ctx: LangErrHandlerContext) -> ! {
    use LangErr::*;

    match ctx.lang_err {
        ParserSource(err) => msg_parser::print_err(err, ctx.source_code, ctx.source_filename),
        ParserSchema(err) => {
            msg_parser::print_err(err, ctx.schema_source_code, ctx.schema_filename)
        }
        SemanalyzerSource(err) => {
            msg_semanalyzer::print_err(err, ctx.source_code, ctx.source_filename)
        }
        SemanalyzerSchema(err) => {
            msg_semanalyzer::print_err(err, ctx.schema_source_code, ctx.schema_filename)
        }
        CsvDataParser(err) => msg_csv_data_parser::print_err(err),
        BindingsValidator(err) => msg_bindings_validator::print_err(err),
        DataBinder(err) => msg_data_binder::print_err(err),
    }

    std::process::exit(1);
}

fn cli_run(conf: &ModeRunConf) {
    // We need to keep source code globally available in order to
    // be able to pass it to the function that handles errors.
    let source_code = match read_file_bytes(&conf.source_code_path) {
        Ok(desc) => desc.contents,
        Err(e) => {
            return msg_fsys::print_err(FSysErr::R {
                msg: e.message,
                path: e.path,
            });
        }
    };

    // We need to keep schema source code globally available in order to
    // be able to pass it to the function that handles errors.
    let schema_source_code = match read_file_bytes(&conf.data_schema_path) {
        Ok(desc) => desc.contents,
        Err(e) => {
            return msg_fsys::print_err(FSysErr::R {
                msg: e.message,
                path: e.path,
            });
        }
    };

    match read_file_string(&conf.data_path) {
        Ok(data_desc) => {
            let run_res = elise::run(&source_code, &data_desc.contents, &schema_source_code, conf)
                .unwrap_or_else(|e| {
                    handle_lang_err(LangErrHandlerContext {
                        lang_err: &e,
                        source_code: &source_code,
                        schema_source_code: &schema_source_code,
                        source_filename: &conf.source_code_path,
                        schema_filename: &conf.data_schema_path,
                    })
                });

            msg_modes::print_run_result(&run_res.output, run_res.ms);

            if run_res.config.print_bytecode {
                print_bytecode(&run_res.bytecode);
            }

            if let Some(path) = run_res.config.output_path.as_ref() {
                match write_file(path, &run_res.output) {
                    Ok(_) => out::utils::print_saved_to(path),
                    Err(err) => msg_fsys::print_err(FSysErr::W {
                        msg: err.message,
                        path: err.path,
                    }),
                }
            }
        }
        Err(data_read_err) => msg_fsys::print_err(FSysErr::R {
            msg: data_read_err.message,
            path: data_read_err.path,
        }),
    }
}

fn cli_build(conf: &ModeBuildConf) {
    let source_code = match read_file_bytes(&conf.source_code_path) {
        Ok(desc) => desc.contents,
        Err(e) => {
            return msg_fsys::print_err(FSysErr::R {
                msg: e.message,
                path: e.path,
            });
        }
    };

    let schema_source_code = match read_file_bytes(&conf.data_schema_path) {
        Ok(desc) => desc.contents,
        Err(e) => {
            return msg_fsys::print_err(FSysErr::R {
                msg: e.message,
                path: e.path,
            });
        }
    };

    let build_res = elise::build(&source_code, &schema_source_code, conf).unwrap_or_else(|e| {
        handle_lang_err(LangErrHandlerContext {
            lang_err: &e,
            source_code: &source_code,
            schema_source_code: &schema_source_code,
            source_filename: &conf.source_code_path,
            schema_filename: &conf.data_schema_path,
        })
    });

    let out_path = &build_res.config.executable_output_path;

    match write_file(out_path, &build_res.executable_output) {
        Ok(_) => msg_modes::print_build_result(out_path, build_res.ms),
        Err(err) => msg_fsys::print_err(FSysErr::W {
            msg: err.message,
            path: err.path,
        }),
    }
}

fn cli_exec(conf: &ModeExecConf) {
    match (
        read_file_bytes(&conf.executable_path),
        read_file_string(&conf.data_path),
    ) {
        (Ok(executable_desc), Ok(data_desc)) => {
            let exec_res = elise::exec(&executable_desc.contents, &data_desc.contents, conf)
                .unwrap_or_else(|e| {
                    handle_lang_err(LangErrHandlerContext {
                        lang_err: &e,
                        source_code: &[],
                        schema_source_code: &[],
                        source_filename: "",
                        schema_filename: "",
                    })
                });

            msg_modes::print_run_result(&exec_res.output, exec_res.ms);
        }
        (Err(executable_read_err), _) => msg_fsys::print_err(FSysErr::R {
            msg: executable_read_err.message,
            path: executable_read_err.path,
        }),
        (_, Err(data_read_err)) => msg_fsys::print_err(FSysErr::R {
            msg: data_read_err.message,
            path: data_read_err.path,
        }),
    };
}

fn cli_validate(conf: &ModeValidateConf) {
    let schema_source_code = match read_file_bytes(&conf.data_schema_path) {
        Ok(desc) => desc.contents,
        Err(e) => {
            return msg_fsys::print_err(FSysErr::R {
                msg: e.message,
                path: e.path,
            });
        }
    };

    match read_file_string(&conf.data_path) {
        Ok(data_desc) => {
            let validate_res = elise::validate(&data_desc.contents, &schema_source_code, conf)
                .unwrap_or_else(|e| {
                    handle_lang_err(LangErrHandlerContext {
                        lang_err: &e,
                        source_code: &[],
                        schema_source_code: &schema_source_code,
                        source_filename: "",
                        schema_filename: &conf.data_schema_path,
                    })
                });

            msg_modes::print_validate_result(validate_res.ms);
        }
        Err(data_read_err) => msg_fsys::print_err(FSysErr::R {
            msg: data_read_err.message,
            path: data_read_err.path,
        }),
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
