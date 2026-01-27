pub mod conf;
pub mod fsys;

use conf::Conf;

pub struct ExecResult {
    pub code: conf::ExecResultCode,
    pub output: String,
    pub bytecode: Option<String>,
    pub config: Conf,
}

pub fn exec(_content: String, config: &Conf) -> ExecResult {
    ExecResult {
        code: conf::ExecResultCode::Success,
        output: String::from("123"),
        bytecode: Some(String::from("CALL a [1] [0]")),
        config: config.clone(), // TODO: remove clone
    }
}

pub fn handle_exec_result(res: &ExecResult, config: &Conf) {
    match &res.code {
        conf::ExecResultCode::Success => {
            println!("{}", res.output);
            if let Some(bytecode) = &res.bytecode {
                if config.print_bytecode {
                    println!("--- bytecode start ---");
                    println!("{}", bytecode);
                    println!("--- bytecode end ---");
                }
            }
        }
        conf::ExecResultCode::Error(reason) => {
            println!("Error during execution: {}", reason);
        }
    }
}
