pub mod input;

use input::input::parse_args;

const DEFAULT_PRINT_BYTECODE: bool = false;
const FILE_EXT: &str = "eli";
const ARG_FILE_PATH: &str = "file-path";
const ARG_PRINT_BYTECODE: &str = "print-bytecode";

/**
 * #ExecutionResultCodes
 */

pub enum ExecResultCode {
    Success,
    Error(String),
}

// ===============================

#[derive(Debug, Clone)]
pub struct Conf {
    pub file_path: String,
    pub print_bytecode: bool,
}

impl Conf {
    pub fn from_cli(args: Vec<String>) -> Self {
        println!("args: {:?}", args);
        return Self { file_path: "".to_string(), print_bytecode: false }
        // let parsed_args = parse_args(
        //     &args.iter().skip(1).collect(),
        //     &vec![ARG_FILE_PATH, ARG_PRINT_BYTECODE],
        // );
        //
        // if parsed_args.len() == 0 {
        //     panic!("No arguments provided. Please provide a file path.");
        // }
        //
        // Self {
        //     file_path: parsed_args[0].value.clone(),
        //     print_bytecode: parsed_args[1]
        //         .value
        //         .parse::<bool>()
        //         .unwrap_or(DEFAULT_PRINT_BYTECODE),
        // }
    }
}

/**
 * ===============================
 * Tests
 * ===============================
 */

#[cfg(test)]
mod tests {
    #[test]
    fn should_require_filepath_arg_if_nothing_provided() {
        panic!("TODO");
    }

    #[test]
    fn should_reject_files_with_invalid_ext() {
        panic!("TODO");
    }

    #[test]
    fn should_not_accept_invalid_path() {}

    #[test]
    fn should_set_print_bytecode_to_true_if_no_value_provided() {
        panic!("TODO");
    }

    #[test]
    fn should_set_print_bytecode_to_true_if_explicitly_provided() {
        panic!("TODO");
    }

    #[test]
    fn should_set_print_bytecode_to_false_if_explicitly_provided() {
        panic!("TODO");
    }
}
