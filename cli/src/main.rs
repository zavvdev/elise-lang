pub mod out;

use elise;
use elise::conf::{Conf, ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf};
use elise::fsys::{read_files, write_file};
use elise_shared::errors::LangError;

use std::env;

fn handle_lang_error(lang_err: &LangError) {
    match lang_err {
        LangError::Parser(parser_error) => out::crash_at(
            parser_error.message,
            parser_error.source_code,
            parser_error.row,
            parser_error.col,
        ),
    }
}

fn cli_run(conf: &ModeRunConf) {
    match read_files(&[
        &conf.source_code_path,
        &conf.data_path,
        &conf.data_schema_path,
    ]) {
        Ok(read_res) => {
            let run_res = elise::run(
                &read_res[0].content,
                &read_res[1].content,
                &read_res[2].content,
                &conf,
            );

            if let Err(run_err) = &run_res {
                handle_lang_error(&run_err);
            }

            let run_res = run_res.unwrap();

            out::print_run_result(&run_res.output, run_res.ms);
            if run_res.config.print_bytecode {
                out::print_bytecode(&run_res.bytecode);
            }
        }
        Err(read_err) => {
            out::print_file_reader_error(&read_err.message, &read_err.path);
        }
    };
}

fn cli_build(conf: &ModeBuildConf) {
    match read_files(&[&conf.source_code_path, &conf.data_schema_path]) {
        Ok(read_res) => {
            let build_res = elise::build(&read_res[0].content, &read_res[1].content, &conf);

            if let Err(build_err) = &build_res {
                handle_lang_error(&build_err);
            }

            let build_res = build_res.unwrap();
            let out_path = &build_res.config.executable_output_path;

            match write_file(out_path, &build_res.executale_output) {
                Ok(_) => out::print_build_result(out_path, build_res.ms),
                Err(err) => out::print_file_writer_error(&err.message, out_path),
            }
        }
        Err(read_err) => out::print_file_reader_error(&read_err.message, &read_err.path),
    };
}

fn cli_exec(conf: &ModeExecConf) {
    match read_files(&[&conf.executable_path, &conf.data_path]) {
        Ok(read_res) => {
            let exec_res = elise::exec(&read_res[0].content, &read_res[1].content, &conf);

            if let Err(exec_err) = &exec_res {
                handle_lang_error(&exec_err);
            }

            let exec_res = exec_res.unwrap();
            out::print_run_result(&exec_res.output, exec_res.ms);
        }
        Err(read_err) => out::print_file_reader_error(&read_err.message, &read_err.path),
    };
}

fn cli_validate(conf: &ModeValidateConf) {
    match read_files(&[&conf.data_path, &conf.data_schema_path]) {
        Ok(read_res) => {
            let validate_res = elise::validate(&read_res[0].content, &read_res[1].content, &conf);

            if let Err(validate_err) = &validate_res {
                handle_lang_error(&validate_err);
            }

            let validate_res = validate_res.unwrap();
            out::print_validate_result(validate_res.ms);
        }
        Err(read_err) => out::print_file_reader_error(&read_err.message, &read_err.path),
    };
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        out::panic_hook(info);
    }));

    let args: Vec<String> = env::args().skip(1).collect();
    let config = Conf::from_cli(&args);

    if let Err(conf_err) = config {
        return out::config_error(&conf_err.message);
    }

    match config.unwrap() {
        Conf::Run(run_conf) => cli_run(&run_conf),
        Conf::Build(build_conf) => cli_build(&build_conf),
        Conf::Exec(exec_conf) => cli_exec(&exec_conf),
        Conf::Validate(validate_conf) => cli_validate(&validate_conf),
    }
}
