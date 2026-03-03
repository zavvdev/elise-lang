// This file is an entry point for all modules in this project.
// If main.ts needs to import something, it needs to import it
// from this file by using use elise::something;

pub mod conf;
pub mod fsys;
pub mod out;

use conf::Conf;

pub enum ExecStatus {
    Success,
    Error(String),
}

pub struct ExecResult<'a> {
    pub code: ExecStatus,
    pub output: String,
    pub bytecode: Option<String>,
    pub config: &'a Conf,
}

// TODO
pub fn exec<'a>(_content: String, config: &'a Conf) -> ExecResult<'a> {
    ExecResult {
        code: ExecStatus::Success,
        output: String::from("123"),
        bytecode: Some(String::from("CALL a [1] [0]")),
        config: config,
    }
}

pub fn handle_exec_result(res: &ExecResult, config: &Conf) {
    match &res.code {
        ExecStatus::Success => {
            out::print_execution_output(&res.output);
            if let Some(bytecode) = &res.bytecode {
                if config.print_bytecode {
                    out::print_bytecode(bytecode);
                }
            }
        }
        ExecStatus::Error(reason) => {
            out::error(reason, None);
        }
    }
}
