// Imports from lib.rs by referencing custom name that
// has been specified in Cargo.toml

pub mod out;

use elise;
use elise::conf::{Conf, ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};
use elise::fsys::read_files;
use elise_shared::errors::LangError;

use std::env;

use crate::out::messages::M_ERROR_CONFIG;

fn cli_run(conf: &ModeRunConf) {
    match read_files(&[
        &conf.source_code_path,
        &conf.data_path,
        &conf.data_schema_path,
    ]) {
        Ok(res) => match elise::run(&res[0].content, &res[1].content, &res[2].content, &conf) {
            Ok(run_res) => {
                out::print_run_result(&run_res.output, run_res.ms);
                if run_res.config.print_bytecode {
                    out::print_bytecode(&run_res.bytecode);
                }
            }
            Err(run_err) => match run_err {
                LangError::Parser(parser_error) => out::crash_at(
                    &parser_error.message,
                    &parser_error.source_code,
                    parser_error.char_pos,
                ),
            },
        },
        Err(err) => {
            out::print_file_reader_error(&err.message, &err.path);
        }
    };
}

fn cli_build(conf: &ModeBuildConf) {
    match read_files(&[&conf.source_code_path, &conf.data_schema_path]) {
        Ok(_res) => {}
        Err(_err) => {}
    };
}

fn cli_exec(conf: &ModeExecConf) {
    match read_files(&[&conf.executable_path, &conf.data_path]) {
        Ok(_res) => {}
        Err(_err) => {}
    };
}

fn cli_validate(conf: &ModeValidateConf) {
    match read_files(&[&conf.data_path, &conf.data_schema_path]) {
        Ok(_res) => {}
        Err(_err) => {}
    };
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    // Accept user input into Vec<Strings> for centralized ownership
    // which starts here.
    let args: Vec<String> = env::args().skip(1).collect();

    // Pass the reference to the args so we can re-use our owned data
    // without copying.
    // Check from_cli for more details.
    let config = Conf::from_cli(&args);

    if let Err(conf_error) = config {
        return out::silent_error(&format!("{}", conf_error.message), Some(M_ERROR_CONFIG));
    }

    match config.unwrap() {
        Conf::Run(run_conf) => cli_run(&run_conf),
        Conf::Build(build_conf) => cli_build(&build_conf),
        Conf::Exec(exec_conf) => cli_exec(&exec_conf),
        Conf::Validate(validate_conf) => cli_validate(&validate_conf),
    }
}
