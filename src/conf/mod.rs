pub mod input;

use input::input::parse_cli_args;

const FILE_EXT: &str = ".eli";

// Argument keys
const ARG_K_FILE_PATH: &str = "file-path";
const ARG_K_PRINT_BYTECODE: &str = "print-bytecode";

// Argument values
const ARG_V_TRUE: &str = "true";
const ARG_V_FALSE: &str = "false";

// ===============================

#[derive(Debug, Clone)]
pub struct Conf {
    file_path: String,
    pub print_bytecode: bool,
}

impl Conf {
    pub fn new(file_path: String, print_bytecode: bool) -> Self {
        if !Self::is_path_valid(&file_path) {
            panic!("Invalid file extension. Expected \"{}\"", FILE_EXT);
        }
        Self {
            file_path,
            print_bytecode,
        }
    }

    fn is_path_valid(path: &str) -> bool {
        path.ends_with(FILE_EXT)
    }

    fn unwrap_arg<'a, F>(name: &'a str, getter: F, default_value: Option<&str>) -> String
    where
        F: Fn() -> Option<&'a String>,
    {
        return match getter() {
            Some(path) => path.to_string(),
            None => {
                return match default_value {
                    Some(def) => def.to_string(),
                    None => panic!("\"{}\" argument is required", name),
                };
            }
        };
    }

    pub fn from_cli(args: Vec<String>) -> Self {
        if args.len() == 0 {
            panic!("No arguments provided. Please provide a file path.");
        }

        let parsed_args = parse_cli_args(&args, &vec![ARG_K_FILE_PATH, ARG_K_PRINT_BYTECODE]);

        if parsed_args.len() == 0 {
            panic!("No valid arguments provided.");
        }

        Self::new(
            Self::unwrap_arg(&ARG_K_FILE_PATH, || parsed_args.get(ARG_K_FILE_PATH), None),
            Self::unwrap_arg(
                &ARG_K_PRINT_BYTECODE,
                || parsed_args.get(ARG_K_PRINT_BYTECODE),
                Some(ARG_V_FALSE),
            ) == ARG_V_TRUE,
        )
    }

    pub fn file_path(&self) -> String {
        let path = &self.file_path;
        return path.to_string();
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
