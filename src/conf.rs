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

#[derive(Debug)]
#[derive(Clone)]
pub struct Conf {
    pub file_path: String,
    pub print_bytecode: bool,
}

impl Conf {
    pub fn from_args(_args: &Vec<String>) -> Self {
        Self {
            file_path: "123".to_string(),
            print_bytecode: false,
        }
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
